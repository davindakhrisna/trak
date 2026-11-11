use crate::components::SquareCheck;
use crate::domain::UserSettings;
use leptos::prelude::*;

const SERVICES: &[(&str, &str)] = &[
    ("instagram", "Instagram"),
    ("tiktok", "TikTok"),
    ("twitter", "Twitter / X"),
    ("reddit", "Reddit"),
    ("youtube", "YouTube"),
];

#[component]
pub fn SettingsAdguardCard(
    settings: ReadSignal<Option<UserSettings>>,
    set_settings: WriteSignal<Option<UserSettings>>,
    set_feedback: WriteSignal<Option<String>>,
) -> impl IntoView {
    let cfg = move || settings.get().unwrap_or_default();

    let is_service_enabled = move |id: &str| {
        cfg()
            .adguard_services
            .split(',')
            .map(|s| s.trim())
            .any(|s| s.eq_ignore_ascii_case(id))
    };

    let toggle_service = move |id: &'static str| {
        set_settings.update(|s| {
            if let Some(ref mut c) = s {
                let mut list: Vec<String> = c
                    .adguard_services
                    .split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect();
                if let Some(pos) = list.iter().position(|x| x.eq_ignore_ascii_case(id)) {
                    list.remove(pos);
                } else {
                    list.push(id.to_string());
                }
                c.adguard_services = list.join(",");
            }
        });
    };

    let on_test = move |_| {
        let curr = cfg();
        set_feedback.set(Some("Testing AdGuard Home connection...".to_string()));
        leptos::task::spawn_local(async move {
            match crate::backend::api_settings::test_adguard(
                Some(curr.adguard_url),
                Some(curr.adguard_username),
                Some(curr.adguard_password),
            )
            .await
            {
                Ok(msg) => set_feedback.set(Some(msg)),
                Err(e) => set_feedback.set(Some(format!("AdGuard test error: {}", e))),
            }
        });
    };

    view! {
        <div class="w-full border border-black dark:border-white p-5 flex flex-col gap-3 bg-white dark:bg-black">
            <div class="pb-1 border-b border-black dark:border-white flex items-center justify-between">
                <span class="font-mono text-xs text-black dark:text-white uppercase tracking-wider font-bold">"AdGuard Home Enforcer"</span>
                <button type="button" on:click=move |_| set_settings.update(|s| if let Some(c) = s { c.adguard_enabled = !c.adguard_enabled; }) class="font-mono text-[10px] text-gray hover:text-black dark:hover:text-white underline cursor-pointer">
                    {move || if cfg().adguard_enabled { "Disable" } else { "Enable" }}
                </button>
            </div>

            <div class="flex flex-col gap-2">
                <label class="font-mono text-[11px] text-gray uppercase">"AdGuard URL"</label>
                <input type="text" prop:value=move || cfg().adguard_url on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.adguard_url = event_target_value(&ev); }) placeholder="http://192.168.1.100:80" class="input-monolith text-xs font-mono" />
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                <div class="flex flex-col gap-1">
                    <label class="font-mono text-[11px] text-gray uppercase">"Username"</label>
                    <input type="text" prop:value=move || cfg().adguard_username on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.adguard_username = event_target_value(&ev); }) class="input-monolith text-xs" />
                </div>
                <div class="flex flex-col gap-1">
                    <label class="font-mono text-[11px] text-gray uppercase">"Password"</label>
                    <input type="password" prop:value=move || cfg().adguard_password on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.adguard_password = event_target_value(&ev); }) class="input-monolith text-xs" />
                </div>
            </div>

            <div class="flex flex-col gap-1">
                <label class="font-mono text-[11px] text-gray uppercase">"Target Client IP / MAC (Optional)"</label>
                <input type="text" prop:value=move || cfg().adguard_client_id on:input=move |ev| set_settings.update(|s| if let Some(c) = s { c.adguard_client_id = event_target_value(&ev); }) placeholder="Leave blank for network-wide" class="input-monolith text-xs font-mono" />
            </div>

            <div class="flex flex-col gap-2 pt-1 border-t border-black dark:border-white">
                <span class="font-mono text-[11px] text-gray uppercase">"Distraction Services to Block"</span>
                <div class="grid grid-cols-2 gap-2">
                    {SERVICES.iter().map(|(id, label)| {
                        let sid = *id;
                        view! {
                            <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| toggle_service(sid)>
                                <SquareCheck checked=is_service_enabled(sid) on_toggle=Callback::new(move |_| toggle_service(sid)) aria_label=format!("Block {}", label) />
                                <span class="text-xs text-black dark:text-white">{*label}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div class="pt-2">
                <button type="button" on:click=on_test class="btn-monolith-ghost w-full text-xs">"Test AdGuard Connection"</button>
            </div>
        </div>
    }
}
