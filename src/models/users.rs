use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/**
*   ======================== Database Schema ========================
* */
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

/**
*   ======================== Route Parameters ========================
* */
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
