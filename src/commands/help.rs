/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
use serenity::client::Context;
use serenity::framework::standard::help_commands;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandGroup;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::HelpOptions;
use serenity::model::id::UserId;
use serenity::model::prelude::Message;

use std::collections::HashSet;

/* ============================================================================================================= */
/* COMMAND                                                                                                       */
/* ============================================================================================================= */
#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
pub fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}
