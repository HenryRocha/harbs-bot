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
    // Get the Game object from the shard's data.
    let mut data = ctx.data.write();
    match data.get_mut::<GameWords>() {
        Some(game) => {
            // Check if the attribute num_words was not set already.
            if game.num_words <= 0 {
                let _ = msg.reply(&ctx, "The number of words each player has to give has not been set yet.");
                return Ok(());
            }

            // Adds the given words only if the number of given words is equal to the attribute num_words.
            if args.len() != (game.num_words as usize) {
                let _ = msg.reply(
                    &ctx,
                    format!("The add_words command takes only {num_words} arguments.", num_words = game.num_words),
                );
                return Ok(());
            }

            // Add each word to the words list.
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
