use super::lib::{Arg, CommandInfo};
use anyhow::Result;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

pub fn get_command_info(command: &ApplicationCommandInteraction) -> Result<CommandInfo, String> {
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
