# windows-reactor: TitleBar controls added after the first layout don't get passthrough regions

A control added to a `TitleBar`'s content after its first layout can't be used: the title bar
still treats its area as a drag region, so dragging it drags the window.

## Run

From the repository root:

```
cargo run -p repro-titlebar-drag-regions
cargo run -p repro-titlebar-drag-regions -- --slider-first
```

Pinned to `microsoft/windows-rs` master `8786f59d1` (2026-10-29) in the workspace `Cargo.toml`. No other dependencies.

## Steps

1. Run it without arguments and click "Add slider". A slider appears in the title bar.
2. Drag the slider's thumb sideways.
   **Actual:** the window moves by the drag distance and the slider doesn't.
   **Expected:** the slider moves, as it does when it's there from the start (`--slider-first`).

WinUI's `TitleBar` makes its content a drag region apart from the enabled controls it finds in it.
With `AutoRefreshDragRegions = false` (the default) it only finds them on the content's first
layout, and Reactor doesn't set it or call `RecomputeDragRegions()` when it re-renders the content.

Reproduces on master `8786f59d1` with Windows App SDK 2.6.4 (2026-10-29): dragging the thumb of a
slider added later moved the window by the drag distance, twice; with `--slider-first` the same
drag left the window in place. A button that's in the title bar from the start but enabled or moved
later does still get its clicks.

Environment: Windows 11, Windows App SDK 2.6.4, Rust 1.97.1, 150% display scale.
