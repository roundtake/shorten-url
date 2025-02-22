use crate::entities::{prelude::*, shorten_url};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Debug, Clone)]
pub struct ShortenUrlRepository {
    pub db_conn: DatabaseConnection,
}

impl ShortenUrlRepository {
    /// Find a record with the same shorten ID
    pub async fn find_by_shorten_id(&self, shorten_id: &str) -> Result<shorten_url::Model> {
        ShortenUrl::find()
            .filter(ShortenUrl::shorten_id.eq(shorten_id))
            .one(&self.db_conn)
            .await
    }

    /// Find all records with the same original URL
    pub async fn find_all_by_original_url(&self, original_url: &str) -> Result<Vec<shorten_url::Model>> {
        ShortenUrl::find()
            .filter(ShortenUrl::original_url.eq(original_url))
            .all(&self.db_conn)
            .await
    }

    /// Get all records from the database
    pub async fn get_all(&self) -> Result<Vec<shorten_url::Model>, sea_orm::error::DbErr> {
        ShortenUrl::find()
            .all(&self.db_conn)
            .await
            // .expect("Error while fetching all shortened URLs")
            // .into_iter()
            // .map(|model| model.shorten_id)
            // .collect()
    }

    /// Create a new record with the given shorten ID and original URL
    pub async fn create(&self, shorten_id: &str, original_url: &str) -> Result<()> {
        let new_shorten_url = shorten_url::ActiveModel {
            shorten_id: Set(shorten_id.to_owned()),
            original_url: Set(original_url.to_owned()),
            ..Default::default()
        };
        
        ShortenUrl::insert(new_shorten_url)
            .exec(&self.db_conn)
            .await
    }

    /// Delete a record with the same shorten ID
    pub async fn delete_by_shorten_id(&self, shorten_id: &str) -> Result<()> {
        let to_be_delete: Option<shorten_url::Model> = ShortenUrl::find()
            .filter(shorten_url::Column::ShortenId.eq(shorten_id))
            .one(&self.db_conn)
            .await?;

        let to_be_delete = to_be_delete.unwrap();
        ShortenUrl::delete(to_be_delete)
            .exec(&self.db_conn)
            .await
    }

    /// Delete all records with the same original URL
    pub async fn delete_by_original_url(&self, original_url: &str) -> Result<(), sea_orm::error::ExecResult> {
        ShortenUrl::delete_many()
            .filter(shorten_url::Column::OriginalUrl.eq(original_url.to_string()))
            .exec(&self.db_conn)
            .await
    }
}
