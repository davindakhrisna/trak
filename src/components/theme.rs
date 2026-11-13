use crate::domain::ThemeMode;
use crate::state::use_app_store;
use leptos::prelude::*;

pub fn apply_theme_to_dom(_mode: ThemeMode) {
    #[cfg(feature = "hydrate")]
    {
        let mode = _mode;
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(root) = doc.document_element() {
                    let is_dark = match mode {
                        ThemeMode::Dark => true,
                        ThemeMode::Light => false,
                        ThemeMode::System => win
                            .match_media("(prefers-color-scheme: dark)")
                            .ok()
                            .flatten()
                            .map(|m| m.matches())
                            .unwrap_or(false),
                    };

                    let classes = root.class_list();
                    if is_dark {
                        let _ = classes.add_1("dark");
                    } else {
                        let _ = classes.remove_1("dark");
                    }

                    if let Ok(Some(storage)) = win.local_storage() {
                        let _ = storage.set_item("trak-theme", mode.as_str());
                    }
                }
            }
        }
    }
}

pub fn read_stored_theme() -> ThemeMode {
    #[cfg(feature = "hydrate")]
    {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                if let Ok(Some(val)) = storage.get_item("trak-theme") {
                    if let Some(mode) = ThemeMode::from_str_opt(&val) {
                        return mode;
                    }
                }
            }
        }
    }
    ThemeMode::System
}

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let store = use_app_store();

    Effect::new(move |_| {
        let stored = read_stored_theme();
        store.theme_mode.set(stored);
        apply_theme_to_dom(stored);

        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            if let Some(win) = web_sys::window() {
                if let Ok(Some(mql)) = win.match_media("(prefers-color-scheme: dark)") {
                    let on_change =
                        Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
                            if store.theme_mode.get_untracked() == ThemeMode::System {
                                apply_theme_to_dom(ThemeMode::System);
                            }
                        });
                    let _ = mql.add_event_listener_with_callback(
                        "change",
                        on_change.as_ref().unchecked_ref(),
                    );
                    on_change.forget();
                }
            }
        }
    });

    let set_mode = move |mode: ThemeMode| {
        store.theme_mode.set(mode);
        apply_theme_to_dom(mode);
    };

    let btn_class = move |target: ThemeMode, is_last: bool| {
        let active = store.theme_mode.get() == target;
        let border_r = if is_last { "" } else { "border-r border-rule" };
        if active {
            format!(
                "px-2.5 py-1 bg-ink text-canvas font-bold {} cursor-pointer select-none transition-colors duration-75",
                border_r
            )
        } else {
            format!(
                "px-2.5 py-1 bg-surface text-ink-muted hover:text-ink {} cursor-pointer select-none transition-colors duration-75",
                border_r
            )
        }
    };

    view! {
        <div
            role="radiogroup"
            aria-label="Theme selector"
            class="inline-flex items-center border border-rule bg-surface font-mono text-[11px] uppercase tracking-wider"
        >
            <button
                type="button"
                role="radio"
                aria-checked=move || (store.theme_mode.get() == ThemeMode::System).to_string()
                on:click=move |_| set_mode(ThemeMode::System)
                class=move || btn_class(ThemeMode::System, false)
                title="Follow system color scheme"
            >
                "sys"
            </button>
            <button
                type="button"
                role="radio"
                aria-checked=move || (store.theme_mode.get() == ThemeMode::Light).to_string()
                on:click=move |_| set_mode(ThemeMode::Light)
                class=move || btn_class(ThemeMode::Light, false)
                title="Force light theme"
            >
                "light"
            </button>
            <button
                type="button"
                role="radio"
                aria-checked=move || (store.theme_mode.get() == ThemeMode::Dark).to_string()
                on:click=move |_| set_mode(ThemeMode::Dark)
                class=move || btn_class(ThemeMode::Dark, true)
                title="Force dark theme"
            >
                "dark"
            </button>
        </div>
    }
}
