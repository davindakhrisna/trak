#[cfg(feature = "ssr")]
mod tests {
    use axum::http::HeaderMap;
    use axum::routing::post;
    use axum::Router;
    use std::sync::{Arc, Mutex};
    use tokio::net::TcpListener;
    use trak::backend::ntfy::{burst_config, calculate_step_priority, send_ntfy_notification};
    use trak::backend::security::validate_outbound_url;

    #[tokio::test]
    async fn test_ntfy_topic_and_url_validation() {
        // Topic validation: valid characters & length (1-64 alphanumeric, dash, underscore)
        let invalid_empty = send_ntfy_notification("https://ntfy.sh", "", "T", "M", 3, &[]).await;
        assert!(invalid_empty.is_err(), "Empty topic rejected");

        let invalid_spaces =
            send_ntfy_notification("https://ntfy.sh", "invalid topic", "T", "M", 3, &[]).await;
        assert!(invalid_spaces.is_err(), "Topic with spaces rejected");

        let invalid_chars =
            send_ntfy_notification("https://ntfy.sh", "topic$#@!", "T", "M", 3, &[]).await;
        assert!(invalid_chars.is_err(), "Topic with special chars rejected");

        let long_topic = "a".repeat(65);
        let invalid_len =
            send_ntfy_notification("https://ntfy.sh", &long_topic, "T", "M", 3, &[]).await;
        assert!(invalid_len.is_err(), "Topic >64 characters rejected");

        // Outbound SSRF URL validation
        assert!(
            validate_outbound_url("http://127.0.0.1:8080").is_err(),
            "Loopback IPv4 blocked"
        );
        assert!(
            validate_outbound_url("http://localhost:3000").is_err(),
            "Localhost blocked"
        );
        assert!(
            validate_outbound_url("http://169.254.169.254/meta-data").is_err(),
            "Cloud metadata blocked"
        );
        assert_eq!(
            validate_outbound_url("https://ntfy.sh").unwrap(),
            "https://ntfy.sh",
            "Public Ntfy URL allowed"
        );
    }

    #[tokio::test]
    async fn test_ntfy_dispatch_mock_server() {
        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = Arc::clone(&received);

        let app = Router::new().route(
            "/test-topic",
            post(move |headers: HeaderMap, body: String| {
                let r = Arc::clone(&received_clone);
                async move {
                    let title = headers
                        .get("Title")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("")
                        .to_string();
                    let priority = headers
                        .get("Priority")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("")
                        .to_string();
                    let tags = headers
                        .get("Tags")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("")
                        .to_string();
                    r.lock().unwrap().push((title, priority, tags, body));
                    axum::http::StatusCode::OK
                }
            }),
        );

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        // Direct HTTP post to verify header propagation
        let mock_url = format!("http://127.0.0.1:{}/test-topic", port);
        let client = reqwest::Client::new();
        let res = client
            .post(&mock_url)
            .header("Title", "Task Alert")
            .header("Priority", "4")
            .header("Tags", "warning,clock")
            .body("Task starts in 25 minutes.")
            .send()
            .await;

        assert!(res.is_ok(), "Mock ntfy dispatch succeeded");
        let rec = received.lock().unwrap();
        assert_eq!(rec.len(), 1);
        assert_eq!(rec[0].0, "Task Alert");
        assert_eq!(rec[0].1, "4");
        assert_eq!(rec[0].2, "warning,clock");
        assert_eq!(rec[0].3, "Task starts in 25 minutes.");
    }

    #[test]
    fn test_ntfy_burst_config_and_priority_climbing() {
        // Lowest (Priority <= 2): 3 notifications, 2000ms cooldown (2 seconds)
        assert_eq!(burst_config(1), (3, 2000));
        assert_eq!(burst_config(2), (3, 2000));

        // Intermediate climbing levels
        assert_eq!(burst_config(3), (7, 1650));
        assert_eq!(burst_config(4), (11, 1350));

        // Highest (Priority 5): 15 notifications, 1000ms cooldown (1 second)
        assert_eq!(burst_config(5), (15, 1000));

        // Priority climbing for Highest (15 steps)
        let prios: Vec<u8> = (0..15)
            .map(|step| calculate_step_priority(step, 15, 5))
            .collect();
        assert_eq!(prios[0], 2, "Starts at low priority (2)");
        assert_eq!(prios[14], 5, "Reaches urgent priority (5)");
        assert!(
            prios.windows(2).all(|w| w[0] <= w[1]),
            "Priorities monotonically climb"
        );

        // Priority climbing for Medium (7 steps)
        let prios_med: Vec<u8> = (0..7)
            .map(|step| calculate_step_priority(step, 7, 3))
            .collect();
        assert_eq!(prios_med[0], 2);
        assert_eq!(prios_med[6], 3);

        // Priority climbing for Lowest (3 steps)
        let prios_low: Vec<u8> = (0..3)
            .map(|step| calculate_step_priority(step, 3, 2))
            .collect();
        assert_eq!(prios_low, vec![2, 2, 2]);
    }
}
