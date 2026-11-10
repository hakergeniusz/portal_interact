// The game closes any screen the player opened while they stand in a nether
// portal (`LocalPlayer.handlePortalTransitionEffect`): an inventory opened
// this tick is closed again in the next one, so it is never really there.
// While the mod asks and a container the player opened on purpose is up, the
// close stands down for that tick — the screen stays. The confusion effect
// freezes for exactly as long as the screen is open (the overlay stops
// growing, the trigger sound waits), and the teleport still happens when the
// server's own timer says so: standing in the portal with your inventory
// open still takes you through, and the dimension change closes the screen
// the game's own way.
// Copyright (c) 2026 hakergeniusz. MIT.
fn portal_interact_keep(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<bool> {
    use crate::game::portal_interact as portal_interact;
    let container_open = matches!(
        game.screen,
        crate::game::Screen::Inventory | crate::game::Screen::Creative
    );
    if game.native_window()
        && game.native_on(portal_interact::ID)
        && portal_interact::keeps_container(
            portal_interact::setting_on(|key| game.native_setting(portal_interact::ID, key)),
            container_open,
        )
    {
        // The player is browsing their inventory: leave it alone this tick.
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
