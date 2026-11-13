use crate::domain::UserSettings;
use leptos::prelude::*;

#[component]
pub fn SettingsLockdownCard(
    settings: ReadSignal<Option<UserSettings>>,
    set_settings: WriteSignal<Option<UserSettings>>,
    api_url: Signal<String>,
    on_regen_secret: Callback<()>,
    on_reset_lockdown: Callback<()>,
) -> impl IntoView {
    let cfg = move || settings.get().unwrap_or_default();
    let is_enabled = move || cfg().lockdown_enabled;
    let is_locked = move || is_enabled() && cfg().locked_until.is_some();

    view! {
        <div class="w-full h-full border border-rule p-5 flex flex-col gap-3.5 bg-surface">
            <div class="pb-2 border-b border-rule flex items-center justify-between gap-2">
                <span class=move || if is_enabled() {
                    "font-mono text-xs text-ink uppercase tracking-wider font-bold"
                } else {
                    "font-mono text-xs text-ink-muted uppercase tracking-wider font-bold"
                }>
                    {move || if is_enabled() { "Mobile App Lockdown (Active)" } else { "Mobile App Lockdown (Disabled)" }}
                </span>
                <button
                    type="button"
                    on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.lockdown_enabled = !c.lockdown_enabled; })
                    class="font-mono text-[11px] text-ink-muted hover:text-ink underline cursor-pointer"
                >
                    {move || if is_enabled() { "Disable" } else { "Enable" }}
                </button>
            </div>

            <div class=move || {
                if !is_enabled() {
                    "w-full p-3 border border-rule bg-muted font-mono text-xs flex flex-col gap-1 text-ink-muted"
                } else if is_locked() {
                    "w-full p-3 border border-signal bg-signal-soft text-signal font-mono text-xs flex flex-col gap-1"
                } else {
                    "w-full p-3 border border-rule-strong bg-surface font-mono text-xs flex flex-col gap-1 text-ink"
                }
            }>
                <div class="flex items-center gap-2 font-bold uppercase tracking-wide">
                    <span class=move || {
                        if !is_enabled() {
                            "inline-block w-2 h-2 border border-ink-muted shrink-0"
                        } else if is_locked() {
                            "inline-block w-2 h-2 bg-signal shrink-0"
                        } else {
                            "inline-block w-2 h-2 bg-ink shrink-0"
                        }
                    } aria-hidden="true"></span>
                    <span>
                        {move || {
                            if !is_enabled() {
                                "STATUS: FEATURE DISABLED"
                            } else if is_locked() {
                                "STATUS: LOCKDOWN ENFORCED"
                            } else {
                                "STATUS: CLEAR"
                            }
                        }}
                    </span>
                </div>
                <span class="text-[11px] text-ink-muted">
                    {move || {
                        if !is_enabled() {
                            "Lockdown enforcement is disabled. Distraction apps remain unlocked.".to_string()
                        } else if is_locked() {
                            format!("Locked until: {}", cfg().locked_until.unwrap_or_default())
                        } else {
                            "No missed deadlines. Distraction apps unlocked.".to_string()
                        }
                    }}
                </span>
            </div>

            <div class="flex flex-col gap-1 text-xs text-ink-muted">
                <p>"1 task missed → locked remainder of day."</p>
                <p>"3+ tasks missed → locked today and tomorrow."</p>
            </div>

            <div class="flex flex-col gap-1.5 pt-2 border-t border-rule">
                <div class="flex items-center justify-between">
                    <span class="font-mono text-[11px] text-ink-muted uppercase">"Private Lockdown API"</span>
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_regen_secret.run(()); }
                        class=move || if is_enabled() {
                            "font-mono text-[11px] text-ink-muted hover:text-ink underline cursor-pointer"
                        } else {
                            "font-mono text-[11px] text-ink-muted opacity-50 no-underline cursor-not-allowed"
                        }
                    >
                        "Regenerate Key"
                    </button>
                </div>
                <div class=move || if is_enabled() {
                    "p-2.5 border border-rule bg-muted font-mono text-[11px] text-ink break-all select-all"
                } else {
                    "p-2.5 border border-rule bg-muted font-mono text-[11px] text-ink-muted opacity-60 break-all select-none cursor-not-allowed"
                }>
                    {move || api_url.get()}
                </div>
            </div>

            <div class="mt-auto pt-2">
                <button
                    type="button"
                    prop:disabled=move || !is_enabled()
                    on:click=move |_| if is_enabled() { on_reset_lockdown.run(()); }
                    class=move || if is_enabled() {
                        "btn-monolith-danger w-full text-xs"
                    } else {
                        "btn-monolith-ghost w-full text-xs opacity-50 cursor-not-allowed"
                    }
                >
                    "Emergency Reset Lockdown"
                </button>
            </div>
        </div>
    }
}
