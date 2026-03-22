//! Domain-scoped CRUD helpers for apps using the `domain -> contains -> record` pattern.
//!
//! These functions extract the identical boilerplate that appears in every domain-scoped
//! module repository: linking records to domains, listing by domain, deleting with edge cleanup,
//! and parsing record IDs from strings.
//!
//! Not applicable to apps that use hierarchical edges (e.g., `series_has_project`) instead
//! of the flat domain-scoping pattern.

use serde::de::DeserializeOwned;
use surrealdb::RecordId;

use crate::db::Db;
use crate::error::CoreError;

/// Link a record to a domain via a `contains` edge.
///
/// Call this after creating a record to scope it to a domain.
pub async fn relate_to_domain(
    db: &Db,
    domain_id: &str,
    record_id: &RecordId,
) -> Result<(), CoreError> {
    let domain_rid = RecordId::from(("domain", domain_id));
    db.query("RELATE $from->contains->$to")
        .bind(("from", domain_rid))
        .bind(("to", record_id.clone()))
        .await?
        .check()
        .map_err(|e| CoreError::QueryFailed(format!("Failed to link record to domain: {e}")))?;
    Ok(())
}

/// List all records of `table` scoped to a domain via reverse graph traversal.
///
/// The `order_by` parameter is appended directly to the query (e.g., `"updated_at DESC"`
/// or `"sort_order ASC"`).
pub async fn list_by_domain<T: DeserializeOwned>(
    db: &Db,
    table: &str,
    domain_id: &str,
    order_by: &str,
) -> Result<Vec<T>, CoreError> {
    let domain_rid = RecordId::from(("domain", domain_id));
    let query = format!(
        "SELECT * FROM {table} WHERE <-contains<-domain CONTAINS $domain ORDER BY {order_by}"
    );
    let mut result = db.query(&query).bind(("domain", domain_rid)).await?;
    let records: Vec<T> = result.take(0)?;
    Ok(records)
}

/// Delete a record and all its `contains` edges.
///
/// Removes edges first, then deletes the record itself.
pub async fn delete_with_edges(db: &Db, table: &str, id: &str) -> Result<(), CoreError> {
    let (tb, key) = parse_record_id(table, id);
    db.query(
        "DELETE FROM contains WHERE out = type::thing($tb, $id);
         DELETE type::thing($tb, $id);",
    )
    .bind(("tb", tb))
    .bind(("id", key))
    .await?;
    Ok(())
}

/// Parse a string ID into (table, key) components.
///
/// Handles both `"table:key"` and bare `"key"` formats.
/// The `default_table` is used when the string doesn't contain a colon.
pub fn parse_record_id(default_table: &str, id: &str) -> (String, String) {
    if let Some(key) = id.strip_prefix(&format!("{default_table}:")) {
        (default_table.to_string(), key.to_string())
    } else {
        (default_table.to_string(), id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{init_db, init_migration_table, run_schema};

    async fn test_db() -> Db {
        let db = init_db("mem://").await.unwrap();
        init_migration_table(&db).await.unwrap();
        run_schema(
            &db,
            "DEFINE TABLE IF NOT EXISTS domain SCHEMAFULL;
             DEFINE FIELD IF NOT EXISTS label ON domain TYPE string;
             DEFINE TABLE IF NOT EXISTS contains TYPE RELATION SCHEMAFULL;
             DEFINE TABLE IF NOT EXISTS test_item SCHEMAFULL;
             DEFINE FIELD IF NOT EXISTS title ON test_item TYPE string;
             DEFINE FIELD IF NOT EXISTS created_at ON test_item TYPE datetime DEFAULT time::now();",
            "test",
        )
        .await
        .unwrap();
        db
    }

    #[derive(Debug, serde::Deserialize)]
    struct TestDomain {
        id: RecordId,
    }

    async fn seed_domain(db: &Db) -> String {
        let mut result = db
            .query("CREATE domain CONTENT { label: 'Test Domain' }")
            .await
            .unwrap();
        let domain: Option<TestDomain> = result.take(0).unwrap();
        domain.unwrap().id.key().to_string()
    }

    #[derive(Debug, serde::Deserialize)]
    struct TestItem {
        id: RecordId,
        title: String,
    }

    #[tokio::test]
    async fn relate_and_list_by_domain() {
        let db = test_db().await;
        let domain_id = seed_domain(&db).await;

        // Create a record
        let mut result = db
            .query("CREATE test_item CONTENT { title: 'Hello' }")
            .await
            .unwrap();
        let item: Option<TestItem> = result.take(0).unwrap();
        let item = item.unwrap();

        // Relate to domain
        relate_to_domain(&db, &domain_id, &item.id).await.unwrap();

        // List by domain
        let items: Vec<TestItem> =
            list_by_domain(&db, "test_item", &domain_id, "created_at DESC")
                .await
                .unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Hello");
    }

    #[tokio::test]
    async fn delete_removes_edges_and_record() {
        let db = test_db().await;
        let domain_id = seed_domain(&db).await;

        let mut result = db
            .query("CREATE test_item CONTENT { title: 'To Delete' }")
            .await
            .unwrap();
        let item: Option<TestItem> = result.take(0).unwrap();
        let item = item.unwrap();

        relate_to_domain(&db, &domain_id, &item.id).await.unwrap();
        delete_with_edges(&db, "test_item", &item.id.to_string())
            .await
            .unwrap();

        let items: Vec<TestItem> =
            list_by_domain(&db, "test_item", &domain_id, "created_at DESC")
                .await
                .unwrap();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn parse_record_id_with_prefix() {
        let (tb, key) = parse_record_id("note", "note:abc123");
        assert_eq!(tb, "note");
        assert_eq!(key, "abc123");
    }

    #[test]
    fn parse_record_id_without_prefix() {
        let (tb, key) = parse_record_id("note", "abc123");
        assert_eq!(tb, "note");
        assert_eq!(key, "abc123");
    }
}
