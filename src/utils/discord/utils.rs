use std::env;

use super::lib::{Arg, CommandInfo};
use anyhow::Result;
use rusty_money::iso::{self, Currency};
use serenity::{
    client::Context,
    model::{id::UserId, interactions::application_command::ApplicationCommandInteraction},
};

lazy_static! {
    static ref OWNER_ID: u64 = get_owner_id();
}

fn get_owner_id() -> u64 {
    let owner_id_var = env::var("OWNER_ID");

    match owner_id_var {
        Ok(id) => id.parse::<u64>().unwrap(),
        Err(_) => 000,
    }
}

pub fn get_command_info(command: &ApplicationCommandInteraction) -> Result<CommandInfo> {
    let args = command
        .data
        .options
        .iter()
        .map(|option| {
            let value = option.value.as_ref().unwrap();

            Arg {
                name: &option.name,
                value: value.clone(),
            }
        })
        .collect();

    Ok(CommandInfo {
        name: &command.data.name,
        args: args,
    })
}

pub fn get_currency_option(command: &ApplicationCommandInteraction) -> Result<Currency> {
    let command_info = get_command_info(&command)?;
    let currency_arg = command_info.get_arg("currency");

    let default = Ok(*iso::USD);

    match currency_arg {
        Some(iso_code) => match iso_code.as_str() {
            Some(iso_str) => match iso::find(iso_str) {
                Some(currency) => Ok(*currency),
                None => default,
            },
            None => default,
        },
        None => default,
    }
}

pub async fn message_owner(ctx: &Context, message: String) {
    let vvv = UserId(*OWNER_ID)
        .to_user(ctx.clone())
        .await
        .expect("Failed to retrieve owner");

    vvv.direct_message(&ctx, |m| m.content(message))
        .await
        .expect("Error sending dm to owner");
}

pub fn get_invoking_user(command: &ApplicationCommandInteraction) -> String {
    let member = command.member.clone();

    match member {
        Some(member) => format!(
            "{} ({})",
            member.user.name,
            member.nick.unwrap_or("No nickname".into())
        ),
        _ => "Anonymous".into(),
    }
}
