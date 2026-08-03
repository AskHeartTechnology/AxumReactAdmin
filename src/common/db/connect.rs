use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Schema,
};

use crate::config::AppConfig;

pub async fn init(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(config.database.url.clone());

    opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    let is_not_production = config.server.mode != "production";
    if is_not_production {
        let _ = create_tables(&db).await;
    }

    Ok(db)
}

async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    // 按依赖顺序创建表；有外键时先建被引用表
    let entities = vec![schema.create_table_from_entity(crate::models::users::Entity)];

    for mut entity in entities {
        if matches!(
            builder,
            DbBackend::Postgres | DbBackend::Sqlite | DbBackend::MySql
        ) {
            entity.if_not_exists();
        }
        db.execute_raw(builder.build(&entity)).await?;
    }

    Ok(())
}
