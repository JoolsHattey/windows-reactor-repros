# windows-reactor: mouse Back/Forward buttons, and presses a child already handled

A `Border` around the whole page is the natural place to handle the mouse's Back and Forward buttons, but it can't tell which button was pressed, and it never hears presses over controls.

## Run

From the repository root:

```
cargo run -p repro-pointer-side-buttons-handled
```

Pinned to `microsoft/windows-rs` master `0496ec1` (2026-10-24) in the workspace `Cargo.toml`. No other dependencies. You need a mouse with Back/Forward (X1/X2) buttons.

## Steps

1. Press the mouse's Back or Forward button over empty space.
   **Actual:** "Presses seen" goes up, but left, right and middle are all `false`: `PointerEventInfo` has no `IsXButton1Pressed` / `IsXButton2Pressed`.
2. Press any button over a list row or in the text box.
   **Actual:** "Presses seen" doesn't change. `Border::on_pointer_pressed` subscribes to the plain `PointerPressed` event, which doesn't fire for presses a child control already handled. WinUI delivers those through `UIElement.AddHandler(PointerPressedEvent, handler, handledEventsToo: true)`.

Both reproduce on master `0496ec1`, as of 2026-10-25:

- After a Back (X1) press over empty space, "Presses seen" goes up, but the last press shows `left=false right=false middle=false`.
- After a left click on "Row two", the row is selected, but "Presses seen" doesn't change.

Environment: Windows 11, Windows App SDK 2.x, Rust 1.97.1, 150% display scale.
