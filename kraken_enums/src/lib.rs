// Updated 17/10/22

mod tradeable_pairs;

pub use tradeable_pairs::*;

use serde::{Serialize, Deserialize};
use strum::{IntoStaticStr, EnumString};
use strum::Display as StrumDisplay;

#[derive(Debug, Serialize, Deserialize, EnumString, IntoStaticStr, Clone, StrumDisplay, PartialEq, Eq)]
pub enum Currency {
    USD,
    EUR,
    CAD,
    AUD,
    GBP,
    CHF,
    JPY,
    AED,


    ZRX,
    //1inch
    #[strum(serialize = "1INCH")]
    #[serde(rename = "1INCH")]
    OneInch,
    AAVE,
    GHST,
    ACA,
    AGLD,
    AKT,
    ALCX,
    ACH,
    ALGO,
    TLM,
    ALPHA,
    AIR,
    ADX,
    FORTH,
    ANKR,
    APE,
    API3,
    ANT,
    ARPA,
    ASTR,
    AUDIO,
    REP,
    REPV2,
    AVAX,
    AXS,
    BADGER,
    BAL,
    BNT,
    BAND,
    BOND,
    BAT,
    BSX,
    BICO,
    BNC,
    BTC,
    BCH,
    BIT,
    BTT,
    BLZ,
    BOBA,
    FIDA,
    ADA,
    CTSI,
    CELR,
    CFG,
    XCN,
    LINK,
    CHZ,
    CHR,
    CVC,
    COMP,
    C98,
    CVX,
    ATOM,
    COTI,
    CQT,
    CSM,
    CRV,
    DAI,
    DASH,
    MANA,
    DENT,
    DOGE,
    DYDX,
    EGLD,
    EWT,
    ENJ,
    MLN,
    EOS,
    ETHW,
    ETH,
    ETC,
    ENS,
    FTM,
    FET,
    FIL,
    FLOW,
    FXS,
    GALA,
    GAL,
    GARI,
    MV,
    GTC,
    GNO,
    GST,
    FARM,
    ICX,
    IDEX,
    RLC,
    IMX,
    INJ,
    TEER,
    INTR,
    ICP,
    JASMY,
    JUNO,
    KAR,
    KAVA,
    ROOK,
    KEEP,
    KP3R,
    KILT,
    KIN,
    KINT,
    KSM,
    KNC,
    LDO,
    LCX,
    LSK,
    LTC,
    LPT,
    LRC,
    MKR,
    MNGO,
    MSOL,
    POND,
    MASK,
    MC,
    MINA,
    MIR,
    XMR,
    GLMR,
    MOVR,
    MULTI,
    MXC,
    ALICE,
    NANO,
    NEAR,
    NODL,
    NMR,
    NYM,
    OCEAN,
    OMG,
    ORCA,
    OXT,
    OGN,
    OXY,
    PARA,
    PAXG,
    PERP,
    PHA,
    PLA,
    DOT,
    POLS,
    MATIC,
    POWR,
    PSTAKE,
    QTUM,
    QNT,
    RAD,
    RARI,
    RAY,
    REN,
    RNDR,
    REQ,
    XRP,
    XRT,
    RPL,
    RBC,
    SBR,
    SAMO,
    SCRT,
    KEY,
    SRM,
    SHIB,
    SDN,
    SC,
    SOL,
    SGB,
    SPELL,
    FIS,
    ATLAS,
    POLIS,
    STG,
    XLM,
    STEP,
    GMT,
    STORJ,
    SUPER,
    RARE,
    SUSHI,
    SXP,
    SYN,
    SNX,
    TBTC,
    LUNA2,
    LUNA,
    UST,
    TVK,
    USDT,
    XTZ,
    GRT,
    SAND,
    RUNE,
    T,
    TOKE,
    TRIBE,
    TRX,
    TRU,
    UNFI,
    UNI,
    UMA,
    USDC,
    WAVES,
    WOO,
    WBTC,
    WETH,
    YFI,
    YGG,
    ZEC,
}

