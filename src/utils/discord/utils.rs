use super::lib::{Arg, CommandInfo};
use anyhow::Result;
use rusty_money::iso::{self, Currency};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

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
