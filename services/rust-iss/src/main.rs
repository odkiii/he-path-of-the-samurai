mod config;
mod domain;
mod handlers;
mod repositories;
mod routes;
mod services;

use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use crate::config::{AppState, env_u64};
use crate::repositories::IssRepository;
use crate::services::IssService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

    let nasa_url = std::env::var("NASA_API_URL")
        .unwrap_or_else(|_| "https://visualization.osdr.nasa.gov/biodata/api/v2/datasets/?format=json".to_string());
    let nasa_key = std::env::var("NASA_API_KEY").unwrap_or_default();

    let fallback_url = std::env::var("WHERE_ISS_URL")
        .unwrap_or_else(|_| "https://api.wheretheiss.at/v1/satellites/25544".to_string());

    let every_osdr   = env_u64("FETCH_EVERY_SECONDS", 600);
    let every_iss    = env_u64("ISS_EVERY_SECONDS",   120);
    let every_apod   = env_u64("APOD_EVERY_SECONDS",  43200); // 12ч
    let every_neo    = env_u64("NEO_EVERY_SECONDS",   7200);  // 2ч
    let every_donki  = env_u64("DONKI_EVERY_SECONDS", 3600);  // 1ч
    let every_spacex = env_u64("SPACEX_EVERY_SECONDS",3600);

    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    IssRepository::init_db(&pool).await?;

    let state = AppState {
        pool: pool.clone(),
        nasa_url: nasa_url.clone(),
        nasa_key,
        fallback_url: fallback_url.clone(),
        every_osdr, every_iss, every_apod, every_neo, every_donki, every_spacex,
    };

    // Background Tasks
    // OSDR
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = IssService::fetch_and_store_osdr(&st).await { error!("osdr err {e:?}") }
                tokio::time::sleep(Duration::from_secs(st.every_osdr)).await;
            }
        });
    }
    // ISS
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = IssService::fetch_and_store_iss(&st.pool, &st.fallback_url).await { error!("iss err {e:?}") }
                tokio::time::sleep(Duration::from_secs(st.every_iss)).await;
            }
        });
    }
    // APOD
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = IssService::fetch_apod(&st).await { error!("apod err {e:?}") }
                tokio::time::sleep(Duration::from_secs(st.every_apod)).await;
            }
        });
    }
    // NeoWs
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = IssService::fetch_neo_feed(&st).await { error!("neo err {e:?}") }
                tokio::time::sleep(Duration::from_secs(st.every_neo)).await;
            }
        });
    }
    // DONKI
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = IssService::fetch_donki(&st).await { error!("donki err {e:?}") }
                tokio::time::sleep(Duration::from_secs(st.every_donki)).await;
            }
        });
    }
    // SpaceX
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = IssService::fetch_spacex_next(&st).await { error!("spacex err {e:?}") }
                tokio::time::sleep(Duration::from_secs(st.every_spacex)).await;
            }
        });
    }

    let app = routes::app_router(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("rust_iss listening on 0.0.0.0:3000");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}