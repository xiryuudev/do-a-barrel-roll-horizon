// Do a Barrel Roll: camera roll/pitch/yaw hook.
// Dipanggil dari Game::native_camera (setelah native_frame, sebelum render) agar
// offset roll/pitch/yaw dari State diterapkan ke view kamera setiap frame.
fn do_a_barrel_roll_camera(game: &mut crate::game::Game<'_>) {
    use crate::game::do_a_barrel_roll as m;
    if !game.native_on(m::ID) {
        return;
    }
    m::apply_camera(game);
}
