pub mod lib;
use anyhow::Result;
use reqwest::StatusCode;

use self::lib::{CoinInfo, GeckoError};

pub async fn get_coin(coin: &str) -> Result<lib::Coin, GeckoError> {
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/{}?sparkline=true",
        coin.to_lowercase()
    );

    let res = reqwest::get(url).await?;

    match res.status() {
        StatusCode::OK => match res.json::<lib::Coin>().await {
            Ok(parsed) => Ok(parsed),
            Err(err) => {
                println!("ERROR PARSING: {}", err);
                Err(GeckoError::ParseError)
            }
        },
        StatusCode::NOT_FOUND => Err(GeckoError::CoinNotFoundError),
        StatusCode::UNAUTHORIZED => Err(GeckoError::LimitError),
        _ => Err(GeckoError::UnknownError),
    }
}

pub async fn get_ohlc(coin: &str) -> Result<Vec<Vec<f64>>, GeckoError> {
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/{}/ohlc?vs_currency=usd&days=7",
        coin.to_lowercase()
    );

    let res = reqwest::get(url).await?;

    match res.status() {
        StatusCode::OK => match res.json::<Vec<Vec<f64>>>().await {
            Ok(parsed) => Ok(parsed),
            Err(_) => Err(GeckoError::ParseError),
        },
        StatusCode::NOT_FOUND => Err(GeckoError::CoinNotFoundError),
        StatusCode::UNAUTHORIZED => Err(GeckoError::LimitError),
        _ => Err(GeckoError::UnknownError),
    }
}

pub async fn get_list() -> Result<Vec<CoinInfo>, GeckoError> {
    let url = "https://api.coingecko.com/api/v3/coins/list";

    let res = reqwest::get(url).await?;

    match res.status() {
        StatusCode::OK => match res.json::<Vec<CoinInfo>>().await {
            Ok(parsed) => Ok(parsed),
            Err(_) => Err(GeckoError::ParseError),
        },
        StatusCode::NOT_FOUND => Err(GeckoError::CoinNotFoundError),
        StatusCode::UNAUTHORIZED => Err(GeckoError::LimitError),
        _ => Err(GeckoError::UnknownError),
    }
}

pub async fn get_top_coins() -> Result<Vec<CoinInfo>, GeckoError> {
    let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=gecko_desc&per_page=100&page=1&sparkline=false";
    let res = reqwest::get(url).await?;

    match res.status() {
        StatusCode::OK => match res.json::<Vec<CoinInfo>>().await {
            Ok(parsed) => Ok(parsed),
            Err(_) => Err(GeckoError::ParseError),
        },
        StatusCode::NOT_FOUND => Err(GeckoError::CoinNotFoundError),
        StatusCode::UNAUTHORIZED => Err(GeckoError::LimitError),
        _ => Err(GeckoError::UnknownError),
    }
}
