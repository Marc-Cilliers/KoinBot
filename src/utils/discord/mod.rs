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

const COIN_COUNT: u8 = 99;

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

    async fn message(&self, _: Context, msg: Message) {
        println!("THE MESSAGE: {:?}", msg);
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        update_commands(&ctx).await;
    }
}

async fn update_commands(ctx: &Context) {
    let update_commands = env::var("UPDATE_COMMANDS").unwrap_or("n".into());

    if update_commands.as_str() != "y" {
        return;
    }

    let coin_list = get_top_coins(COIN_COUNT).await.unwrap();

    let global_commands = ApplicationCommand::get_global_application_commands(&ctx.http)
        .await
        .unwrap();

    println!("===== All Global Commands =====");
    global_commands
        .iter()
        .for_each(|cmd| println!("({}) [{}]", cmd.id, cmd.name));

    ApplicationCommand::set_global_application_commands(&ctx.http, |command| {
        // Coin commands
        coin_list.into_iter().for_each(|coin| {
            command.create_application_command(|cmd| {
                cmd.name(coin.id).description(format!(
                    "Fetch price info for {} ({})",
                    coin.name, coin.symbol
                ))
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
        })
    })
    .await
    .expect("Error creating global commands");
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
