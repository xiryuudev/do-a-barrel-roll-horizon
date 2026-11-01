// Do a Barrel Roll: mouse wheel hook — claimed when the mod is active.
fn barrel_roll_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::do_a_barrel_roll as m;
    if game.native_on(m::ID) && m::wheel(steps) {
        *claimed = true;
    }
}
