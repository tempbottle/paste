use super::super::schema::api_keys;
use super::users::User;
use crate::models::id::{UserId, ApiKeyId};

#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[primary_key(key)]
#[belongs_to(User)]
pub struct ApiKey {
  pub(crate) key: ApiKeyId,
  pub(crate) user_id: UserId,
  pub(crate) name: String,
}

#[derive(Insertable)]
#[table_name = "api_keys"]
pub struct NewApiKey {
  key: ApiKeyId,
  user_id: UserId,
  name: String,
}

impl NewApiKey {
  pub fn new(name: String, key: ApiKeyId, user_id: UserId) -> Self {
    NewApiKey { name, key, user_id }
  }
}
