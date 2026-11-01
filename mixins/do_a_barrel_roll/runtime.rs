// Do a Barrel Roll for Horizon — full pitch, yaw, and roll control in elytra flight.
// Port of enjarai's Do a Barrel Roll (Fabric) to a Horizon native mixin bundle.
// Hooks native_frame, native_key, native_wheel, native_mods, native_settings,
// native_settings_click (API v1 hooks).
//
// Catatan: Horizon's native hooks API saat ini TIDAK mengekspos hook untuk
// memodifikasi view matrix / sudut kamera. Mod ini menyusun struktur lengkap
// (daftar di Options > Mods, halaman pengaturan, tombol kunci, frame hook)
// dan melacak offset roll/pitch/yaw, tetapi menerapkannya ke kamera memerlukan
// tambahan hook point di `horizon-client` (mis. native_camera atau memodifikasi
// view_matrix() di game.rs).
use std::cell::RefCell;

/// The mod's id: its settings folder and its key's line.
pub const ID: &str = "do_a_barrel_roll";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Full pitch, yaw, and roll control in elytra flight. Hold the mod key to take over camera orientation.";
/// The key's name, and its default binding.
pub const KEY: &str = "do_a_barrel_roll";
pub const DEFAULT_KEY: &str = "key.keyboard.left_control";

/// The mod's settings, stored per-key in `horizon-mods/.horizon-mod-data/do_a_barrel_roll/`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Roll sensitivity multiplier (mouse X / strafe -> roll).
    pub roll_sensitivity: f32,
    /// Pitch sensitivity multiplier (mouse Y -> pitch).
    pub pitch_sensitivity: f32,
    /// Yaw sensitivity multiplier (A/D -> yaw).
    pub yaw_sensitivity: f32,
    /// Invert roll direction.
    pub invert_roll: bool,
    /// Invert pitch direction.
    pub invert_pitch: bool,
    /// Smoothly ease camera orientation changes.
    pub smooth_camera: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            roll_sensitivity: 1.0,
            pitch_sensitivity: 1.0,
            yaw_sensitivity: 1.0,
            invert_roll: false,
            invert_pitch: false,
            smooth_camera: true,
        }
    }
}

impl Settings {
    /// From stored values; defaults for missing keys.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let f = |key: &str, def: f32| {
            get(key)
                .and_then(|v| v.trim().parse::<f32>().ok())
                .unwrap_or(def)
                .clamp(0.1, 5.0)
        };
        let flag = |key: &str| get(key).is_none_or(|v| v.trim() != "false");
        Self {
            roll_sensitivity: f("roll_sensitivity", 1.0),
            pitch_sensitivity: f("pitch_sensitivity", 1.0),
            yaw_sensitivity: f("yaw_sensitivity", 1.0),
            invert_roll: flag("invert_roll"),
            invert_pitch: flag("invert_pitch"),
            smooth_camera: flag("smooth_camera"),
        }
    }

    /// The settings page's buttons, as labels.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Roll sensitivity: {:.1}x", self.roll_sensitivity),
            format!("Pitch sensitivity: {:.1}x", self.pitch_sensitivity),
            format!("Yaw sensitivity: {:.1}x", self.yaw_sensitivity),
            format!("Invert roll: {}", on(self.invert_roll)),
            format!("Invert pitch: {}", on(self.invert_pitch)),
            format!("Smooth camera: {}", on(self.smooth_camera)),
        ]
    }

    /// A click on button `i` of `labels`.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.roll_sensitivity = (self.roll_sensitivity + 0.5).clamp(0.1, 5.0),
            1 => self.pitch_sensitivity = (self.pitch_sensitivity + 0.5).clamp(0.1, 5.0),
            2 => self.yaw_sensitivity = (self.yaw_sensitivity + 0.5).clamp(0.1, 5.0),
            3 => self.invert_roll = !self.invert_roll,
            4 => self.invert_pitch = !self.invert_pitch,
            5 => self.smooth_camera = !self.smooth_camera,
            _ => {}
        }
    }

    /// What to store, key by key.
    pub fn stored(&self) -> [(&'static str, String); 6] {
        [
            ("roll_sensitivity", self.roll_sensitivity.to_string()),
            ("pitch_sensitivity", self.pitch_sensitivity.to_string()),
            ("yaw_sensitivity", self.yaw_sensitivity.to_string()),
            ("invert_roll", self.invert_roll.to_string()),
            ("invert_pitch", self.invert_pitch.to_string()),
            ("smooth_camera", self.smooth_camera.to_string()),
        ]
    }
}

