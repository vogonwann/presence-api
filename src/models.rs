use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserItem {
    pub id: usize,
    pub name: String,
    pub email: String,
    pub password: String,
    pub status: bool,
    pub phone: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserItemUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub status: Option<bool>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserItemLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserItemLoginResponse {
    pub id: usize,
    pub name: String,
    pub email: String,
    pub status: bool,
    pub phone: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct WeekItem {
    pub id: usize,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub president: UserItem,
    pub vice_president: UserItem,
    pub hygienist: UserItem,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct WeekItemUpdate {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub president: Option<UserItem>,
    pub vice_president: Option<UserItem>,
    pub hygienist: Option<UserItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDay {
    pub id: usize,
    pub user: UserItem,
    pub day: DateTime<Utc>,
    pub present: bool,
    pub comment: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDayUpdate {
    pub user: Option<UserItem>,
    pub day: Option<DateTime<Utc>>,
    pub present: Option<bool>,
    pub comment: Option<String>,
}