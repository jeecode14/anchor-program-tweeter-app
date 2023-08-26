use anchor_lang::prelude::*;
use std::str::FromStr;

//pub mod errors;
pub mod constant;
pub mod states;

//use crate::constant::*;
use crate::states::*;

declare_id!("ARVmA87wQaxWkstZksLnmcjLmUhHbYYyJF6SNmWX1Cqk");

#[program]
pub mod tweeter_app_program {
    use super::*;

    pub fn initialize_user_info(ctx: Context<InitializeUserInfo>) -> Result<()> {
        let user_info = &mut ctx.accounts.user_info;

        user_info.name = ("").to_string();
        user_info.email = ("").to_string();
        user_info.wallet_address = ctx.accounts.authority.key();
        user_info.tweet_count = 0;

        Ok(())
    }

    pub fn write_tweet(
        ctx: Context<WriteTweet>,
        message: String,
        tweet_id: String,
        user_public_key: String,
    ) -> Result<()> {
        let tweeter = &mut ctx.accounts.tweeter;
        let twcount = &mut ctx.accounts.user_info;

        if tweeter.tweet_id == tweet_id {
            return err!(ErrorCode::CannotUpdateTweet);
        }

        if message.trim().is_empty() {
            return err!(ErrorCode::EmtpyMessage);
        }

        twcount.tweet_count += 1;

        tweeter.message = message;
        tweeter.likes = 0;
        tweeter.tweet_id = tweet_id;
        tweeter.creator = user_public_key;

        msg!("Tweet: {:?}", tweeter.message);
        msg!("Tweet ID: {:?}", tweeter.tweet_id);
        msg!("Tweet Owner: {:?}", tweeter.creator);

        Ok(())
    }

    pub fn like_tweet(
        ctx: Context<TweetLikers>,
        user_liking_tweet: String,
        tweet_id: String,
    ) -> Result<()> {
        let tweet_account = &mut ctx.accounts.tweet_account;
        let new_tweetlike = &mut ctx.accounts.new_tweetlike;

        if tweet_account.message.trim().is_empty() {
            return err!(ErrorCode::NotValidTweet);
        }

        if tweet_account.creator == user_liking_tweet {
            return err!(ErrorCode::OwnerLikedTweet);
        }

        if tweet_account.tweet_id == tweet_id {
            tweet_account.likes += 1;

            new_tweetlike.tweet_id = tweet_account.tweet_id.clone();
            new_tweetlike.likers = user_liking_tweet;

            msg!("Likes: {:?}", tweet_account.likes);
            msg!("Tweet ID: {:?}", tweet_account.tweet_id);
            msg!("Tweet Owner: {:?}", tweet_account.creator);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUserInfo<'info> {
    #[account(
        init,  
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        payer = authority, 
        space = 8 + 2048, 
        
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WriteTweet<'info> {
    #[account(
        init,
        seeds = [TWEET_SEED, authority.key().as_ref(), user_info.key().as_ref(), &[user_info.tweet_count]],
        bump,
        payer = authority, 
        space = 9000,

    )]
    pub tweeter: Account<'info, Tweet>,

    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TweetLikers<'info> {
    #[account(
        init,
        seeds = [(Pubkey::from_str(&tweet_account.tweet_id).unwrap()).as_ref(), &[tweet_account.likes]],
        bump,
        payer = authority, 
        space = 9000,

    )]
    pub new_tweetlike: Account<'info, UserTweetLikers>,

    #[account(mut)]
    pub tweet_account: Account<'info, Tweet>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUserInfo<'info> {
    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,
}

#[derive(Accounts)]
pub struct LikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Tweet message cannot be updated")]
    CannotUpdateTweet,

    #[msg("Message cannot be empty")]
    EmtpyMessage,

    #[msg("Invalid tweet id")]
    InvalidTweetId,

    #[msg("Cannot like a tweet without a valid message")]
    NotValidTweet,

    #[msg("User has already liked the tweet")]
    UserLikedTweet,

    #[msg("Owner is not allowed to like the tweet")]
    OwnerLikedTweet,
}

#[constant]
pub const LIKERS_SEED: &[u8] = b"tweetlover";
pub const TWEET_SEED: &[u8] = b"tweetinsolana";
pub const USER_SEED: &[u8] = b"tweetuser";