#[derive(Default)]
struct State {
    /// Read on first use, then kept.
    settings: Option<Settings>,
    /// Whether the mod is currently active (mod key held).
    enabled: bool,
    /// Accumulated roll offset (degrees).
    roll: f32,
    /// Accumulated pitch offset (degrees).
    pitch: f32,
    /// Accumulated yaw offset (degrees).
    yaw: f32,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (the settings page changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings));
}

/// The mod key went down or up (only while playing).
pub fn key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    if pressed && !playing {
        return;
    }
    STATE.with_borrow_mut(|s| {
        s.enabled = pressed;
        if !pressed {
            s.roll = 0.0;
            s.pitch = 0.0;
            s.yaw = 0.0;
        }
    });
}

/// Wheel steps: claimed when the mod is active so they don't scroll the hotbar.
pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow(|s| s.enabled && steps.abs() > 0.0)
}

/// Whether the mod is currently active.
pub fn is_enabled() -> bool {
    STATE.with_borrow(|s| s.enabled)
}

/// Called once per frame: computes roll/pitch/yaw offsets from input.
/// The offsets are tracked in `State`; applying them to the camera requires
/// a native camera hook that Horizon doesn't expose yet.
pub fn frame(game: &mut crate::game::Game<'_>, _elapsed: std::time::Duration) {
    if !game.native_on(ID) {
        return;
    }
    if !game.native_playing() {
        return;
    }
    let settings = settings(|key| game.native_setting(ID, key));
    let (roll, pitch, yaw) = STATE.with_borrow_mut(|s| {
        if !s.enabled {
            return (0.0, 0.0, 0.0);
        }
        let (a_down, _) = game.native_mapping("key.left").unwrap_or((false, String::new()));
        let (d_down, _) = game.native_mapping("key.right").unwrap_or((false, String::new()));
        let (space_down, _) = game.native_mapping("key.jump").unwrap_or((false, String::new()));
        let (shift_down, _) = game.native_mapping("key.sneak").unwrap_or((false, String::new()));
        let (attack_down, _) = game.native_mapping("key.attack").unwrap_or((false, String::new()));
        let (use_down, _) = game.native_mapping("key.use").unwrap_or((false, String::new()));

        let dir = |down: bool, inv: bool| {
            if !down {
                0.0
            } else if inv {
                -1.0
            } else {
                1.0
            }
        };

        let roll = dir(d_down, settings.invert_roll) - dir(a_down, settings.invert_roll);
        let pitch = dir(space_down, settings.invert_pitch) - dir(shift_down, settings.invert_pitch);
        let yaw = dir(attack_down, false) - dir(use_down, false);

        s.roll = roll * settings.roll_sensitivity;
        s.pitch = pitch * settings.pitch_sensitivity;
        s.yaw = yaw * settings.yaw_sensitivity;

        (s.roll, s.pitch, s.yaw)
    });
    // Hook point yang diperlukan untuk menerapkan roll/pitch/yaw ke kamera
    // belum tersedia di Horizon. Setelah tersedia, gunakan `game` di sini untuk
    // memodifikasi view matrix atau sudut kamera.
    let _ = (roll, pitch, yaw);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn reads_default_settings() {
        let s = Settings::read(stored(&[]));
        assert_eq!(s.roll_sensitivity, 1.0);
        assert_eq!(s.pitch_sensitivity, 1.0);
        assert_eq!(s.yaw_sensitivity, 1.0);
        assert!(!s.invert_roll);
        assert!(!s.invert_pitch);
        assert!(s.smooth_camera);
    }

    #[test]
    fn reads_stored_settings() {
        let s = Settings::read(stored(&[
            ("roll_sensitivity", "2.5\n"),
            ("pitch_sensitivity", "0.5\n"),
            ("invert_roll", "true\n"),
            ("smooth_camera", "false\n"),
        ]));
        assert_eq!(s.roll_sensitivity, 2.5);
        assert_eq!(s.pitch_sensitivity, 0.5);
        assert!(s.invert_roll);
        assert!(!s.smooth_camera);
    }

    #[test]
    fn buttons_cycle_sensitivities() {
        let mut s = Settings::default();
        s.click(0);
        assert_eq!(s.roll_sensitivity, 1.5);
        s.click(0);
        assert_eq!(s.roll_sensitivity, 2.0);
        s.click(3);
        assert!(s.invert_roll);
        s.click(3);
        assert!(!s.invert_roll);
    }

    #[test]
    fn stored_roundtrips() {
        let s = Settings {
            roll_sensitivity: 3.0,
            pitch_sensitivity: 0.5,
            yaw_sensitivity: 2.0,
            invert_roll: true,
            invert_pitch: false,
            smooth_camera: true,
        };
        let round: Vec<(&str, String)> = s.stored().into();
        let back: Vec<(&str, &str)> = round.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&back)), s);
    }
}
