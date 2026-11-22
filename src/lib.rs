//! Portal Interact, the native-route native mod.
//!
//! The brain is pure standard library — no game types — so it compiles and
//! tests wherever. While the player stands in a nether portal, the game
//! refuses the inventory key (the transition is about to move them, and the
//! screen would be destroyed by the dimension change anyway) and closes any
//! inventory opened meanwhile. This mod's toggle answers one question the
//! game's gates ask: may the player interact from inside the portal? The
//! gates read it through two mixins — the transition check becomes "no"
//! (so the inventory key works) and the forced close stands down while a
//! container is open (so it stays open). The teleport itself is the
//! server's, and runs on regardless: the mod only lets the player keep
//! their own screens until the dimension actually changes.
//!
//! Clean-room: an independent helper, written for Horizon; it copies
//! nothing from any mod.
//!
//! Copyright (c) 2026 hakergeniusz. MIT.

use mod_api::{Hooks, RegistrationContext, Registrar, SettingSpec};

/// The mod's registry id: the settings page's owner.
pub const ID: &str = "portal_interact";

/// Whether the mod is on: a missing setting means ON — the mod exists to
/// allow the interaction, so installing it is asking for it; only `false`
/// turns it off, trimmed like the game reads its settings.
pub fn setting_on(value: Option<&str>) -> bool {
    !matches!(value, Some(v) if v.trim() == "false")
}

/// Whether a container screen should survive the portal's forced close this
/// tick: the mod is on and the screen is one the player opened on purpose.
/// The game's own screens — the pause menu, the chat, a death screen — are
/// none of this mod's business.
pub fn keeps_container(on: bool, container_open: bool) -> bool {
    on && container_open
}

#[derive(Default)]
pub struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    /// The Enabled toggle on the mod's page.
    fn register(&self, registrar: &mut dyn Registrar, _: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enabled",
            "portal_interact.setting.enabled",
            true,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Installed and never touched: the setting does not exist yet, and the
    /// mod is on — installing it is asking for the interaction. Only an
    /// explicit `false` turns it off.
    #[test]
    fn installed_means_on_until_declined() {
        assert!(setting_on(None), "the default must be on");
        assert!(setting_on(Some(" true \n")));
        assert!(setting_on(Some("on")), "anything but false is on");
        assert!(!setting_on(Some("false")));
    }

    /// The forced close stands down exactly while a container the player
    /// opened is up, and never for the mod being off.
    #[test]
    fn the_container_survives_only_while_kept() {
        assert!(keeps_container(true, true), "an open container is kept");
        assert!(!keeps_container(true, false), "no container, nothing to keep");
        assert!(!keeps_container(false, true), "the toggle answers for it");
    }
}
