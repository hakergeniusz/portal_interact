// Portal Interact's brain: pure standard library, no game types, so it
// compiles and tests wherever. While the player stands in a nether portal,
// the game refuses the inventory key (the transition is about to move them,
// and the screen would be destroyed by the dimension change anyway) and
// closes any inventory opened meanwhile. This mod's toggle answers one
// question the game's gates ask: may the player interact from inside the
// portal? The gates read it through two helpers — the transition check
// becomes "no" (so the inventory key works) and the forced close stands
// down while a container is open (so it stays open). The teleport itself is
// the server's, and runs on regardless: the mod only lets the player keep
// their own screens until the dimension actually changes.
//
// Copyright (c) 2026 hakergeniusz. MIT — an independent helper, written for
// Horizon; it copies nothing from any mod.

/// The mod's registry id: the settings folder and the Options > Mods row.
pub const ID: &str = "portal_interact";
/// The version the row shows.
pub const VERSION: &str = "0.1.0";
/// The mod's description for its row.
pub const DESCRIPTION: &str = "Let you open your inventory (and a mount's) and keep it open while you \
stand in a nether portal. The teleport still happens — the game just stops \
slamming your screens shut.";

/// The toggle's setting file (one file per key under the mod's data dir),
/// which every gate re-reads: a change lands within the tick that follows it.
pub const SETTING: &str = "enabled";

/// Whether the toggle is on. A missing file means ON — the mod exists to
/// allow the interaction, so installing it is asking for it; anything but
/// `false` in the file is on, trimmed like the game reads its settings.
pub fn setting_on(get: impl FnOnce(&str) -> Option<String>) -> bool {
    !matches!(get(SETTING).as_deref(), Some(v) if v.trim() == "false")
}

/// Whether a container screen should survive the portal's forced close this
/// tick: the mod is on and the screen is one the player opened on purpose.
/// The game's own screens — the pause menu, the chat, a death screen — are
/// none of this mod's business.
pub fn keeps_container(on: bool, container_open: bool) -> bool {
    on && container_open
}

/// The settings page's single row.
pub fn label(on: bool) -> String {
    format!("Enabled: {}", if on { "ON" } else { "OFF" })
}

/// A click on the settings page's row `i`: the first row toggles the mod,
/// the rest are nothing (the page has the one row). Returns the new value
/// when the click was the toggle's, so the caller stores only real clicks.
pub fn click(i: usize, on: bool) -> Option<bool> {
    (i == 0).then_some(!on)
}

/// What to store for a value: the one setting key and its file text.
pub fn stored(on: bool) -> (&'static str, String) {
    (SETTING, if on { "true" } else { "false" }.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setting(value: &str) -> impl Fn(&str) -> Option<String> + '_ {
        move |key| (key == SETTING).then(|| value.to_owned())
    }

    /// Installed and never touched: the setting file does not exist yet, and
    /// the mod is on — installing it is asking for the interaction. Only an
    /// explicit `false` turns it off, and what the click writes reads back
    /// as that state.
    #[test]
    fn installed_means_on_until_declined() {
        assert!(setting_on(|_| None), "the default must be on");
        assert!(setting_on(setting(" true \n")));
        assert!(setting_on(setting("on")), "anything but false is on");
        assert!(!setting_on(setting("false")));
        for on in [true, false] {
            let (key, value) = stored(on);
            assert_eq!(key, SETTING);
            assert_eq!(setting_on(setting(&value)), on, "{value} reads back {on}");
        }
    }

    /// The forced close stands down exactly while a container the player
    /// opened is up, and never for the mod being off.
    #[test]
    fn the_container_survives_only_while_kept() {
        assert!(keeps_container(true, true), "an open container is kept");
        assert!(
            !keeps_container(true, false),
            "no container, nothing to keep"
        );
        assert!(!keeps_container(false, true), "the toggle answers for it");
    }

    /// The settings page shows the one row in the state the files hold, and
    /// a click on it flips the state; clicks elsewhere change nothing.
    #[test]
    fn the_first_click_toggles_and_the_rest_are_nothing() {
        let (on, shown) = (false, label(false));
        assert_eq!(shown, "Enabled: OFF");
        assert_eq!(label(true), "Enabled: ON");
        assert_eq!(click(0, on), Some(true));
        assert_eq!(click(1, on), None, "the page has the one row");
        assert_eq!(click(7, true), None);
    }
}
