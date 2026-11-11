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
