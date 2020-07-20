/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
use crate::game_words::game::GameWords;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

/* ============================================================================================================= */
/* COMMAND                                                                                                       */
/* ============================================================================================================= */
#[command]
#[aliases("add", "a")]
#[description("Adds the given words to the word list.")]
pub fn add_words(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    // Check if the number of given arguments is equal to 3.
    if args.len() != 3 {
        let _ = msg.reply(&ctx, "The add_words command takes only 3 arguments.");
        return Ok(());
    }

    let mut data = ctx.data.write();

    match data.get_mut::<GameWords>() {
        Some(game) => {
            for arg in args.iter::<String>() {
                game.words.push(arg.unwrap());
            }
        }
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        }
    }

    return Ok(());
}
