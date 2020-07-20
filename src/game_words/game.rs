/* ============================================================================================================= */
/* PACKAGES                                                                                                      */
/* ============================================================================================================= */
use serenity::prelude::TypeMapKey;

/* ============================================================================================================= */
/* STRUCTS                                                                                                       */
/* ============================================================================================================= */
pub struct Game {
    pub players: Vec<String>,
    pub words: Vec<String>,
}

pub struct GameWords;

impl TypeMapKey for GameWords {
    type Value = Game;
}