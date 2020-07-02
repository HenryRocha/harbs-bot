/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
use crate::game::Game;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

/* ============================================================================================================= */
/* COMMAND                                                                                                       */
/* ============================================================================================================= */
#[command]
pub fn show_words(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    match data.get::<Game>() {
        Some(words) => {
            let _ = msg.reply(&ctx, format!("Current words are: {:?}", words));
        }
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        }
    }

    return Ok(());
}
