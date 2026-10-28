# windows-reactor repros

Small, self-contained repros for `windows-reactor` issues. Each folder is one crate, with a README giving the steps, the expected and actual result, and what's known about the cause.

All of them depend only on `windows-reactor`, pinned to `microsoft/windows-rs` master `0496ec1` (2026-10-24) in the workspace `Cargo.toml`. Run one from the repository root:

```
cargo run -p repro-<folder name>
```

| Folder | Issue | Checked on master |
|---|---|---|
| [`listview-reorder-selected-index`](listview-reorder-selected-index) | ListView aborts with `E_INVALIDARG` when the clicked row moves to the top | Reproduces (4/4 runs) |
| [`titlebar-right-padding-scale`](titlebar-right-padding-scale) | TitleBar leaves a gap before the caption buttons above 100% scaling | Reproduces |
| [`titlebar-height`](titlebar-height) | No way to make a TitleBar taller than `Tall`, or stretch its content | Reproduces |
| [`pointer-side-buttons-handled`](pointer-side-buttons-handled) | Mouse Back/Forward buttons, and presses a child already handled | Reproduces |
| [`tray-menu-wait-cursor`](tray-menu-wait-cursor) | Tray menu shows the busy cursor on every right-click | Reproduces |

Tested on Windows 11 with the Windows App SDK 2 runtime, Rust 1.97.1, at 150% display scale.

WinUI bugs that Reactor works around get a plain C# WinUI repro instead, outside the Cargo workspace. Each folder's README says how to run it.

| Folder | WinUI issue | Checked on |
|---|---|---|
| [`winui-minimised-first-layout`](winui-minimised-first-layout) | A `TitleBar` set as the window's title bar fails fast when the window's first layout happens while it's minimised | Windows App SDK 2.5.1 and 2.6.4: reproduces on every run |
