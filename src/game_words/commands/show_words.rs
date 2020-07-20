/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
use crate::game_words::game::GameWords;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

/* ============================================================================================================= */
/* COMMAND                                                                                                       */
/* ============================================================================================================= */
#[command]
#[aliases("show", "s")]
#[description("Show all words in the word list.")]
#[required_permissions(ADMINISTRATOR)]
pub fn show_words(ctx: &mut Context, msg: &Message) -> CommandResult {
    // Get the Game object from the shard's data.
    let data = ctx.data.read();
    match data.get::<GameWords>() {
        Some(game) => {
            // Sends a reply with the current words in the game's word list.
            let _ = msg.reply(&ctx, format!("Current words are: {:?}", game.words));
        }
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        }
    }

    return Ok(());
}
