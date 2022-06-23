mod command_handler;
mod commands;
pub mod lib;
mod utils;

use std::env;

use command_handler::handle_command;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandOptionType,
};
use serenity::{
    async_trait,
    model::interactions::Interaction,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::utils::gecko::get_top_coins;

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

    async fn message(&self, ctx: Context, msg: Message) {
        println!("THE MESSAGE: {:?}", msg);
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let coin_list = get_top_coins().await.unwrap();

        coin_list.into_iter().for_each(|coin| {
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                ApplicationCommand::create_global_application_command(&ctx_clone.http, |command| {
                    command
                        .name(coin.id)
                        .description(format!(
                            "Fetch price info for {} ({})",
                            coin.name, coin.symbol
                        ))
                        .create_option(|option| {
                            option
                                .name("is_ohlc")
                                .description("Choose whether the graph is ohlc")
                                .kind(ApplicationCommandOptionType::Boolean)
                                .required(false)
                        })
                })
                .await
            });
        });
    }
}

#[tokio::main]
pub async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Token missing");

    let mut client = Client::builder(&token, GatewayIntents::default())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
