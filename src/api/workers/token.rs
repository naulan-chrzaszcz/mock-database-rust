use crate::api::{models::token::Token, repositories::token::TokenRepository};

use super::errors::TokenError;

impl Token 
{
    pub fn create<T>() -> Result<Token, TokenError>
    where
        T: TokenRepository,
    {
        match T::insert(String::from("a_token_very_unique!!!!")) {
            Err(_) => Err(TokenError::InternalServer),
            Ok(_) => Ok(Token {
                token: String::from("a_token_very_unique!!!!")
            })
        }
    }
} 

