# Glyphie Smoke Test

Run these checks after changes that touch launch, clipboard, search, or keyboard handling.

## Native

1. Run `just validate`.
2. Build and run with `just run`.
3. Confirm the picker opens on **Recent** and the search field is focused.
4. Type `grinning face`, press `Enter`, then paste into another app and confirm `😀` was copied.
5. Start `glyphie` again while it is already open and confirm the existing picker is focused instead of opening a second window.
6. Enable multiselect, select two emojis, press `Enter`, then paste and confirm both emojis were copied in selection order.
7. Change skin tone and gender filter, close and reopen, and confirm the preferences persisted.

## Auto-paste

Needs COSMIC 1.7 or newer. To start over with the permission prompt, delete `~/.local/state/cosmic/com.lkbddh.Glyphie/v1/paste_restore_token`.

1. **Normal app.** Focus a text editor, launch Glyphie, and confirm COSMIC asks for keyboard control the first time. Allow it, pick `😀`, and confirm it is pasted into the editor. Launch again and confirm there is no second prompt.
2. **Multi-codepoint emoji.** Pick `👩🏽‍💻` (with a skin tone set), `🏳️‍🌈`, and `🇫🇷` in turn and confirm each pastes as one emoji.
3. **Terminal.** In Settings, set **Paste shortcut** to **Ctrl+Shift+V**. Focus COSMIC Terminal, pick an emoji, and confirm it is pasted at the prompt. Set it back to **Ctrl+V** afterwards.
4. **Permission denied.** Delete the restore token, launch Glyphie, and deny the prompt. Pick an emoji and confirm the picker closes, nothing is pasted, and the emoji is on the clipboard. The debug log shows `auto-paste unavailable, copying only`.
5. **Portal missing.** Run on a session without the RemoteDesktop portal (for example, `systemctl --user stop xdg-desktop-portal-cosmic` and restart `xdg-desktop-portal`, or another desktop). Confirm Glyphie still copies and closes.
6. **Turned off.** Turn off **Paste after copying** in Settings, pick an emoji, and confirm it is only copied.
7. **Re-trigger.** Pick an emoji and immediately launch Glyphie again. Confirm the paste still lands; the second launch is ignored while the paste is pending.

## Flatpak

1. Regenerate `cargo-sources.json` if dependencies changed.
2. Build and install with `flatpak-builder --user --install --force-clean build-dir com.lkbddh.Glyphie.yml`.
3. Run with `flatpak run com.lkbddh.Glyphie`.
4. Repeat the native clipboard, single-instance, multiselect, persistence, and auto-paste checks.
