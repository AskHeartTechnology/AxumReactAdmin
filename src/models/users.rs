use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "ara_user", comment = "用户表")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, comment = "用户ID")]
    pub id: Uuid,

    #[sea_orm(unique, comment = "用户名")]
    pub username: String,

    #[sea_orm(unique, comment = "用户邮箱")]
    pub email: Option<String>,

    #[sea_orm(comment = "用户密码")]
    pub password: String,

    #[sea_orm(comment = "用户状态")]
    pub status: i16,

    #[sea_orm(comment = "用户昵称")]
    pub nickname: Option<String>,

    #[sea_orm(comment = "用户头像")]
    pub avatar: Option<String>,

    #[sea_orm(comment = "用户手机号")]
    pub phone: Option<String>,

    #[sea_orm(comment = "创建时间")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(comment = "创建人ID")]
    pub created_by: Option<i64>,

    #[sea_orm(comment = "更新时间")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(comment = "更新人ID")]
    pub updated_by: Option<i64>,

    #[sea_orm(comment = "是否已删除")]
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username: String::new(),
            password: String::new(),
            status: 1,
            email: None,
            nickname: None,
            avatar: None,
            phone: None,
            created_at: now,
            created_by: None,
            updated_at: now,
            updated_by: None,
            is_deleted: false,
        }
    }
}
