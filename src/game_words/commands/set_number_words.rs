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
#[aliases("setn", "snw")]
#[description("Sets the number of words each player has to give.")]
pub fn set_number_words(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    // Check if the number of given arguments is equal to 1.
    if args.len() != 1 {
        let _ = msg.reply(&ctx, "The set_number_words command takes only 1 arguments.");
        return Ok(());
    }

    // Get the Game object from the shard's data.
    let mut data = ctx.data.write();
    match data.get_mut::<GameWords>() {
        Some(game) => {
            // Check if the attribute num_words was not already set.
            if game.num_words > 0 {
                let _ = msg.reply(
                    &ctx,
                    "The number of words has already been set. If you wish to change it, reset the game.",
                );
            }

            // Sets the attribute num_words to the given argument, after transforming it to a u8.
            game.num_words = args.single::<u8>().unwrap();
        }
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        }
    }

    return Ok(());
}
