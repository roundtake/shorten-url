use crate::entities::{prelude::*, shorten_url::ActiveModel, *};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

#[derive(Debug, Clone)]
pub struct ShortenUrlRepository {
    pub db_conn: DatabaseConnection,
}

impl ShortenUrlRepository {
    /// Find a record with the same shorten ID
    pub async fn find_by_shorten_id(&self, shorten_id: &str) -> Result<Option<shorten_url::Model>, DbErr> {
        ShortenUrl::find()
            .filter(shorten_url::Column::ShortenId.eq(shorten_id))
            .one(&self.db_conn)
            .await
    }

    /// Find all records with the same original URL
    pub async fn find_all_by_original_url(&self, original_url: &str) -> Result<Vec<shorten_url::Model>, DbErr> {
        ShortenUrl::find()
            .filter(shorten_url::Column::OriginalUrl.eq(original_url))
            .all(&self.db_conn)
            .await
    }

    /// Get all records from the database
    pub async fn get_all(&self) -> Result<Vec<shorten_url::Model>, DbErr> {
        ShortenUrl::find()
            .all(&self.db_conn)
            .await
    }

    /// Create a new record with the given shorten ID and original URL
    pub async fn create(&self, shorten_id: &str, original_url: &str) -> Result<(), DbErr> {
        // TODO: check whether the shorten ID already exists
        let existing_shorten_url = ShortenUrl::find()
            .filter(shorten_url::Column::ShortenId.eq(shorten_id))
            .one(&self.db_conn)
            .await?;

        if existing_shorten_url.is_some() {
            return Err(DbErr::Custom("Shorten ID already exists".into()));
        }

        // create a new record
        let new_shorten_url = shorten_url::ActiveModel {
            shorten_id: ActiveValue::Set(shorten_id.to_owned()),
            original_url: ActiveValue::Set(original_url.to_owned()),
            ..Default::default()
        };
        
        let res = ShortenUrl::insert(new_shorten_url)
            .exec(&self.db_conn)
            .await?;

        Ok(())
    }

    /// Delete a record with the same shorten ID
    pub async fn delete_by_shorten_id(&self, shorten_id: &str) -> Result<(), DbErr> {
        let to_be_delete: Option<shorten_url::Model> = ShortenUrl::find()
            .filter(shorten_url::Column::ShortenId.eq(shorten_id))
            .one(&self.db_conn)
            .await?;

        let to_be_delete: ActiveModel = to_be_delete.unwrap().into();
        ShortenUrl::delete(to_be_delete)
            .exec(&self.db_conn)
            .await?;

        Ok(())
    }

    /// Delete all records with the same original URL
    pub async fn delete_by_original_url(&self, original_url: &str) -> Result<(), DbErr> {
        ShortenUrl::delete_many()
            .filter(shorten_url::Column::OriginalUrl.eq(original_url.to_string()))
            .exec(&self.db_conn)
            .await?;
        Ok(())
    }
}
