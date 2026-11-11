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

    let on_register = move |ev: SubmitEvent| {
        ev.prevent_default();
        match store.register_first_admin(username.get(), password.get()) {
            Ok(()) => {
                set_username.set(String::new());
                set_password.set(String::new());
                set_feedback.set(Some(
                    "Initial administrator created. Registration is now locked.".to_string(),
                ));
            }
            Err(err) => set_feedback.set(Some(err)),
        }
    };

    view! {
        <MonolithShell active_path="/login" hide_nav=true>
            <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">
                        {move || {
                            if store.has_any_user() {
                                "Registration closed"
                            } else {
                                "Initial admin setup"
                            }
                        }}
                    </h1>
                    <p class="text-xs text-gray">
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
                                class="w-full border border-black dark:border-white p-6 flex flex-col items-center gap-3 bg-white dark:bg-black"
                            >
                                <p class="text-xs text-gray">
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
                                <button type="submit" class="btn-monolith-solid">
                                    "Claim admin account"
                                </button>
                            </form>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="w-full border border-black dark:border-white p-6 flex flex-col items-center gap-4 bg-white dark:bg-black">
                                <p class="text-sm text-black dark:text-white">
                                    "The initial administrator has already been registered. New accounts can only be added by an admin in /users."
                                </p>
                                <a href="/login" class="btn-monolith-solid">
                                    "Go to Sign in"
                                </a>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        store.reset_to_first_boot();
                                        set_feedback.set(None);
                                    }
                                    class="btn-monolith-ghost w-full py-2.5"
                                >
                                    "Reset to 0 users (preview initial setup)"
                                </button>
                            </div>
                        }
                            .into_any()
                    }
                }}

                {move || {
                    feedback.get().map(|msg| {
                        view! {
                            <p class="w-full border border-black dark:border-white px-3 py-2 font-mono text-xs text-black dark:text-white">
                                {msg}
                            </p>
                        }
                    })
                }}
            </section>
        </MonolithShell>
    }
}
