use crate::{Deserialize, Serialize};
use chrono::Utc;
use derive_getters::Getters;
use derive_new::new;
use derive_setters::Setters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde_with::formats::CommaSeparator;
use serde_with::{
    serde_as, skip_serializing_none, DefaultOnError, StringWithSeparator, TimestampSeconds,
    TimestampSecondsWithFrac,
};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use strum::Display as EnumDisplay;
use strum::EnumString;
use thiserror::Error;

pub type AccountBalances = HashMap<String, Decimal>;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new, Clone)]
pub struct TradeBalancesParams {
    asset: String,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
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

#[skip_serializing_none]
#[derive(
    Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new, Default, Clone,
)]
pub struct OpenOrdersParams {
    trades: bool,
    userref: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Open,
    Closed,
    Canceled,
    Expired,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    Last,
    Index,
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Last
    }
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderMiscInfo {
    #[strum(serialize = "stopped")]
    Stopped,
    #[strum(serialize = "touched")]
    Touched,
    #[strum(serialize = "liquidated")]
    Liquidated,
    #[strum(serialize = "partial")]
    Partial,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderFlags {
    ///post-only order (available when ordertype = limit)
    #[strum(serialize = "post")]
    Post,
    ///prefer fee in base currency (default if selling)
    #[strum(serialize = "fcib")]
    Fcib,
    ///prefer fee in quote currency (default if buying, mutually exclusive with fcib)
    #[strum(serialize = "fciq")]
    Fciq,
    ///disable market price protection for market orders
    #[strum(serialize = "nompp")]
    Nompp,
    ///order volume expressed in quote currency. This is supported only for market orders.
    #[strum(serialize = "viqc")]
    Viqc,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct OrderDescription {
    pair: kraken_enums::TradeablePair,
    #[serde(rename = "type")]
    side: Side,
    order_type: Option<OrderType>,
    price: Decimal,
    #[serde(rename = "price2")]
    secondary_price: Decimal,
    #[serde_as(deserialize_as = "DefaultOnError")]
    leverage: Option<Decimal>,
    #[serde(rename = "order")]
    order_description: String,
    #[serde(rename = "close")]
    conditional_close_description: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct OrderBase {
    #[serde(rename = "refid")]
    referral_order_transaction_id: Option<String>,
    status: OrderStatus,
    #[serde(rename = "opentm")]
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    open_time: chrono::DateTime<Utc>,
    #[serde(rename = "starttm")]
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    start_time: chrono::DateTime<Utc>,
    #[serde(rename = "expiretm")]
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    expire_time: chrono::DateTime<Utc>,
    #[serde(rename = "descr")]
    description: OrderDescription,
    #[serde(rename = "vol")]
    volume: Decimal,
    #[serde(rename = "vol_exec")]
    vol_executed: Decimal,
    cost: Decimal,
    fee: Decimal,
    price: Decimal,
    #[serde(rename = "stopprice")]
    stop_price: Decimal,
    #[serde(rename = "limitprice")]
    limit_price: Decimal,
    #[serde(default)]
    trigger: Trigger,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, OrderMiscInfo>")]
    misc: Vec<OrderMiscInfo>,
    #[serde(rename = "oflags")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, OrderFlags>")]
    order_flags: Vec<OrderFlags>,
    #[serde(default)]
    trades: Vec<String>,
}

pub type OpenOrder = OrderBase;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
pub struct OpenOrdersWrapper {
    pub open: HashMap<String, OpenOrder>,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TimeType {
    Open,
    Close,
    Both,
}
impl Default for TimeType {
    fn default() -> Self {
        Self::Both
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(
    Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new, Default, Clone,
)]
pub struct ClosedOrdersParams {
    trades: bool,
    userref: Option<i32>,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    start: Option<chrono::DateTime<Utc>>,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    end: Option<chrono::DateTime<Utc>>,
    offset: Option<usize>,
    #[serde(rename = "closetime")]
    close_time: TimeType,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct ClosedOrder {
    #[serde(flatten)]
    order: OrderBase,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    close_time: chrono::DateTime<Utc>,
    reason: Option<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct ClosedOrders {
    closed: HashMap<String, OrderBase>,
    count: usize,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Default, Clone)]
pub struct QueryOrderParams {
    trades: bool,
    userref: Option<i32>,
    #[serde(rename = "txid")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    transaction_ids: Vec<String>,
}

#[derive(Debug, Error)]
#[error("{name} can only have {num} transaction ID's at a time")]
pub struct TooManyTransactionsError {
    name: String,
    num: usize,
}
impl TooManyTransactionsError {
    pub fn new(name: &str, num: usize) -> Self {
        Self {
            name: name.to_string(),
            num,
        }
    }
}

impl QueryOrderParams {
    pub fn validate(&self) -> bool {
        self.transaction_ids.len() < 50 && !self.transaction_ids.is_empty()
    }
    pub fn add_transaction(&mut self, id: String) -> Result<(), TooManyTransactionsError> {
        const MAX_NUM_TRANSACTIONS: usize = 50;
        if self.transaction_ids.len() == MAX_NUM_TRANSACTIONS {
            return Err(TooManyTransactionsError::new(
                "query order params",
                MAX_NUM_TRANSACTIONS,
            ));
        }
        self.transaction_ids.push(id);
        Ok(())
    }
    pub fn with_transactions(transaction_ids: Vec<String>) -> Self {
        Self {
            trades: false,
            userref: None,
            transaction_ids,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
#[serde(untagged)]
pub enum Order {
    Closed(ClosedOrder),
    Open(OpenOrder),
}

pub type Orders = HashMap<String, Order>;

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
pub enum TradeType {
    #[serde(rename = "all")]
    #[strum(serialize = "all")]
    All,
    #[serde(rename = "any position")]
    #[strum(serialize = "any position")]
    AnyPosition,
    #[serde(rename = "closed position")]
    #[strum(serialize = "closed position")]
    ClosedPosition,
    #[serde(rename = "closing position")]
    #[strum(serialize = "closing position")]
    ClosingPosition,
    #[serde(rename = "no position")]
    #[strum(serialize = "no position")]
    NoPosition,
}

impl Default for TradeType {
    fn default() -> Self {
        Self::All
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(
    Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Default, Clone, new,
)]
pub struct TradeHistoryParams {
    #[serde(rename = "type")]
    trade_type: TradeType,
    trades: bool,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    start: Option<chrono::DateTime<Utc>>,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    end: Option<chrono::DateTime<Utc>>,
    #[serde(rename = "ofs")]
    offset: usize,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TradeMiscInfo {
    #[strum(serialize = "closing")]
    Closing,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PositionStatus {
    Open,
    Closed,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Trade {
    #[serde(rename = "ordertxid")]
    order_id: String,
    pair: kraken_enums::TradeablePair,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    time: chrono::DateTime<Utc>,
    #[serde(rename = "type")]
    side: Side,
    #[serde(rename = "ordertype")]
    order_type: OrderType,
    price: Decimal,
    cost: Decimal,
    fee: Decimal,
    #[serde(rename = "vol")]
    volume: Decimal,
    margin: Decimal,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, TradeMiscInfo>")]
    #[serde(default)]
    misc: Vec<TradeMiscInfo>,
    #[serde(rename = "posstatus")]
    position_status: Option<PositionStatus>,
    #[serde(rename = "cprice")]
    close_price: Option<Decimal>,
    #[serde(rename = "ccost")]
    close_cost: Option<Decimal>,
    #[serde(rename = "cfee")]
    close_fee: Option<Decimal>,
    #[serde(rename = "cvol")]
    close_volume: Option<Decimal>,
    #[serde(rename = "cmargin")]
    close_margin: Option<Decimal>,
    net: Option<Decimal>,
    #[serde(default)]
    trades: Vec<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct TradeHistory {
    trades: HashMap<String, Trade>,
    count: usize,
}

#[serde_as]
#[skip_serializing_none]
#[derive(
    Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Default, Clone, new,
)]
pub struct QueryTradeInfoParams {
    #[serde(rename = "txid")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    transaction_ids: Vec<String>,
    trades: bool,
}

impl QueryTradeInfoParams {
    pub fn add_transaction(&mut self, txn_id: &str) -> Result<(), TooManyTransactionsError> {
        const MAX_NUM_TRANSACTIONS: usize = 20;
        if self.transaction_ids.len() == MAX_NUM_TRANSACTIONS {
            return Err(TooManyTransactionsError::new(
                "query trade info",
                MAX_NUM_TRANSACTIONS,
            ));
        }
        self.transaction_ids.push(txn_id.to_string());
        Ok(())
    }
}

pub type TradeInfo = HashMap<String, Trade>;

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ConsolidationType {
    Market,
}
impl Default for ConsolidationType {
    fn default() -> Self {
        ConsolidationType::Market
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Clone)]
pub struct OpenPositionParams {
    #[serde(rename = "txid")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    transaction_ids: Vec<String>,
    #[serde(rename = "docalcs")]
    do_profit_loss_calcualtions: bool,
    consolidation: Option<ConsolidationType>,
}

impl Default for OpenPositionParams {
    fn default() -> Self {
        Self {
            transaction_ids: Default::default(),
            do_profit_loss_calcualtions: Default::default(),
            consolidation: Some(ConsolidationType::default()),
        }
    }
}

impl OpenPositionParams {
    pub fn add_transaction(&mut self, txn_id: &str) {
        self.transaction_ids.push(txn_id.to_string());
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct OpenPosition {
    #[serde(rename = "ordertxid")]
    order_id: String,
    pair: kraken_enums::TradeablePair,
    #[serde(rename = "posstatus")]
    position_status: Option<PositionStatus>,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    time: chrono::DateTime<Utc>,
    #[serde(rename = "type")]
    side: Side,
    cost: Decimal,
    fee: Decimal,
    #[serde(rename = "vol")]
    volume: Decimal,
    #[serde(rename = "vol_closed")]
    volume_closed: Decimal,
    margin: Decimal,
    value: Option<Decimal>,
    net: Option<Decimal>,
    terms: String,
    #[serde(rename = "rollovertm")]
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    rollover_time: chrono::DateTime<Utc>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde(default)]
    misc: Vec<String>,
    #[serde(rename = "oflags")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde(default)]
    order_flags: Vec<String>,
}

pub type OpenPositions = HashMap<String, OpenPosition>;

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LedgerType {
    All,
    Deposit,
    Withdrawal,
    Trade,
    Margin,
    Rollover,
    Credit,
    Transfer,
    Settled,
    Staking,
    Sale,
    Spend,
    Receive,
    Adjustment,
}

impl Default for LedgerType {
    fn default() -> Self {
        Self::All
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Clone)]
pub struct LedgerInfoParams {
    #[serde(rename = "asset")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, kraken_enums::Currency>")]
    assets: Vec<kraken_enums::Currency>,
    #[serde(rename = "aclass")]
    class: String,
    #[serde(rename = "type")]
    ledger_type: LedgerType,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    start: Option<chrono::DateTime<Utc>>,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    end: Option<chrono::DateTime<Utc>>,
    #[serde(rename = "ofs")]
    offset: usize,
}

impl Default for LedgerInfoParams {
    fn default() -> Self {
        Self {
            assets: vec![],
            class: "currency".into(),
            ledger_type: Default::default(),
            start: None,
            end: None,
            offset: 0,
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Ledger {
    #[serde(rename = "refid")]
    reference_id: String,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    open_time: chrono::DateTime<Utc>,
    #[serde(rename = "type")]
    ledger_type: LedgerType,
    sub_type: String,
    #[serde(rename = "aclass")]
    class: kraken_enums::CurrencyType,
    asset: kraken_enums::Currency,
    amount: Decimal,
    fee: Decimal,
    balance: Decimal,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct LedgerInfo {
    ledger: HashMap<String, Ledger>,
    count: usize,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Clone, Default)]
pub struct QueryLedgerParams {
    #[serde(rename = "id")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    ledger_ids: Vec<String>,
    trades: bool,
}

impl QueryLedgerParams {
    pub fn add_ledger_id(&mut self, ledger_id: &str) {
        self.ledger_ids.push(ledger_id.to_string());
    }
}

pub type Ledgers = HashMap<String, Ledger>;
