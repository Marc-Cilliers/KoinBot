mod command_handler;
mod commands;
pub mod lib;
mod utils;

use std::env;
use std::time::Instant;

use command_handler::handle_command;
use rusty_money::iso::{self, Currency};
use serenity::builder::CreateApplicationCommandOption;
use serenity::model::guild::Guild;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandOptionType,
};
use serenity::{async_trait, model::gateway::Ready, model::interactions::Interaction, prelude::*};

use crate::utils::gecko::get_top_coins;

use self::utils::message_owner;

const COIN_COUNT: u8 = 99;

lazy_static! {
    static ref TOP_CURRENCIES: Vec<&'static Currency> = vec![
        iso::USD,
        iso::EUR,
        iso::JPY,
        iso::GBP,
        iso::AUD,
        iso::CAD,
        iso::CHF,
        iso::CNY,
        iso::HKD,
        iso::NZD,
        iso::SEK,
        iso::KRW,
        iso::SGD,
        iso::NOK,
        iso::MXN,
        iso::INR,
        iso::RUB,
        iso::ZAR,
        iso::TRY,
        iso::BRL,
        iso::TWD,
        iso::DKK,
        iso::PLN,
        iso::THB,
        iso::IDR
    ];
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                tokio::spawn(async move { handle_command(ctx, command).await })
            }
            _ => return,
        };
        // Ignore any non-commands for now
    }

    async fn guild_create(&self, ctx: Context, guild: Guild) {
        let mut msg = format!("__{} Created__\n\n", guild.name);

        msg = format!("{}• {} members\n", msg, guild.member_count);
        msg = format!("{}• joined at {}\n", msg, guild.joined_at);
        msg = format!("{}• is it large? {}\n", msg, guild.large);

        message_owner(&ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut msg = format!("__Connection Success__\n\n");

        msg = format!("{}• {} guilds connected;\n", msg, ready.guilds.len());
        ready.guilds.iter().for_each(|guild| {
            msg = format!("{}    • {}\n", msg, guild.id);
        });
        msg = format!("{}\n", msg);

        match ready.shard {
            Some(shards) => {
                msg = format!("{}• shards: {:?}\n", msg, shards);
            }
            _ => return,
        };
        message_owner(&ctx, msg).await;

        update_commands(&ctx).await;
    }
}

async fn update_commands(ctx: &Context) {
    let update_commands = env::var("UPDATE_COMMANDS").unwrap_or("n".into());

    if update_commands.as_str() != "y" {
        return;
    }

    let start = Instant::now();

    let coin_list = get_top_coins(COIN_COUNT).await.unwrap();

    // Create the currencies option
    let mut currency_option = CreateApplicationCommandOption::default();
    currency_option.name("currency");
    currency_option.description("Your preferred currency. Default is: USD");
    currency_option.kind(ApplicationCommandOptionType::String);

    TOP_CURRENCIES.iter().for_each(|currency| {
        currency_option.add_string_choice(currency.name, currency.iso_alpha_code);
    });

    ApplicationCommand::set_global_application_commands(&ctx.http, |command| {
        // Coin commands
        coin_list.into_iter().for_each(|coin| {
            let currency_option1 = currency_option.clone();
            command.create_application_command(|cmd| {
                cmd.name(coin.id)
                    .description(format!(
                        "Fetch price info for {} ({})",
                        coin.name, coin.symbol
                    ))
                    .add_option(currency_option1)
            });
        });

        // Custom commands
        command.create_application_command(|cmd| {
            cmd.name("niche")
                .description("Fetch price info for a (more niche) coin")
                .create_option(|option| {
                    option
                        .name("coin")
                        .description("The coin's name")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                })
                .add_option(currency_option)
        })
    })
    .await
    .expect("Error creating global commands");

    let elapsed = start.elapsed();
    message_owner(
        &ctx,
        format!("Global Command Updates Success! ({:.3?} elapsed)", elapsed),
    )
    .await;
}

#[tokio::main]
pub async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Token missing");

    let mut client = Client::builder(&token, GatewayIntents::default())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
