use crate::components::{MonolithShell, SquareCheck};
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn UsersPage() -> impl IntoView {
    let store = use_app_store();
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (as_admin, set_as_admin) = signal(false);
    let (feedback, set_feedback) = signal(Option::<String>::None);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(users) = crate::backend::api_users::fetch_users_list().await {
                store.users.set(users);
            }
        });
    });

    let on_create = move |ev: SubmitEvent| {
        ev.prevent_default();
        let u = username.get();
        let p = password.get();
        let adm = as_admin.get();
        leptos::task::spawn_local(async move {
            match crate::backend::api_users::admin_create_user(u.clone(), p, adm).await {
                Ok(new_acc) => {
                    store.users.update(|list| list.push(new_acc));
                    set_username.set(String::new());
                    set_password.set(String::new());
                    set_as_admin.set(false);
                    set_feedback.set(Some(format!("Added user '{}'.", u.trim())));
                }
                Err(err) => set_feedback.set(Some(crate::domain::clean_err(err))),
            }
        });
    };

    view! {
        <MonolithShell active_path="/users" wide=true>
            {move || {
                if !store.auth_initialized.get() {
                    view! { <div class="py-12 font-mono text-xs text-ink-muted">"Verifying session..."</div> }.into_any()
                } else if store.current_user_id.get().is_none() {
                    view! { <div class="py-12 font-mono text-xs text-ink-muted">"Redirecting to sign in..."</div> }.into_any()
                } else if !store.is_admin() {
                    #[cfg(target_arch = "wasm32")]
                    if let Some(win) = web_sys::window() {
                        let _ = win.location().set_href("/");
                    }
                    view! { <div class="py-12 font-mono text-xs text-ink-muted">"Admin privileges required. Redirecting..."</div> }.into_any()
                } else {
                    view! {
                        <section class="w-full flex flex-col items-center justify-center gap-6">
                            <div class="w-full flex flex-col items-center justify-center gap-1">
                                <h1 class="text-3xl font-bold tracking-tight text-ink">
                                    "Users"
                                </h1>
                                <p class="text-xs text-ink-muted">
                                    "Add accounts and assign administrator access."
                                </p>
                            </div>

                            <div class="w-full grid grid-cols-1 md:grid-cols-2 gap-5 items-start text-left">
                                <form
                                    on:submit=on_create
                                    class="w-full border border-rule p-5 flex flex-col gap-3 bg-surface"
                                >
                                    <div class="pb-1.5 border-b border-rule">
                                        <span class="font-mono text-xs text-ink uppercase tracking-wider font-medium">
                                            "Provision account"
                                        </span>
                                    </div>
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

                                    <div
                                        on:click=move |_| set_as_admin.update(|v| *v = !*v)
                                        class="w-full px-1 py-2 flex items-center justify-start gap-3 cursor-pointer select-none"
                                    >
                                        <SquareCheck
                                            checked=as_admin.get()
                                            on_toggle=Callback::new(move |_| set_as_admin.update(|v| *v = !*v))
                                            aria_label="Set new user as admin".to_string()
                                        />
                                        <span class="text-xs text-ink font-medium">
                                            "Set as admin"
                                        </span>
                                    </div>

                                    <button type="submit" class="btn-monolith-solid w-full">
                                        "Add user"
                                    </button>

                                    {move || {
                                        feedback
                                            .get()
                                            .map(|msg| {
                                                view! {
                                                    <p class="w-full border border-rule bg-muted px-3 py-2 font-mono text-xs text-ink">
                                                        {msg}
                                                    </p>
                                                }
                                            })
                                    }}
                                </form>

                                <div class="w-full flex flex-col gap-2">
                                    <div class="flex items-center justify-between px-1 pb-1.5 border-b border-rule">
                                        <span class="font-mono text-xs text-ink uppercase tracking-wider font-medium">
                                            "Users"
                                        </span>
                                        <span class="font-mono text-xs tabular-nums text-ink-muted">
                                            {move || {
                                                let count = store.users.get().len();
                                                if count == 1 {
                                                    "1 account".to_string()
                                                } else {
                                                    format!("{} accounts", count)
                                                }
                                            }}
                                        </span>
                                    </div>

                                    <ul class="w-full flex flex-col gap-2 max-h-[304px] overflow-y-auto scroll-monolith">
                                {move || {
                                    store
                                        .users
                                        .get()
                                        .into_iter()
                                        .map(|u| {
                                            let uid_toggle = u.id.clone();
                                            let uid_remove = u.id.clone();
                                            let is_root = u.is_first_admin;
                                            let role = u.role;
                                            let display_name = u.username.clone();
                                            let title_name = u.username;
                                            view! {
                                                <li class="cell-row">
                                                    <span class="font-mono text-[11px] px-2 py-0.5 border border-rule bg-muted text-ink shrink-0 font-medium">
                                                        {role.label()}
                                                    </span>
                                                    <span
                                                        title=title_name
                                                        class="flex-1 min-w-0 px-2 text-left text-sm font-medium text-ink truncate"
                                                    >
                                                        {display_name}
                                                    </span>
                                                    {if is_root {
                                                        view! {
                                                            <span class="font-mono text-[11px] text-ink-muted shrink-0">
                                                                "Initial admin"
                                                            </span>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! {
                                                            <div class="flex items-center gap-1.5 shrink-0">
                                                                <button
                                                                    type="button"
                                                                    on:click={
                                                                        let uid = uid_toggle.clone();
                                                                        move |_| {
                                                                            let uid_c = uid.clone();
                                                                            leptos::task::spawn_local(async move {
                                                                                if let Ok(new_role) =
                                                                                    crate::backend::api_users::admin_toggle_user_role(uid_c.clone()).await
                                                                                {
                                                                                    store.users.update(|list| {
                                                                                        if let Some(u) = list.iter_mut().find(|u| u.id == uid_c) {
                                                                                            u.role = new_role;
                                                                                        }
                                                                                    });
                                                                                }
                                                                            });
                                                                        }
                                                                    }
                                                                    class="btn-monolith-ghost shrink-0"
                                                                >
                                                                    {if role.is_admin() {
                                                                        "Revoke admin"
                                                                    } else {
                                                                        "Set admin"
                                                                    }}
                                                                </button>
                                                                <button
                                                                    type="button"
                                                                    aria-label="Remove user"
                                                                    on:click={
                                                                        let uid = uid_remove.clone();
                                                                        move |_| {
                                                                            let uid_c = uid.clone();
                                                                            leptos::task::spawn_local(async move {
                                                                                if crate::backend::api_users::admin_delete_user(uid_c.clone()).await.is_ok() {
                                                                                    store.users.update(|list| list.retain(|u| u.id != uid_c));
                                                                                }
                                                                            });
                                                                        }
                                                                    }
                                                                    class="w-7 h-7 border border-transparent text-ink-muted hover:border-signal hover:text-signal hover:bg-signal-soft font-mono text-xs flex items-center justify-center transition-colors duration-75 cursor-pointer shrink-0"
                                                                >
                                                                    "×"
                                                                </button>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }}
                                                </li>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </ul>
                                </div>
                            </div>
                        </section>
                    }
                        .into_any()
                }
            }}
        </MonolithShell>
    }
}
