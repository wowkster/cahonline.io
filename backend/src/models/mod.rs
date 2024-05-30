use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use serde::de::DeserializeOwned;

use crate::error::Result;

pub mod session;

pub trait Model
where
    Self: Sized + DeserializeOwned + Unpin + Send + Sync,
{
    const COLLECTION_NAME: &'static str;

    fn id(&self) -> ObjectId;

    fn get_collection(db: &Database) -> Collection<Self> {
        db.collection::<Self>(Self::COLLECTION_NAME)
    }

    async fn from_id(db: &Database, id: ObjectId) -> Result<Option<Self>> {
        let user = Self::get_collection(db)
            .find_one(
                doc! {
                    "_id": id
                },
                None,
            )
            .await?;

        Ok(user)
    }
}
