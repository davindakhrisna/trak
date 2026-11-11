use crate::components::theme::ThemeSelector;
use crate::state::use_app_store;
use leptos::prelude::*;

#[component]
pub fn AppNavigation(active_path: &'static str) -> impl IntoView {
    let store = use_app_store();

    let sidebar_link_class = move |path: &'static str| {
        if active_path == path {
            "w-full px-3.5 py-2 font-mono text-xs font-bold bg-black text-white dark:bg-white dark:text-black border border-black dark:border-white text-left select-none transition-colors duration-75"
        } else {
            "w-full px-3.5 py-2 font-mono text-xs text-black dark:text-white bg-white dark:bg-black border border-transparent hover:border-black dark:hover:border-white text-left select-none transition-colors duration-75"
        }
    };

    let mobile_link_class = move |path: &'static str| {
        if active_path == path {
            "px-3 py-1 font-mono text-xs font-bold bg-black text-white dark:bg-white dark:text-black border border-black dark:border-white shrink-0 select-none transition-colors duration-75"
        } else {
            "px-3 py-1 font-mono text-xs text-black dark:text-white bg-white dark:bg-black border border-transparent hover:border-black dark:hover:border-white shrink-0 select-none transition-colors duration-75"
        }
    };

    let on_sign_out = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        leptos::task::spawn_local(async move {
            let _ = crate::backend::api_auth::logout().await;
            store.current_user_id.set(None);
            #[cfg(target_arch = "wasm32")]
            if let Some(win) = web_sys::window() {
                let _ = win.location().set_href("/login");
            }
        });
    };

    view! {
        // Desktop Sidebar (hidden on mobile, fixed full-height rail on md+)
        <aside class="hidden md:flex md:w-56 lg:w-64 shrink-0 flex-col justify-between border-r border-black dark:border-white p-6 bg-white dark:bg-black sticky top-0 h-screen overflow-y-auto">
            <div class="flex flex-col gap-6">
                <div class="flex flex-col gap-0.5 border-b border-black dark:border-white pb-4 text-left">
                    <span class="font-sans font-bold text-xl tracking-tighter text-black dark:text-white">
                        "TRAK"
                    </span>
                    <span class="font-mono text-[9px] text-gray uppercase tracking-wider">
                        "Task & Discipline"
                    </span>
                </div>

                <nav aria-label="Sidebar navigation" class="flex flex-col gap-1 w-full">
                    <a href="/" class=move || sidebar_link_class("/")>"Today"</a>
                    <a href="/templates" class=move || sidebar_link_class("/templates")>"Templates"</a>
                    <a href="/settings" class=move || sidebar_link_class("/settings")>"Settings"</a>
                    {move || if store.is_admin() {
                        view! { <a href="/users" class=move || sidebar_link_class("/users")>"Users"</a> }.into_any()
                    } else {
                        ().into_any()
                    }}
                </nav>
            </div>

            <div class="flex flex-col gap-3 border-t border-black dark:border-white pt-4 text-left font-mono text-[11px] text-gray">
                {move || {
                    if let Some(user) = store.current_user() {
                        let display_name = user.username.clone();
                        let title_name = user.username;
                        view! {
                            <div class="flex flex-col gap-1">
                                <span class="truncate text-black dark:text-white font-medium" title=title_name>
                                    {display_name}
                                </span>
                                <a href="/login" on:click=on_sign_out class="hover:text-black dark:hover:text-white hover:underline cursor-pointer">
                                    "Sign out"
                                </a>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <a href="/login" class="hover:text-black dark:hover:text-white hover:underline cursor-pointer">
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

        // Mobile Top Header (hidden on desktop md+)
        <header class="flex md:hidden w-full flex-col border-b border-black dark:border-white bg-white dark:bg-black sticky top-0 z-30">
            <div class="w-full flex items-center justify-between px-4 py-2.5 border-b border-black dark:border-white">
                <span class="font-sans font-bold text-lg tracking-tight">"TRAK"</span>
                <div class="flex items-center gap-2 font-mono text-[11px] text-gray">
                    {move || {
                        if let Some(user) = store.current_user() {
                            let display_name = user.username.clone();
                            let title_name = user.username;
                            view! {
                                <span class="truncate max-w-[120px]" title=title_name>{display_name}</span>
                                <span>"·"</span>
                                <a href="/login" on:click=on_sign_out class="hover:text-black dark:hover:text-white hover:underline cursor-pointer">"Out"</a>
                                <span>"·"</span>
                            }.into_any()
                        } else {
                            view! {
                                <a href="/login" class="hover:text-black dark:hover:text-white hover:underline cursor-pointer">"In"</a>
                                <span>"·"</span>
                            }.into_any()
                        }
                    }}
                    <ThemeSelector />
                </div>
            </div>

            <nav aria-label="Mobile navigation" class="w-full flex items-center justify-start gap-1 px-3 py-1.5 overflow-x-auto scroll-monolith">
                <a href="/" class=move || mobile_link_class("/")>"Today"</a>
                <a href="/templates" class=move || mobile_link_class("/templates")>"Templates"</a>
                <a href="/settings" class=move || mobile_link_class("/settings")>"Settings"</a>
                {move || if store.is_admin() {
                    view! { <a href="/users" class=move || mobile_link_class("/users")>"Users"</a> }.into_any()
                } else {
                    ().into_any()
                }}
            </nav>
        </header>
    }
}
