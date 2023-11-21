use diesel::result::Error as DieselError;

use crate::{ api::repositories::token::TokenRepository, database::models::token::Token };

static mut TOKENS: Vec<Token> = Vec::new();

pub struct InMemoryTokenRepository;
impl TokenRepository for InMemoryTokenRepository 
{
    fn insert(token: String) -> Result<(), DieselError> 
    {
        println!("mock side");
        unsafe { TOKENS.push(Token { token }) };
        Ok(())
    }
}
