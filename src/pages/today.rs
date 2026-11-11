use crate::components::{MonolithShell, SquareCheck};
use crate::domain::Weekday;
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn TodayPage() -> impl IntoView {
    let store = use_app_store();
    let (draft, set_draft) = signal(String::new());

    let today_name = Weekday::today().name();
    let today_date = chrono::Local::now().format("%d %b %Y").to_string();

    let remaining_count = Signal::derive(move || {
        store
            .today_tasks
            .get()
            .iter()
            .filter(|t| !t.completed)
            .count()
    });

    let total_count = Signal::derive(move || store.today_tasks.get().len());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let text = draft.get();
        if !text.trim().is_empty() {
            store.add_today_task(text);
            set_draft.set(String::new());
        }
    };

    view! {
        <MonolithShell active_path="/">
            <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">
                        {today_name}
                    </h1>
                    <p class="font-mono text-xs text-gray">
                        {move || {
                            format!(
                                "{} — {} of {} remaining",
                                today_date,
                                remaining_count.get(),
                                total_count.get()
                            )
                        }}
                    </p>
                </div>

                <form
                    on:submit=on_submit
                    class="w-full flex flex-col items-center gap-2"
                >
                    <div class="w-full flex items-stretch border border-black dark:border-white bg-white dark:bg-black">
                        <input
                            type="text"
                            placeholder="Write a task for today..."
                            maxlength="200"
                            prop:value=move || draft.get()
                            on:input=move |ev| set_draft.set(event_target_value(&ev))
                            class="flex-1 min-w-0 bg-white dark:bg-black text-black dark:text-white placeholder-gray px-4 py-3 text-left text-sm focus:outline-none"
                        />
                        <button
                            type="submit"
                            class="px-5 py-3 bg-black dark:bg-white text-white dark:text-black font-mono text-xs border-l border-black dark:border-white hover:bg-white hover:text-black dark:hover:bg-black dark:hover:text-white transition-colors duration-75 cursor-pointer shrink-0"
                        >
                            "Add"
                        </button>
                    </div>
                </form>

                <div class="w-full flex flex-col items-center gap-2">
                    {move || {
                        let tasks = store.today_tasks.get();
                        if tasks.is_empty() {
                            view! {
                                <div class="w-full border border-gray py-10 px-4 text-center">
                                    <p class="text-sm text-gray">
                                        "No tasks left for today."
                                    </p>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ul class="w-full flex flex-col gap-2 max-h-[340px] overflow-y-auto scroll-monolith pr-2">
                                    {tasks
                                        .into_iter()
                                        .map(|task| {
                                            let task_id = task.id;
                                            let is_done = task.completed;
                                            let title_text = task.title.clone();
                                            let check_label = format!("Toggle {}", task.title);
                                            let remove_label = format!("Remove {}", task.title);

                                            view! {
                                                <li class=if is_done {
                                                    "cell-row-muted"
                                                } else {
                                                    "cell-row"
                                                }>
                                                    <SquareCheck
                                                        checked=is_done
                                                        on_toggle=Callback::new(move |_| {
                                                            store.toggle_today_task(task_id)
                                                        })
                                                        aria_label=check_label
                                                    />
                                                    <span class=if is_done {
                                                        "flex-1 min-w-0 px-1 text-left text-sm line-through text-gray break-words"
                                                    } else {
                                                        "flex-1 min-w-0 px-1 text-left text-sm text-black dark:text-white font-medium break-words"
                                                    }>
                                                        {title_text}
                                                    </span>
                                                    <button
                                                        type="button"
                                                        aria-label=remove_label
                                                        on:click=move |_| store.remove_today_task(task_id)
                                                        class="shrink-0 w-6 h-6 flex items-center justify-center font-mono text-xs text-gray hover:text-black dark:hover:text-white hover:border hover:border-black dark:hover:border-white cursor-pointer"
                                                    >
                                                        "×"
                                                    </button>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </section>
        </MonolithShell>
    }
}
