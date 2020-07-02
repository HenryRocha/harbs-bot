/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
use crate::game::Game;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

/* ============================================================================================================= */
/* COMMAND                                                                                                       */
/* ============================================================================================================= */
#[command]
pub fn add_words(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    // Check if the number of given arguments is equal to 3.
    if args.len() != 3 {
        let _ = msg.reply(&ctx, "The add_words command takes only 3 arguments.");
        return Ok(());
    }

    let mut data = ctx.data.write();

    match data.get_mut::<Game>() {
        Some(words) => {
            for arg in args.iter::<String>() {
                words.push(arg.unwrap());
            }
        }
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        }
    }

    // if let Some(words) = data.get_mut::<Game>() {
    //     for arg in args.iter::<String>() {
    //         words.push(arg.unwrap());
    //     }
    // } else {
    //     let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
    // }

    return Ok(());
}
