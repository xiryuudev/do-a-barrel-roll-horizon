// Do a Barrel Roll: its settings page buttons.
fn barrel_roll_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::do_a_barrel_roll as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    buttons.extend(settings.labels());
}
