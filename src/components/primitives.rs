use crate::components::nav::{DesktopSidebar, MobileNav};
use crate::components::theme::ThemeSelector;
use leptos::prelude::*;

#[component]
pub fn MonolithShell(
    active_path: &'static str,
    #[prop(optional)] hide_nav: bool,
    #[prop(optional)] wide: bool,
    children: Children,
) -> impl IntoView {
    let spine_width_class = if wide {
        "w-full max-w-spine-wide lg:w-[780px]"
    } else {
        "w-full max-w-spine lg:w-[440px]"
    };

    if hide_nav {
        let spine_class = if wide {
            "monolith-spine-wide"
        } else {
            "monolith-spine"
        };
        return view! {
            <div class="monolith-page">
                <div class=spine_class>
                    <main class="w-full flex flex-col items-center justify-center text-center">
                        {children()}
                    </main>
                    <footer class="mt-8 flex items-center justify-center">
                        <ThemeSelector />
                    </footer>
                </div>
            </div>
        }
        .into_any();
    }

    view! {
        <div class="min-h-screen w-full overflow-x-hidden bg-canvas text-ink flex flex-col items-center justify-start transition-colors duration-75">
            <MobileNav active_path=active_path />
            <div class="w-full flex-1 lg:flex-none flex flex-col items-center justify-start px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-12 my-auto">
                <div class="w-full lg:w-fit max-w-full mx-auto flex flex-col lg:flex-row items-stretch justify-center gap-6 lg:gap-8">
                    <DesktopSidebar active_path=active_path />
                    <main class="w-full flex-1 flex flex-col items-stretch lg:items-center justify-start text-center">
                        <div class=format!("{} w-full max-w-full mx-auto flex flex-col items-center text-center", spine_width_class)>
                            {children()}
                        </div>
                    </main>
                </div>
            </div>
        </div>
    }
    .into_any()
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
                view! { <span class="block w-2 h-2 bg-canvas"></span> }.into_any()
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
                "flex-1 min-w-0 px-1 text-left text-sm line-through text-ink-muted break-words"
            } else {
                "flex-1 min-w-0 px-1 text-left text-sm text-ink font-medium break-words"
            }>
                {title_text}
            </span>
            {time_badge.map(|tb| {
                view! {
                    <span class="font-mono text-[11px] tabular-nums text-ink-muted bg-muted border border-rule px-2 py-0.5 shrink-0 whitespace-nowrap">
                        {tb}
                    </span>
                }
            })}
            <button
                type="button"
                aria-label=format!("Remove {}", task.title)
                on:click=move |_| on_remove.run(task_id_remove.clone())
                class="shrink-0 w-7 h-7 sm:w-6 sm:h-6 flex items-center justify-center font-mono text-sm sm:text-xs text-ink-muted border border-transparent hover:border-signal hover:text-signal hover:bg-signal-soft transition-colors duration-75 cursor-pointer"
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
            <span class="font-mono text-[11px] text-ink bg-muted border border-rule px-2 py-0.5 shrink-0 font-medium">
                {day_code}
            </span>
            <span class="flex-1 min-w-0 px-1 text-left text-sm text-ink font-medium break-words">
                {title_text}
            </span>
            {time_badge.map(|tb| {
                view! {
                    <span class="font-mono text-[11px] tabular-nums text-ink-muted bg-muted border border-rule px-2 py-0.5 shrink-0 whitespace-nowrap">
                        {tb}
                    </span>
                }
            })}
            <button
                type="button"
                aria-label=del_label
                on:click=move |_| on_delete.run(tpl_id.clone())
                class="shrink-0 w-7 h-7 sm:w-6 sm:h-6 flex items-center justify-center font-mono text-sm sm:text-xs text-ink-muted border border-transparent hover:border-signal hover:text-signal hover:bg-signal-soft transition-colors duration-75 cursor-pointer"
            >
                "×"
            </button>
        </li>
    }
}
