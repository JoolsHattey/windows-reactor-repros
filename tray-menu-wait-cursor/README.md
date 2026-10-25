# windows-reactor: tray menu shows the busy cursor on every right-click

## Run

From the repository root:

```
cargo run -p repro-tray-menu-wait-cursor
```

Pinned to `microsoft/windows-rs` master `0496ec1` (2026-10-24) in the workspace `Cargo.toml`. No other dependencies. It's the same as `crates/samples/reactor/notifyicon-basic`, with a few more menu items.

## Steps

1. Right-click the tray icon a few times and watch the pointer.

**Expected:** the arrow pointer stays and the menu responds immediately.
**Actual:** the busy (wait) cursor shows for roughly 15-200 ms before the menu responds.

## Cause

Two windows end up under the pointer:

- the transient menu host, a 2x2 px window placed at the click point;
- the flyout's popup window, which is larger than the visible menu by its shadow margin.

Until the popup's input site is ready, the pointer is over a window that isn't processing input.

Making the host `WS_EX_TRANSPARENT` and moving the anchor by the shadow margin (16 DIPs at the monitor's DPI) fixes it.

Not yet confirmed on master `0496ec1` (it was seen on 0.100.0 before #5055 changed where the menu is anchored).

Environment: Windows 11 (build 26340), Windows App SDK 2.x, Rust 1.97.1, 150% display scale.
