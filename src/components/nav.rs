use crate::components::theme::ThemeSelector;
use crate::state::use_app_store;
use leptos::prelude::*;

fn nav_link_class(active: bool) -> &'static str {
    if active {
        "w-full px-3 py-1.5 font-mono text-xs font-bold bg-ink text-canvas border border-rule-strong text-left select-none transition-colors duration-75"
    } else {
        "w-full px-3 py-1.5 font-mono text-xs text-ink-muted hover:text-ink bg-transparent border border-transparent hover:border-rule text-left select-none transition-colors duration-75"
    }
}

fn mobile_link_class(active: bool) -> &'static str {
    if active {
        "px-3 py-1 font-mono text-xs font-bold bg-ink text-canvas border border-rule-strong shrink-0 select-none transition-colors duration-75"
    } else {
        "px-3 py-1 font-mono text-xs text-ink-muted hover:text-ink bg-transparent border border-transparent hover:border-rule shrink-0 select-none transition-colors duration-75"
    }
}

fn handle_logout(store: crate::state::AppStore) {
    leptos::task::spawn_local(async move {
        let _ = crate::backend::api_auth::logout().await;
        store.current_user_id.set(None);
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            let _ = win.location().set_href("/login");
        }
    });
}

#[component]
pub fn DesktopSidebar(active_path: &'static str) -> impl IntoView {
    let store = use_app_store();
    let on_sign_out = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        handle_logout(store);
    };

    view! {
        <aside class="hidden lg:flex flex-col justify-between w-40 lg:w-44 shrink-0 text-left border-r border-rule pr-6 py-1 self-stretch min-h-[280px] max-h-[calc(100vh-6rem)] sticky top-8 lg:top-12 gap-6">
            <div class="flex flex-col gap-4">
                <div class="flex flex-col gap-0.5 border-b border-rule pb-3 text-left">
                    <span class="font-sans font-bold text-lg tracking-tighter text-ink">
                        "TRAK"
                    </span>
                    <span class="font-mono text-[11px] text-ink-muted uppercase tracking-wider">
                        "Task & Discipline"
                    </span>
                </div>

                <nav aria-label="Desktop sidebar navigation" class="flex flex-col gap-1 w-full">
                    <a href="/" class=nav_link_class(active_path == "/")>"Today"</a>
                    <a href="/templates" class=nav_link_class(active_path == "/templates")>"Templates"</a>
                    <a href="/settings" class=nav_link_class(active_path == "/settings")>"Settings"</a>
                    {move || if store.is_admin() {
                        view! { <a href="/users" class=nav_link_class(active_path == "/users")>"Users"</a> }.into_any()
                    } else {
                        ().into_any()
                    }}
                </nav>
            </div>

            <div class="flex flex-col gap-3 border-t border-rule pt-4 text-left font-mono text-[11px] text-ink-muted mt-auto">
                {move || {
                    if let Some(user) = store.current_user() {
                        let display_name = user.username.clone();
                        let title_name = user.username;
                        view! {
                            <div class="flex flex-col gap-1">
                                <span class="truncate text-ink font-medium" title=title_name>
                                    {display_name}
                                </span>
                                <a href="/login" on:click=on_sign_out class="text-ink-muted hover:text-ink hover:underline cursor-pointer">
                                    "Sign out"
                                </a>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <a href="/login" class="text-ink-muted hover:text-ink hover:underline cursor-pointer">
                                "Sign in"
                            </a>
                        }.into_any()
                    }
                }}
                <div class="pt-1">
                    <ThemeSelector />
                </div>
            </div>
        </aside>
    }
}

#[component]
pub fn MobileNav(active_path: &'static str) -> impl IntoView {
    let store = use_app_store();
    let on_sign_out = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        handle_logout(store);
    };

    view! {
        <header class="flex lg:hidden w-full flex-col border-b border-rule bg-surface sticky top-0 z-30">
            <div class="w-full flex items-center justify-between px-4 sm:px-6 py-2.5 border-b border-rule">
                <div class="flex items-center gap-2.5 sm:gap-3">
                    <span class="font-sans font-bold text-base sm:text-lg tracking-tight text-ink">"TRAK"</span>
                    <span class="hidden sm:inline-block font-mono text-[11px] text-ink-muted uppercase tracking-wider">
                        "Task & Discipline"
                    </span>
                </div>
                <div class="flex items-center gap-2.5 font-mono text-[11px] text-ink-muted">
                    {move || {
                        if let Some(user) = store.current_user() {
                            let display_name = user.username.clone();
                            let title_name = user.username;
                            view! {
                                <span class="truncate max-w-[100px] sm:max-w-[160px] font-medium text-ink" title=title_name>
                                    {display_name}
                                </span>
                                <span class="h-3 w-px bg-rule" aria-hidden="true"></span>
                                <a href="/login" on:click=on_sign_out class="hover:text-ink hover:underline cursor-pointer whitespace-nowrap">"Sign out"</a>
                            }.into_any()
                        } else {
                            view! {
                                <a href="/login" class="hover:text-ink hover:underline cursor-pointer whitespace-nowrap">"Sign in"</a>
                            }.into_any()
                        }
                    }}
                    <ThemeSelector />
                </div>
            </div>

            <nav aria-label="Mobile navigation" class="w-full flex items-center justify-start sm:justify-center gap-1.5 px-3 sm:px-6 py-2 overflow-x-auto scroll-monolith">
                <a href="/" class=mobile_link_class(active_path == "/")>"Today"</a>
                <a href="/templates" class=mobile_link_class(active_path == "/templates")>"Templates"</a>
                <a href="/settings" class=mobile_link_class(active_path == "/settings")>"Settings"</a>
                {move || if store.is_admin() {
                    view! { <a href="/users" class=mobile_link_class(active_path == "/users")>"Users"</a> }.into_any()
                } else {
                    ().into_any()
                }}
            </nav>
        </header>
    }
}
