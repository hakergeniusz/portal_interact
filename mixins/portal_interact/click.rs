// A click on the mod's settings page: flip Enabled and keep it in the
// setting file the gates re-read, so the change lands within the tick that
// follows the click.
// Copyright (c) 2026 hakergeniusz. MIT.
fn portal_interact_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::portal_interact as portal_interact;
    if mod_id != portal_interact::ID {
        return;
    }
    let clicked = {
        let game = &*game;
        portal_interact::click(button, portal_interact::setting_on(|key| {
            game.native_setting(portal_interact::ID, key)
        }))
    };
    if let Some(on) = clicked {
        let (key, value) = portal_interact::stored(on);
        game.native_set_setting(portal_interact::ID, key, &value);
    }
}
