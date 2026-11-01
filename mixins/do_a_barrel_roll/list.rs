// Do a Barrel Roll: its row in Options > Mods, with its key.
fn barrel_roll_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::do_a_barrel_roll as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("Do a Barrel Roll {}", m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Mod key".into(), m::DEFAULT_KEY.into()]],
    });
}
