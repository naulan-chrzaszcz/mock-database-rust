use crate::{database::models::token::Token as TokenRepository, api::models::token::Token};

use super::mock::in_memory_token_repository::InMemoryTokenRepository;


#[test]
fn test_mock_side() {
    let _ = Token::create::<InMemoryTokenRepository>();
}

#[test]
fn test_database_side() {
    let _ = Token::create::<TokenRepository>();
}
