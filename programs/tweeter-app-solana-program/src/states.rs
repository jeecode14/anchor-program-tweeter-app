use anchor_lang::prelude::*;

//use borsh::{BorshDeserialize, BorshSerialize};

#[account]
#[derive(Default)]
pub struct UserInfo {
    pub name: String,
    pub email: String,
    pub wallet_address: Pubkey,
    pub tweet_count: u8,
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct Tweet {
    pub message: String,
    pub likes: u8,
    pub creator: String,
    pub tweet_id: String,
}

#[account]
#[derive(Default)]
pub struct UserTweetLikers {
    pub likers: String,
    pub tweet_id: String,
}
