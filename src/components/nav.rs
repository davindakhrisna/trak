use crate::components::theme::ThemeSelector;
use crate::state::use_app_store;
use leptos::prelude::*;

#[component]
pub fn CenteredNav(active_path: &'static str) -> impl IntoView {
    let store = use_app_store();

    let nav_item_class = move |path: &'static str| {
        if active_path == path {
            "px-3.5 py-1.5 font-mono text-xs bg-black text-white dark:bg-white dark:text-black border border-black dark:border-white transition-colors duration-75"
        } else {
            "px-3.5 py-1.5 font-mono text-xs bg-white text-black dark:bg-black dark:text-white border border-transparent hover:border-black dark:hover:border-white transition-colors duration-75"
        }
    };

    view! {
        <header class="w-full flex flex-col items-center justify-center mb-10 gap-2">
            <nav
                aria-label="Primary navigation"
                class="w-full flex flex-wrap items-center justify-center gap-1 border-b border-black dark:border-white pb-3"
            >
                <a href="/" class=move || nav_item_class("/")>
                    "Today"
                </a>
                <a href="/templates" class=move || nav_item_class("/templates")>
                    "Templates"
                </a>
                {move || {
                    if store.is_admin() {
                        view! {
                            <a href="/users" class=move || nav_item_class("/users")>
                                "Users"
                            </a>
                        }
                            .into_any()
                    } else {
                        ().into_any()
                    }
                }}
            </nav>

            <div class="w-full flex flex-wrap items-center justify-center gap-2 font-mono text-[11px] text-gray">
                {move || {
                    if let Some(user) = store.current_user() {
                        let display_name = user.username.clone();
                        let title_name = user.username;
                        view! {
                            <span class="truncate max-w-[200px]" title=title_name>
                                {display_name}
                            </span>
                            <span>"·"</span>
                            <a
                                href="/login"
                                on:click=move |_| store.logout()
                                class="hover:text-black dark:hover:text-white hover:underline cursor-pointer"
                            >
                                "Sign out"
                            </a>
                            <span>"·"</span>
                        }
                            .into_any()
                    } else {
                        ().into_any()
                    }
                }}
                <ThemeSelector />
            </div>
        </header>
    }
}
