use super::*;
use crate::test_helpers::*;
use anyhow::Result;
use log::info;

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

#[tokio::test(flavor = "multi_thread")]
async fn test_open_orders() -> Result<()> {
    let bin = setup();
    let params = OpenOrdersParams::default();
    let open_orders = open_orders(&bin, &params).await?;
    info!("open orders are {:?}", open_orders);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_closed_orders() -> Result<()> {
    let bin = setup();
    let params = ClosedOrdersParams::default();
    let open_orders = closed_orders(&bin, &params).await?;
    info!("closed orders are {:?}", open_orders);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_query_orders() -> Result<()> {
    let bin = setup();
    let open_orders = open_orders(&bin, &OpenOrdersParams::default()).await?;
    let mut params = QueryOrderParams::default();
    let _ = params.add_transaction(open_orders.keys().next().expect("There was no open transactions").to_string());
    let queried_orders = query_orders(&bin, &params).await?;
    info!("orders are {:?}", queried_orders);
    assert_eq!(queried_orders.len(), 1);

    let closed_orders = closed_orders(&bin, &ClosedOrdersParams::default()).await?;
    let mut params = QueryOrderParams::default();
    let _ = params.add_transaction(closed_orders.closed().keys().next().expect("There was no closed transactions").to_string());
    let queried_orders = query_orders(&bin, &params).await?;
    info!("orders are {:?}", queried_orders);
    assert_eq!(queried_orders.len(), 1);

    let mut params = QueryOrderParams::default();
    let _ = params.add_transaction(open_orders.keys().next().expect("There was no open transactions").to_string());
    let _ = params.add_transaction(closed_orders.closed().keys().next().expect("There was no closed transactions").to_string());
    let queried_orders = query_orders(&bin, &params).await?;
    info!("orders are {:?}", queried_orders);
    assert_eq!(queried_orders.len(), 2);
    Ok(())
}
