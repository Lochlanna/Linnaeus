extern crate core;

#[cfg(test)]
mod test_utils;
pub mod messages;
pub mod error;


use std::hash::Hash;
use std::sync::Arc;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast;
use std::sync::atomic::{AtomicU64, Ordering};
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::{error, trace, warn};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message as TungstenMessage};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use url::Url;
use crate::messages::ChannelMessageWrapper;

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;


#[derive(DebugAsJson, DisplayAsJsonPretty, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct SubscriptionKey {
    channel: messages::Channel,
    pair: Option<messages::Pair>,
}

impl SubscriptionKey {
    fn new(channel: messages::Channel, pair: messages::Pair) -> Self {
        SubscriptionKey {
            channel,
            pair: Some(pair),
        }
    }

    fn without_pair(channel: messages::Channel) -> Self {
        SubscriptionKey {
            channel,
            pair: None,
        }
    }
}


fn websocket_error_is_fatal(error: &tokio_tungstenite::tungstenite::Error) -> bool {
    use tokio_tungstenite::tungstenite::Error::*;
    matches!(error, ConnectionClosed | AlreadyClosed | Io(_) | Tls(_) | Url(_) | Http(_) | HttpFormat(_))
}

#[derive(Debug)]
pub struct LinnaeusWebsocket {
    subscriptions: DashMap<SubscriptionKey, broadcast::Sender<messages::ChannelMessageWrapper>>,
    request_id: AtomicU64,
    pending_requests: DashMap<u64, tokio::sync::oneshot::Sender<messages::Event>>,
    recent_events: DashMap<messages::EventType, messages::Event>,
    writer: tokio::sync::Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungstenMessage>>,
    reader: std::sync::Mutex<Option<tokio::task::JoinHandle<ReadStream>>>,
    closer: tokio::sync::oneshot::Sender<()>
}

impl LinnaeusWebsocket {
    pub async fn new(url: Url) -> Arc<Self> {
        let (ws_stream, _) = connect_async(url.clone()).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");
        let (write, read) = ws_stream.split();

        let (close_sender, close_receiver) = tokio::sync::oneshot::channel();

        let linnaeus_websocket = Arc::new(Self {
            subscriptions: Default::default(),
            request_id: Default::default(),
            pending_requests: Default::default(),
            recent_events: Default::default(),
            writer: tokio::sync::Mutex::new(write),
            reader: Default::default(),
            closer: close_sender
        });

        let reader_handle = tokio::spawn(Self::reader(linnaeus_websocket.clone(), read, close_receiver));

        {
            let mut reader = linnaeus_websocket.reader.lock().expect("could get lock mutex");
            *reader = Some(reader_handle);
        }


        linnaeus_websocket
    }

