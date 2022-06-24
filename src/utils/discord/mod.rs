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
        check_for_command_recalibration(&ctx).await;

        let coin_list = get_top_coins().await.unwrap();

        ApplicationCommand::create_global_application_command(&ctx.http, |command| {
            command
                .name("niche")
                .description("Fetch price info for a niche coin")
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

        coin_list.into_iter().for_each(|coin| {
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                ApplicationCommand::create_global_application_command(&ctx_clone.http, |command| {
                    command
                        .name(coin.id)
                        .description(format!("Fetch price info for {} ({})", "thingy", "thingy"))
                })
                .await
            });
        });
    }
}

async fn check_for_command_recalibration(ctx: &Context) {
    let reset_commands = env::var("RESET_COMMANDS").unwrap_or("n".into());

    println!("Checking for command recalibration");

    match reset_commands.as_str() {
        "y" => {
            let current_commands = ApplicationCommand::get_global_application_commands(&ctx.http)
                .await
                .expect("Error fetching current commands");

            let mut deletion_handles: Vec<JoinHandle<()>> = vec![];

            current_commands.into_iter().for_each(|command| {
                let ctx1 = ctx.clone();
                let handle = tokio::spawn(async move {
                    ApplicationCommand::delete_global_application_command(&ctx1.http, command.id)
                        .await
                        .ok();
                });
                deletion_handles.push(handle)
            });

            join_all(deletion_handles).await;
        }
        _ => return,
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
