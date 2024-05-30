use bson::{doc, oid::ObjectId, DateTime};
use chrono::{Duration, Utc};
use mongodb::Database;
use serde::{Deserialize, Serialize};

use super::Model;
use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub token: String,
    pub username: String,
    pub created_at: DateTime,
    pub expires_at: DateTime,
    pub ip_address: String,
    pub user_agent: String,
    pub revoked: bool,
}

impl Model for Session {
    const COLLECTION_NAME: &'static str = "sessions";

    fn id(&self) -> ObjectId {
        self.id.unwrap()
    }
}

#[allow(unused)]
impl Session {
    pub async fn new(
        db: &Database,
        username: &str,
        ip_address: &str,
        user_agent: &str,
    ) -> Result<Self> {
        let collection = Self::get_collection(db);

        let created_at = Utc::now();
        let expires_at = created_at + Duration::days(1);

        let session = Session {
            id: None,
            token: nanoid::nanoid!(),
            username: username.to_owned(),
            created_at: created_at.into(),
            expires_at: expires_at.into(),
            ip_address: ip_address.to_owned(),
            user_agent: ip_address.to_owned(),
            revoked: false,
        };

        let inserted = collection.insert_one(session, None).await?;
        let inserted_id = inserted.inserted_id.as_object_id().unwrap();

        let session = Session::from_id(db, inserted_id).await?.unwrap();

        Ok(session)
    }

    pub async fn from_token(db: &Database, token: &str) -> Result<Option<Self>> {
        let collection = Self::get_collection(db);

        let session = collection
            .find_one(
                doc! {
                    "token": token
                },
                None,
            )
            .await?;

        Ok(session)
    }
}
