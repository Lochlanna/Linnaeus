use std::ops::Sub;
use super::*;
use crate::test_helpers::*;
use anyhow::Result;
use chrono::Utc;
use pretty_assertions::{assert_eq};
use log::info;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use kraken_enums::{Pair, TradeablePair};
use kraken_enums::Currency::{BTC, USD, ETH};

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
    let params = AssetInfoParams::new(vec![ETH, BTC]);
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
    params.add_pair(TradeablePair::XXBTZUSD);
    params.add_pair(TradeablePair::XETHXXBT);
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
#[ignore]
async fn list_all_trading_asset_pairs() -> Result<()> {
    let bin = setup();
    let taps = all_tradable_asset_pairs(&bin).await?;
    let mut summary_data = Vec::with_capacity(taps.len());
    for (name, info) in &taps {
        summary_data.push(format!("{}\t{}\t{}\t{}", name, info.base_asset_id(), info.quote_asset_id(), info.alt_name()));
    }
    let summary = summary_data.join("\n");
    let mut out_file = File::create("tap_summary.txt").await?;
    out_file.write_all(summary.as_bytes()).await?;
    Ok(())
}


#[tokio::test]
async fn test_ticker_info() -> Result<()> {
    let bin = setup();
    let params = TickerInfoParams::new(Pair::new(BTC, USD).try_into().expect("couldn't get tradable pair from pair"));
    let ticker_info = ticker_information(&bin, &params).await?;
    info!("ticker info for XBTUSD is {:?}", ticker_info);
    assert_eq!(ticker_info.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_ohlc() -> Result<()> {
    let bin = setup();
    let since = Utc::now().sub(chrono::Duration::minutes(2));
    let params = OHLCDataParams::new(TradeablePair::XBTUSDT, Some(Interval::OneMin), Some(since));
    let ohlc_info = ohlc(&bin, &params).await?;
    info!("ohlc info for XBTUSD is {:?}", ohlc_info);
    assert_eq!(ohlc_info.tick_data().len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_order_book() -> Result<()> {
    let bin = setup();
    let params = OrderBookParams::new(TradeablePair::XBTUSDT);
    let recent_trades = order_book(&bin, &params).await?;
    info!("order book for XBTUSD is {:?}", recent_trades);
    assert_eq!(recent_trades.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_recent_trades() -> Result<()> {
    let bin = setup();
    let since = Utc::now().sub(chrono::Duration::minutes(1));
    let params = RecentTradesParams::new(TradeablePair::XBTUSDT, Some(since));
    let recent_trades = recent_trades(&bin, &params).await?;
    info!("recent trades for XBTUSD is {:?}", recent_trades);
    assert_eq!(recent_trades.trade_data().len(), 1);
    Ok(())
}