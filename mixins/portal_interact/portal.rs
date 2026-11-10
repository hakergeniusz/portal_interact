// The game's own question — "is the player mid portal transition?" — asked
// wherever the transition should take the player's screens away: the
// inventory key (and a ridden mount's) does nothing while it answers yes.
// While the mod asks, the answer is no, so the key opens the inventory as
// it would anywhere else. Everything the game does with the real answer on
// the server's side is untouched: the teleport is the server thread's, and
// runs on regardless.
// Copyright (c) 2026 hakergeniusz. MIT.
fn portal_interact_transition(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<bool> {
    use crate::game::portal_interact as portal_interact;
    if game.native_window()
        && game.native_on(portal_interact::ID)
        && portal_interact::setting_on(|key| game.native_setting(portal_interact::ID, key))
    {
        // Not mid transition, as far as the player's screens are concerned.
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
