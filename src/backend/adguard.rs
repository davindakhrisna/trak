#[cfg(feature = "ssr")]
use std::time::Duration;

#[cfg(feature = "ssr")]
pub async fn test_adguard_connection(
    url: &str,
    username: &str,
    password: &str,
) -> Result<String, String> {
    let clean_url = url.trim().trim_end_matches('/');
    if clean_url.is_empty() {
        return Err("AdGuard Home URL is empty.".to_string());
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .map_err(|e| e.to_string())?;

    let res = client
        .get(format!("{}/control/status", clean_url))
        .basic_auth(username.trim(), Some(password))
        .send()
        .await
        .map_err(|e| format!("Failed to reach AdGuard Home: {}", e))?;

    if res.status().is_success() {
        Ok("Connected to AdGuard Home successfully!".to_string())
    } else {
        Err(format!("AdGuard Home returned status {}", res.status()))
    }
}

#[cfg(feature = "ssr")]
pub async fn apply_adguard_lockdown(
    url: &str,
    username: &str,
    password: &str,
    _client_id: &str,
    services: &[String],
    locked: bool,
) -> Result<(), String> {
    let clean_url = url.trim().trim_end_matches('/');
    if clean_url.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .map_err(|e| e.to_string())?;

    let blocked_list: Vec<&str> = if locked {
        services.iter().map(|s| s.as_str()).collect()
    } else {
        Vec::new()
    };

    let res = client
        .post(format!("{}/control/blocked_services/update", clean_url))
        .basic_auth(username.trim(), Some(password))
        .json(&blocked_list)
        .send()
        .await
        .map_err(|e| format!("AdGuard update error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!(
            "AdGuard update failed with status {}",
            res.status()
        ));
    }

    Ok(())
}
