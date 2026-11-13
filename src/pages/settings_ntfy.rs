use crate::components::SquareCheck;
use crate::domain::UserSettings;
use leptos::prelude::*;

#[component]
pub fn SettingsNtfyCard(
    settings: ReadSignal<Option<UserSettings>>,
    set_settings: WriteSignal<Option<UserSettings>>,
    on_test_ntfy: Callback<Option<u8>>,
    on_regen_topic: Callback<()>,
) -> impl IntoView {
    let cfg = move || settings.get().unwrap_or_default();
    let is_enabled = move || cfg().ntfy_enabled;

    view! {
        <div class="w-full h-full border border-rule p-5 flex flex-col gap-3 bg-surface">
            <div class="pb-2 border-b border-rule flex items-center justify-between gap-2">
                <span class=move || if is_enabled() {
                    "font-mono text-xs text-ink uppercase tracking-wider font-bold"
                } else {
                    "font-mono text-xs text-ink-muted uppercase tracking-wider font-bold"
                }>
                    {move || if is_enabled() { "Ntfy Alerts (Active)" } else { "Ntfy Alerts (Disabled)" }}
                </span>
                <button type="button" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.ntfy_enabled = !c.ntfy_enabled; }) class="font-mono text-[11px] text-ink-muted hover:text-ink underline cursor-pointer">
                    {move || if is_enabled() { "Disable" } else { "Enable" }}
                </button>
            </div>

            <div class="flex flex-col gap-1.5">
                <label class="font-mono text-[11px] text-ink-muted uppercase">"Server URL"</label>
                <input
                    type="text"
                    readonly
                    prop:readonly=true
                    prop:disabled=move || !is_enabled()
                    prop:value=move || cfg().ntfy_server
                    class=move || if is_enabled() {
                        "input-monolith text-xs font-mono select-all cursor-default bg-muted"
                    } else {
                        "input-monolith text-xs font-mono opacity-60 cursor-not-allowed bg-muted text-ink-muted select-none"
                    }
                />
            </div>

            <div class="flex flex-col gap-1.5">
                <div class="flex items-center justify-between">
                    <label class="font-mono text-[11px] text-ink-muted uppercase">"Personal Topic"</label>
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_regen_topic.run(()); }
                        class=move || if is_enabled() {
                            "font-mono text-[11px] text-ink-muted hover:text-ink underline cursor-pointer"
                        } else {
                            "font-mono text-[11px] text-ink-muted opacity-50 no-underline cursor-not-allowed"
                        }
                    >
                        "Regenerate"
                    </button>
                </div>
                <input
                    type="text"
                    readonly
                    prop:readonly=true
                    prop:disabled=move || !is_enabled()
                    prop:value=move || cfg().ntfy_topic
                    class=move || if is_enabled() {
                        "input-monolith text-xs font-mono select-all cursor-default bg-muted"
                    } else {
                        "input-monolith text-xs font-mono opacity-60 cursor-not-allowed bg-muted text-ink-muted select-none"
                    }
                />
            </div>

            <div class="flex flex-col gap-2 pt-2 border-t border-rule">
                <span class="font-mono text-[11px] text-ink-muted uppercase">"Event Triggers"</span>
                <p class="font-mono text-[11px] text-ink-muted leading-snug">"Alerts escalate in frequency and urgency (lowest 3x @ 2s cooldown to highest 15x @ 1s cooldown)."</p>
                {move || {
                    let c = cfg();
                    let enabled = is_enabled();
                    view! {
                        <div class=if enabled { "flex flex-col gap-2" } else { "flex flex-col gap-2 opacity-50 pointer-events-none" }>
                            <div class="flex items-center gap-2.5 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_start = !x.notify_start; }); }>
                                <SquareCheck checked=c.notify_start on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_start = !x.notify_start; }); }) aria_label="Notify on task start".to_string() />
                                <span class="text-xs text-ink">"Task Start (Lowest: 3x, 2s cooldown)"</span>
                            </div>
                            <div class="flex items-center gap-2.5 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_1h = !x.notify_1h; }); }>
                                <SquareCheck checked=c.notify_1h on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_1h = !x.notify_1h; }); }) aria_label="Notify 1 hour prior to deadline".to_string() />
                                <span class="text-xs text-ink">"1h Before Deadline (Medium: 7x, 1.6s cooldown)"</span>
                            </div>
                            <div class="flex items-center gap-2.5 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_25m = !x.notify_25m; }); }>
                                <SquareCheck checked=c.notify_25m on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_25m = !x.notify_25m; }); }) aria_label="Notify 25 minutes prior to deadline".to_string() />
                                <span class="text-xs text-ink">"25m Before Deadline (High: 11x, 1.3s cooldown)"</span>
                            </div>
                            <div class="flex items-center gap-2.5 cursor-pointer" on:click=move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_crossed = !x.notify_crossed; }); }>
                                <SquareCheck checked=c.notify_crossed on_toggle=Callback::new(move |_| if enabled { set_settings.update(|s| if let Some(x) = s { x.notify_crossed = !x.notify_crossed; }); }) aria_label="Notify when deadline crossed".to_string() />
                                <span class="text-xs text-ink">"Deadline Crossed (Highest: 15x, 1s cooldown)"</span>
                            </div>
                        </div>
                    }
                }}
            </div>

            <div class="flex flex-col gap-1.5 pt-2 border-t border-rule">
                <label class="font-mono text-[11px] text-ink-muted uppercase">"Timezone (IANA)"</label>
                <input
                    type="text"
                    prop:disabled=move || !is_enabled()
                    prop:value=move || cfg().timezone
                    on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.timezone = event_target_value(&ev); })
                    class=move || if is_enabled() {
                        "input-monolith text-xs font-mono"
                    } else {
                        "input-monolith text-xs font-mono opacity-60 cursor-not-allowed bg-muted text-ink-muted"
                    }
                />
            </div>

            <div class="flex flex-col gap-2 pt-2 border-t border-rule">
                <div class="flex items-center justify-between">
                    <label class="font-mono text-[11px] text-ink-muted uppercase">"Test Priority Notifications"</label>
                    <span class="font-mono text-[11px] text-ink-muted">"Verify device alerts"</span>
                </div>
                <div class="grid grid-cols-2 gap-2">
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_test_ntfy.run(Some(2)); }
                        class=move || if is_enabled() {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums"
                        } else {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums opacity-50 cursor-not-allowed"
                        }
                    >
                        "Lowest (3x)"
                    </button>
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_test_ntfy.run(Some(3)); }
                        class=move || if is_enabled() {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums"
                        } else {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums opacity-50 cursor-not-allowed"
                        }
                    >
                        "Medium (7x)"
                    </button>
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_test_ntfy.run(Some(4)); }
                        class=move || if is_enabled() {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums"
                        } else {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums opacity-50 cursor-not-allowed"
                        }
                    >
                        "High (11x)"
                    </button>
                    <button
                        type="button"
                        prop:disabled=move || !is_enabled()
                        on:click=move |_| if is_enabled() { on_test_ntfy.run(Some(5)); }
                        class=move || if is_enabled() {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums"
                        } else {
                            "btn-monolith-ghost text-[11px] py-1.5 px-2 tabular-nums opacity-50 cursor-not-allowed"
                        }
                    >
                        "Highest (15x)"
                    </button>
                </div>
                <button
                    type="button"
                    prop:disabled=move || !is_enabled()
                    on:click=move |_| if is_enabled() { on_test_ntfy.run(None); }
                    class=move || if is_enabled() {
                        "btn-monolith-ghost w-full text-xs py-1.5"
                    } else {
                        "btn-monolith-ghost w-full text-xs py-1.5 opacity-50 cursor-not-allowed"
                    }
                >
                    "Send Test (All 4 Priority Examples)"
                </button>
            </div>
        </div>
    }
}
