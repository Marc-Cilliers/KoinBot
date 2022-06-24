mod command_handler;
mod commands;
pub mod lib;
mod utils;

use std::env;

use command_handler::handle_command;
use serenity::futures::future::join_all;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandOptionType,
};
use serenity::{
    async_trait,
    model::interactions::Interaction,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use tokio::task::JoinHandle;

use crate::utils::gecko::get_top_coins;

use super::gecko::lib::CoinInfo;

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
        check_for_coin_updates(&ctx).await;
    }
}

async fn check_for_coin_updates(ctx: &Context) {
    let update_coins = env::var("UPDATE_COINS").unwrap_or("n".into());

    if update_coins.as_str() != "y" {
        return;
    }

    let mut current_commands = ApplicationCommand::get_global_application_commands(&ctx.http)
        .await
        .expect("Error fetching current commands");

    let mut coin_list = get_top_coins().await.expect("Error fetching top coins");
    coin_list.sort_by(|a, b| a.id.cmp(&b.id));
    current_commands.sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));

    match current_commands.binary_search_by(|c| c.name.cmp(&"niche".into())) {
        Ok(niche_index) => current_commands.remove(niche_index),
        _ => return,
    };

    let mut update_handles: Vec<JoinHandle<()>> = vec![];

    for i in 0..coin_list.len() - 1 {
        let possible_command = current_commands.get(i);

        match possible_command {
            Some(command) => {
                let coin_changed = command.name.to_string() != coin_list[i].id;

                if coin_changed {
                    let handle = replace_coin(&ctx, &command, &coin_list[i]);
                    update_handles.push(handle);
                }
            }
            None => {
                let handle = add_coin(&ctx, &coin_list[i]);
                update_handles.push(handle);
            }
        };
    }

    join_all(update_handles).await;

    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command
            .name("niche")
            .description("Fetch price info for a (more niche) coin")
            .create_option(|option| {
                option
                    .name("coin")
                    .description("The coin's name")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    })
    .await
    .expect("Error creating niche command");
}

fn add_coin(ctx: &Context, coin: &CoinInfo) -> JoinHandle<()> {
    let ctx1 = ctx.clone();
    let coin1 = coin.clone();

    tokio::spawn(async move {
        let id = coin1.id.clone();
        println!("[{}] ==> Adding...", id);
        ApplicationCommand::create_global_application_command(&ctx1.http, |command| {
            command.name(coin1.id).description(format!(
                "Fetch price info for {} ({})",
                coin1.name, coin1.symbol
            ))
        })
        .await
        .ok();
        println!("[{}] ==> Complete!", id);
    })
}

fn replace_coin(ctx: &Context, command: &ApplicationCommand, coin: &CoinInfo) -> JoinHandle<()> {
    let ctx1 = ctx.clone();
    let command1 = command.clone();
    let coin1 = coin.clone();

    tokio::spawn(async move {
        let name = command1.name.clone();
        let id = coin1.name.clone();
        println!("[{} -> {}] ==> Replacing...", name, id);
        ApplicationCommand::delete_global_application_command(&ctx1.http, command1.id)
            .await
            .ok();

        ApplicationCommand::create_global_application_command(&ctx1.http, |command| {
            command.name(coin1.id).description(format!(
                "Fetch price info for {} ({})",
                coin1.name, coin1.symbol
            ))
        })
        .await
        .ok();
        println!("[{} -> {}] ==> Complete!", name, id);
    })
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
