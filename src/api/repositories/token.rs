use diesel::result::Error as DieselError;

pub trait TokenRepository 
{
    fn insert(token: String) -> Result<(), DieselError>;
}

