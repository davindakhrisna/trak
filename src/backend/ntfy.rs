#[cfg(feature = "ssr")]
pub fn burst_config(priority: u8) -> (usize, u64) {
    match priority {
        0..=2 => (3, 2000), // Lowest: 3 notifications, 2s cooldown
        3 => (7, 1650),     // Medium: 7 notifications, 1.65s cooldown
        4 => (11, 1350),    // High: 11 notifications, 1.35s cooldown
        _ => (15, 1000),    // Highest: 15 notifications, 1.0s cooldown
    }
}

#[cfg(feature = "ssr")]
pub fn calculate_step_priority(step: usize, total: usize, max_prio: u8) -> u8 {
    let effective_max = max_prio.max(3);
    if total <= 1 || effective_max <= 3 {
        return effective_max;
    }
    let min_prio = 3u8;
    let progress = step as f32 / (total - 1) as f32;
    let prio_float = min_prio as f32 + (effective_max - min_prio) as f32 * progress;
    (prio_float.round() as u8).clamp(3, effective_max)
}

#[cfg(feature = "ssr")]
async fn send_single_post(
    client: &reqwest::Client,
    url: &str,
    title: &str,
    message: &str,
    priority: u8,
    tags_str: &str,
) -> Result<(), String> {
    let mut req = client
        .post(url)
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
    let (total_count, cooldown_ms) = burst_config(priority);

    let initial_prio = calculate_step_priority(0, total_count, priority);
    let initial_title = format!("{} [1/{}]", title, total_count);
    send_single_post(
        &client,
        &url,
        &initial_title,
        message,
        initial_prio,
        &tags_str,
    )
    .await?;

    if total_count > 1 {
        let client_bg = client.clone();
        let url_bg = url;
        let title_bg = title.to_string();
        let message_bg = message.to_string();
        let tags_bg = tags_str;

        tokio::spawn(async move {
            for i in 1..total_count {
                tokio::time::sleep(std::time::Duration::from_millis(cooldown_ms)).await;
                let step_prio = calculate_step_priority(i, total_count, priority);
                let step_title = format!("{} [{}/{}]", title_bg, i + 1, total_count);
                if send_single_post(
                    &client_bg,
                    &url_bg,
                    &step_title,
                    &message_bg,
                    step_prio,
                    &tags_bg,
                )
                .await
                .is_err()
                {
                    break;
                }
            }
        });
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn send_single_notification(
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
    let effective_prio = priority.max(3);
    send_single_post(&client, &url, title, message, effective_prio, &tags_str).await
}

#[cfg(feature = "ssr")]
pub async fn send_test_all_priority_examples(server: &str, topic: &str) -> Result<(), String> {
    send_single_notification(
        server,
        topic,
        "Trak Test [1/4]: Lowest Priority",
        "Example Priority 3: Routine reminder (audible sound & vibration on iOS)",
        3,
        &["bell"],
    )
    .await?;

    let server_s = server.to_string();
    let topic_s = topic.to_string();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        if send_single_notification(
            &server_s,
            &topic_s,
            "Trak Test [2/4]: Medium Priority",
            "Example Priority 3-4: 1h deadline warning (escalating sound & vibration)",
            3,
            &["clock1"],
        )
        .await
        .is_err()
        {
            return;
        }

        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        if send_single_notification(
            &server_s,
            &topic_s,
            "Trak Test [3/4]: High Priority",
            "Example Priority 4: 25m deadline warning (urgent alert chime & vibration)",
            4,
            &["warning", "rotating_light"],
        )
        .await
        .is_err()
        {
            return;
        }

        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        let _ = send_single_notification(
            &server_s,
            &topic_s,
            "Trak Test [4/4]: Critical Priority",
            "Example Priority 5: Crossed deadline alarm (maximum volume override & vibration)",
            5,
            &["rotating_light", "sos"],
        )
        .await;
    });

    Ok(())
}
