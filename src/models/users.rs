use serde::{Deserialize, Serialize};

/**
*   ======================== Database Schema ========================
* */
#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

/**
*   ======================== Route Parameters ========================
* */
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
