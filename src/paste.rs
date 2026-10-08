//! Auto-paste through the XDG `RemoteDesktop` portal.
//!
//! The picker copies with `wl-copy` as usual, closes its window so focus goes
//! back to the previous app, then asks the portal to press the paste shortcut.
//! Pasting from the clipboard keeps ZWJ sequences, skin tones, and flags
//! intact, which typing keysyms one by one would not.

use std::fmt;
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use ashpd::desktop::remote_desktop::{DeviceType, KeyState, RemoteDesktop};
use ashpd::desktop::{PersistMode, Session};
use serde::{Deserialize, Serialize};

use crate::fl;

// evdev key codes (linux/input-event-codes.h). The compositor maps them
// through the active keymap, like a physical key press.
const KEY_LEFTCTRL: i32 = 29;
const KEY_LEFTSHIFT: i32 = 42;
const KEY_V: i32 = 47;

/// Gives up on a portal call that hangs, so the picker still exits.
const PORTAL_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum PasteShortcut {
    #[default]
    CtrlV,
    /// For terminals, which keep Ctrl+V for themselves.
    CtrlShiftV,
}

// Cached so the settings view doesn't allocate each frame. Same order as `ALL`.
static PASTE_SHORTCUT_LABELS: LazyLock<[String; 2]> = LazyLock::new(|| {
    [
        fl!("paste-shortcut-ctrl-v"),
        fl!("paste-shortcut-ctrl-shift-v"),
    ]
});

impl PasteShortcut {
    pub const ALL: [PasteShortcut; 2] = [PasteShortcut::CtrlV, PasteShortcut::CtrlShiftV];

    pub fn labels() -> &'static [String] {
        &*PASTE_SHORTCUT_LABELS
    }

    pub fn position(self) -> usize {
        Self::ALL.iter().position(|&s| s == self).unwrap_or(0)
    }

    fn keycodes(self) -> &'static [i32] {
        match self {
            PasteShortcut::CtrlV => &[KEY_LEFTCTRL, KEY_V],
            PasteShortcut::CtrlShiftV => &[KEY_LEFTCTRL, KEY_LEFTSHIFT, KEY_V],
        }
    }
}

/// Where auto-paste stands for this run of the picker.
#[derive(Debug, Default)]
pub(crate) enum PasteState {
    /// Off, unavailable, or denied: picking copies only.
    #[default]
    Idle,
    /// Waiting for the portal, which may be showing its permission prompt.
    Starting,
    Ready(PasteSession),
    /// The window is closed and the paste shortcut is on its way; the process
    /// exits once it is sent.
    Pending,
}

impl PasteState {
    pub(crate) fn is_pending(&self) -> bool {
        matches!(self, PasteState::Pending)
    }
}

/// A started `RemoteDesktop` session with keyboard access. Cheap to clone.
#[derive(Clone)]
pub(crate) struct PasteSession(Arc<Inner>);

struct Inner {
    proxy: RemoteDesktop<'static>,
    session: Session<'static, RemoteDesktop<'static>>,
}

impl fmt::Debug for PasteSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PasteSession")
    }
}

impl PasteSession {
    /// Starts a keyboard-only session. With a valid `restore_token` the portal
    /// skips its permission prompt. Returns the session and the token to save
    /// for next time.
    pub(crate) async fn connect(
        restore_token: Option<String>,
    ) -> Result<(Self, Option<String>), String> {
        let proxy = RemoteDesktop::new()
            .await
            .map_err(|e| format!("RemoteDesktop portal unavailable: {e}"))?;
        let session = proxy
            .create_session()
            .await
            .map_err(|e| format!("failed to create portal session: {e}"))?;
        proxy
            .select_devices(
                &session,
                DeviceType::Keyboard.into(),
                restore_token.as_deref(),
                PersistMode::ExplicitlyRevoked,
            )
            .await
            .map_err(|e| format!("failed to select portal devices: {e}"))?;
        let selected = proxy
            .start(&session, None)
            .await
            .and_then(|request| request.response())
            .map_err(|e| format!("portal session not started: {e}"))?;

        if !selected.devices().contains(DeviceType::Keyboard) {
            return Err("portal did not grant keyboard access".into());
        }

        let token = selected.restore_token().map(str::to_owned);
        Ok((Self(Arc::new(Inner { proxy, session })), token))
    }

    /// Presses `shortcut`, then releases its keys in reverse order.
    pub(crate) async fn send_paste(&self, shortcut: PasteShortcut) -> Result<(), String> {
        tokio::time::timeout(PORTAL_TIMEOUT, self.press(shortcut.keycodes()))
            .await
            .map_err(|_| "portal timed out sending the paste shortcut".to_string())?
            .map_err(|e| format!("failed to send the paste shortcut: {e}"))
    }

    async fn press(&self, keycodes: &[i32]) -> ashpd::Result<()> {
        let Inner { proxy, session } = &*self.0;
        let mut result = Ok(());
        let mut pressed = 0;
        for &code in keycodes {
            result = proxy
                .notify_keyboard_keycode(session, code, KeyState::Pressed)
                .await;
            if result.is_err() {
                break;
            }
            pressed += 1;
        }
        // Release whatever went down, even after an error, so no modifier sticks.
        for &code in keycodes[..pressed].iter().rev() {
            let released = proxy
                .notify_keyboard_keycode(session, code, KeyState::Released)
                .await;
            result = result.and(released);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paste_shortcuts_end_with_v_after_ctrl() {
        assert_eq!(PasteShortcut::CtrlV.keycodes(), &[29, 47]);
        assert_eq!(PasteShortcut::CtrlShiftV.keycodes(), &[29, 42, 47]);
    }
}
