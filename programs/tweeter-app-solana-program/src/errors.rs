use anchor_lang::prelude::*;

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
}
