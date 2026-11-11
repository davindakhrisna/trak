use crate::components::SquareCheck;
use crate::domain::UserSettings;
use leptos::prelude::*;

#[component]
pub fn SettingsNtfyCard(
    settings: ReadSignal<Option<UserSettings>>,
    set_settings: WriteSignal<Option<UserSettings>>,
    on_test_ntfy: Callback<()>,
    on_regen_topic: Callback<()>,
) -> impl IntoView {
    let cfg = move || settings.get().unwrap_or_default();
    let is_enabled = move || cfg().ntfy_enabled;

    view! {
        <div class="w-full border border-black dark:border-white p-5 flex flex-col gap-3 bg-white dark:bg-black">
            <div class="pb-1 border-b border-black dark:border-white flex items-center justify-between">
                <span class=move || if is_enabled() {
                    "font-mono text-xs text-black dark:text-white uppercase tracking-wider font-bold"
                } else {
                    "font-mono text-xs text-gray uppercase tracking-wider font-bold"
                }>
                    {move || if is_enabled() { "Ntfy Alerts (Active)" } else { "Ntfy Alerts (Disabled)" }}
                </span>
                <button type="button" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.ntfy_enabled = !c.ntfy_enabled; }) class="font-mono text-[10px] text-gray hover:text-black dark:hover:text-white underline cursor-pointer">
                    {move || if is_enabled() { "Disable" } else { "Enable" }}
                </button>
            </div>

            <div class="flex flex-col gap-2">
                <label class="font-mono text-[11px] text-gray uppercase">"Server URL"</label>
                <input
                    type="text"
                    prop:disabled=move || !is_enabled()
                    prop:value=move || cfg().ntfy_server
                    on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.ntfy_server = event_target_value(&ev); })
                    class=move || if is_enabled() {
                        "input-monolith text-xs font-mono"
                    } else {
                        "input-monolith text-xs font-mono opacity-40 cursor-not-allowed bg-gray/10"
                    }
                />
            </div>

            <div class="flex flex-col gap-2">
                <div class="flex items-center justify-between">
                    <label class="font-mono text-[11px] text-gray uppercase">"Personal Topic"</label>
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_regen_topic.run(()); }
                        class=move || if is_enabled() {
                            "font-mono text-[10px] text-gray hover:text-black dark:hover:text-white underline cursor-pointer"
                        } else {
                            "font-mono text-[10px] text-gray/40 no-underline cursor-not-allowed"
                        }
                    >
                        "Regenerate"
                    </button>
                </div>
                <input
                    type="text"
                    prop:disabled=move || !is_enabled()
                    prop:value=move || cfg().ntfy_topic
                    on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.ntfy_topic = event_target_value(&ev); })
                    class=move || if is_enabled() {
                        "input-monolith text-xs font-mono"
                    } else {
                        "input-monolith text-xs font-mono opacity-40 cursor-not-allowed bg-gray/10"
                    }
                />
            </div>

            <div class="flex flex-col gap-2 pt-1 border-t border-black dark:border-white">
                <span class="font-mono text-[11px] text-gray uppercase">"Event Triggers"</span>
                <p class="font-mono text-[10px] text-gray leading-tight">"Alerts escalate in frequency and urgency (lowest 3x @ 2s cooldown to highest 15x @ 1s cooldown)."</p>
                {move || {
                    let c = cfg();
                    let enabled = is_enabled();
                    view! {
                        <div class=if enabled { "flex flex-col gap-2" } else { "flex flex-col gap-2 opacity-40 pointer-events-none" }>
                            <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_start = !x.notify_start; }); }>
                                <SquareCheck checked=c.notify_start on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_start = !x.notify_start; }); }) aria_label="Notify on task start".to_string() />
                                <span class="text-xs text-black dark:text-white">"Task Start (Lowest: 3x, 2s cooldown)"</span>
                            </div>
                            <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_1h = !x.notify_1h; }); }>
                                <SquareCheck checked=c.notify_1h on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_1h = !x.notify_1h; }); }) aria_label="Notify 1 hour prior to deadline".to_string() />
                                <span class="text-xs text-black dark:text-white">"1h Before Deadline (Medium: 7x, 1.6s cooldown)"</span>
                            </div>
                            <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_25m = !x.notify_25m; }); }>
                                <SquareCheck checked=c.notify_25m on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_25m = !x.notify_25m; }); }) aria_label="Notify 25 minutes prior to deadline".to_string() />
                                <span class="text-xs text-black dark:text-white">"25m Before Deadline (High: 11x, 1.3s cooldown)"</span>
                            </div>
                            <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_crossed = !x.notify_crossed; }); }>
                                <SquareCheck checked=c.notify_crossed on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_crossed = !x.notify_crossed; }); }) aria_label="Notify when deadline crossed".to_string() />
                                <span class="text-xs text-black dark:text-white">"Deadline Crossed (Highest: 15x, 1s cooldown)"</span>
                            </div>
                        </div>
                    }
                }}
            </div>

            <div class="flex flex-col gap-2 pt-1 border-t border-black dark:border-white">
                <label class="font-mono text-[11px] text-gray uppercase">"Timezone (IANA)"</label>
                <input
                    type="text"
                    prop:disabled=move || !is_enabled()
                    prop:value=move || cfg().timezone
                    on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.timezone = event_target_value(&ev); })
                    class=move || if is_enabled() {
                        "input-monolith text-xs font-mono"
                    } else {
                        "input-monolith text-xs font-mono opacity-40 cursor-not-allowed bg-gray/10"
                    }
                />
            </div>

            <div class="pt-2">
                <button
                    type="button"
                    prop:disabled=move || !is_enabled()
                    on:click=move |_| if is_enabled() { on_test_ntfy.run(()); }
                    class=move || if is_enabled() {
                        "btn-monolith-ghost w-full text-xs"
                    } else {
                        "btn-monolith-ghost w-full text-xs opacity-40 cursor-not-allowed"
                    }
                >
                    "Send Test"
                </button>
            </div>
        </div>
    }
}
