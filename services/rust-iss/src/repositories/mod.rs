use sqlx::{PgPool, Row};
use serde_json::Value;
use chrono::{DateTime, Utc};
use crate::domain::SpaceCacheItem;

pub struct IssRepository;

impl IssRepository {
    pub async fn init_db(pool: &PgPool) -> anyhow::Result<()> {
        // ISS
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS iss_fetch_log(
                id BIGSERIAL PRIMARY KEY,
                fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                source_url TEXT NOT NULL,
                payload JSONB NOT NULL
            )"
        ).execute(pool).await?;

        // OSDR
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS osdr_items(
                id BIGSERIAL PRIMARY KEY,
                dataset_id TEXT,
                title TEXT,
                status TEXT,
                updated_at TIMESTAMPTZ,
                inserted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                raw JSONB NOT NULL
            )"
        ).execute(pool).await?;
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS ux_osdr_dataset_id
             ON osdr_items(dataset_id) WHERE dataset_id IS NOT NULL"
        ).execute(pool).await?;

        // Space Cache
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS space_cache(
                id BIGSERIAL PRIMARY KEY,
                source TEXT NOT NULL,
                fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                payload JSONB NOT NULL
            )"
        ).execute(pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS ix_space_cache_source ON space_cache(source,fetched_at DESC)").execute(pool).await?;

        Ok(())
    }

    pub async fn log_iss_fetch(pool: &PgPool, url: &str, payload: Value) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO iss_fetch_log (source_url, payload) VALUES ($1, $2)")
            .bind(url).bind(payload).execute(pool).await?;
        Ok(())
    }

    pub async fn get_last_iss(pool: &PgPool) -> anyhow::Result<Option<(i64, DateTime<Utc>, String, Value)>> {
        let row_opt = sqlx::query(
            "SELECT id, fetched_at, source_url, payload
             FROM iss_fetch_log
             ORDER BY id DESC LIMIT 1"
        ).fetch_optional(pool).await?;

        if let Some(row) = row_opt {
            Ok(Some((
                row.get("id"),
                row.get("fetched_at"),
                row.get("source_url"),
                row.try_get("payload").unwrap_or(serde_json::json!({})),
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn get_iss_trend_data(pool: &PgPool) -> anyhow::Result<Vec<(DateTime<Utc>, Value)>> {
        let rows = sqlx::query("SELECT fetched_at, payload FROM iss_fetch_log ORDER BY id DESC LIMIT 2")
            .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| (r.get("fetched_at"), r.get("payload"))).collect())
    }

    pub async fn upsert_osdr_item(pool: &PgPool, dataset_id: Option<String>, title: Option<String>, status: Option<String>, updated_at: Option<DateTime<Utc>>, raw: Value) -> anyhow::Result<()> {
        if let Some(ds) = dataset_id {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)
                 ON CONFLICT (dataset_id) DO UPDATE
                 SET title=EXCLUDED.title, status=EXCLUDED.status,
                     updated_at=EXCLUDED.updated_at, raw=EXCLUDED.raw"
            ).bind(ds).bind(title).bind(status).bind(updated_at).bind(raw).execute(pool).await?;
        } else {
            sqlx::query(
                "INSERT INTO osdr_items(dataset_id, title, status, updated_at, raw)
                 VALUES($1,$2,$3,$4,$5)"
            ).bind::<Option<String>>(None).bind(title).bind(status).bind(updated_at).bind(raw).execute(pool).await?;
        }
        Ok(())
    }

    pub async fn get_osdr_list(pool: &PgPool, limit: i64) -> anyhow::Result<Vec<Value>> {
        let rows = sqlx::query(
            "SELECT id, dataset_id, title, status, updated_at, inserted_at, raw
             FROM osdr_items
             ORDER BY inserted_at DESC
             LIMIT $1"
        ).bind(limit).fetch_all(pool).await?;

        Ok(rows.into_iter().map(|r| {
            serde_json::json!({
                "id": r.get::<i64,_>("id"),
                "dataset_id": r.get::<Option<String>,_>("dataset_id"),
                "title": r.get::<Option<String>,_>("title"),
                "status": r.get::<Option<String>,_>("status"),
                "updated_at": r.get::<Option<DateTime<Utc>>,_>("updated_at"),
                "inserted_at": r.get::<DateTime<Utc>, _>("inserted_at"),
                "raw": r.get::<Value,_>("raw"),
            })
        }).collect())
    }
    
    pub async fn get_osdr_count(pool: &PgPool) -> anyhow::Result<i64> {
        let count: i64 = sqlx::query("SELECT count(*) AS c FROM osdr_items")
            .fetch_one(pool).await.map(|r| r.get::<i64,_>("c")).unwrap_or(0);
        Ok(count)
    }

    pub async fn write_space_cache(pool: &PgPool, source: &str, payload: Value) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO space_cache(source, payload) VALUES ($1,$2)")
            .bind(source).bind(payload).execute(pool).await?;
        Ok(())
    }

    pub async fn get_latest_space_cache(pool: &PgPool, source: &str) -> anyhow::Result<Option<SpaceCacheItem>> {
        let row = sqlx::query(
            "SELECT fetched_at, payload FROM space_cache
             WHERE source = $1 ORDER BY id DESC LIMIT 1"
        ).bind(source).fetch_optional(pool).await?;

        if let Some(r) = row {
            Ok(Some(SpaceCacheItem {
                source: source.to_string(),
                fetched_at: r.get("fetched_at"),
                payload: r.get("payload"),
            }))
        } else {
            Ok(None)
        }
    }
}