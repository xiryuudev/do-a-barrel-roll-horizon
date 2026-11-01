// Do a Barrel Roll: its settings page click handler.
fn barrel_roll_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::do_a_barrel_roll as m;
    if mod_id != m::ID {
        return;
    }
    let mut settings = m::settings(|key| {
        game.native_setting(m::ID, key)
    });
    settings.click(button);
    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
