pub mod messages;


#[cfg(test)]
mod tests {
    use anyhow::bail;
    use super::*;
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_str_eq;

    fn load_test_json(name: &str) -> std::io::Result<String> {
        std::fs::read_to_string(format!("test_json/{}.json", name))
    }

    #[test]
    fn deserialize_channel_message() -> anyhow::Result<()> {
        let j = load_test_json("public/ticker")?;
        let channel_message: messages::ChannelMessageWrapper = serde_json::from_str(&j)?;
        assert_str_eq!(channel_message.pair(), "XBT/USD");
        let messages::ChannelMessage::Ticker(ticker) = channel_message.message() else {
            bail!("expected ticker type");
        };
        assert_eq!(*ticker.ask().whole_lot_volume(), 1);

        let j = load_test_json("public/ohlc-5")?;
        let channel_message: messages::ChannelMessageWrapper = serde_json::from_str(&j)?;
        assert_str_eq!(channel_message.pair(), "XBT/USD");
        let messages::ChannelMessage::OHLC(ohlc) = channel_message.message() else {
            bail!("expected ohlc type");
        };
        assert_eq!(*ohlc.count(), 2);
        match channel_message.channel() {
            messages::Channel::OHLC(interval) => assert!(matches!(interval, messages::general_messages::Interval::FiveMin)),
            _ => panic!("invalid interval. expected 5")
        }
        Ok(())
    }

    #[test]
    fn deserialize_any_message() -> anyhow::Result<()> {
        let j = load_test_json("public/ticker")?;
        let channel_message: messages::Message = serde_json::from_str(&j)?;
        match channel_message {
            messages::Message::ChannelMessage(_) => {},
            messages::Message::Event(_) => bail!("expected channel message not event")
        }

        let j = load_test_json("general/ping")?;
        let channel_message: messages::Message = serde_json::from_str(&j)?;
        match channel_message {
            messages::Message::ChannelMessage(_) => bail!("expected event got channel message"),
            messages::Message::Event(_) => { }
        }
        Ok(())
    }
}
