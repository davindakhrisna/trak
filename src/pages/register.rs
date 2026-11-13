use crate::components::MonolithShell;
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let store = use_app_store();
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (feedback, set_feedback) = signal(Option::<String>::None);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(st) = crate::backend::api_auth::check_auth().await {
                store.first_register_claimed.set(st.has_admin);
            }
        });
    });

    let on_register = move |ev: SubmitEvent| {
        ev.prevent_default();
        let u = username.get();
        let p = password.get();
        leptos::task::spawn_local(async move {
            match crate::backend::api_auth::register_first_admin(u, p).await {
                Ok(acc) => {
                    store.current_user_id.set(Some(acc.id.clone()));
                    store.first_register_claimed.set(true);
                    store.users.update(|list| {
                        if !list.iter().any(|existing| existing.id == acc.id) {
                            list.push(acc);
                        }
                    });
                    set_username.set(String::new());
                    set_password.set(String::new());
                    set_feedback.set(Some(
                        "Initial administrator created. Redirecting...".to_string(),
                    ));
                    #[cfg(target_arch = "wasm32")]
                    if let Some(win) = web_sys::window() {
                        let _ = win.location().set_href("/");
                    }
                }
                Err(err) => set_feedback.set(Some(crate::domain::clean_err(err))),
            }
        });
    };

    view! {
        <MonolithShell active_path="/login" hide_nav=true>
            <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-ink">
                        {move || {
                            if store.has_any_user() {
                                "Registration closed"
                            } else {
                                "Initial admin setup"
                            }
                        }}
                    </h1>
                    <p class="text-xs text-ink-muted">
                        {move || {
                            if store.has_any_user() {
                                "Administrator account claimed. Sign in to continue."
                            } else {
                                "The first registered user is granted administrator access."
                            }
                        }}
                    </p>
                </div>

                {move || {
                    if !store.has_any_user() {
                        view! {
                            <form
                                on:submit=on_register
                                class="w-full border border-rule p-6 flex flex-col items-center gap-3 bg-surface"
                            >
                                <p class="text-xs text-ink-muted">
                                    "Create the primary administrator account for this instance. Public registration locks immediately after."
                                </p>
                                <input
                                    type="text"
                                    placeholder="Root admin username"
                                    maxlength="32"
                                    prop:value=move || username.get()
                                    on:input=move |ev| set_username.set(event_target_value(&ev))
                                    class="input-monolith"
                                />
                                <input
                                    type="password"
                                    placeholder="Password (min 4 chars)"
                                    maxlength="128"
                                    prop:value=move || password.get()
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                    class="input-monolith"
                                />
                                <button type="submit" class="btn-monolith-solid w-full">
                                    "Claim admin account"
                                </button>
                            </form>
                        }
                            .into_any()
                    } else {
                        #[cfg(target_arch = "wasm32")]
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_href("/login");
                        }
                        view! {
                            <div class="w-full border border-rule p-6 flex flex-col items-center gap-4 bg-surface">
                                <p class="text-sm text-ink">
                                    "Registration is closed. Redirecting to sign in..."
                                </p>
                                <a href="/login" class="btn-monolith-solid w-full">
                                    "Go to Sign in"
                                </a>
                            </div>
                        }
                            .into_any()
                    }
                }}

                {move || {
                    feedback.get().map(|msg| {
                        view! {
                            <p class="w-full border border-rule bg-muted px-3 py-2 font-mono text-xs text-ink">
                                {msg}
                            </p>
                        }
                    })
                }}
            </section>
        </MonolithShell>
    }
}
