use crate::components::{MonolithShell, TodayTaskCell};
use crate::domain::Weekday;
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn TodayPage() -> impl IntoView {
    let store = use_app_store();
    let (draft, set_draft) = signal(String::new());
    let (start_time, set_start_time) = signal(String::new());
    let (deadline, set_deadline) = signal(String::new());

    let today_name = Weekday::today().name();
    let today_date = Signal::derive(|| chrono::Local::now().format("%d %b %Y").to_string());

    let in_progress = Signal::derive(move || {
        let tasks = store.today_tasks.get();
        tasks
            .into_iter()
            .filter(|t| !t.completed)
            .collect::<Vec<_>>()
    });

    let done = Signal::derive(move || {
        let tasks = store.today_tasks.get();
        tasks
            .into_iter()
            .filter(|t| t.completed)
            .collect::<Vec<_>>()
    });

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(tasks) = crate::backend::api_tasks::fetch_today_tasks(None).await {
                store.today_tasks.set(tasks);
            }
        });
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let text = draft.get();
        if !text.trim().is_empty() {
            let st_opt = (!start_time.get().trim().is_empty()).then(|| start_time.get());
            let dl_opt = (!deadline.get().trim().is_empty()).then(|| deadline.get());
            set_draft.set(String::new());
            set_start_time.set(String::new());
            set_deadline.set(String::new());

            let text_c = text.clone();
            let st_c = st_opt.clone();
            let dl_c = dl_opt.clone();
            leptos::task::spawn_local(async move {
                if let Ok(task) =
                    crate::backend::api_tasks::create_today_task(text_c, st_c, dl_c, None).await
                {
                    store.today_tasks.update(|l| l.insert(0, task));
                } else {
                    store.add_today_task(text, st_opt, dl_opt);
                }
            });
        }
    };

    let on_toggle_task = Callback::new(move |id: String| {
        store.toggle_today_task(&id);
        leptos::task::spawn_local(async move {
            let _ = crate::backend::api_tasks::toggle_task_completed(id).await;
        });
    });

    let on_remove_task = Callback::new(move |id: String| {
        store.remove_today_task(&id);
        leptos::task::spawn_local(async move {
            let _ = crate::backend::api_tasks::delete_today_task(id).await;
        });
    });

    view! {
        <MonolithShell active_path="/" wide=true>
            {move || {
                if store.auth_initialized.get() && store.current_user_id.get().is_none() {
                    view! {
                        <section class="w-full border border-black dark:border-white p-8 flex flex-col items-center justify-center gap-4 bg-white dark:bg-black">
                            <h1 class="text-2xl font-bold text-black dark:text-white">
                                {if store.has_any_user() { "Sign In Required" } else { "Welcome to Trak" }}
                            </h1>
                            <p class="text-sm text-gray">
                                {if store.has_any_user() {
                                    "Sign in to manage today's tasks."
                                } else {
                                    "Setup the initial administrator account."
                                }}
                            </p>
                            <a
                                href=if store.has_any_user() { "/login" } else { "/register" }
                                class="btn-monolith-solid"
                            >
                                {if store.has_any_user() { "Go to Sign in" } else { "Setup root admin" }}
                            </a>
                        </section>
                    }
                        .into_any()
                } else {
                    view! {
                        <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">
                        {today_name}
                    </h1>
                    <p class="font-mono text-xs text-gray">
                        {move || {
                            format!(
                                "{} — {} in progress, {} done",
                                today_date.get(),
                                in_progress.get().len(),
                                done.get().len(),
                            )
                        }}
                    </p>
                </div>

                <form on:submit=on_submit class="w-full flex flex-col items-center gap-2">
                    <div class="w-full flex items-stretch border border-black dark:border-white bg-white dark:bg-black">
                        <input
                            type="text"
                            placeholder="Write a task..."
                            maxlength="200"
                            prop:value=move || draft.get()
                            on:input=move |ev| set_draft.set(event_target_value(&ev))
                            class="flex-1 min-w-0 bg-white dark:bg-black text-black dark:text-white placeholder-gray px-3 sm:px-4 py-3 text-left text-sm focus:outline-none"
                        />
                        <input
                            type="text"
                            placeholder="Start"
                            title="Start time (e.g. 09:00)"
                            maxlength="20"
                            prop:value=move || start_time.get()
                            on:input=move |ev| set_start_time.set(event_target_value(&ev))
                            class="w-20 sm:w-28 border-l border-black dark:border-white bg-white dark:bg-black text-black dark:text-white placeholder-gray px-2 sm:px-3 py-3 text-left text-xs font-mono focus:outline-none shrink-0"
                        />
                        <input
                            type="text"
                            placeholder="Due"
                            title="Deadline (e.g. 17:00)"
                            maxlength="20"
                            prop:value=move || deadline.get()
                            on:input=move |ev| set_deadline.set(event_target_value(&ev))
                            class="w-20 sm:w-28 border-l border-black dark:border-white bg-white dark:bg-black text-black dark:text-white placeholder-gray px-2 sm:px-3 py-3 text-left text-xs font-mono focus:outline-none shrink-0"
                        />
                        <button
                            type="submit"
                            class="px-3 sm:px-5 py-3 bg-black dark:bg-white text-white dark:text-black font-mono text-xs border-l border-black dark:border-white hover:bg-white hover:text-black dark:hover:bg-black dark:hover:text-white transition-colors duration-75 cursor-pointer shrink-0"
                        >
                            "Add"
                        </button>
                    </div>
                </form>

                <div class="w-full grid grid-cols-1 md:grid-cols-2 gap-5 items-start text-left">
                    <div class="w-full flex flex-col gap-2">
                        <div class="flex items-center justify-between px-1 pb-1 border-b border-black dark:border-white">
                            <span class="font-mono text-xs text-black dark:text-white uppercase tracking-wider">
                                "In Progress"
                            </span>
                            <span class="font-mono text-xs text-gray">
                                {move || format!("{} items", in_progress.get().len())}
                            </span>
                        </div>

                        <ul class="w-full flex flex-col gap-2 max-h-[360px] overflow-y-auto scroll-monolith">
                            {move || {
                                let list = in_progress.get();
                                if list.is_empty() {
                                    view! {
                                        <li class="w-full border border-gray py-8 px-4 text-center">
                                            <p class="text-sm text-gray">"No tasks in progress."</p>
                                        </li>
                                    }
                                        .into_any()
                                } else {
                                    list.into_iter()
                                        .map(|t| {
                                            view! {
                                                <TodayTaskCell
                                                    task=t
                                                    is_done=false
                                                    on_toggle=on_toggle_task
                                                    on_remove=on_remove_task
                                                />
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                            }}
                        </ul>
                    </div>

                    <div class="w-full flex flex-col gap-2">
                        <div class="flex items-center justify-between px-1 pb-1 border-b border-black dark:border-white">
                            <span class="font-mono text-xs text-black dark:text-white uppercase tracking-wider">
                                "Done"
                            </span>
                            <span class="font-mono text-xs text-gray">
                                {move || format!("{} completed", done.get().len())}
                            </span>
                        </div>

                        <ul class="w-full flex flex-col gap-2 max-h-[360px] overflow-y-auto scroll-monolith">
                            {move || {
                                let list = done.get();
                                if list.is_empty() {
                                    view! {
                                        <li class="w-full border border-gray py-8 px-4 text-center">
                                            <p class="text-sm text-gray">"No completed tasks yet."</p>
                                        </li>
                                    }
                                        .into_any()
                                } else {
                                    list.into_iter()
                                        .map(|t| {
                                            view! {
                                                <TodayTaskCell
                                                    task=t
                                                    is_done=true
                                                    on_toggle=on_toggle_task
                                                    on_remove=on_remove_task
                                                />
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
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
