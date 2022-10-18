use log::info;
use super::*;
use crate::test_helpers::*;
use anyhow::Result;

#[tokio::test(flavor = "multi_thread")]
async fn test_account_balances() -> Result<()> {
    let bin = setup();
    let ab = account_balances(&bin).await?;
    info!("account balances are {:?}", ab);
    assert!(!ab.is_empty());
    assert!(ab.contains_key("ZAUD"));
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_trade_balances() -> Result<()> {
    let bin = setup();
    let params = TradeBalancesParams::new("ZAUD".into());
    let tb = trade_balances(&bin, &params).await?;
    info!("trade balances are {:?}", tb);
    Ok(())
}