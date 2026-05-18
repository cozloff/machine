use sea_orm::{
    ActiveModelBehavior, ActiveValue::Set, ConnectionTrait, DatabaseConnection, DbErr,
    DeriveEntityModel, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Schema,
    TransactionTrait, entity::prelude::*, sea_query::OnConflict,
};

pub struct SeedSqlite {
    database: SqliteDatabase,
}

impl SeedSqlite {
    pub fn new(database: SqliteDatabase) -> Self {
        Self { database }
    }

    pub async fn seed(&self) -> Result<(), DbErr> {
        let db = self.database.connect().await?;

        // Check if database already exists
        

        Ok(())
    }
}