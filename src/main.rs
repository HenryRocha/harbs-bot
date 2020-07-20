/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
mod commands;
mod game_words;
mod handler;
mod shard;

use shard::ShardManagerContainer;

use commands::help::*;
use commands::ping::*;

use game_words::commands::add_words::*;
use game_words::commands::show_words::*;

use dotenv;

use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::DispatchError;
use serenity::framework::standard::StandardFramework;

use std::collections::HashSet;
use std::sync::Arc;
/* ============================================================================================================= */
/* STRUCTS                                                                                                       */
/* ============================================================================================================= */
#[group]
#[description = "Miscellaneous commands."]
#[commands(ping)]
struct General;

#[group]
#[prefixes("words", "w")]
#[description("Commands for the words game.")]
#[commands(add_words, show_words)]
struct GameWords;

/* ============================================================================================================= */
/* MAIN                                                                                                          */
/* ============================================================================================================= */
fn main() {
    // Verify is an .env file exists.
    dotenv::dotenv().expect("Expected a .env file is the same directory as this executable");

    // Configure the client with your Discord bot token in the environment.
    let token: String = dotenv::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN key in the .env file");

    // Create a new instance of the Client, logging in as a bot.
    let mut client: Client = Client::new(&token, handler::Handler).expect("Error creating client");

    // Fetching the bot's owners and id.
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // The purpose of the data field is to be accessible and persistent across contexts; that is, data can be
    // modified by one context, and will persist through the future and be accessible through other contexts.
    // This is useful for anything that should "live" through the program: counters, database connections,
    // custom user caches, etc.
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<game_words::game::GameWords>(game_words::game::Game {
            players: vec![],
            words: vec![],
        });
    }

    // Configures the client, allowing for options to mutate how the framework functions.
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.with_whitespace(false).on_mention(Some(bot_id)).prefix(";").owners(owners))
            .before(|_, msg, command_name| {
                println!("Received command '{}' by user '{}'", command_name, msg.author.name);
                return true;
            })
            .after(|_, _, command_name, error| match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            })
            .unrecognised_command(|_, _, unknown_command_name| {
                println!("Could not find command named '{}'", unknown_command_name);
            })
            .on_dispatch_error(|ctx, msg, error| {
                if let DispatchError::Ratelimited(seconds) = error {
                    let _ = msg.channel_id.say(&ctx.http, &format!("Try this again in {} seconds.", seconds));
                }
            })
            .help(&MY_HELP)
            .bucket("utilities", |b| b.delay(5))
            .group(&GENERAL_GROUP)
            .group(&GAMEWORDS_GROUP),
    );

    // Start a single shard, and start listening to events.
    match client.start() {
        Ok(()) => {
            println!("Client started");
        }
        Err(e) => {
            println!("Client error: {:?}", e);
        }
    }
}
