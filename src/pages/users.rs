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

    let on_create = move |ev: SubmitEvent| {
        ev.prevent_default();
        match store.admin_create_user(username.get(), password.get(), as_admin.get()) {
            Ok(()) => {
                let created_name = username.get();
                set_username.set(String::new());
                set_password.set(String::new());
                set_as_admin.set(false);
                set_feedback.set(Some(format!("Added user '{}'.", created_name.trim())));
            }
            Err(err) => set_feedback.set(Some(err)),
        }
    };

    view! {
        <MonolithShell active_path="/users" wide=true>
            {move || {
                if !store.is_admin() {
                    view! {
                        <section class="w-full border border-black dark:border-white p-8 flex flex-col items-center justify-center gap-4 bg-white dark:bg-black">
                            <h1 class="text-2xl font-bold text-black dark:text-white">"Admin Only"</h1>
                            <p class="text-sm text-gray">
                                "Only administrators can view and manage users."
                            </p>
                            <a href="/login" class="btn-monolith-solid">
                                "Sign in with admin account"
                            </a>
                        </section>
                    }
                        .into_any()
                } else {
                    view! {
                        <section class="w-full flex flex-col items-center justify-center gap-6">
                            <div class="w-full flex flex-col items-center justify-center gap-1">
                                <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">
                                    "Users"
                                </h1>
                                <p class="text-xs text-gray">
                                    "Add accounts and assign administrator access."
                                </p>
                            </div>

                            <div class="w-full grid grid-cols-1 md:grid-cols-2 gap-5 items-start text-left">
                                <form
                                    on:submit=on_create
                                    class="w-full border border-black dark:border-white p-5 flex flex-col gap-3 bg-white dark:bg-black"
                                >
                                    <div class="pb-1 border-b border-black dark:border-white">
                                        <span class="font-mono text-xs text-black dark:text-white uppercase tracking-wider">
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
                                        class="w-full border border-gray px-4 py-2.5 flex items-center justify-center gap-3 cursor-pointer select-none"
                                    >
                                        <SquareCheck
                                            checked=as_admin.get()
                                            on_toggle=Callback::new(move |_| set_as_admin.update(|v| *v = !*v))
                                            aria_label="Set new user as admin".to_string()
                                        />
                                        <span class="text-xs text-black dark:text-white font-medium">
                                            "Set as admin"
                                        </span>
                                    </div>

                                    <button type="submit" class="btn-monolith-solid">
                                        "Add user"
                                    </button>

                                    {move || {
                                        feedback
                                            .get()
                                            .map(|msg| {
                                                view! {
                                                    <p class="w-full border border-black dark:border-white px-3 py-2 font-mono text-xs text-black dark:text-white">
                                                        {msg}
                                                    </p>
                                                }
                                            })
                                    }}
                                </form>

                                <div class="w-full flex flex-col gap-2">
                                    <div class="flex items-center justify-between px-1 pb-1 border-b border-black dark:border-white">
                                        <span class="font-mono text-xs text-black dark:text-white uppercase tracking-wider">
                                            "Directory"
                                        </span>
                                        <span class="font-mono text-xs text-gray">
                                            {move || format!("{} accounts", store.users.get().len())}
                                        </span>
                                    </div>

                                    <ul class="w-full flex flex-col gap-2 max-h-[340px] overflow-y-auto scroll-monolith pr-2">
                                {move || {
                                    store
                                        .users
                                        .get()
                                        .into_iter()
                                        .map(|u| {
                                            let uid = u.id;
                                            let is_root = u.is_first_admin;
                                            let role = u.role;
                                            let display_name = u.username.clone();
                                            let title_name = u.username;
                                            view! {
                                                <li class="cell-row">
                                                    <span class="font-mono text-[11px] px-2 py-0.5 border border-black dark:border-white bg-white dark:bg-black text-black dark:text-white shrink-0">
                                                        {role.label()}
                                                    </span>
                                                    <span
                                                        title=title_name
                                                        class="flex-1 min-w-0 px-2 text-left text-sm font-medium text-black dark:text-white truncate"
                                                    >
                                                        {display_name}
                                                    </span>
                                                    {if is_root {
                                                        view! {
                                                            <span class="font-mono text-[11px] text-gray shrink-0">
                                                                "Initial admin"
                                                            </span>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! {
                                                            <div class="flex items-center gap-1.5 shrink-0">
                                                                <button
                                                                    type="button"
                                                                    on:click=move |_| store.admin_toggle_role(uid)
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
                                                                    on:click=move |_| store.admin_remove_user(uid)
                                                                    class="w-7 h-7 border border-gray text-gray hover:border-black dark:hover:border-white hover:text-black dark:hover:text-white font-mono text-xs flex items-center justify-center cursor-pointer shrink-0"
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
