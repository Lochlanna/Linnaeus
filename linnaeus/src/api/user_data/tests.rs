use log::info;
use super::*;
use crate::test_helpers::*;
use anyhow::Result;

#[tokio::test(flavor = "multi_thread")]
async fn test_Account_balances() -> Result<()> {
    let bin = setup();
    let ab = account_balances(&bin).await?;
    info!("account balances are {:?}", ab);
    Ok(())
}