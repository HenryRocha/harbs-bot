/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
mod commands;
mod game;
mod handler;
mod shard;

use commands::add_words::*;
use commands::ping::*;
use commands::show_words::*;
use dotenv;
use game::Game;
use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use shard::ShardManagerContainer;
use std::sync::Arc;

/* ============================================================================================================= */
/* STRUCTS                                                                                                       */
/* ============================================================================================================= */
#[group]
#[commands(ping, add_words, show_words)]
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

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<Game>(vec![]);
    }
    // Start a single shard, and start listening to events.
    if let Err(e) = client.start() {
        println!("Client error: {:?}", e);
    }
}
