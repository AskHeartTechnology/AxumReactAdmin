use crate::{
    common::error::AppError,
    models::users::{CreateUser, UpdateUser, User},
};

use std::sync::{LazyLock, Mutex};

static USERS: LazyLock<Mutex<Vec<User>>> = LazyLock::new(|| {
    Mutex::new(vec![
        User {
            id: 1,
            email: "tom@test.com".into(),
            name: "Tom".into(),
        },
        User {
            id: 2,
            email: "amy@test.com".into(),
            name: "Amy".into(),
        },
    ])
});

fn lock_users() -> Result<std::sync::MutexGuard<'static, Vec<User>>, AppError> {
    USERS
        .lock()
        .map_err(|_| AppError::Internal("users lock poisoned".into()))
}

pub fn get_user_list() -> Result<Vec<User>, AppError> {
    Ok(lock_users()?.clone())
}

pub fn get_user_detail(id: u64) -> Result<User, AppError> {
    lock_users()?
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .ok_or_else(|| AppError::NotFound("user not found".into()))
}

pub fn create_user(req: CreateUser) -> Result<User, AppError> {
    let mut users = lock_users()?;
    if users.iter().any(|u| u.email == req.email) {
        return Err(AppError::BadRequest("email exists".into()));
    }
    let id = users.iter().map(|u| u.id).max().unwrap_or(0) + 1;
    let user = User {
        id,
        email: req.email,
        name: req.name,
    };
    users.push(user.clone());
    Ok(user)
}

pub fn update_user(id: u64, req: UpdateUser) -> Result<User, AppError> {
    let mut users = lock_users()?;
    let user = users
        .iter_mut()
        .find(|u| u.id == id)
        .ok_or_else(|| AppError::NotFound("user not found".into()))?;

    if let Some(name) = req.name {
        user.name = name;
    }
    if let Some(email) = req.email {
        user.email = email;
    }
    Ok(user.clone())
}

pub fn delete_user(id: u64) -> Result<(), AppError> {
    let mut users = lock_users()?;
    let before = users.len();
    users.retain(|u| u.id != id);
    if users.len() == before {
        return Err(AppError::NotFound("user not found".into()));
    }
    Ok(())
}
