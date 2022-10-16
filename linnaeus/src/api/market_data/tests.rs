use std::ops::Sub;
use super::*;
use crate::test_helpers::*;
use anyhow::Result;
use chrono::Utc;
use pretty_assertions::{assert_eq};
use log::info;

#[tokio::test]
async fn test_time() -> Result<()> {
    let bin = setup();
    let server_time = server_time(&bin).await?;
    info!("time is {}", server_time);
    Ok(())
}

#[tokio::test]
async fn test_system_status() -> Result<()> {
    let bin = setup();
    let ss = system_status(&bin).await?;
    info!("status is {}", ss);
    Ok(())
}

#[tokio::test]
async fn test_asset_info() -> Result<()> {
    let bin = setup();
    let params = AssetInfoParams::new(vec!["ETH".into(), "BTC".into()]);
    let ai = asset_info(&bin, &params).await?;
    info!("ETH asset info is is {:?}", ai);
    assert_eq!(ai.len(), 2);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_all_asset_info() -> Result<()> {
    let bin = setup();
    let aai = all_asset_info(&bin).await?;
    info!("all asset info is is {:?}", aai);
    Ok(())
}

#[tokio::test]
async fn test_trading_asset_pairs() -> Result<()> {
    let bin = setup();
    let mut params = TradableAssetPairsParams::default();
    params.add_pair("XXBTZUSD".to_string());
    params.add_pair("XETHXXBT".to_string());
    let tap = tradable_asset_pairs(&bin, &params).await?;
    info!("tradable asset pair info is {:?}", tap);
    assert_eq!(tap.len(), 2);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_all_trading_asset_pairs() -> Result<()> {
    let bin = setup();
    let tap = all_tradable_asset_pairs(&bin).await?;
    info!("tradable asset pair info is {:?}", tap);
    Ok(())
}


#[tokio::test]
async fn test_ticker_info() -> Result<()> {
    let bin = setup();
    let params = TickerInfoParams::new("XBTUSD".to_string());
    let ticker_info = ticker_information(&bin, &params).await?;
    info!("ticker info for XBTUSD is {:?}", ticker_info);
    assert_eq!(ticker_info.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_ohlc() -> Result<()> {
    let bin = setup();
    let since = Utc::now().sub(chrono::Duration::minutes(2));
    let params = OHLCDataParams::new("XBTUSD".to_string(), Some(Interval::OneMin), Some(since));
    let ohlc_info = ohlc(&bin, &params).await?;
    info!("ohlc info for XBTUSD is {:?}", ohlc_info);
    assert_eq!(ohlc_info.tick_data().len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_recent_trades() -> Result<()> {
    let bin = setup();
    let since = Utc::now().sub(chrono::Duration::minutes(2));
    let params = RecentTradesParams::new("XBTUSD".to_string(), Some(since));
    let recent_trades = recent_trades(&bin, &params).await?;
    info!("recent trades for XBTUSD is {:?}", recent_trades);
    assert_eq!(recent_trades.trade_data().len(), 1);
    Ok(())
}