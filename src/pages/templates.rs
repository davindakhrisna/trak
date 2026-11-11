use crate::components::{MonolithShell, TemplateItemCell};
use crate::domain::Weekday;
use crate::state::use_app_store;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn TemplatesPage() -> impl IntoView {
    let store = use_app_store();
    let (draft, set_draft) = signal(String::new());
    let (start_time, set_start_time) = signal(String::new());
    let (deadline, set_deadline) = signal(String::new());

    let day_templates = Signal::derive(move || {
        let day = store.selected_day.get();
        store
            .templates
            .get()
            .into_iter()
            .filter(|tpl| tpl.day == day)
            .collect::<Vec<_>>()
    });

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(list) = crate::backend::api_templates::fetch_templates().await {
                store.templates.set(list);
            }
        });
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let text = draft.get();
        if !text.trim().is_empty() {
            let st = start_time.get();
            let st_opt = if st.trim().is_empty() { None } else { Some(st) };
            let dl = deadline.get();
            let dl_opt = if dl.trim().is_empty() { None } else { Some(dl) };
            set_draft.set(String::new());
            set_start_time.set(String::new());
            set_deadline.set(String::new());

            let sel_day = store.selected_day.get();
            let text_c = text.clone();
            let st_c = st_opt.clone();
            let dl_c = dl_opt.clone();
            leptos::task::spawn_local(async move {
                if let Ok(tpl) =
                    crate::backend::api_templates::create_template(sel_day, text_c, st_c, dl_c)
                        .await
                {
                    store.templates.update(|l| l.push(tpl));
                }
            });
        }
    };

    let on_delete_template = Callback::new(move |id: String| {
        store.remove_template(&id);
        leptos::task::spawn_local(async move {
            let _ = crate::backend::api_templates::delete_template(id).await;
        });
    });

    view! {
        <MonolithShell active_path="/templates" wide=true>
            {move || {
                if !store.auth_initialized.get() {
                    view! { <div class="py-12 font-mono text-xs text-gray">"Verifying session..."</div> }.into_any()
                } else if store.current_user_id.get().is_none() {
                    view! { <div class="py-12 font-mono text-xs text-gray">"Redirecting to sign in..."</div> }.into_any()
                } else {
                    view! {
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
                            let count_for_day = move || store.templates.get().iter().filter(|t| t.day == day).count();

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
                            placeholder="New recurring task..."
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
                            title="Deadline (e.g. 10:00)"
                            maxlength="20"
                            prop:value=move || deadline.get()
                            on:input=move |ev| set_deadline.set(event_target_value(&ev))
                            class="w-20 sm:w-28 border-l border-black dark:border-white bg-white dark:bg-black text-black dark:text-white placeholder-gray px-2 sm:px-3 py-3 text-left text-xs font-mono focus:outline-none shrink-0"
                        />
                        <button
                            type="submit"
                            class="px-3 sm:px-5 py-3 bg-black dark:bg-white text-white dark:text-black font-mono text-xs border-l border-black dark:border-white hover:bg-white hover:text-black dark:hover:bg-black dark:hover:text-white transition-colors duration-75 cursor-pointer shrink-0"
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
                                <ul class="w-full flex flex-col gap-2 max-h-[340px] overflow-y-auto scroll-monolith">
                                    {list
                                        .into_iter()
                                        .map(|tpl| {
                                            view! {
                                                <TemplateItemCell
                                                    tpl=tpl
                                                    on_delete=on_delete_template
                                                />
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
                    }
                        .into_any()
                }
            }}
        </MonolithShell>
    }
}
