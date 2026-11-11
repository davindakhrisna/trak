use crate::components::MonolithShell;
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    let store = use_app_store();
    let (username, set_username) = signal("admin".to_string());
    let (password, set_password) = signal("password".to_string());
    let (status, set_status) = signal(Option::<String>::None);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        match store.login(username.get(), password.get()) {
            Ok(()) => {
                set_status.set(Some("Signed in. Open Today to view tasks.".to_string()));
            }
            Err(err) => set_status.set(Some(err)),
        }
    };

    view! {
        <MonolithShell active_path="/login" hide_nav=true>
            <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">"Sign in"</h1>
                    <p class="text-xs text-gray">
                        {move || {
                            if store.has_any_user() {
                                "Registration is closed. Contact an administrator for an account."
                            } else {
                                "Instance uninitialized. Register the first administrator account."
                            }
                        }}
                    </p>
                </div>

                {move || {
                    if !store.has_any_user() {
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

                <div class="w-full border border-gray p-4 flex flex-col items-center gap-2 bg-white dark:bg-black">
                    <span class="text-xs text-gray">
                        "Preview session"
                    </span>
                    <div class="w-full flex flex-wrap items-center justify-center gap-2">
                        <button
                            type="button"
                            on:click=move |_| {
                                let _ = store.login("admin".to_string(), "password".to_string());
                                set_status.set(Some("Active session: admin (admin)".to_string()));
                            }
                            class="btn-monolith-ghost"
                        >
                            "Admin"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| {
                                let _ = store.login("marcus_aurelius_operations".to_string(), "password".to_string());
                                set_status.set(Some("Active session: marcus_aurelius_operations (user)".to_string()));
                            }
                            class="btn-monolith-ghost"
                        >
                            "Standard user"
                        </button>
                        <a
                            href="/register"
                            on:click=move |_| store.reset_to_first_boot()
                            class="btn-monolith-ghost"
                        >
                            "Simulate first register"
                        </a>
                    </div>
                </div>
            </section>
        </MonolithShell>
    }
}
