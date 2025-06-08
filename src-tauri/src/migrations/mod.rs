use sea_orm_migration::prelude::*;

pub mod m20240101_000001_create_tables;
pub mod m20240102_000001_add_cascade_constraints;
pub mod m20240103_000001_add_game_headers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_tables::Migration),
            Box::new(m20240102_000001_add_cascade_constraints::Migration),
            Box::new(m20240103_000001_add_game_headers_table::Migration),
        ]
    }
}
