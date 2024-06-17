use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub is_staff: bool,
    pub is_active: bool,
    pub date_joined: String,
    pub last_login: Option<String>,
    pub updated_at: String,
    pub password: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "content"]
pub struct Content {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: Uuid,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "assets"]
pub struct Asset {
    pub id: Uuid,
    pub content_id: Uuid,
    pub image_url: String,
}
