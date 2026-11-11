#[cfg(feature = "ssr")]
pub async fn send_ntfy_notification(
    server: &str,
    topic: &str,
    title: &str,
    message: &str,
    priority: u8,
    tags: &[&str],
) -> Result<(), String> {
    use crate::backend::security::validate_outbound_url;

    let clean_server = validate_outbound_url(server)?;
    let clean_topic = topic.trim();
    if clean_topic.is_empty() || clean_topic.len() > 64 {
        return Err("Ntfy topic must be between 1 and 64 characters.".to_string());
    }
    if !clean_topic
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Ntfy topic contains invalid characters.".to_string());
    }

    let url = format!("{}/{}", clean_server, clean_topic);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let tags_str = tags.join(",");
    let mut req = client
        .post(&url)
        .header("Title", title)
        .header("Priority", priority.to_string())
        .body(message.to_string());

    if !tags_str.is_empty() {
        req = req.header("Tags", tags_str);
    }

    let res = req
        .send()
        .await
        .map_err(|e| format!("Failed to reach Ntfy server: {}", e))?;
    if !res.status().is_success() {
        return Err(format!(
            "Ntfy rejected message with status {}",
            res.status()
        ));
    }

    Ok(())
}
