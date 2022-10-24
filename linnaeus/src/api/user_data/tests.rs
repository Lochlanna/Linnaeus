use super::*;
use crate::test_helpers::*;
use anyhow::Result;
use log::info;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_account_balances() -> Result<()> {
    let bin = setup();
    let ab = account_balances(&bin).await.error()?;
    info!("account balances are {:?}", ab);
    assert!(!ab.is_empty());
    assert!(ab.contains_key("ZAUD"));
    Ok(())
}

#[tokio::test]
async fn test_trade_balances() -> Result<()> {
    let bin = setup();
    let params = TradeBalancesParams::new("ZAUD".into());
    let tb = trade_balances(&bin, &params).await.error()?;
    info!("trade balances are {:?}", tb);
    Ok(())
}

#[tokio::test]
async fn test_open_orders() -> Result<()> {
    let bin = setup();
    let params = OpenOrdersParams::default();
    let open_orders = open_orders(&bin, &params).await.error()?;
    assert!(!open_orders.is_empty());
    info!("open orders are {:?}", open_orders);
    Ok(())
}

#[tokio::test]
async fn test_closed_orders() -> Result<()> {
    let bin = setup();
    let params = ClosedOrdersParams::default();
    let closed_orders = closed_orders(&bin, &params).await.error()?;
    info!("closed orders are {:?}", closed_orders);
    assert!(!closed_orders.closed().is_empty());
    assert_eq!(*closed_orders.count(), closed_orders.closed().len());
    Ok(())
}

#[tokio::test]
async fn test_query_orders() -> Result<()> {
    let bin = setup();
    let open_orders = open_orders(&bin, &OpenOrdersParams::default())
        .await
        .error()?;
    let mut params = QueryOrderParams::default();
    let _ = params.add_transaction(
        open_orders
            .keys()
            .next()
            .expect("There was no open transactions")
            .to_string(),
    );
    let queried_orders = query_orders(&bin, &params).await.error()?;
    info!("orders are {:?}", queried_orders);
    assert_eq!(queried_orders.len(), 1);

    let closed_orders = closed_orders(&bin, &ClosedOrdersParams::default())
        .await
        .error()?;
    let mut params = QueryOrderParams::default();
    let _ = params.add_transaction(
        closed_orders
            .closed()
            .keys()
            .next()
            .expect("There was no closed transactions")
            .to_string(),
    );
    let queried_orders = query_orders(&bin, &params).await.error()?;
    info!("orders are {:?}", queried_orders);
    assert_eq!(queried_orders.len(), 1);

    let mut params = QueryOrderParams::default();
    let _ = params.add_transaction(
        open_orders
            .keys()
            .next()
            .expect("There was no open transactions")
            .to_string(),
    );
    let _ = params.add_transaction(
        closed_orders
            .closed()
            .keys()
            .next()
            .expect("There was no closed transactions")
            .to_string(),
    );
    let queried_orders = query_orders(&bin, &params).await.error()?;
    info!("orders are {:?}", queried_orders);
    assert_eq!(queried_orders.len(), 2);
    Ok(())
}

#[tokio::test]
async fn test_trade_history() -> Result<()> {
    let bin = setup();
    let params = TradeHistoryParams::default();
    let trade_history = trade_history(&bin, &params).await.error()?;
    info!("trade history is  are {:?}", trade_history);
    Ok(())
}

#[tokio::test]
async fn test_trade_info() -> Result<()> {
    let bin = setup();
    let trade_history = trade_history(&bin, &TradeHistoryParams::default())
        .await
        .error()?;
    let mut params = QueryTradeInfoParams::default();
    params.add_transaction(
        trade_history
            .trades()
            .keys()
            .next()
            .expect("There was no trades!"),
    )?;
    let trade_info = query_trade_info(&bin, &params).await.error()?;
    info!("trade info is {:?}", trade_info);
    Ok(())
}

#[tokio::test]
async fn test_open_positions() -> Result<()> {
    let bin = setup();
    let params = OpenPositionParams::default();
    let trade_history = open_positions(&bin, &params).await.error()?;
    info!("open positions are are {:?}", trade_history);
    Ok(())
}

#[tokio::test]
async fn test_ledger_info() -> Result<()> {
    let bin = setup();
    let params = LedgerInfoParams::default();
    let ledger_info = get_ledger_info(&bin, &params).await.error()?;
    info!("ledger info is {:?}", ledger_info);
    Ok(())
}

#[tokio::test]
async fn test_query_ledger_info() -> Result<()> {
    let bin = setup();
    let params = LedgerInfoParams::default();
    let ledger_info = get_ledger_info(&bin, &params).await.error()?;

    let mut params = QueryLedgerParams::default();
    params.add_ledger_id(
        ledger_info
            .ledger()
            .keys()
            .next()
            .expect("there were no ledgers!"),
    );
    let ledger_info = query_ledger(&bin, &params).await.error()?;
    info!("ledger info is {:?}", ledger_info);
    Ok(())
}
