use crate::components::{MonolithShell, SquareCheck};
use crate::domain::{TodayTask, Weekday};
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
    let today_date = chrono::Local::now().format("%d %b %Y").to_string();

    let in_progress = Signal::derive(move || {
        store
            .today_tasks
            .get()
            .into_iter()
            .filter(|t| !t.completed)
            .collect::<Vec<TodayTask>>()
    });

    let done = Signal::derive(move || {
        store
            .today_tasks
            .get()
            .into_iter()
            .filter(|t| t.completed)
            .collect::<Vec<TodayTask>>()
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let text = draft.get();
        if !text.trim().is_empty() {
            let st = start_time.get();
            let st_opt = if st.trim().is_empty() { None } else { Some(st) };
            let dl = deadline.get();
            let dl_opt = if dl.trim().is_empty() { None } else { Some(dl) };
            store.add_today_task(text, st_opt, dl_opt);
            set_draft.set(String::new());
            set_start_time.set(String::new());
            set_deadline.set(String::new());
        }
    };

    let render_task = move |task: TodayTask, is_done: bool| {
        let task_id_toggle = task.id.clone();
        let task_id_remove = task.id.clone();
        let title_text = task.title.clone();
        let time_badge = match (&task.start, &task.deadline) {
            (Some(s), Some(d)) => Some(format!("{} - {}", s, d)),
            (Some(s), None) => Some(s.clone()),
            (None, Some(d)) => Some(d.clone()),
            (None, None) => None,
        };
        let check_label = format!("Toggle {}", task.title);
        let remove_label = format!("Remove {}", task.title);

        view! {
            <li class=if is_done { "cell-row-muted" } else { "cell-row" }>
                <SquareCheck
                    checked=is_done
                    on_toggle=Callback::new(move |_| store.toggle_today_task(&task_id_toggle))
                    aria_label=check_label
                />
                <span class=if is_done {
                    "flex-1 min-w-0 px-1 text-left text-sm line-through text-gray break-words"
                } else {
                    "flex-1 min-w-0 px-1 text-left text-sm text-black dark:text-white font-medium break-words"
                }>
                    {title_text}
                </span>
                {time_badge.map(|tb| {
                    view! {
                        <span class="font-mono text-[10px] text-gray border border-gray px-1.5 py-0.5 shrink-0 whitespace-nowrap">
                            {tb}
                        </span>
                    }
                })}
                <button
                    type="button"
                    aria-label=remove_label
                    on:click=move |_| store.remove_today_task(&task_id_remove)
                    class="shrink-0 w-6 h-6 flex items-center justify-center font-mono text-xs text-gray hover:text-black dark:hover:text-white hover:border hover:border-black dark:hover:border-white cursor-pointer"
                >
                    "×"
                </button>
            </li>
        }
    };

    view! {
        <MonolithShell active_path="/" wide=true>
            <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">
                        {today_name}
                    </h1>
                    <p class="font-mono text-xs text-gray">
                        {move || {
                            format!(
                                "{} — {} in progress, {} done",
                                today_date,
                                in_progress.get().len(),
                                done.get().len()
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
                                    list.into_iter().map(|t| render_task(t, false)).collect_view().into_any()
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
                                    list.into_iter().map(|t| render_task(t, true)).collect_view().into_any()
                                }
                            }}
                        </ul>
                    </div>
                </div>
            </section>
        </MonolithShell>
    }
}
