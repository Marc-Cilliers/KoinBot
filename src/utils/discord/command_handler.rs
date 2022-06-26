use super::commands;
use serenity::{
    client::Context,
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

pub async fn handle_command(ctx: Context, command: ApplicationCommandInteraction) {
    let command_name = &command.data.name;
    println!("Handling command: {}", command_name);

    let command_copy = command.clone();
    let ctx_copy = ctx.clone();

    let res = match command_name.as_str() {
        "niche" => commands::niche::main(ctx_copy, command_copy).await,
        _ => commands::coin::main(ctx_copy, command_copy).await,
    };

    if let Err(err) = res {
        println!("{:?}", err);

        command
            .create_interaction_response(&ctx.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(format!("{}", err)))
            })
            .await
            .expect("Error reporting Discord error");
    }
}
