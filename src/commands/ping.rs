/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

/* ============================================================================================================= */
/* COMMAND                                                                                                       */
/* ============================================================================================================= */
#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let start: i64 = chrono::offset::Utc::now().timestamp_millis();
    let latency: i64 = start - msg.timestamp.timestamp_millis();

    let _ = msg.channel_id.say(&ctx.http, format!("Pong! {latency}ms", latency = latency));

    return Ok(());
}
