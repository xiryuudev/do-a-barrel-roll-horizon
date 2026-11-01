# Do a Barrel Roll — Horizon Native Mod

Full pitch, yaw, and roll control in elytra flight for Horizon (Rust Minecraft rewrite).

Port of [enjarai's Do a Barrel Roll](https://modrinth.com/mod/do-a-barrel-roll) (Fabric) to a Horizon native mixin bundle.

## Controls (while the mod key is held)

| Action | Key |
|--------|-----|
| Roll right | D |
| Roll left | A |
| Pitch up | Space |
| Pitch down | Shift |
| Yaw right | Attack |
| Yaw left | Use |

Default mod key: `Left Control` (configurable in Options > Mods).

## Settings

- Roll / pitch / yaw sensitivity multipliers
- Invert roll / pitch
- Smooth camera easing

## Importing in Horizon Launcher

1. Open Horizon Launcher → Mods.
2. Click "Add a native mod from GitHub".
3. Enter: `https://github.com/xiryuudev/do-a-barrel-roll-horizon.git`
4. The launcher downloads, builds, and installs the native mod.
5. Enable it in Options > Mods → Do a Barrel Roll.

## Build requirements

- Horizon with native hook points (commit `942e3b2` / hooks v7).
- `--trust-native-code` and `--allow-version-mismatch` flags for native-build.
