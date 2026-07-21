use axum::{
    Json,
    extract::{FromRequest, Request},
};

use crate::common::error::AppError;

pub struct AppJson<T>(pub T);

impl<S, T> FromRequest<S> for AppJson<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<S>,
    <Json<T> as FromRequest<S>>::Rejection: std::fmt::Display,
    T: Send,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(AppJson(value)),
            Err(err) => Err(AppError::BadRequest(err.to_string())),
        }
    }
}
