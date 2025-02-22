use std::env;
use rand::Rng;
use sea_orm::{Database, DatabaseConnection, DbErr};
use crate::repository::ShortenUrlRepository;

const AVAILABLE_CHARS: [char; 57] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q',
    'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', '2', '3', '4', '5', '6', '7', '8', '9', '-'
];

/// Generate a random ID with the given length
pub fn gen_random_id(length: i32) -> String {
    let mut rng = rand::rng();
    let random_id: String = (0..length)
        .map(|_| rng.random_range(0..AVAILABLE_CHARS.len()) as usize)
        .map(|index| AVAILABLE_CHARS[index])
        .collect();
    random_id
}

/// Get a database connection
pub async fn get_db_conn() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@localhost:5432/shorten_url".to_string());

    let db_conn = Database::connect(database_url).await?;
    Ok(db_conn)
}

/// Get a repository
pub async fn get_repo() -> Result<ShortenUrlRepository, DbErr> {
    let db_conn = get_db_conn().await?;
    Ok(ShortenUrlRepository { db_conn })
}
