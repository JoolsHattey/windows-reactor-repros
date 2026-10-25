# windows-reactor: ListView aborts with E_INVALIDARG when the clicked row moves to the top

A "recently played" list: the selection shows what's playing, and once a song has loaded it moves to the top. If the rows contain a `Button`, clicking a lower row aborts the process in the update that moves it.

## Run

From the repository root:

```
cargo run -p repro-listview-reorder-selected-index
```

Pinned to `microsoft/windows-rs` master `0496ec1` (2026-10-24) in the workspace `Cargo.toml`. No other dependencies.

## Steps

1. Click "Three" with the mouse (on the row, not its "..." button).

**Expected:** about 300 ms later "Three" moves to the top and is selected.
**Actual:** the process aborts:

```
thread 'main' panicked at crates\libs\reactor\src\native\winui\adapter.rs:260:41:
called `Result::unwrap()` on an `Err` value: Native(Error { code: HRESULT(0x80070057), message: "The parameter is incorrect." })
```

The panic comes from `WinUiAdapter::apply` → `apply_mutations(..).unwrap()`, in the render after the background "load" finishes (`ComponentHost::drain` → `apply_render`). [backtrace.txt](backtrace.txt) has the full symbolized backtrace.

## What it takes

It reproduced on 4 of 4 runs with a real mouse click.

- **A `Button` in each row's content.** With plain text rows (`DataItem::new(key, text)` and no content), the same steps don't crash.
- In this repro, the click's own update leaves the list alone: `selected_index` stays at what's playing, so the selection snaps back. The move and the new `selected_index` arrive together in a later update. With plain text rows, doing the move in the click's own update didn't crash either; I haven't tried that with button rows.
- With plain text rows, selecting through UI Automation (`SelectionItemPattern.Select`, no keyboard focus) or the Down arrow didn't crash. I haven't tried those with button rows.

Environment: Windows 11, Windows App SDK 2.x, Rust 1.97.1, 150% display scale.
