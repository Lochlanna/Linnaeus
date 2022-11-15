extern crate core;

pub mod error;
pub mod messages;
#[cfg(test)]
mod test_utils;

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;

use crate::messages::{Channel, ChannelMessageWrapper, Event, EventType};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::{error, info, trace, warn};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message as TungstenMessage};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

fn websocket_error_is_fatal(error: &tokio_tungstenite::tungstenite::Error) -> bool {
    use tokio_tungstenite::tungstenite::Error::*;
    matches!(
        error,
        ConnectionClosed | AlreadyClosed | Io(_) | Tls(_) | Url(_) | Http(_) | HttpFormat(_)
    )
}

#[derive(Debug)]
pub struct LinnaeusWebsocket {
    subscriptions: DashMap<u64, broadcast::Sender<ChannelMessageWrapper>, ahash::RandomState>,
    request_id: AtomicU64,
    pending_requests: DashMap<u64, tokio::sync::oneshot::Sender<Event>, ahash::RandomState>,
    recent_events: DashMap<EventType, Event, ahash::RandomState>,
    writer:
        tokio::sync::Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungstenMessage>>,
    reader: tokio::sync::Mutex<Option<tokio::task::JoinHandle<ReadStream>>>,
    closer: tokio::sync::oneshot::Sender<()>,
}

impl LinnaeusWebsocket {
    pub async fn new(url: &str) -> Result<Arc<Self>, error::LinnaeusWebsocketError> {
        if !url.starts_with("wss://") {
            return Err(error::LinnaeusWebsocketError::Url {
                reason: "websocket url must start with \"wss://\"",
            });
        }
        let url = url::Url::parse(url)?;

        let (ws_stream, _) = connect_async(url.clone()).await?;
        info!("WebSocket handshake has been successfully completed");
        let (write, mut read) = ws_stream.split();

        let (close_sender, close_receiver) = tokio::sync::oneshot::channel();

        let linnaeus_websocket = Arc::new(Self {
            subscriptions: Default::default(),
            request_id: Default::default(),
            pending_requests: Default::default(),
            recent_events: Default::default(),
            writer: tokio::sync::Mutex::new(write),
            reader: Default::default(),
            closer: close_sender,
        });

        let online = linnaeus_websocket.wait_for_system_status(&mut read).await;

        if !online {
            return Err(error::LinnaeusWebsocketError::KrakenOffline);
        }

        let reader_handle = tokio::spawn(Self::reader(
            linnaeus_websocket.clone(),
            read,
            close_receiver,
        ));

        {
            let mut reader = linnaeus_websocket.reader.lock().await;
            *reader = Some(reader_handle);
        }

        Ok(linnaeus_websocket)
    }

    async fn wait_for_system_status(self: &Arc<Self>, read: &mut ReadStream) -> bool {
        //TODO configurable timeout
        let timer = tokio::time::timeout(Duration::from_secs_f64(10.0), async {
            while let Some(msg) = read.next().await {
                let Ok(msg) = msg else {
                    return false;
                };

                let TungstenMessage::Text(msg) = msg else {
                    return false;
                };

                let event: Event = match serde_json::from_str(&msg) {
                    Ok(event) => event,
                    Err(_) => return false,
                };

                match &event {
                    Event::Heartbeat => continue,
                    Event::SystemStatus(ss) => {
                        info!("got system status {:?}", ss);
                        let online = matches!(
                            ss.status(),
                            messages::general_messages::SystemStatusCode::Online
                        );
                        self.recent_events.insert(EventType::SystemStatus, event);
                        return online;
                    }
                    _ => return false,
                }
            }
            false
        })
        .await;

        timer.unwrap_or(false)
    }

