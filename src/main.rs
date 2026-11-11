#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos::prelude::provide_context;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::services::ServeDir;
    use trak::backend::{db::init_db, provision::spawn_midnight_ticker, AppState};
    use trak::{shell, App};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:trak.db?mode=rwc".to_string());
    let pool = init_db(&database_url)
        .await
        .expect("Failed to initialize SQLite database");

    let secret = std::env::var("SESSION_SECRET")
        .map(|s| s.into_bytes())
        .unwrap_or_else(|_| b"trak-insecure-default-development-secret-key-32b!".to_vec());

    spawn_midnight_ticker(pool.clone());

    let app_state = AppState {
        pool: pool.clone(),
        secret,
        leptos_options: leptos_options.clone(),
    };

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let state = app_state.clone();
                move || {
                    provide_context(state.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .nest_service(
            "/pkg",
            ServeDir::new(format!("{}/pkg", leptos_options.site_root)),
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("⚡ Trak server listening on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
