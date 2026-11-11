use crate::components::nav::CenteredNav;
use crate::components::theme::ThemeSelector;
use leptos::prelude::*;

#[component]
pub fn MonolithShell(
    active_path: &'static str,
    #[prop(optional)] hide_nav: bool,
    #[prop(optional)] wide: bool,
    children: Children,
) -> impl IntoView {
    let spine_class = if wide {
        "monolith-spine-wide"
    } else {
        "monolith-spine"
    };

    view! {
        <div class="monolith-page">
            <div class=spine_class>
                {if !hide_nav {
                    view! { <CenteredNav active_path=active_path /> }.into_any()
                } else {
                    ().into_any()
                }}
                <main class="w-full flex flex-col items-center justify-center text-center">
                    {children()}
                </main>
                {if hide_nav {
                    view! {
                        <footer class="mt-8 flex items-center justify-center">
                            <ThemeSelector />
                        </footer>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }}
            </div>
        </div>
    }
}

#[component]
pub fn SquareCheck(checked: bool, on_toggle: Callback<()>, aria_label: String) -> impl IntoView {
    view! {
        <button
            type="button"
            role="checkbox"
            aria-checked=if checked { "true" } else { "false" }
            aria-label=aria_label
            on:click=move |_| on_toggle.run(())
            class=if checked { "check-square-active" } else { "check-square" }
        >
            {if checked {
                view! { <span class="block w-2 h-2 bg-white dark:bg-black"></span> }.into_any()
            } else {
                ().into_any()
            }}
        </button>
    }
}

#[component]
pub fn TodayTaskCell(
    task: crate::domain::TodayTask,
    is_done: bool,
    on_toggle: Callback<String>,
    on_remove: Callback<String>,
) -> impl IntoView {
    let task_id_toggle = task.id.clone();
    let task_id_remove = task.id.clone();
    let title_text = task.title.clone();
    let time_badge = match (&task.start, &task.deadline) {
        (Some(s), Some(d)) => Some(format!("{} - {}", s, d)),
        (Some(s), None) => Some(s.clone()),
        (None, Some(d)) => Some(d.clone()),
        (None, None) => None,
    };
    view! {
        <li class=if is_done { "cell-row-muted" } else { "cell-row" }>
            <SquareCheck
                checked=is_done
                on_toggle=Callback::new(move |_| on_toggle.run(task_id_toggle.clone()))
                aria_label=format!("Toggle {}", task.title)
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
                aria-label=format!("Remove {}", task.title)
                on:click=move |_| on_remove.run(task_id_remove.clone())
                class="shrink-0 w-6 h-6 flex items-center justify-center font-mono text-xs text-gray hover:text-black dark:hover:text-white hover:border hover:border-black dark:hover:border-white cursor-pointer"
            >
                "×"
            </button>
        </li>
    }
}

#[component]
pub fn TemplateItemCell(
    tpl: crate::domain::RecurringTemplate,
    on_delete: Callback<String>,
) -> impl IntoView {
    let tpl_id = tpl.id.clone();
    let title_text = tpl.title.clone();
    let day_code = tpl.day.code();
    let time_badge = match (&tpl.start, &tpl.deadline) {
        (Some(s), Some(d)) => Some(format!("{} - {}", s, d)),
        (Some(s), None) => Some(s.clone()),
        (None, Some(d)) => Some(d.clone()),
        (None, None) => None,
    };
    let del_label = format!("Delete {}", tpl.title);
    view! {
        <li class="cell-row">
            <span class="font-mono text-[11px] text-gray border border-gray px-1.5 py-0.5 shrink-0">
                {day_code}
            </span>
            <span class="flex-1 min-w-0 px-1 text-left text-sm text-black dark:text-white font-medium break-words">
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
                aria-label=del_label
                on:click=move |_| on_delete.run(tpl_id.clone())
                class="shrink-0 w-6 h-6 flex items-center justify-center font-mono text-xs text-gray hover:text-black dark:hover:text-white hover:border hover:border-black dark:hover:border-white cursor-pointer"
            >
                "×"
            </button>
        </li>
    }
}
