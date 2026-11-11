use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GuidePlatform {
    Ios,
    Android,
    Adguard,
    Ntfy,
}

#[component]
pub fn SettingsGuideTabs(api_url: Signal<String>, topic_url: Signal<String>) -> impl IntoView {
    let (platform, set_platform) = signal(GuidePlatform::Ios);

    let tab_class = move |target: GuidePlatform| {
        if platform.get() == target {
            "px-2.5 py-1 font-mono text-xs bg-black text-white dark:bg-white dark:text-black border border-black dark:border-white font-bold transition-colors cursor-pointer select-none"
        } else {
            "px-2.5 py-1 font-mono text-xs bg-white text-black dark:bg-black dark:text-white border border-gray hover:border-black dark:hover:border-white transition-colors cursor-pointer select-none"
        }
    };

    view! {
        <div class="w-full border border-black dark:border-white p-5 flex flex-col gap-4 bg-white dark:bg-black text-left">
            <div class="flex items-center justify-between border-b border-black dark:border-white pb-2 flex-wrap gap-2">
                <span class="font-mono text-xs text-black dark:text-white uppercase tracking-wider font-bold">
                    "Setup & Automation Guides"
                </span>
                <div class="flex items-center gap-1 flex-wrap">
                    <button type="button" on:click=move |_| set_platform.set(GuidePlatform::Ios) class=move || tab_class(GuidePlatform::Ios)>"iOS (Shortcuts)"</button>
                    <button type="button" on:click=move |_| set_platform.set(GuidePlatform::Android) class=move || tab_class(GuidePlatform::Android)>"Android (MacroDroid)"</button>
                    <button type="button" on:click=move |_| set_platform.set(GuidePlatform::Adguard) class=move || tab_class(GuidePlatform::Adguard)>"AdGuard Home"</button>
                    <button type="button" on:click=move |_| set_platform.set(GuidePlatform::Ntfy) class=move || tab_class(GuidePlatform::Ntfy)>"Ntfy Pairing"</button>
                </div>
            </div>

            {move || match platform.get() {
                GuidePlatform::Ios => view! {
                    <div class="flex flex-col gap-2 font-sans text-xs text-black dark:text-white leading-relaxed">
                        <p class="font-mono text-[11px] text-gray uppercase">"Enforce lockdown via native iOS Shortcuts automation:"</p>
                        <ol class="list-decimal pl-5 flex flex-col gap-1 text-black dark:text-white">
                            <li>"Open iOS " <strong class="font-mono">"Shortcuts"</strong> " app -> tap " <strong class="font-mono">"Automation"</strong> " tab -> " <strong class="font-mono">"New Automation"</strong> "."</li>
                            <li>"Select " <strong class="font-mono">"App"</strong> " -> Choose distraction apps (Instagram, TikTok, X) -> check " <strong class="font-mono">"Is Opened"</strong> " & " <strong class="font-mono">"Run Immediately"</strong> "."</li>
                            <li>"Add Action: " <strong class="font-mono">"Get Contents of URL"</strong> " -> Paste your private API URL:"
                                <div class="my-1.5 p-2 bg-white dark:bg-black border border-black dark:border-white font-mono text-[11px] break-all select-all">{move || api_url.get()}</div>
                            </li>
                            <li>"Add Action: " <strong class="font-mono">"Get Value for Key"</strong> " -> key: " <code class="font-mono bg-gray/20 px-1">"locked"</code> "."</li>
                            <li>"Add Action: " <strong class="font-mono">"If"</strong> " value is true -> Add " <strong class="font-mono">"Go to Home Screen"</strong> " action."</li>
                        </ol>
                    </div>
                }.into_any(),
                GuidePlatform::Android => view! {
                    <div class="flex flex-col gap-2 font-sans text-xs text-black dark:text-white leading-relaxed">
                        <p class="font-mono text-[11px] text-gray uppercase">"Enforce lockdown on Android using MacroDroid or Tasker:"</p>
                        <ol class="list-decimal pl-5 flex flex-col gap-1 text-black dark:text-white">
                            <li>"Install " <strong class="font-mono">"MacroDroid"</strong> " from Google Play."</li>
                            <li>"Trigger: " <strong class="font-mono">"Application Launched"</strong> " -> Select distraction apps."</li>
                            <li>"Action: " <strong class="font-mono">"HTTP Request (GET)"</strong> " -> Set target URL to:"
                                <div class="my-1.5 p-2 bg-white dark:bg-black border border-black dark:border-white font-mono text-[11px] break-all select-all">{move || api_url.get()}</div>
                            </li>
                            <li>"Action: " <strong class="font-mono">"If HTTP Response"</strong> " contains " <code class="font-mono bg-gray/20 px-1">"\"locked\":true"</code> ":"</li>
                            <li>"Action: " <strong class="font-mono">"Press Home Button"</strong> " + Display Toast: " <code class="font-mono bg-gray/20 px-1">"TRAK LOCKDOWN ACTIVE"</code> "."</li>
                        </ol>
                    </div>
                }.into_any(),
                GuidePlatform::Adguard => view! {
                    <div class="flex flex-col gap-2 font-sans text-xs text-black dark:text-white leading-relaxed">
                        <p class="font-mono text-[11px] text-gray uppercase">"Automated DNS-level lockdown via AdGuard Home:"</p>
                        <ol class="list-decimal pl-5 flex flex-col gap-1 text-black dark:text-white">
                            <li>"Connect your mobile devices to AdGuard Home via Encrypted DNS (DoH/DoT) profile or router DNS."</li>
                            <li>"Enter your AdGuard URL (e.g. " <code class="font-mono bg-gray/20 px-1">"http://192.168.1.100:80"</code> ") and admin credentials above."</li>
                            <li>"Select distraction services to block (Instagram, TikTok, Twitter/X, Reddit, YouTube)."</li>
                            <li>"Click " <strong class="font-mono">"Test AdGuard Connection"</strong> " to verify reachability and authentication."</li>
                            <li>"Save settings. When a deadline is crossed, AdGuard blocks selected services immediately."</li>
                            <li>"DNS access is restored automatically when punishment expires or via Emergency Reset."</li>
                        </ol>
                    </div>
                }.into_any(),
                GuidePlatform::Ntfy => view! {
                    <div class="flex flex-col gap-2 font-sans text-xs text-black dark:text-white leading-relaxed">
                        <p class="font-mono text-[11px] text-gray uppercase">"Subscribe to your personal Trak push channel:"</p>
                        <ol class="list-decimal pl-5 flex flex-col gap-1 text-black dark:text-white">
                            <li>"Install " <strong class="font-mono">"ntfy"</strong> " from App Store (iOS) or Google Play / F-Droid (Android)."</li>
                            <li>"Tap " <strong class="font-mono">"+ (Subscribe to topic)"</strong> "."</li>
                            <li>"Enter your personal subscription URL or scan in browser:"
                                <div class="my-1.5 p-2 bg-white dark:bg-black border border-black dark:border-white font-mono text-[11px] break-all select-all">{move || topic_url.get()}</div>
                            </li>
                            <li>"Grant notification permissions. On Android, exempt ntfy from battery optimization."</li>
                            <li>"Click " <strong class="font-mono">"Send Test"</strong> " in settings above to confirm delivery."</li>
                        </ol>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}