    async fn reader(
        client: Arc<Self>,
        mut read: ReadStream,
        mut close_receiver: tokio::sync::oneshot::Receiver<()>,
    ) -> ReadStream {
        let client = client.as_ref();

        loop {
            let msg: Option<Result<TungstenMessage, _>> = tokio::select! {
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
                    let key = channel_message.get_channel_identifier();
                    if let Some(sub) = client.subscriptions.get(&key) {
                        if let Err(e) = sub.send(channel_message) {
                            error!("failed to send channel message over broadcast -> {}", e);
                        }
                    }
                }
                messages::Message::Event(event) => {
                    trace!(
                        "got new event message with type {}",
                        messages::EventType::from(&event)
                    );
                    client.recent_events.insert((&event).into(), event.clone());
                    if let Some(request_id) = event.get_request_id() {
                        if let Some((_, pending)) =
                            client.pending_requests.remove(&(request_id as u64))
                        {
                            if let Err(e) = pending.send(event) {
                                error!("failed to send event to pending request channel -> {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    async fn send_event(&self, event: Event) -> Result<(), error::LinnaeusWebsocketError> {
        let event_json = serde_json::to_string(&event)?;
        trace!("sending json over websocket {}", event_json);
        let message = TungstenMessage::Text(event_json);
        let mut writer = self.writer.lock().await;
        writer.send(message).await?;
        Ok(())
    }

    fn next_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    pub async fn ping(
        &self,
    ) -> Result<tokio::sync::oneshot::Receiver<Event>, error::LinnaeusWebsocketError> {
        let id = self.next_id();

        let ping = Event::Ping(messages::general_messages::Ping::new(id as i64));

        let (one_shot_sender, one_shot_receiver) = tokio::sync::oneshot::channel();
        self.pending_requests.insert(id, one_shot_sender);

        self.send_event(ping).await?;

        Ok(one_shot_receiver)
    }

    pub fn get_recent_event(&self, event_type: EventType) -> Option<Event> {
        self.recent_events.get(&event_type).map(|e| e.clone())
    }

    pub async fn subscribe(
        &self,
        sub_event: messages::general_messages::Subscribe,
    ) -> Result<Vec<broadcast::Receiver<ChannelMessageWrapper>>, error::LinnaeusWebsocketError>
    {
        let channel = Channel::from(sub_event.subscription());

        let keys = sub_event.pair().as_ref().map_or(
            vec![channel.generate_identifier_no_pair()],
            |pairs| {
                pairs
                    .iter()
                    .map(|pair| channel.generate_identifier(pair))
                    .collect()
            },
        );

        self.send_event(Event::Subscribe(sub_event)).await?;

        let mut receivers = Vec::with_capacity(keys.len());

        for key in keys {
            match self.subscriptions.entry(key) {
                Entry::Occupied(entry) => {
                    receivers.push(entry.get().subscribe());
                }
                Entry::Vacant(vacant) => {
                    let (sender, receiver) = broadcast::channel(100);
                    vacant.insert(sender);
                    receivers.push(receiver)
                }
            }
        }

        Ok(receivers)
    }

    pub async fn shutdown(self) {
        self.closer
            .send(())
            .expect("couldn't send shutdown command");
        let reader_join_handle;
        {
            let mut reader = self.reader.lock().await;
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
            return;
        };

        if let Err(err) = websocket
            .close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: Default::default(),
            }))
            .await
        {
            error!(
                "error while sending close message over websocket -> {}",
                err
            );
        }
    }
}

#[cfg(test)]
mod websocket_tests {
    use super::*;
    use crate::messages::general_messages::{Depth, Interval};
    use crate::messages::Event;
    use crate::test_utils::setup;
    use log::info;
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_str_eq;
    use std::time::Duration;
    use tokio::sync::Mutex;

    static SHARED_LWS: Lazy<Mutex<Option<Arc<LinnaeusWebsocket>>>> = Lazy::new(|| Mutex::new(None));

    async fn get_shared_lws() -> Arc<LinnaeusWebsocket> {
        let mut shared = SHARED_LWS.lock().await;
        info!("got lock");
        if shared.is_none() {
            info!("is none");
            let lws = LinnaeusWebsocket::new("wss://ws.kraken.com")
                .await
                .expect("Couldn't create lws for testing");
            *shared = Some(lws.clone());
            return lws;
        } else {
            info!("is not none");
        }
        shared.as_ref().unwrap().clone()
    }

    #[tokio::test]
    async fn test_ping() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;
        let r = lws.ping().await.expect("couldn't ping");
        let value = tokio::time::timeout(Duration::from_secs_f64(5.0), r)
            .await
            .expect("timeout while waiting for response")
            .expect("couldn't get response from event receiver");
        match value {
            Event::Pong(p) => {
                println!("got pong with id {}", p.request_id())
            }
            _ => panic!("got the wrong event back on ping"),
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_subscribe_ticker() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;

        let sub_req = messages::general_messages::Subscribe::new(Channel::Ticker)
            .with_pair("XBT/USD".into())
            .with_pair("XBT/EUR".into());
        let mut receivers = lws.subscribe(sub_req).await.expect("couldn't subscribe");
        assert_eq!(receivers.len(), 2);

        let expected_pairs = vec!["XBT/USD", "XBT/EUR"];

        for (receiver, expected_pair) in receivers.iter_mut().zip(expected_pairs) {
            let Ok(value) = tokio::time::timeout(Duration::from_secs_f64(5.0), receiver.recv()).await else {
                panic!("timed out while waiting for response");
            };
            let Ok(value) = value else {
                panic!("error while receiving message from broadcast");
            };

            println!("Got a message! {}", value);
            assert_str_eq!(
                value.pair().as_ref().expect("ticker doesn't have pair"),
                expected_pair
            );

            assert!(matches!(value.channel(), Channel::Ticker));
        }

        Ok(())
    }

    #[tokio::test]
    async fn multi_subscribe_ohlc() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;

        let sub_req = messages::general_messages::Subscribe::new(Channel::OHLC(Interval::FiveMin))
            .with_pair("XBT/USD".into())
            .with_pair("XBT/EUR".into());
        let mut receivers = lws.subscribe(sub_req).await.expect("couldn't subscribe");
        assert_eq!(receivers.len(), 2);

        let expected_pairs = vec!["XBT/USD", "XBT/EUR"];

        for (receiver, expected_pair) in receivers.iter_mut().zip(expected_pairs) {
            let Ok(value) = tokio::time::timeout(Duration::from_secs_f64(5.0), receiver.recv()).await else {
                panic!("timed out while waiting for response");
            };
            let Ok(value) = value else {
                panic!("error while receiving message from broadcast");
            };

            println!("Got a message! {}", value);
            assert_str_eq!(
                value.pair().as_ref().expect("ticker doesn't have pair"),
                expected_pair
            );

            assert!(matches!(value.channel(), Channel::OHLC(Interval::FiveMin)));
        }

        Ok(())
    }

    #[tokio::test]
    async fn multi_subscribe_book() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;

        let sub_req = messages::general_messages::Subscribe::new(Channel::Book(Depth::Ten))
            .with_pair("XBT/USD".into())
            .with_pair("XBT/EUR".into());
        let mut receivers = lws.subscribe(sub_req).await.expect("couldn't subscribe");
        assert_eq!(receivers.len(), 2);

        let expected_pairs = vec!["XBT/USD", "XBT/EUR"];

        for (receiver, expected_pair) in receivers.iter_mut().zip(expected_pairs) {
            let Ok(value) = tokio::time::timeout(Duration::from_secs_f64(15.0), receiver.recv()).await else {
                panic!("timed out while waiting for response");
            };
            let Ok(value) = value else {
                panic!("error while receiving message from broadcast");
            };

            println!("Got a message! {}", value);
            assert_str_eq!(
                value.pair().as_ref().expect("ticker doesn't have pair"),
                expected_pair
            );

            assert!(matches!(value.channel(), Channel::Book(Depth::Ten)));
        }

        Ok(())
    }

    #[tokio::test]
    async fn multi_subscribe_spread() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;

        let sub_req = messages::general_messages::Subscribe::new(Channel::Spread)
            .with_pair("XBT/USD".into())
            .with_pair("XBT/EUR".into());
        let mut receivers = lws.subscribe(sub_req).await.expect("couldn't subscribe");
        assert_eq!(receivers.len(), 2);

        let expected_pairs = vec!["XBT/USD", "XBT/EUR"];

        for (receiver, expected_pair) in receivers.iter_mut().zip(expected_pairs) {
            let Ok(value) = tokio::time::timeout(Duration::from_secs_f64(5.0), receiver.recv()).await else {
                panic!("timed out while waiting for response");
            };
            let Ok(value) = value else {
                panic!("error while receiving message from broadcast");
            };

            println!("Got a message! {}", value);
            assert_str_eq!(
                value.pair().as_ref().expect("ticker doesn't have pair"),
                expected_pair
            );

            assert!(matches!(value.channel(), Channel::Spread));
        }

        Ok(())
    }

    #[tokio::test]
    async fn system_status_received() -> anyhow::Result<()> {
        setup();
        let lws = get_shared_lws().await;
        let event = lws.get_recent_event(EventType::SystemStatus);
        let Some(event) = event else {
            panic!("couldn't get a system status event");
        };

        let event = match event {
            Event::SystemStatus(ss) => ss,
            _ => panic!("didnt' get a system status event. Got {}", event),
        };

        assert!(matches!(
            event.status(),
            messages::general_messages::SystemStatusCode::Online
        ));
        assert_str_eq!(event.version(), "1.9.0");
        Ok(())
    }
}