    async fn reader(client: Arc<Self>, mut read: ReadStream, mut close_receiver: tokio::sync::oneshot::Receiver<()>) -> ReadStream {
        let client = client.as_ref();

        loop {
            let msg:Option<Result<TungstenMessage, _>> = tokio::select! {
                msg = read.next() => msg,
                _ = &mut close_receiver => {
                    return read;
                }
            };

            let Some(msg) = msg else {
                continue;
            };

            let msg = match msg {
                Ok(msg) => msg,
                Err(err) => {
                    error!("error while reading from websocket -> {}", err);
                    if websocket_error_is_fatal(&err) {
                        return read;
                    }
                    continue;
                }
            };
            let msg = match msg {
                TungstenMessage::Text(msg) => msg,
                _ => {
                    warn!("got non text based message on the websocket");
                    continue;
                }
            };

            trace!("got new json message {}", msg);

            let msg: messages::Message = match serde_json::from_str(&msg) {
                Ok(msg) => msg,
                Err(e) => {
                    error!("error while deserializing websocket message {}", e);
                    continue;
                }
            };

            match msg {
                messages::Message::ChannelMessage(channel_message) => {
                    //TODO this feels expensive...
                    let key = SubscriptionKey {
                        channel: channel_message.channel().clone(),
                        pair: Some(channel_message.pair().clone()),
                    };
                    if let Some(sub) = client.subscriptions.get(&key) {
                        if let Err(e) = sub.send(channel_message) {
                            error!("failed to send channel message over broadcast -> {}", e);
                        }
                    }
                }
                messages::Message::Event(event) => {
                    trace!("got new event message with type {}", messages::EventType::from(&event));
                    client.recent_events.insert((&event).into(), event.clone());
                    if let Some(request_id) = event.get_request_id() {
                        if let Some((_, pending)) = client.pending_requests.remove(&(request_id as u64)) {
                            if let Err(e) = pending.send(event) {
                                error!("failed to send event to pending request channel -> {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    async fn send_event(&self, event: messages::Event) -> Result<(), error::LinnaeusWebsocketError> {
        let event_json = serde_json::to_string(&event)?;
        trace!("sending json over websocket {}", event_json);
        let message = TungstenMessage::Text(event_json);
        let mut writer = self.writer.lock().await;
        writer.send(message).await?;
        Ok(())
    }

    fn next_id(&self) -> u64 {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        id
    }

    pub async fn ping(&self) -> Result<tokio::sync::oneshot::Receiver<messages::Event>, error::LinnaeusWebsocketError> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);

        let ping = messages::Event::Ping(messages::general_messages::Ping::new(id as i64));

        let (one_shot_sender, one_shot_receiver) = tokio::sync::oneshot::channel();
        self.pending_requests.insert(id, one_shot_sender);

        self.send_event(ping).await?;

        Ok(one_shot_receiver)
    }

    pub fn get_recent_event(&self, event_type: messages::EventType) -> Option<messages::Event> {
        self.recent_events.get(&event_type).map(|e| e.clone())
    }

    pub async fn subscribe(&self, channel: messages::Channel, pair: String) -> Result<broadcast::Receiver<ChannelMessageWrapper>, error::LinnaeusWebsocketError> {
        let sub_event = messages::general_messages::Subscribe::from_channel(channel.clone(), self.next_id() as i64, pair.clone());

        self.send_event(messages::Event::Subscribe(sub_event)).await?;

        //TODO This is nasty
        let sub_key = SubscriptionKey {
            channel,
            pair: Some(pair)
        };

        let entry = self.subscriptions.entry(sub_key);
        match entry {
            Entry::Occupied(entry) => {
                Ok(entry.get().subscribe())
            }
            Entry::Vacant(vacant) => {
                let (sender, receiver) = broadcast::channel(100);
                vacant.insert(sender);
                Ok(receiver)
            }
        }
    }
    
    
    pub async fn shutdown(self) {
        self.closer.send(()).expect("couldn't send shutdown command");
        let reader_join_handle;
        {
            let mut reader = self.reader.lock().expect("couldn't get lock on reader jh");
            let handle = std::mem::replace(&mut (*reader), None);
            let Some(handle) = handle else {
                warn!("couldn't get the join handle during shutdown");
                return;
            };
            reader_join_handle = handle;
        }

        let Ok(read_sink) = reader_join_handle.await else {
            warn!("couldn't get read stream from jh during shutdown");
            return;
        };
        let Ok(mut websocket) = self.writer.into_inner().reunite(read_sink) else {
            warn!("couldn't reunite sender and receiver for websocket shutdown");
            return
        };

        if let Err(err) = websocket.close(Some(CloseFrame{ code: CloseCode::Normal, reason: Default::default() })).await {
            error!("error while sending close message over websocket -> {}", err);

        }
    }
}

#[cfg(test)]
mod websocket_tests {
    use super::*;
    use std::sync::Mutex;
    use std::time::Duration;
    use log::info;
    use crate::messages::Event;
    use crate::test_utils::setup;
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_str_eq;


    static SHARED_LWS: Lazy<Mutex<Option<Arc<LinnaeusWebsocket>>>> = Lazy::new(|| {
        Default::default()
    });

    async fn get_shared_lws() -> Arc<LinnaeusWebsocket> {
        let mut shared = SHARED_LWS.lock().expect("couldn't get the lock on shared lws");
        if shared.is_none() {
            let lws = LinnaeusWebsocket::new(url::Url::parse("wss://ws.kraken.com").expect("couldn't create url")).await;
            *shared = Some(lws.clone());
            return lws;
        }
        shared.as_ref().unwrap().clone()
    }

    #[tokio::test]
    async fn test_ping() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;
        let r = lws.ping().await.expect("couldn't ping");
        let value = tokio::time::timeout(Duration::from_secs_f64(5.0), r).await
            .expect("timeout while waiting for response")
            .expect("couldn't get response from event receiver");
        match value {
            Event::Pong(p) => {
                println!("got pong with id {}", p.request_id())
            }
            _ => panic!("got the wrong event back on ping")
        }
        Ok(())
    }

    #[tokio::test]
    async fn subscribe() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;
        let mut r = lws.subscribe(messages::Channel::Ticker, "XBT/USD".into()).await.expect("couldn't subscribe");
        let Ok(value) = tokio::time::timeout(Duration::from_secs_f64(10.0), r.recv()).await else {
            panic!("timed out while waiting for response");
        };
        let Ok(value) = value else {
            panic!("error while receiving message from broadcast");
        };

        println!("Got a message! {}", value);

        Ok(())
    }

    #[tokio::test]
    async fn system_status_received() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;
        let mut event = lws.get_recent_event(messages::EventType::SystemStatus);
        let mut counter = 0;
        while event.is_none() && counter < 10 {
            tokio::time::sleep(Duration::from_secs_f64(0.1)).await;
            event = lws.get_recent_event(messages::EventType::SystemStatus);
            counter += 1;
        }
        assert!(event.is_some());

        info!("got system status after {} iterations", counter);

        let event = event.unwrap();
        let event = match event {
            Event::SystemStatus(ss) => ss,
            _ => panic!("didnt' get a system status event. Got {}", event)
        };

        assert!(matches!(event.status(), messages::general_messages::SystemStatusCode::Online));
        assert_str_eq!(event.version(), "1.9.0");
        Ok(())
    }
}