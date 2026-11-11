use crate::components::MonolithShell;
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    let store = use_app_store();
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (status, set_status) = signal(Option::<String>::None);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(st) = crate::backend::api_auth::check_auth().await {
                store.first_register_claimed.set(st.has_admin);
            }
        });
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let u = username.get();
        let p = password.get();
        leptos::task::spawn_local(async move {
            match crate::backend::api_auth::login(u, p).await {
                Ok(acc) => {
                    store.current_user_id.set(Some(acc.id.clone()));
                    store.users.update(|list| {
                        if !list.iter().any(|existing| existing.id == acc.id) {
                            list.push(acc);
                        }
                    });
                    set_status.set(Some("Signed in. Redirecting...".to_string()));
                    #[cfg(target_arch = "wasm32")]
                    if let Some(win) = web_sys::window() {
                        let _ = win.location().set_href("/");
                    }
                }
                Err(err) => set_status.set(Some(crate::domain::clean_err(err))),
            }
        });
    };

    view! {
        <MonolithShell active_path="/login" hide_nav=true>
            <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">"Sign in"</h1>
                    <p class="text-xs text-gray">
                        {move || {
                            if store.auth_initialized.get() && !store.has_any_user() {
                                "Instance uninitialized. Register the first administrator account."
                            } else {
                                "Enter your credentials to continue."
                            }
                        }}
                    </p>
                </div>

                {move || {
                    if store.auth_initialized.get() && !store.has_any_user() {
                        view! {
                            <div class="w-full border border-black dark:border-white p-6 flex flex-col items-center gap-4 bg-white dark:bg-black">
                                <p class="text-sm text-black dark:text-white">
                                    "No accounts exist yet. The first registered account becomes the administrator."
                                </p>
                                <a href="/register" class="btn-monolith-solid">
                                    "Register first admin"
                                </a>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <form
                                on:submit=on_submit
                                class="w-full border border-black dark:border-white p-6 flex flex-col items-center gap-3 bg-white dark:bg-black"
                            >
                                <input
                                    type="text"
                                    placeholder="Username"
                                    maxlength="32"
                                    prop:value=move || username.get()
                                    on:input=move |ev| set_username.set(event_target_value(&ev))
                                    class="input-monolith"
                                />
                                <input
                                    type="password"
                                    placeholder="Password"
                                    maxlength="128"
                                    prop:value=move || password.get()
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                    class="input-monolith"
                                />
                                <button type="submit" class="btn-monolith-solid">
                                    "Sign in"
                                </button>

                                {move || {
                                    status.get().map(|msg| {
                                        view! {
                                            <p class="w-full border border-black dark:border-white px-3 py-2 font-mono text-xs text-black dark:text-white">
                                                {msg}
                                            </p>
                                        }
                                    })
                                }}
                            </form>
                        }
                            .into_any()
                    }
                }}
            </section>
        </MonolithShell>
    }
}
