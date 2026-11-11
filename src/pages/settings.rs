use crate::components::MonolithShell;
use crate::domain::UserSettings;
use crate::pages::settings_adguard::SettingsAdguardCard;
use crate::pages::settings_guide::SettingsGuideTabs;
use crate::pages::settings_lockdown::SettingsLockdownCard;
use crate::pages::settings_ntfy::SettingsNtfyCard;
use crate::state::use_app_store;
use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let store = use_app_store();
    let (settings, set_settings) = signal(Option::<UserSettings>::None);
    let (feedback, set_feedback) = signal(Option::<String>::None);
    let (origin, _set_origin) = signal("http://127.0.0.1:3000".to_string());

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(st) = crate::backend::api_settings::fetch_settings().await {
                set_settings.set(Some(st));
            }
        });
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            if let Ok(loc) = win.location().origin() {
                _set_origin.set(loc);
            }
        }
    });

    let on_save = Callback::new(move |_| {
        if let Some(mut curr) = settings.get() {
            curr.ntfy_server = curr.ntfy_server.trim().to_string();
            curr.ntfy_topic = curr.ntfy_topic.trim().to_string();
            curr.timezone = curr.timezone.trim().to_string();
            curr.adguard_url = curr.adguard_url.trim().to_string();
            leptos::task::spawn_local(async move {
                match crate::backend::api_settings::update_settings(curr.clone()).await {
                    Ok(saved) => {
                        set_settings.set(Some(saved));
                        set_feedback.set(Some("Settings saved successfully.".to_string()));
                    }
                    Err(e) => set_feedback.set(Some(e.to_string())),
                }
            });
        }
    });

    let on_test_ntfy = Callback::new(move |_| {
        set_feedback.set(Some("Sending test notification...".to_string()));
        leptos::task::spawn_local(async move {
            match crate::backend::api_settings::send_test_ntfy().await {
                Ok(()) => set_feedback.set(Some(
                    "Test notification sent! Check your device.".to_string(),
                )),
                Err(e) => set_feedback.set(Some(e.to_string())),
            }
        });
    });

    let on_regen_topic = Callback::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(new_t) = crate::backend::api_settings::regenerate_ntfy_topic().await {
                set_settings.update(|s| {
                    if let Some(ref mut c) = s {
                        c.ntfy_topic = new_t;
                    }
                });
                set_feedback.set(Some("Generated new personal topic.".to_string()));
            }
        });
    });

    let on_regen_secret = Callback::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(new_sec) = crate::backend::api_settings::regenerate_lockdown_secret().await {
                set_settings.update(|s| {
                    if let Some(ref mut c) = s {
                        c.lockdown_secret = new_sec;
                    }
                });
                set_feedback.set(Some("Generated new lockdown API secret.".to_string()));
            }
        });
    });

    let on_reset_lockdown = Callback::new(move |_| {
        leptos::task::spawn_local(async move {
            if crate::backend::api_settings::emergency_reset_lockdown()
                .await
                .is_ok()
            {
                set_settings.update(|s| {
                    if let Some(ref mut c) = s {
                        c.locked_until = None;
                    }
                });
                set_feedback.set(Some(
                    "Lockdown reset to CLEAR. Distraction apps unblocked.".to_string(),
                ));
            }
        });
    });

    let api_url = Signal::derive(move || {
        let sec = settings
            .get()
            .map(|s| s.lockdown_secret)
            .unwrap_or_default();
        format!("{}/api/lockdown/{}", origin.get(), sec)
    });

    let topic_url = Signal::derive(move || {
        if let Some(s) = settings.get() {
            format!("{}/{}", s.ntfy_server.trim_end_matches('/'), s.ntfy_topic)
        } else {
            String::new()
        }
    });

    view! {
        <MonolithShell active_path="/settings" wide=true>
            {move || {
                if store.auth_initialized.get() && store.current_user_id.get().is_none() {
                    view! {
                        <section class="w-full border border-black dark:border-white p-8 flex flex-col items-center justify-center gap-4 bg-white dark:bg-black">
                            <h1 class="text-2xl font-bold text-black dark:text-white">"Sign In Required"</h1>
                            <p class="text-sm text-gray">"Sign in to manage notifications, AdGuard Home, and lockdown."</p>
                            <a href="/login" class="btn-monolith-solid">"Go to Sign in"</a>
                        </section>
                    }.into_any()
                } else if settings.get().is_some() {
                    view! {
                        <section class="w-full flex flex-col items-center justify-center gap-6">
                            <div class="w-full flex flex-col items-center justify-center gap-1">
                                <h1 class="text-3xl font-bold tracking-tight text-black dark:text-white">"Settings"</h1>
                                <p class="text-xs text-gray">"Configure Ntfy alerts, AdGuard Home DNS enforcer, and mobile lockdown."</p>
                            </div>

                            <div class="w-full grid grid-cols-1 lg:grid-cols-3 gap-5 items-start text-left">
                                <SettingsNtfyCard settings=settings set_settings=set_settings on_save=on_save on_test_ntfy=on_test_ntfy on_regen_topic=on_regen_topic />
                                <SettingsAdguardCard settings=settings set_settings=set_settings set_feedback=set_feedback />
                                <SettingsLockdownCard settings=settings set_settings=set_settings api_url=api_url on_regen_secret=on_regen_secret on_reset_lockdown=on_reset_lockdown />
                            </div>

                            <div class="w-full flex justify-end">
                                <button type="button" on:click=move |_| on_save.run(()) class="btn-monolith-solid px-8">"Save All Settings"</button>
                            </div>

                            {move || feedback.get().map(|msg| view! {
                                <div class="w-full border border-black dark:border-white p-3 font-mono text-xs text-black dark:text-white bg-white dark:bg-black">
                                    {msg}
                                </div>
                            })}

                            <SettingsGuideTabs api_url=api_url topic_url=topic_url />
                        </section>
                    }.into_any()
                } else {
                    view! {
                        <div class="py-12 font-mono text-xs text-gray">"Loading settings..."</div>
                    }.into_any()
                }
            }}
        </MonolithShell>
    }
}
