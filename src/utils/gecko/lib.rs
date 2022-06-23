use regex::{Captures, Regex};
use rust_decimal::Decimal;
use rusty_money::{iso, Money};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum GeckoError {
    CoinNotFoundError,
    LimitError,
    ParseError,
    UnknownError,
}
impl std::error::Error for GeckoError {}
impl fmt::Display for GeckoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeckoError::CoinNotFoundError => {
                write!(f, "Coin not found! Try its full name, eg. bitcoin")
            }
            GeckoError::LimitError => {
                write!(f, "Uh-oh! Seems like I've reached the API limit")
            }
            GeckoError::ParseError => {
                write!(f, "Whoops! An expected parse error occured")
            }
            GeckoError::UnknownError => {
                write!(f, "An unknown API error occured")
            }
        }
    }
}

impl From<reqwest::Error> for GeckoError {
    fn from(_: reqwest::Error) -> Self {
        Self::UnknownError
    }
}

pub fn object_empty_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    for<'a> T: Deserialize<'a>,
{
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Aux<T> {
        T(T),
        Empty(Empty),
        Null,
    }

    match Deserialize::deserialize(deserializer)? {
        Aux::T(t) => Ok(Some(t)),
        Aux::Empty(_) | Aux::Null => Ok(None),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CurrencyConversions {
    pub aed: Decimal,
    pub ars: Decimal,
    pub aud: Decimal,
    pub bch: Decimal,
    pub bdt: Decimal,
    pub bhd: Decimal,
    pub bmd: Decimal,
    pub bnb: Decimal,
    pub brl: Decimal,
    pub btc: Decimal,
    pub cad: Decimal,
    pub chf: Decimal,
    pub clp: Decimal,
    pub cny: Decimal,
    pub czk: Decimal,
    pub dkk: Decimal,
    pub dot: Decimal,
    pub eos: Decimal,
    pub eth: Decimal,
    pub eur: Decimal,
    pub gbp: Decimal,
    pub hkd: Decimal,
    pub huf: Decimal,
    pub idr: Decimal,
    pub ils: Decimal,
    pub inr: Decimal,
    pub jpy: Decimal,
    pub krw: Decimal,
    pub kwd: Decimal,
    pub lkr: Decimal,
    pub ltc: Decimal,
    pub mmk: Decimal,
    pub mxn: Decimal,
    pub myr: Decimal,
    pub ngn: Decimal,
    pub nok: Decimal,
    pub nzd: Decimal,
    pub php: Decimal,
    pub pkr: Decimal,
    pub pln: Decimal,
    pub rub: Decimal,
    pub sar: Decimal,
    pub sek: Decimal,
    pub sgd: Decimal,
    pub thb: Decimal,
    #[serde(alias = "try")]
    pub try_: Decimal,
    pub twd: Decimal,
    pub uah: Decimal,
    pub usd: Decimal,
    pub vef: Decimal,
    pub vnd: Decimal,
    pub xag: Decimal,
    pub xau: Decimal,
    pub xdr: Decimal,
    pub xlm: Decimal,
    pub xrp: Decimal,
    pub yfi: Decimal,
    pub zar: Decimal,
    pub bits: Decimal,
    pub link: Decimal,
    pub sats: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Sparkline {
    pub price: Vec<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MarketData {
    pub current_price: CurrencyConversions,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub ath: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub ath_change_percentage: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub atl: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub atl_change_percentage: Option<CurrencyConversions>,
    pub market_cap: CurrencyConversions,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub fully_diluted_valuation: Option<CurrencyConversions>,
    pub total_volume: CurrencyConversions,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub high_24h: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub low_24h: Option<CurrencyConversions>,
    pub price_change_24h: Option<Decimal>,
    pub price_change_percentage_24h: Option<Decimal>,
    pub price_change_percentage_7d: Option<Decimal>,
    pub price_change_percentage_14d: Option<Decimal>,
    pub price_change_percentage_30d: Option<Decimal>,
    pub price_change_percentage_60d: Option<Decimal>,
    pub price_change_percentage_200d: Option<Decimal>,
    pub price_change_percentage_1y: Option<Decimal>,
    pub market_cap_change_24h: Option<Decimal>,
    pub market_cap_change_percentage_24h: Option<Decimal>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub price_change_24h_in_currency: Option<CurrencyConversions>,
    pub price_change_percentage_1h_in_currency: CurrencyConversions,
    pub price_change_percentage_24h_in_currency: CurrencyConversions,
    pub price_change_percentage_7d_in_currency: CurrencyConversions,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub price_change_percentage_14d_in_currency: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub price_change_percentage_30d_in_currency: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub price_change_percentage_60d_in_currency: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub price_change_percentage_200d_in_currency: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub price_change_percentage_1y_in_currency: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub market_cap_change_24h_in_currency: Option<CurrencyConversions>,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub market_cap_change_percentage_24h_in_currency: Option<CurrencyConversions>,
    pub total_supply: Option<Decimal>,
    pub max_supply: Option<Decimal>,
    pub circulating_supply: Option<Decimal>,
    pub sparkline_7d: Sparkline,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    pub thumb: String,
    pub small: String,
    pub large: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Localization {
    pub en: String,
    pub de: String,
    pub es: String,
    pub fr: String,
    pub it: String,
    pub pl: String,
    pub ro: String,
    pub hu: String,
    pub nl: String,
    pub pt: String,
    pub sv: String,
    pub vi: String,
    pub tr: String,
    pub ru: String,
    pub ja: String,
    pub zh: String,
    #[serde(alias = "zh-tw")]
    pub zh_tw: String,
    pub ko: String,
    pub ar: String,
    pub th: String,
    pub id: String,
    pub cs: String,
    pub da: String,
    pub el: String,
    pub hi: String,
    pub no: String,
    pub sk: String,
    pub uk: String,
    pub he: String,
    pub fi: String,
    pub bg: String,
    pub hr: String,
    pub lt: String,
    pub sl: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Links {
    pub homepage: Vec<String>,
    pub blockchain_site: Vec<String>,
    pub official_forum_url: Vec<String>,
    pub chat_url: Vec<String>,
    pub announcement_url: Vec<String>,
    pub subreddit_url: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub hashing_algorithm: Option<String>,
    pub localization: Localization,
    pub description: Localization,
    pub links: Links,
    pub image: Image,
    pub sentiment_votes_up_percentage: Decimal,
    pub sentiment_votes_down_percentage: Decimal,
    pub market_cap_rank: Decimal,
    pub coingecko_rank: Decimal,
    pub coingecko_score: Decimal,
    pub developer_score: Decimal,
    pub community_score: Decimal,
    pub liquidity_score: Decimal,
    pub public_interest_score: Decimal,
    pub market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub enum MarketChange {
    PercentageChange1h,
    PercentageChange24h,
    PercentageChange7d,
}

pub enum Amount {
    CurrentPrice,
    Volume24h,
    MarketCap,
}

const ANCHOR_REGEX: &str = r##"<a href="(.+?)">(.+?)</a>"##;

impl Coin {
    pub fn get_short_description(self: &Self) -> String {
        let full_desc = &self.description.en;
        let re = Regex::new(ANCHOR_REGEX).unwrap();

        let mut paragraphs = full_desc.split("\r\n\r\n");

        let first_paragraph = paragraphs.nth(0).unwrap();

        // Replace anchor tags with hyperlinks
        let cleaned_paragraph = re.replace_all(first_paragraph, |caps: &Captures| {
            format!("[{}]({})", &caps[2], &caps[1])
        });

        cleaned_paragraph.to_string()
    }

    pub fn get_formatted_amount(self: &Self, amount: Amount) -> String {
        let value = match amount {
            Amount::CurrentPrice => self.market_data.current_price.usd,
            Amount::Volume24h => self.market_data.total_volume.usd,
            Amount::MarketCap => self.market_data.market_cap.usd,
        };

        let current_price = Money::from_decimal(value, iso::USD);
        format!("```{}```", current_price)
    }

    pub fn get_formatted_change(self: &Self, market_change: MarketChange) -> String {
        let change = match market_change {
            MarketChange::PercentageChange1h => {
                self.market_data.price_change_percentage_1h_in_currency.usd
            }
            MarketChange::PercentageChange24h => {
                self.market_data.price_change_percentage_24h_in_currency.usd
            }
            MarketChange::PercentageChange7d => {
                self.market_data.price_change_percentage_7d_in_currency.usd
            }
        };

        let rounded = change.round_dp(1);
        let prefix = if change.is_sign_positive() { "+" } else { "" };
        format!("```diff\n{}{:.1}%```", prefix, rounded)
    }
}
