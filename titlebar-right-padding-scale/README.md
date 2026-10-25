# windows-reactor: TitleBar leaves a gap before the caption buttons above 100% scaling

## Run

From the repository root:

```
cargo run -p repro-titlebar-right-padding-scale
```

Pinned to `microsoft/windows-rs` master `0496ec1` (2026-10-24) in the workspace `Cargo.toml`. No other dependencies.

## Steps

1. Run it on a display set to 150% scaling.
2. Look at the red `right_header` and the minimize button.

**Expected:** the right header ends where the caption buttons begin, as it does at 100%.
**Actual:** there's a gap between them, and it grows with the scale. Changing the scale while the app runs leaves the old width in place.

![At 150%](screenshot-150.png)

## Cause

WinUI's `TitleBar` sizes its `RightPaddingColumn` from `AppWindowTitleBar.RightInset`, which is in physical pixels, without dividing by `XamlRoot.RasterizationScale`. It only does so when the template is applied, so the width goes stale after a DPI change.

Environment: Windows 11 (build 26340), Windows App SDK 2.x, Rust 1.97.1, 150% display scale.
