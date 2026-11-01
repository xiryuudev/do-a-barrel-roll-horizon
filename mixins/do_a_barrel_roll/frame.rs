// Do a Barrel Roll: per-frame camera control hook.
fn barrel_roll_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::do_a_barrel_roll as m;
    if !game.native_on(m::ID) {
        return;
    }
    m::frame(game, elapsed);
    let _ = elapsed;
}
