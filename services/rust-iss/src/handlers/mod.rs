use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;
use std::collections::HashMap;
use chrono::Utc;
use crate::config::AppState;
use crate::repositories::IssRepository;
use crate::services::IssService;
use crate::domain::{Trend, Health};

pub async fn health_check() -> Json<Health> {
    Json(Health { status: "ok", now: Utc::now() })
}

pub async fn last_iss(State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
    let row_opt = IssRepository::get_last_iss(&st.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some((id, fetched_at, source_url, payload)) = row_opt {
        return Ok(Json(serde_json::json!({
            "id": id, "fetched_at": fetched_at, "source_url": source_url, "payload": payload
        })));
    }
    Ok(Json(serde_json::json!({"message":"no data"})))
}

pub async fn trigger_iss(State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
    IssService::fetch_and_store_iss(&st.pool, &st.fallback_url).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    last_iss(State(st)).await
}

pub async fn iss_trend(State(st): State<AppState>) -> Result<Json<Trend>, (StatusCode, String)> {
    let rows = IssRepository::get_iss_trend_data(&st.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if rows.len() < 2 {
        return Ok(Json(Trend {
            movement: false, delta_km: 0.0, dt_sec: 0.0, velocity_kmh: None,
            from_time: None, to_time: None,
            from_lat: None, from_lon: None, to_lat: None, to_lon: None
        }));
    }

    let (t2, p2) = &rows[0];
    let (t1, p1) = &rows[1];

    let lat1 = num(&p1["latitude"]);
    let lon1 = num(&p1["longitude"]);
    let lat2 = num(&p2["latitude"]);
    let lon2 = num(&p2["longitude"]);
    let v2 = num(&p2["velocity"]);

    let mut delta_km = 0.0;
    let mut movement = false;
    if let (Some(a1), Some(o1), Some(a2), Some(o2)) = (lat1, lon1, lat2, lon2) {
        delta_km = haversine_km(a1, o1, a2, o2);
        movement = delta_km > 0.1;
    }
    let dt_sec = (*t2 - *t1).num_milliseconds() as f64 / 1000.0;

    Ok(Json(Trend {
        movement,
        delta_km,
        dt_sec,
        velocity_kmh: v2,
        from_time: Some(*t1),
        to_time: Some(*t2),
        from_lat: lat1, from_lon: lon1, to_lat: lat2, to_lon: lon2,
    }))
}

pub async fn osdr_sync(State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
    let written = IssService::fetch_and_store_osdr(&st).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "written": written })))
}

pub async fn osdr_list(State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
    let limit = std::env::var("OSDR_LIST_LIMIT").ok()
        .and_then(|s| s.parse::<i64>().ok()).unwrap_or(20);

    let items = IssRepository::get_osdr_list(&st.pool, limit).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "items": items })))
}

pub async fn space_latest(Path(src): Path<String>, State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
    let item_opt = IssRepository::get_latest_space_cache(&st.pool, &src).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(item) = item_opt {
        return Ok(Json(serde_json::json!({ "source": item.source, "fetched_at": item.fetched_at, "payload": item.payload })));
    }
    Ok(Json(serde_json::json!({ "source": src, "message":"no data" })))
}

pub async fn space_refresh(Query(q): Query<HashMap<String,String>>, State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
    let list = q.get("src").cloned().unwrap_or_else(|| "apod,neo,flr,cme,spacex".to_string());
    let mut done = Vec::new();
    for s in list.split(',').map(|x| x.trim().to_lowercase()) {
        match s.as_str() {
            "apod"   => { let _ = IssService::fetch_apod(&st).await;       done.push("apod"); }
            "neo"    => { let _ = IssService::fetch_neo_feed(&st).await;   done.push("neo"); }
            "flr"    => { let _ = IssService::fetch_donki(&st).await;      done.push("flr"); }
            "cme"    => { let _ = IssService::fetch_donki(&st).await;      done.push("cme"); }
            "spacex" => { let _ = IssService::fetch_spacex_next(&st).await; done.push("spacex"); }
            _ => {}
        }
    }
    Ok(Json(serde_json::json!({ "refreshed": done })))
}

pub async fn space_summary(State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
    let apod   = IssRepository::get_latest_space_cache(&st.pool, "apod").await.unwrap_or(None);
    let neo    = IssRepository::get_latest_space_cache(&st.pool, "neo").await.unwrap_or(None);
    let flr    = IssRepository::get_latest_space_cache(&st.pool, "flr").await.unwrap_or(None);
    let cme    = IssRepository::get_latest_space_cache(&st.pool, "cme").await.unwrap_or(None);
    let spacex = IssRepository::get_latest_space_cache(&st.pool, "spacex").await.unwrap_or(None);

    let iss_last = IssRepository::get_last_iss(&st.pool).await.unwrap_or(None);
    let osdr_count = IssRepository::get_osdr_count(&st.pool).await.unwrap_or(0);

    Ok(Json(serde_json::json!({
        "apod": apod.map(|x| serde_json::json!({"at": x.fetched_at, "payload": x.payload})).unwrap_or(serde_json::json!({})),
        "neo": neo.map(|x| serde_json::json!({"at": x.fetched_at, "payload": x.payload})).unwrap_or(serde_json::json!({})),
        "flr": flr.map(|x| serde_json::json!({"at": x.fetched_at, "payload": x.payload})).unwrap_or(serde_json::json!({})),
        "cme": cme.map(|x| serde_json::json!({"at": x.fetched_at, "payload": x.payload})).unwrap_or(serde_json::json!({})),
        "spacex": spacex.map(|x| serde_json::json!({"at": x.fetched_at, "payload": x.payload})).unwrap_or(serde_json::json!({})),
        "iss": iss_last.map(|(_, at, _, p)| serde_json::json!({"at": at, "payload": p})).unwrap_or(serde_json::json!({})),
        "osdr_count": osdr_count
    })))
}

fn num(v: &Value) -> Option<f64> {
    if let Some(x) = v.as_f64() { return Some(x); }
    if let Some(s) = v.as_str() { return s.parse::<f64>().ok(); }
    None
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let rlat1 = lat1.to_radians();
    let rlat2 = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2) + rlat1.cos() * rlat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    6371.0 * c
}