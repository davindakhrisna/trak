#![recursion_limit = "256"]

pub mod backend;
pub mod components;
pub mod domain;
pub mod pages;
pub mod state;

use crate::pages::{LoginPage, RegisterPage, SettingsPage, TemplatesPage, TodayPage, UsersPage};
use crate::state::provide_app_store;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
                <link
                    href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;700&family=Space+Grotesk:wght@400;500;700&display=swap"
                    rel="stylesheet"
                />
                <script>
                    "try{var p=new URLSearchParams(window.location.search).get('theme');if(p){localStorage.setItem('trak-theme',p);}var t=localStorage.getItem('trak-theme');var d=t==='dark'||((!t||t==='system')&&window.matchMedia('(prefers-color-scheme: dark)').matches);if(t==='light')d=false;if(d){document.documentElement.classList.add('dark');}else{document.documentElement.classList.remove('dark');}}catch(e){}"
                </script>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-white dark:bg-black text-black dark:text-white antialiased font-sans">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let store = provide_app_store();

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(status) = crate::backend::api_auth::check_auth().await {
                store.first_register_claimed.set(status.has_admin);
                if status.authenticated {
                    if let Some(user) = status.user {
                        let uid = user.id.clone();
                        store.users.update(|list| {
                            if !list.iter().any(|u| u.id == uid) {
                                list.push(user);
                            }
                        });
                        store.current_user_id.set(Some(uid));
                    }
                } else {
                    store.current_user_id.set(None);
                }
                store.auth_initialized.set(true);
            }
        });
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/trak.css"/>
        <Title text="Trak — Today"/>

        <Router>
            <Routes fallback=|| {
                view! {
                    <div class="monolith-page">
                        <div class="monolith-spine gap-4">
                            <h1 class="text-2xl font-bold text-black">"404"</h1>
                            <p class="font-mono text-xs text-gray">"ROUTE NOT FOUND"</p>
                            <a href="/" class="btn-monolith-solid">"Return to Today"</a>
                        </div>
                    </div>
                }
            }>
                <Route path=path!("") view=TodayPage/>
                <Route path=path!("login") view=LoginPage/>
                <Route path=path!("register") view=RegisterPage/>
                <Route path=path!("registers") view=RegisterPage/>
                <Route path=path!("templates") view=TemplatesPage/>
                <Route path=path!("settings") view=SettingsPage/>
                <Route path=path!("users") view=UsersPage/>
            </Routes>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
