use leptos::ev::KeyboardEvent;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GuidePlatform {
    Ios,
    Android,
    Ntfy,
}

#[component]
pub fn SettingsGuideModal(
    open: ReadSignal<bool>,
    on_close: Callback<()>,
    api_url: Signal<String>,
    topic_url: Signal<String>,
    ntfy_enabled: Signal<bool>,
    lockdown_enabled: Signal<bool>,
) -> impl IntoView {
    let (platform, set_platform) = signal(GuidePlatform::Ios);

    let tab_class = move |target: GuidePlatform| {
        if platform.get() == target {
            "px-2.5 py-1 font-mono text-xs bg-ink text-canvas border border-rule-strong font-bold transition-colors duration-75 cursor-pointer select-none"
        } else {
            "px-2.5 py-1 font-mono text-xs bg-surface text-ink-muted hover:text-ink border border-rule hover:border-rule-strong transition-colors duration-75 cursor-pointer select-none"
        }
    };

    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close.run(());
        }
    };

    view! {
        <div
            class=move || if open.get() {
                "fixed inset-0 z-50 flex items-center justify-center p-4 modal-backdrop"
            } else {
                "hidden"
            }
            aria-hidden=move || if open.get() { "false" } else { "true" }
            on:click=move |_| on_close.run(())
            on:keydown=on_keydown
            tabindex="-1"
        >
            <div
                role="dialog"
                aria-modal="true"
                aria-labelledby="guides-modal-title"
                class="w-full max-w-[640px] max-h-[85vh] overflow-y-auto scroll-monolith border border-rule-strong p-5 sm:p-6 flex flex-col gap-4 bg-surface text-left"
                on:click=move |ev| ev.stop_propagation()
            >
                <div class="flex items-center justify-between border-b border-rule pb-3 gap-3">
                    <span
                        id="guides-modal-title"
                        class="font-mono text-xs text-ink uppercase tracking-wider font-bold"
                    >
                        "Setup & Automation Guides"
                    </span>
                    <button
                        type="button"
                        aria-label="Close guides"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            on_close.run(());
                        }
                        class="w-7 h-7 flex items-center justify-center font-mono text-sm text-ink-muted border border-rule hover:border-rule-strong hover:text-ink transition-colors duration-75 cursor-pointer shrink-0"
                    >
                        "×"
                    </button>
                </div>

                    <div class="flex items-center gap-1.5 flex-wrap">
                        <button type="button" on:click=move |_| set_platform.set(GuidePlatform::Ios) class=move || tab_class(GuidePlatform::Ios)>
                            {move || if lockdown_enabled.get() { "iOS (Shortcuts)" } else { "iOS (Shortcuts) [Off]" }}
                        </button>
                        <button type="button" on:click=move |_| set_platform.set(GuidePlatform::Android) class=move || tab_class(GuidePlatform::Android)>
                            {move || if lockdown_enabled.get() { "Android (MacroDroid)" } else { "Android (MacroDroid) [Off]" }}
                        </button>
                        <button type="button" on:click=move |_| set_platform.set(GuidePlatform::Ntfy) class=move || tab_class(GuidePlatform::Ntfy)>
                            {move || if ntfy_enabled.get() { "Ntfy Pairing" } else { "Ntfy Pairing [Off]" }}
                        </button>
                    </div>

                    {move || match platform.get() {
                        GuidePlatform::Ios => view! {
                            <div class="flex flex-col gap-2.5 font-sans text-xs text-ink leading-relaxed">
                                {move || (!lockdown_enabled.get()).then(|| view! {
                                    <div class="border border-rule p-2.5 bg-muted text-ink-muted font-mono text-[11px] flex items-center gap-2">
                                        <span class="inline-block w-1.5 h-1.5 bg-ink-muted shrink-0" aria-hidden="true"></span>
                                        <span>"Mobile App Lockdown is currently disabled. Enable it to enforce distraction blocking."</span>
                                    </div>
                                })}
                                <p class="font-mono text-[11px] text-ink-muted uppercase">"Enforce lockdown via native iOS Shortcuts automation:"</p>
                                <ol class="list-decimal pl-5 flex flex-col gap-1.5 text-ink">
                                    <li>"Open iOS " <strong class="font-mono">"Shortcuts"</strong> " app → tap " <strong class="font-mono">"Automation"</strong> " tab → " <strong class="font-mono">"New Automation"</strong> "."</li>
                                    <li>"Select " <strong class="font-mono">"App"</strong> " → Choose distraction apps (Instagram, TikTok, X) → check " <strong class="font-mono">"Is Opened"</strong> " & " <strong class="font-mono">"Run Immediately"</strong> "."</li>
                                    <li>"Add Action: " <strong class="font-mono">"Get Contents of URL"</strong> " → Paste your private API URL:"
                                        <div class=move || if lockdown_enabled.get() {
                                            "my-1.5 p-2 bg-muted border border-rule font-mono text-[11px] text-ink break-all select-all"
                                        } else {
                                            "my-1.5 p-2 bg-muted border border-rule font-mono text-[11px] text-ink-muted opacity-60 break-all select-none cursor-not-allowed"
                                        }>{move || api_url.get()}</div>
                                    </li>
                                    <li>"Add Action: " <strong class="font-mono">"Get Value for Key"</strong> " → key: " <code class="font-mono bg-muted border border-rule px-1.5 py-0.5 text-ink">"locked"</code> "."</li>
                                    <li>"Add Action: " <strong class="font-mono">"If"</strong> " value is true → Add " <strong class="font-mono">"Go to Home Screen"</strong> " action."</li>
                                </ol>
                            </div>
                        }.into_any(),
                        GuidePlatform::Android => view! {
                            <div class="flex flex-col gap-2.5 font-sans text-xs text-ink leading-relaxed">
                                {move || (!lockdown_enabled.get()).then(|| view! {
                                    <div class="border border-rule p-2.5 bg-muted text-ink-muted font-mono text-[11px] flex items-center gap-2">
                                        <span class="inline-block w-1.5 h-1.5 bg-ink-muted shrink-0" aria-hidden="true"></span>
                                        <span>"Mobile App Lockdown is currently disabled. Enable it to enforce distraction blocking."</span>
                                    </div>
                                })}
                                <p class="font-mono text-[11px] text-ink-muted uppercase">"Enforce lockdown on Android using MacroDroid (100% Free - 1 Macro Needed):"</p>
                                <ol class="list-decimal pl-5 flex flex-col gap-1.5 text-ink">
                                    <li>"Install " <strong class="font-mono">"MacroDroid"</strong> " from Google Play (free tier allows 5 macros; Trak needs only 1)."</li>
                                    <li>"Tap " <strong class="font-mono">"Add Macro"</strong> " → Trigger: " <strong class="font-mono">"Application Launched"</strong> " → Select distraction apps."</li>
                                    <li>"Action: " <strong class="font-mono">"HTTP Request (GET)"</strong> " → Set target URL to:"
                                        <div class=move || if lockdown_enabled.get() {
                                            "my-1.5 p-2 bg-muted border border-rule font-mono text-[11px] text-ink break-all select-all"
                                        } else {
                                            "my-1.5 p-2 bg-muted border border-rule font-mono text-[11px] text-ink-muted opacity-60 break-all select-none cursor-not-allowed"
                                        }>{move || api_url.get()}</div>
                                    </li>
                                    <li>"Action: " <strong class="font-mono">"If HTTP Response"</strong> " contains " <code class="font-mono bg-muted border border-rule px-1.5 py-0.5 text-ink">"\"locked\":true"</code> ":"</li>
                                    <li>"Action: " <strong class="font-mono">"Press Home Button"</strong> " (or Kill Process) + Toast: " <code class="font-mono bg-muted border border-rule px-1.5 py-0.5 text-ink">"TRAK LOCKDOWN ACTIVE"</code> "."</li>
                                </ol>
                            </div>
                        }.into_any(),
                        GuidePlatform::Ntfy => view! {
                            <div class="flex flex-col gap-2.5 font-sans text-xs text-ink leading-relaxed">
                                {move || (!ntfy_enabled.get()).then(|| view! {
                                    <div class="border border-rule p-2.5 bg-muted text-ink-muted font-mono text-[11px] flex items-center gap-2">
                                        <span class="inline-block w-1.5 h-1.5 bg-ink-muted shrink-0" aria-hidden="true"></span>
                                        <span>"Ntfy Alerts are currently disabled. Enable Ntfy Alerts to receive push notifications."</span>
                                    </div>
                                })}
                                <p class="font-mono text-[11px] text-ink-muted uppercase">"Subscribe to your personal Trak push channel:"</p>
                                <ol class="list-decimal pl-5 flex flex-col gap-1.5 text-ink">
                                    <li>"Install " <strong class="font-mono">"ntfy"</strong> " from App Store (iOS) or Google Play / F-Droid (Android)."</li>
                                    <li>"Tap " <strong class="font-mono">"+ (Subscribe to topic)"</strong> "."</li>
                                    <li>"Enter your personal subscription URL or scan in browser:"
                                        <div class=move || if ntfy_enabled.get() {
                                            "my-1.5 p-2 bg-muted border border-rule font-mono text-[11px] text-ink break-all select-all"
                                        } else {
                                            "my-1.5 p-2 bg-muted border border-rule font-mono text-[11px] text-ink-muted opacity-60 break-all select-none cursor-not-allowed"
                                        }>{move || topic_url.get()}</div>
                                    </li>
                                    <li>"Grant notification permissions. On Android, exempt ntfy from battery optimization."</li>
                                    <li>"Click " <strong class="font-mono">"Send Test"</strong> " in settings to confirm delivery."</li>
                                </ol>
                            </div>
                        }.into_any(),
                    }}
            </div>
        </div>
    }
}
