use std::time::Instant;

use crate::utils::discord::utils::message_owner;

use super::{commands, utils::get_invoking_user};
use serenity::{
    client::Context,
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

pub async fn handle_command(ctx: Context, command: ApplicationCommandInteraction) {
    let start = Instant::now();
    let command_name = &command.data.name;
    let user = get_invoking_user(&command);

    let command_copy = command.clone();
    let ctx_copy = ctx.clone();

    let res = match command_name.as_str() {
        "niche" => commands::niche::main(ctx_copy, command_copy).await,
        _ => commands::coin::main(ctx_copy, command_copy).await,
    };

    let elapsed = start.elapsed();

    if let Err(err) = res {
        message_owner(
            &ctx,
            format!(
                "Error occurred for [{}] ({:.3?}): {:?}",
                command_name, elapsed, err
            ),
        )
        .await;

        command
            .create_interaction_response(&ctx.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(format!("{}", err)))
            })
            .await
            .ok();
        return;
    }

    message_owner(
        &ctx,
        format!(
            "{} => [{}]  success. ({:.3?} elapsed)",
            user, command_name, elapsed
        ),
    )
    .await;
}
