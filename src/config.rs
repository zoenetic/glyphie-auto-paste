use std::any::TypeId;
use std::collections::VecDeque;

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};
use cosmic::iced::Subscription;
use serde::{Deserialize, Serialize};

use crate::emoji_data::{GenderFilter, SkinTone};
use crate::paste::PasteShortcut;
use crate::APP_ID;

pub(crate) const CONFIG_VERSION: u64 = 1;
pub(crate) const MAX_RECENT_EMOJIS: usize = 36;

// ── User Preferences ────────────────────────────────────────────

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, CosmicConfigEntry)]
#[version = 1]
pub struct GlyphieConfig {
    pub skin_tone: SkinTone,
    pub gender_filter: GenderFilter,
    pub show_subcategories: bool,
    /// Paste into the previous window after copying, through the `RemoteDesktop` portal.
    pub auto_paste: bool,
    pub paste_shortcut: PasteShortcut,
    /// How long to wait after closing for focus to return to the previous window.
    pub paste_delay_ms: u64,
}

impl Default for GlyphieConfig {
    fn default() -> Self {
        Self {
            skin_tone: SkinTone::Default,
            gender_filter: GenderFilter::default(),
            show_subcategories: true,
            auto_paste: true,
            paste_shortcut: PasteShortcut::default(),
            paste_delay_ms: 150,
        }
    }
}

impl GlyphieConfig {
    pub fn load() -> (Option<cosmic_config::Config>, Self) {
        match cosmic_config::Config::new(APP_ID, CONFIG_VERSION) {
            Ok(config_handler) => {
                let config = match Self::get_entry(&config_handler) {
                    Ok(ok) => ok,
                    Err((errs, config)) => {
                        log::info!("errors loading config: {errs:?}");
                        config
                    }
                };
                (Some(config_handler), config)
            }
            Err(err) => {
                log::error!("failed to create config handler: {err}");
                (None, Self::default())
            }
        }
    }

    pub fn subscription() -> Subscription<cosmic_config::Update<Self>> {
        struct ConfigSubscription;
        cosmic_config::config_subscription(
            TypeId::of::<ConfigSubscription>(),
            APP_ID.into(),
            CONFIG_VERSION,
        )
    }
}

// ── Runtime State ───────────────────────────────────────────────

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, CosmicConfigEntry)]
#[version = 1]
pub struct GlyphieState {
    pub recent_emojis: VecDeque<String>,
    /// `RemoteDesktop` portal token that skips the permission prompt next time.
    pub paste_restore_token: Option<String>,
}

impl GlyphieState {
    pub fn load() -> (Option<cosmic_config::Config>, Self) {
        match cosmic_config::Config::new_state(APP_ID, CONFIG_VERSION) {
            Ok(config_handler) => {
                let config = match Self::get_entry(&config_handler) {
                    Ok(ok) => ok,
                    Err((errs, config)) => {
                        log::info!("errors loading state: {errs:?}");
                        config
                    }
                };
                (Some(config_handler), config)
            }
            Err(err) => {
                log::error!("failed to create state handler: {err}");
                (None, Self::default())
            }
        }
    }

    pub fn subscription() -> Subscription<cosmic_config::Update<Self>> {
        struct StateSubscription;
        cosmic_config::config_state_subscription(
            TypeId::of::<StateSubscription>(),
            APP_ID.into(),
            CONFIG_VERSION,
        )
    }
}
