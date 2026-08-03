use uuid::Uuid;

use crate::{
    common::error::AppError,
    dto::users::{CreateUser, UpdateUser},
    models::users::Model as User,
};

use std::sync::{LazyLock, Mutex};

static USERS: LazyLock<Mutex<Vec<User>>> = LazyLock::new(|| {
    Mutex::new(vec![
        User {
            id: Uuid::new_v4(),
            username: "Tom".into(),
            password: "123456".into(),
            email: Some("tom@test.com".into()),
            status: 1,
            ..Default::default()
        },
        User {
            id: Uuid::new_v4(),
            username: "Amy".into(),
            password: "123456".into(),
            email: Some("amy@test.com".into()),
            ..Default::default()
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

pub fn get_user_detail(id: Uuid) -> Result<User, AppError> {
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
    let user = User {
        id: Uuid::new_v4(),
        email: req.email,
        username: req.username,
        ..Default::default()
    };
    users.push(user.clone());
    Ok(user)
}

pub fn update_user(id: Uuid, req: UpdateUser) -> Result<User, AppError> {
    let mut users = lock_users()?;
    let user = users
        .iter_mut()
        .find(|u| u.id == id)
        .ok_or_else(|| AppError::NotFound("user not found".into()))?;

    if let Some(name) = req.username {
        user.password = name;
    }
    if let Some(email) = req.email {
        user.email = Some(email);
    }
    Ok(user.clone())
}

pub fn delete_user(id: Uuid) -> Result<(), AppError> {
    let mut users = lock_users()?;
    let before = users.len();
    users.retain(|u| u.id != id);
    if users.len() == before {
        return Err(AppError::NotFound("user not found".into()));
    }
    Ok(())
}
