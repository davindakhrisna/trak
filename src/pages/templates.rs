use crate::components::MonolithShell;
use crate::domain::Weekday;
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn TemplatesPage() -> impl IntoView {
    let store = use_app_store();
    let (draft, set_draft) = signal(String::new());

    let day_templates = Signal::derive(move || {
        let day = store.selected_day.get();
        store
            .templates
            .get()
            .into_iter()
            .filter(|tpl| tpl.day == day)
            .collect::<Vec<_>>()
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let text = draft.get();
        if !text.trim().is_empty() {
            store.add_template(store.selected_day.get(), text);
            set_draft.set(String::new());
        }
    };

    view! {
        <MonolithShell active_path="/templates">
            <section class="w-full flex flex-col items-center justify-center gap-6">
                <div class="w-full flex flex-col items-center justify-center gap-1">
                    <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">
                        "Templates"
                    </h1>
                    <p class="text-xs text-gray">
                        "Select a day to provision recurring tasks."
                    </p>
                </div>

                <div
                    role="tablist"
                    aria-label="Select day of week"
                    class="w-full grid grid-cols-7 border border-black dark:border-white bg-white dark:bg-black"
                >
                    {Weekday::ALL
                        .into_iter()
                        .map(|day| {
                            let is_selected = move || store.selected_day.get() == day;
                            let count_for_day = move || {
                                store
                                    .templates
                                    .get()
                                    .iter()
                                    .filter(|t| t.day == day)
                                    .count()
                            };

                            view! {
                                <button
                                    type="button"
                                    role="tab"
                                    aria-selected=move || if is_selected() { "true" } else { "false" }
                                    on:click=move |_| store.selected_day.set(day)
                                    class=move || {
                                        if is_selected() {
                                            "py-3 flex flex-col items-center justify-center gap-0.5 bg-black dark:bg-white text-white dark:text-black border-r border-black dark:border-white last:border-r-0 font-mono text-xs cursor-pointer transition-colors duration-75"
                                        } else {
                                            "py-3 flex flex-col items-center justify-center gap-0.5 bg-white dark:bg-black text-black dark:text-white hover:bg-black hover:text-white dark:hover:bg-white dark:hover:text-black border-r border-black dark:border-white last:border-r-0 font-mono text-xs transition-colors duration-75 cursor-pointer"
                                        }
                                    }
                                >
                                    <span class="font-bold">{day.code()}</span>
                                    <span class=move || {
                                        if is_selected() {
                                            "text-[10px] text-white dark:text-black"
                                        } else {
                                            "text-[10px] text-gray"
                                        }
                                    }>{move || count_for_day()}</span>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                <form
                    on:submit=on_submit
                    class="w-full flex flex-col items-center gap-2"
                >
                    <div class="w-full flex items-stretch border border-black dark:border-white bg-white dark:bg-black">
                        <input
                            type="text"
                            placeholder=move || {
                                format!(
                                    "Add task recurring every {}...",
                                    store.selected_day.get().name()
                                )
                            }
                            maxlength="200"
                            prop:value=move || draft.get()
                            on:input=move |ev| set_draft.set(event_target_value(&ev))
                            class="flex-1 min-w-0 bg-white dark:bg-black text-black dark:text-white placeholder-gray px-4 py-3 text-left text-sm focus:outline-none"
                        />
                        <button
                            type="submit"
                            class="px-5 py-3 bg-black dark:bg-white text-white dark:text-black font-mono text-xs border-l border-black dark:border-white hover:bg-white hover:text-black dark:hover:bg-black dark:hover:text-white transition-colors duration-75 cursor-pointer shrink-0"
                        >
                            "Save"
                        </button>
                    </div>
                </form>

                <div class="w-full flex flex-col items-center gap-2">
                    {move || {
                        let list = day_templates.get();
                        let day_label = store.selected_day.get().name();
                        if list.is_empty() {
                            view! {
                                <div class="w-full border border-gray py-8 px-4 text-center">
                                    <p class="text-sm text-gray">
                                        {format!("No recurring tasks set for {}.", day_label)}
                                    </p>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ul class="w-full flex flex-col gap-2 max-h-[340px] overflow-y-auto scroll-monolith pr-2">
                                    {list
                                        .into_iter()
                                        .map(|tpl| {
                                            let tpl_id = tpl.id;
                                            let title_text = tpl.title.clone();
                                            let day_code = tpl.day.code();
                                            let del_label = format!("Delete {}", tpl.title);

                                            view! {
                                                <li class="cell-row">
                                                    <span class="font-mono text-[11px] text-gray border border-gray px-1.5 py-0.5 shrink-0">
                                                        {day_code}
                                                    </span>
                                                    <span class="flex-1 min-w-0 px-1 text-left text-sm text-black dark:text-white font-medium break-words">
                                                        {title_text}
                                                    </span>
                                                    <button
                                                        type="button"
                                                        aria-label=del_label
                                                        on:click=move |_| store.remove_template(tpl_id)
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
