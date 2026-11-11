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
        <div class="w-full border border-black dark:border-white p-5 flex flex-col gap-3 bg-white dark:bg-black">
            <div class="pb-1 border-b border-black dark:border-white flex items-center justify-between">
                <span class=move || if is_enabled() {
                    "font-mono text-xs text-black dark:text-white uppercase tracking-wider font-bold"
                } else {
                    "font-mono text-xs text-gray uppercase tracking-wider font-bold"
                }>
                    {move || if is_enabled() { "Mobile App Lockdown (Active)" } else { "Mobile App Lockdown (Disabled)" }}
                </span>
                <button
                    type="button"
                    on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.lockdown_enabled = !c.lockdown_enabled; })
                    class="font-mono text-[10px] text-gray hover:text-black dark:hover:text-white underline cursor-pointer"
                >
                    {move || if is_enabled() { "Disable" } else { "Enable" }}
                </button>
            </div>

            <div class=move || {
                if !is_enabled() {
                    "w-full p-3 border border-gray/40 bg-gray/5 font-mono text-xs flex flex-col gap-1 text-gray"
                } else if is_locked() {
                    "w-full p-3 border border-black dark:border-white bg-black dark:bg-white text-white dark:text-black font-mono text-xs flex flex-col gap-1"
                } else {
                    "w-full p-3 border border-gray bg-white dark:bg-black font-mono text-xs flex flex-col gap-1 text-black dark:text-white"
                }
            }>
                <span class="font-bold">
                    {move || {
                        if !is_enabled() {
                            "○ STATUS: FEATURE DISABLED"
                        } else if is_locked() {
                            "⚠️ STATUS: LOCKDOWN ENFORCED"
                        } else {
                            "✓ STATUS: CLEAR"
                        }
                    }}
                </span>
                <span class="text-[11px] opacity-80">
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

            <div class="flex flex-col gap-1 text-xs text-gray">
                <p>"1 task missed -> locked remainder of day."</p>
                <p>"3+ tasks missed -> locked today and tomorrow."</p>
            </div>

            <div class="flex flex-col gap-2 pt-2 border-t border-black dark:border-white">
                <div class="flex items-center justify-between">
                    <span class="font-mono text-[11px] text-gray uppercase">"Private Lockdown API"</span>
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_regen_secret.run(()); }
                        class=move || if is_enabled() {
                            "font-mono text-[10px] text-gray hover:text-black dark:hover:text-white underline cursor-pointer"
                        } else {
                            "font-mono text-[10px] text-gray/40 no-underline cursor-not-allowed"
                        }
                    >
                        "Regenerate Key"
                    </button>
                </div>
                <div class=move || if is_enabled() {
                    "p-2 border border-black dark:border-white bg-white dark:bg-black font-mono text-[11px] text-black dark:text-white break-all select-all"
                } else {
                    "p-2 border border-gray/40 bg-gray/5 font-mono text-[11px] text-gray/50 break-all select-none cursor-not-allowed"
                }>
                    {move || api_url.get()}
                </div>
            </div>

            <div class="pt-2">
                <button
                    type="button"
                    prop:disabled=move || !is_enabled()
                    on:click=move |_| if is_enabled() { on_reset_lockdown.run(()); }
                    class=move || if is_enabled() {
                        "btn-monolith-ghost w-full text-xs"
                    } else {
                        "btn-monolith-ghost w-full text-xs opacity-40 cursor-not-allowed"
                    }
                >
                    "Emergency Reset Lockdown"
                </button>
            </div>
        </div>
    }
}