impl Currency {
    fn to_full_name(&self) -> &'static str {
        match self {
            Currency::USD => "United States Dollar",
            Currency::EUR => "Euro",
            Currency::CAD => "Canadian Dollars",
            Currency::AUD => "Australian Dollars",
            Currency::GBP => "Great British Pound",
            Currency::CHF => "Swiss Franc",
            Currency::JPY => "Japanese Yen",
            Currency::AED => "United Arab Emirates Durham",
            Currency::ZRX => "ZRX",
            Currency::OneInch => "1inch",
            Currency::AAVE => "Aave",
            Currency::GHST => "Aavegotchi",
            Currency::ACA => "Acala",
            Currency::AGLD => "Adventure Gold",
            Currency::AKT => "Akash",
            Currency::ALCX => "Alchemix",
            Currency::ACH => "Alchemy Pay",
            Currency::ALGO => "Algorand",
            Currency::TLM => "Alien Worlds",
            Currency::ALPHA => "Alpha Venture DAO",
            Currency::AIR => "Altair",
            Currency::ADX => "Ambire AdEx",
            Currency::FORTH => "Ampleforth Governance Token",
            Currency::ANKR => "Ankr",
            Currency::APE => "ApeCoin",
            Currency::API3 => "API3",
            Currency::ANT => "Aragon",
            Currency::ARPA => "Arpa Chain",
            Currency::ASTR => "Astar",
            Currency::AUDIO => "Audius",
            Currency::REP => "Augur",
            Currency::REPV2 => "Augur v2",
            Currency::AVAX => "Avalanche",
            Currency::AXS => "Axie Infinity",
            Currency::BADGER => "Badger DAO",
            Currency::BAL => "Balancer",
            Currency::BNT => "Bancor",
            Currency::BAND => "Band Protocol",
            Currency::BOND => "Barnbridge",
            Currency::BAT => "Basic Attention Token",
            Currency::BSX => "Basilisk",
            Currency::BICO => "Biconomy",
            Currency::BNC => "Bifrost",
            Currency::BTC => "Bitcoin",
            Currency::BCH => "Bitcoin Cash",
            Currency::BIT => "BitDAO",
            Currency::BTT => "Bittorrent",
            Currency::BLZ => "Bluzelle",
            Currency::BOBA => "Boba Network",
            Currency::FIDA => "Bonfida",
            Currency::ADA => "Cardano",
            Currency::CTSI => "Cartesi",
            Currency::CELR => "Celer Network",
            Currency::CFG => "Centrifuge",
            Currency::XCN => "Chain",
            Currency::LINK => "Chainlink",
            Currency::CHZ => "Chiliz",
            Currency::CHR => "Chromia",
            Currency::CVC => "Civic",
            Currency::COMP => "Compound",
            Currency::C98 => "Coin98",
            Currency::CVX => "Convex Finance",
            Currency::ATOM => "Cosmos",
            Currency::COTI => "COTI",
            Currency::CQT => "Covalent",
            Currency::CSM => "Crust Shadow",
            Currency::CRV => "Curve",
            Currency::DAI => "Dai",
            Currency::DASH => "Dash",
            Currency::MANA => "Decentraland",
            Currency::DENT => "Dent",
            Currency::DOGE => "Dogecoin",
            Currency::DYDX => "dYdX",
            Currency::EGLD => "Elrond",
            Currency::EWT => "Energy Web Token",
            Currency::ENJ => "Enjin Coin",
            Currency::MLN => "Enzyme Finance",
            Currency::EOS => "EOS",
            Currency::ETHW => "EthereumPoW",
            Currency::ETH => "Ethereum (\"Ether\")",
            Currency::ETC => "Ethereum Classic",
            Currency::ENS => "Ethereum Name Service",
            Currency::FTM => "Fantom",
            Currency::FET => "Fetch.ai",
            Currency::FIL => "Filecoin",
            Currency::FLOW => "Flow",
            Currency::FXS => "Frax Share",
            Currency::GALA => "Gala Games",
            Currency::GAL => "Galxe",
            Currency::GARI => "Gari Network",
            Currency::MV => "GensoKishi Metaverse",
            Currency::GTC => "Gitcoin",
            Currency::GNO => "Gnosis",
            Currency::GST => "Green Satoshi Token",
            Currency::FARM => "Harvest Finance",
            Currency::ICX => "ICON",
            Currency::IDEX => "IDEX",
            Currency::RLC => "iExec",
            Currency::IMX => "Immutable X",
            Currency::INJ => "Injective Protocol",
            Currency::TEER => "Integritee",
            Currency::INTR => "Interlay",
            Currency::ICP => "Internet Computer",
            Currency::JASMY => "Jasmy",
            Currency::JUNO => "JUNO",
            Currency::KAR => "Karura",
            Currency::KAVA => "Kava",
            Currency::ROOK => "KeeperDAO",
            Currency::KEEP => "Keep Network",
            Currency::KP3R => "Keep3r Network",
            Currency::KILT => "KILT",
            Currency::KIN => "Kin",
            Currency::KINT => "Kintsugi",
            Currency::KSM => "Kusama",
            Currency::KNC => "Kyber Network",
            Currency::LDO => "Lido DAO",
            Currency::LCX => "Liechtenstein Cryptoassets Exchange",
            Currency::LSK => "Lisk",
            Currency::LTC => "Litecoin",
            Currency::LPT => "Livepeer",
            Currency::LRC => "Loopring",
            Currency::MKR => "Maker",
            Currency::MNGO => "Mango",
            Currency::MSOL => "Marinade SOL",
            Currency::POND => "Marlin",
            Currency::MASK => "Mask Network",
            Currency::MC => "Merit Circle",
            Currency::MINA => "Mina",
            Currency::MIR => "Mirror Protocol",
            Currency::XMR => "Monero",
            Currency::GLMR => "Moonbeam",
            Currency::MOVR => "Moonriver",
            Currency::MULTI => "Multichain",
            Currency::MXC => "MXC",
            Currency::ALICE => "My Neighbor Alice",
            Currency::NANO => "Nano",
            Currency::NEAR => "Near Protocol",
            Currency::NODL => "Nodle",
            Currency::NMR => "Numeraire",
            Currency::NYM => "Nym",
            Currency::OCEAN => "Ocean",
            Currency::OMG => "OMG Network",
            Currency::ORCA => "Orca",
            Currency::OXT => "Orchid",
            Currency::OGN => "Origin Protocol",
            Currency::OXY => "Oxygen",
            Currency::PARA => "Parallel Finance",
            Currency::PAXG => "PAX Gold",
            Currency::PERP => "Perpetual Protocol",
            Currency::PHA => "Phala",
            Currency::PLA => "PlayDapp",
            Currency::DOT => "Polkadot",
            Currency::POLS => "Polkastarter",
            Currency::MATIC => "Polygon",
            Currency::POWR => "Powerledger",
            Currency::PSTAKE => "pSTAKE",
            Currency::QTUM => "Qtum",
            Currency::QNT => "Quant",
            Currency::RAD => "Radicle",
            Currency::RARI => "Rarible",
            Currency::RAY => "Raydium",
            Currency::REN => "REN Protocol",
            Currency::RNDR => "Render",
            Currency::REQ => "Request",
            Currency::XRP => "Ripple",
            Currency::XRT => "Robonomics",
            Currency::RPL => "Rocket Pool",
            Currency::RBC => "Rubic",
            Currency::SBR => "Saber",
            Currency::SAMO => "Samoyed Coin",
            Currency::SCRT => "Secret",
            Currency::KEY => "SelfKey",
            Currency::SRM => "Serum",
            Currency::SHIB => "Shiba Inu",
            Currency::SDN => "Shiden",
            Currency::SC => "Siacoin",
            Currency::SOL => "Solana",
            Currency::SGB => "Songbird",
            Currency::SPELL => "Spell Token",
            Currency::FIS => "Stafi Protocol",
            Currency::ATLAS => "Star Atlas",
            Currency::POLIS => "Star Atlas DAO",
            Currency::STG => "Stargate Finance",
            Currency::XLM => "Stellar Lumens",
            Currency::STEP => "Step Finance",
            Currency::GMT => "STEPN",
            Currency::STORJ => "Storj",
            Currency::SUPER => "SuperFarm",
            Currency::RARE => "SuperRare",
            Currency::SUSHI => "Sushi",
            Currency::SXP => "Swipe",
            Currency::SYN => "Synapse",
            Currency::SNX => "Synthetix",
            Currency::TBTC => "tBTC",
            Currency::LUNA2 => "Terra 2.0",
            Currency::LUNA => "Terra Classic",
            Currency::UST => "TerraUSD Classic",
            Currency::TVK => "Terra Virtual Kolect",
            Currency::USDT => "Tether",
            Currency::XTZ => "Tezos",
            Currency::GRT => "The Graph",
            Currency::SAND => "The Sandbox",
            Currency::RUNE => "Thorchain",
            Currency::T => "Threshold",
            Currency::TOKE => "Tokemak",
            Currency::TRIBE => "Tribe",
            Currency::TRX => "Tron",
            Currency::TRU => "TrueFi",
            Currency::UNFI => "Unifi Protocol DAO",
            Currency::UNI => "Uniswap",
            Currency::UMA => "Universal Market Access",
            Currency::USDC => "USD Coin",
            Currency::WAVES => "WAVES",
            Currency::WOO => "Woo Network",
            Currency::WBTC => "Wrapped Bitcoin",
            Currency::WETH => "Wrapped Ether",
            Currency::YFI => "Yearn Finance",
            Currency::YGG => "Yield Guild Games",
            Currency::ZEC => "Zcash",
        }
    }

    pub fn is_cash(&self) -> bool {
        matches!(self, Currency::USD | Currency::EUR | Currency::CAD | Currency::AUD | Currency::GBP | Currency::CHF | Currency::JPY | Currency::AED)
    }
    pub fn is_crypto(&self) -> bool {
        !self.is_cash()
    }
}

