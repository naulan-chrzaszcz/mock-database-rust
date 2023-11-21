use diesel::result::Error as DieselError;

use crate::api::repositories::token::TokenRepository;

use super::models::token::Token;


impl TokenRepository for Token 
{
    fn insert(_token: String) -> Result<(), DieselError> 
    {
        println!("database side");
        Ok(())
    }
}

