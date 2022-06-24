use std::fs;

use anyhow::Result;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::Timestamp;
use serenity::utils::Colour;
use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

use crate::utils::gecko::lib::{Amount, Coin};
use crate::utils::gecko::{get_coin, lib::MarketChange};
use crate::utils::plotter::get_line_chart;

pub async fn main(ctx: Context, command: ApplicationCommandInteraction) -> Result<()> {
    let command_name = &command.data.name;
    let coin = get_coin(&command_name).await?;
    let coin1 = coin.clone();

    let graph_handle = tokio::spawn(async move { build_graph(coin1).await });
    let message_handle = tokio::spawn(async move { build_message(coin).await });

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
) -> Result<(String, String, String, String, Vec<(String, String, bool)>)> {
    let title = coin.localization.en.clone();
    let description = coin.get_short_description();
    let thumbnail = coin.image.large.clone();
    let title_url = coin.links.homepage[0].clone();
    let fields: Vec<(String, String, bool)> = vec![
        (
            "Price".into(),
            coin.get_formatted_amount(Amount::CurrentPrice),
            true,
        ),
        (
            "24h Volume".into(),
            coin.get_formatted_amount(Amount::Volume24h),
            true,
        ),
        (
            "Market Cap".into(),
            coin.get_formatted_amount(Amount::MarketCap),
            true,
        ),
        (
            "1h".into(),
            coin.get_formatted_change(MarketChange::PercentageChange1h),
            true,
        ),
        (
            "24h".into(),
            coin.get_formatted_change(MarketChange::PercentageChange24h),
            true,
        ),
        (
            "7d".into(),
            coin.get_formatted_change(MarketChange::PercentageChange7d),
            true,
        ),
    ];

    Ok((title, title_url, description, thumbnail, fields))
}

async fn build_graph(coin: Coin) -> Result<String> {
    get_line_chart(&coin)
}