use std::collections::HashMap;
use derive_getters::Getters;
use derive_setters::Setters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use crate::{Deserialize, Serialize};
use derive_new::new;

pub type AccountBalances = HashMap<String, Decimal>;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new)]
pub struct TradeBalancesParams {
    asset: String
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct TradeBalances {
    ///Equivalent balance (combined balance of all currencies)
    #[serde(rename = "eb")]
    equivalent_balance: Decimal,
    ///Trade balance (combined balance of all equity currencies)
    #[serde(rename = "tb")]
    trade_balance: Decimal,
    ///Margin amount of open positions
    #[serde(rename = "m")]
    margin: Decimal,
    ///Unrealized net profit/loss of open positions
    #[serde(rename = "n")]
    net_open: Decimal,
    ///Cost basis of open positions
    #[serde(rename = "c")]
    cost_basis_open: Decimal,
    ///Current floating valuation of open positions
    #[serde(rename = "v")]
    floating_valuation_open: Decimal,
    ///Equity: trade balance + unrealized net profit/loss
    #[serde(rename = "e")]
    equity: Decimal,
    ///Free margin: Equity - initial margin (maximum margin available to open new positions)
    #[serde(rename = "mf")]
    free_margin: Decimal,
    ///Margin level: (equity / initial margin) * 100
    #[serde(rename = "ml")]
    margin_level: Option<Decimal>,
}