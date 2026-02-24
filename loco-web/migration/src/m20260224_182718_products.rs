use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "products",
            &[
            
            ("id", ColType::PkAuto),
            
            ("name", ColType::StringNull),
            ("slug", ColType::StringNull),
            ("description", ColType::TextNull),
            ("price", ColType::DecimalNull),
            ("currency", ColType::StringNull),
            ("sku", ColType::StringNull),
            ("stock", ColType::IntegerNull),
            ("is_active", ColType::BooleanNull),
            ("image_url", ColType::StringNull),
            ("weight", ColType::DecimalNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "products").await
    }
}
