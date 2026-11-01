// Do a Barrel Roll: mod key press/release hook.
fn barrel_roll_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::do_a_barrel_roll as m;
    if !game.native_on(m::ID) {
        return;
    }
    let bound = game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY);
    if key != bound {
        return;
    }
    let playing = game.native_playing();
    let game = &*game;
    m::key(playing, pressed, |name| game.native_setting(m::ID, name));
}
