// The mod's settings page: the single Enabled row, in the state the file
// holds (read fresh from disk, so the page always shows what the gates act
// on).
// Copyright (c) 2026 hakergeniusz. MIT.
fn portal_interact_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::portal_interact as portal_interact;
    if mod_id != portal_interact::ID {
        return;
    }
    let game = &*game;
    let on = portal_interact::setting_on(|key| game.native_setting(portal_interact::ID, key));
    buttons.push(portal_interact::label(on));
}
