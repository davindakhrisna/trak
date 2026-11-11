use crate::components::SquareCheck;
use crate::domain::UserSettings;
use leptos::prelude::*;

#[component]
pub fn SettingsNtfyCard(
    settings: ReadSignal<Option<UserSettings>>,
    set_settings: WriteSignal<Option<UserSettings>>,
    on_save: Callback<()>,
    on_test_ntfy: Callback<()>,
    on_regen_topic: Callback<()>,
) -> impl IntoView {
    let cfg = move || settings.get().unwrap_or_default();

    view! {
        <div class="w-full border border-black dark:border-white p-5 flex flex-col gap-3 bg-white dark:bg-black">
            <div class="pb-1 border-b border-black dark:border-white flex items-center justify-between">
                <span class="font-mono text-xs text-black dark:text-white uppercase tracking-wider font-bold">"Ntfy Alerts"</span>
                <button type="button" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.ntfy_enabled = !c.ntfy_enabled; }) class="font-mono text-[10px] text-gray hover:text-black dark:hover:text-white underline cursor-pointer">
                    {move || if cfg().ntfy_enabled { "Disable" } else { "Enable" }}
                </button>
            </div>

            <div class="flex flex-col gap-2">
                <label class="font-mono text-[11px] text-gray uppercase">"Server URL"</label>
                <input type="text" prop:value=move || cfg().ntfy_server on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.ntfy_server = event_target_value(&ev); }) class="input-monolith text-xs font-mono" />
            </div>

            <div class="flex flex-col gap-2">
                <div class="flex items-center justify-between">
                    <label class="font-mono text-[11px] text-gray uppercase">"Personal Topic"</label>
                    <button type="button" on:click=move |_| on_regen_topic.run(()) class="font-mono text-[10px] text-gray hover:text-black dark:hover:text-white underline cursor-pointer">"Regenerate"</button>
                </div>
                <input type="text" prop:value=move || cfg().ntfy_topic on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.ntfy_topic = event_target_value(&ev); }) class="input-monolith text-xs font-mono" />
            </div>

            <div class="flex flex-col gap-2 pt-1 border-t border-black dark:border-white">
                <span class="font-mono text-[11px] text-gray uppercase">"Event Triggers"</span>
                <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.notify_start = !c.notify_start; })>
                    <SquareCheck checked=cfg().notify_start on_toggle=Callback::new(move |_| set_settings.update(|s| if let Some(c) = s { c.notify_start = !c.notify_start; })) aria_label="Notify on task start".to_string() />
                    <span class="text-xs text-black dark:text-white">"Task Start (Low priority)"</span>
                </div>
                <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.notify_1h = !c.notify_1h; })>
                    <SquareCheck checked=cfg().notify_1h on_toggle=Callback::new(move |_| set_settings.update(|s| if let Some(c) = s { c.notify_1h = !c.notify_1h; })) aria_label="Notify 1 hour prior to deadline".to_string() />
                    <span class="text-xs text-black dark:text-white">"1h Before Deadline (Medium priority)"</span>
                </div>
                <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.notify_25m = !c.notify_25m; })>
                    <SquareCheck checked=cfg().notify_25m on_toggle=Callback::new(move |_| set_settings.update(|s| if let Some(c) = s { c.notify_25m = !c.notify_25m; })) aria_label="Notify 25 minutes prior to deadline".to_string() />
                    <span class="text-xs text-black dark:text-white">"25m Before Deadline (High priority)"</span>
                </div>
                <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.notify_crossed = !c.notify_crossed; })>
                    <SquareCheck checked=cfg().notify_crossed on_toggle=Callback::new(move |_| set_settings.update(|s| if let Some(c) = s { c.notify_crossed = !c.notify_crossed; })) aria_label="Notify when deadline crossed".to_string() />
                    <span class="text-xs text-black dark:text-white">"Deadline Crossed (High priority)"</span>
                </div>
            </div>

            <div class="flex flex-col gap-2 pt-1 border-t border-black dark:border-white">
                <label class="font-mono text-[11px] text-gray uppercase">"Timezone (IANA)"</label>
                <input type="text" prop:value=move || cfg().timezone on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.timezone = event_target_value(&ev); }) class="input-monolith text-xs font-mono" />
            </div>

            <div class="flex items-center gap-2 pt-2">
                <button type="button" on:click=move |_| on_save.run(()) class="btn-monolith-solid flex-1">"Save Settings"</button>
                <button type="button" on:click=move |_| on_test_ntfy.run(()) class="btn-monolith-ghost text-xs">"Send Test"</button>
            </div>
        </div>
    }
}
