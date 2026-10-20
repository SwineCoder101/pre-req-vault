use anchor_lang::prelude::*;

#[constant]
pub const COUNTER_SEED: &[u8] = b"counter";

/// GitHub handle registered with the Turbin3 application account.
#[constant]
pub const GITHUB_ID: &str = "Swinecoder101";

#[constant]
pub const HELLO_WORLD_LAMPORTS: u64 = 1;

#[constant]
pub const MAX_COUNT: u64 = 10;
