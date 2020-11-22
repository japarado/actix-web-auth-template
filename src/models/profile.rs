use crate::schema::profiles;
use serde::{Serialize, Deserialize};
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Profile {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub user_id: i32
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "profiles"]
pub struct NewProfile {
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub user_id: i32,
}
