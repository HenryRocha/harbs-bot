/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
mod commands;
mod handler;
mod shard;

use commands::ping::*;
use dotenv;
use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

/* ============================================================================================================= */
/* STRUCTS                                                                                                       */
/* ============================================================================================================= */
#[group]
#[commands(ping)]
struct General;

/* ============================================================================================================= */
/* MAIN                                                                                                          */
/* ============================================================================================================= */
fn main() {
    // Verify is an .env file exists.
    dotenv::dotenv().expect(".env file not found.");

    // Configure the client with your Discord bot token in the environment.
    let token: String = dotenv::var("DISCORD_TOKEN").unwrap();

    // Create a new instance of the Client, logging in as a bot.
    let mut client: Client = Client::new(&token, handler::Handler).expect("Err creating client");

    client.with_framework(StandardFramework::new().configure(|c| c.prefix(";")).group(&GENERAL_GROUP));

    // Start a single shard, and start listening to events.
    if let Err(e) = client.start() {
        println!("Client error: {:?}", e);
    }
}
