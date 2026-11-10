// The mod's row in Options > Mods: one toggle, no keys.
// Copyright (c) 2026 hakergeniusz. MIT.
fn portal_interact_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::portal_interact as portal_interact;
    mods.push(crate::game::native_hooks::NativeMod {
        id: portal_interact::ID.into(),
        title: format!("Portal Interact {}", portal_interact::VERSION),
        description: portal_interact::DESCRIPTION.into(),
        keys: vec![],
    });
}
