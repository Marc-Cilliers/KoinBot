use std::fs;

use anyhow::Result;
use rusty_money::iso::Currency;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::Timestamp;
use serenity::utils::Colour;
use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::utils::discord::utils::{get_currency_option, get_graph_option};
use crate::utils::gecko::get_ohlc;
use crate::utils::gecko::lib::{Amount, Coin};
use crate::utils::gecko::{get_coin, lib::MarketChange};
use crate::utils::plotter::{get_line_chart, get_ohlc_chart};

pub async fn main(ctx: Context, command: ApplicationCommandInteraction) -> Result<()> {
    let command_name = command.data.name.clone();
    let command_name1 = command_name.clone();

    let currency = get_currency_option(&command)?;
    let graph = get_graph_option(&command)?;
    let (tx, rx): (Sender<Coin>, Receiver<Coin>) = mpsc::channel(1);
    let graph_handle = tokio::spawn(async move { build_graph(rx, &command_name, graph).await });

    let coin = get_coin(&command_name1).await?;
    tx.send(coin.clone()).await.ok();

    let message_handle = tokio::spawn(async move { build_message(coin, currency).await });

    let (title, title_url, description, thumbnail, fields) = message_handle.await??;
    let filename = graph_handle.await??;
    let attachment = format!("attachment://{}", filename);

    command
        .create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .embed(|e| {
                            e.author(|a| a.icon_url(thumbnail).name(title).url(title_url))
                                .description(description)
                                .fields(fields)
                                .color(Colour::DARK_GOLD)
                                .timestamp(Timestamp::now())
                                .image(attachment)
                        })
                        .add_file(filename.as_str())
                })
        })
        .await?;

    fs::remove_file(filename)?;

    Ok(())
}

async fn build_message(
    coin: Coin,
    currency: Currency,
) -> Result<(String, String, String, String, Vec<(String, String, bool)>)> {
    let title = coin.localization.en.clone();
    let description = coin.get_short_description();
    let thumbnail = coin.image.large.clone();
    let homepage = coin.links.homepage[0].clone();
    let title_url = if !homepage.contains("http") {
        format!("https://{}", homepage)
    } else {
        homepage
    };

    let fields: Vec<(String, String, bool)> = vec![
        (
            "Price".into(),
            coin.get_formatted_amount(Amount::CurrentPrice, currency),
            true,
        ),
        (
            "24h Volume".into(),
            coin.get_formatted_amount(Amount::Volume24h, currency),
            true,
        ),
        (
            "Market Cap".into(),
            coin.get_formatted_amount(Amount::MarketCap, currency),
            true,
        ),
        (
            "1h".into(),
            coin.get_formatted_change(MarketChange::PercentageChange1h, currency),
            true,
        ),
        (
            "24h".into(),
            coin.get_formatted_change(MarketChange::PercentageChange24h, currency),
            true,
        ),
        (
            "7d".into(),
            coin.get_formatted_change(MarketChange::PercentageChange7d, currency),
            true,
        ),
    ];

    Ok((title, title_url, description, thumbnail, fields))
}

async fn build_graph(mut rx: Receiver<Coin>, coin_name: &str, graph: String) -> Result<String> {
    match graph.as_str() {
        "line" => {
            let coin = rx.recv().await.unwrap();
            get_line_chart(&coin)
        }
        "ohlc" => {
            let ohlc_data = get_ohlc(&coin_name).await?;
            get_ohlc_chart(&ohlc_data, &coin_name)
        }
        _ => Ok("".into()),
    }
}
