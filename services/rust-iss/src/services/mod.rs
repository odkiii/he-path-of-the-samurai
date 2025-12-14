use crate::config::AppState;
use crate::repositories::IssRepository;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde_json::Value;
use std::time::Duration;

pub struct IssService;

impl IssService {
    pub async fn fetch_and_store_iss(pool: &sqlx::PgPool, url: &str) -> anyhow::Result<()> {
        let client = reqwest::Client::builder().timeout(Duration::from_secs(20)).build()?;
        let resp = client.get(url).send().await?;
        let json: Value = resp.json().await?;
        IssRepository::log_iss_fetch(pool, url, json).await
    }

    pub async fn fetch_and_store_osdr(st: &AppState) -> anyhow::Result<usize> {
        let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
        let resp = client.get(&st.nasa_url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("OSDR request status {}", resp.status());
        }
        let json: Value = resp.json().await?;
        let items = if let Some(a) = json.as_array() { a.clone() }
            else if let Some(v) = json.get("items").and_then(|x| x.as_array()) { v.clone() }
            else if let Some(v) = json.get("results").and_then(|x| x.as_array()) { v.clone() }
            else { vec![json.clone()] };

        let mut written = 0usize;
        for item in items {
            let id = Self::s_pick(&item, &["dataset_id","id","uuid","studyId","accession","osdr_id"]);
            let title = Self::s_pick(&item, &["title","name","label"]);
            let status = Self::s_pick(&item, &["status","state","lifecycle"]);
            let updated = Self::t_pick(&item, &["updated","updated_at","modified","lastUpdated","timestamp"]);
            
            IssRepository::upsert_osdr_item(&st.pool, id, title, status, updated, item).await?;
            written += 1;
        }
        Ok(written)
    }

    pub async fn fetch_apod(st: &AppState) -> anyhow::Result<()> {
        let url = "https://api.nasa.gov/planetary/apod";
        let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
        let mut req = client.get(url).query(&[("thumbs","true")]);
        if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
        let json: Value = req.send().await?.json().await?;
        IssRepository::write_space_cache(&st.pool, "apod", json).await
    }

    pub async fn fetch_neo_feed(st: &AppState) -> anyhow::Result<()> {
        let today = Utc::now().date_naive();
        let start = today - chrono::Days::new(2);
        let url = "https://api.nasa.gov/neo/rest/v1/feed";
        let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
        let mut req = client.get(url).query(&[
            ("start_date", start.to_string()),
            ("end_date", today.to_string()),
        ]);
        if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
        let json: Value = req.send().await?.json().await?;
        IssRepository::write_space_cache(&st.pool, "neo", json).await
    }

    pub async fn fetch_donki(st: &AppState) -> anyhow::Result<()> {
        let _ = Self::fetch_donki_flr(st).await;
        let _ = Self::fetch_donki_cme(st).await;
        Ok(())
    }

    async fn fetch_donki_flr(st: &AppState) -> anyhow::Result<()> {
        let (from,to) = Self::last_days(5);
        let url = "https://api.nasa.gov/DONKI/FLR";
        let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
        let mut req = client.get(url).query(&[("startDate",from),("endDate",to)]);
        if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
        let json: Value = req.send().await?.json().await?;
        IssRepository::write_space_cache(&st.pool, "flr", json).await
    }

    async fn fetch_donki_cme(st: &AppState) -> anyhow::Result<()> {
        let (from,to) = Self::last_days(5);
        let url = "https://api.nasa.gov/DONKI/CME";
        let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
        let mut req = client.get(url).query(&[("startDate",from),("endDate",to)]);
        if !st.nasa_key.is_empty() { req = req.query(&[("api_key",&st.nasa_key)]); }
        let json: Value = req.send().await?.json().await?;
        IssRepository::write_space_cache(&st.pool, "cme", json).await
    }

    pub async fn fetch_spacex_next(st: &AppState) -> anyhow::Result<()> {
        let url = "https://api.spacexdata.com/v4/launches/next";
        let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
        let json: Value = client.get(url).send().await?.json().await?;
        IssRepository::write_space_cache(&st.pool, "spacex", json).await
    }

    fn last_days(n: i64) -> (String,String) {
        let to = Utc::now().date_naive();
        let from = to - chrono::Days::new(n as u64);
        (from.to_string(), to.to_string())
    }

    fn s_pick(v: &Value, keys: &[&str]) -> Option<String> {
        for k in keys {
            if let Some(x) = v.get(*k) {
                if let Some(s) = x.as_str() { if !s.is_empty() { return Some(s.to_string()); } }
                else if x.is_number() { return Some(x.to_string()); }
            }
        }
        None
    }

    fn t_pick(v: &Value, keys: &[&str]) -> Option<DateTime<Utc>> {
        for k in keys {
            if let Some(x) = v.get(*k) {
                if let Some(s) = x.as_str() {
                    if let Ok(dt) = s.parse::<DateTime<Utc>>() { return Some(dt); }
                    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                        return Some(Utc.from_utc_datetime(&ndt));
                    }
                } else if let Some(n) = x.as_i64() {
                    return Some(Utc.timestamp_opt(n, 0).single().unwrap_or_else(Utc::now));
                }
            }
        }
        None
    }
}