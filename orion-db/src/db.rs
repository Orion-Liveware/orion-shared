use surrealdb::Surreal;
use surrealdb::engine::any::Any;

use crate::error::CoreError;

/// Type alias for the shared database handle.
/// Uses `Any` to erase the storage engine, so app crates
/// don't depend on a specific backend (SurrealKV, RocksDB, Mem, etc.).
pub type Db = Surreal<Any>;

/// Initialize the database connection.
/// For desktop, pass a `surrealkv://` or `rocksdb://` path.
/// For tests, pass `mem://`.
pub async fn init_db(url: &str) -> Result<Db, CoreError> {
    let db = surrealdb::engine::any::connect(url).await?;
    db.use_ns("app").use_db("main").await?;
    Ok(db)
}

/// Create the `_migration` table for tracking data migrations.
/// Called once at startup before any app-specific schema definitions.
pub async fn init_migration_table(db: &Db) -> Result<(), CoreError> {
    db.query(
        "DEFINE TABLE IF NOT EXISTS _migration SCHEMAFULL;
         DEFINE FIELD IF NOT EXISTS name ON _migration TYPE string;
         DEFINE FIELD IF NOT EXISTS applied_at ON _migration TYPE datetime DEFAULT time::now();
         DEFINE INDEX IF NOT EXISTS idx_migration_name ON _migration FIELDS name UNIQUE;",
    )
    .await?
    .check()
    .map_err(|e| CoreError::QueryFailed(e.to_string()))?;

    tracing::info!("Migration table initialized");
    Ok(())
}

/// Run a SQL schema definition string against the database.
/// Use this in each module's `schema::initialize()` to DRY the pattern:
/// ```rust,ignore
/// pub async fn initialize(db: &Db) -> Result<(), CoreError> {
///     orion_db::db::run_schema(db, include_str!("../schemas/my_table.surql"), "my_module").await
/// }
/// ```
pub async fn run_schema(db: &Db, sql: &str, module_name: &str) -> Result<(), CoreError> {
    db.query(sql)
        .await?
        .check()
        .map_err(|e| CoreError::QueryFailed(e.to_string()))?;

    tracing::info!("{module_name} schema initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db_in_memory() {
        let db = init_db("mem://")
            .await
            .expect("should connect to in-memory db");
        init_migration_table(&db)
            .await
            .expect("should initialize migration table");
    }

    #[tokio::test]
    async fn test_run_schema() {
        let db = init_db("mem://").await.unwrap();
        run_schema(
            &db,
            "DEFINE TABLE IF NOT EXISTS test_table SCHEMAFULL;
             DEFINE FIELD IF NOT EXISTS name ON test_table TYPE string;",
            "test",
        )
        .await
        .unwrap();

        // Verify table exists by inserting
        db.query("CREATE test_table CONTENT { name: 'hello' }")
            .await
            .unwrap();
    }
}
