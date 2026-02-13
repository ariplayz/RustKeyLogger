# Rust KeyLogger

A Windows keyboard state monitoring application written in Rust.

## Features

This project implements a `KeyState` struct and `get_key_state()` function that captures the current state of all keyboard keys using Windows low-level APIs.

### KeyState Struct

The `KeyState` struct holds boolean values (`is_pressed`) for each key on the keyboard:

- **Alphanumeric keys**: a-z
- **Number keys**: 0-9 (named as `key_0` through `key_9`)
- **Function keys**: F1-F12
- **Modifier keys**: Shift, Ctrl, Alt, Win
- **Navigation keys**: Arrow keys, Home, End, Page Up, Page Down
- **Special keys**: Enter, Backspace, Tab, Escape, Delete, Insert, Space, Caps Lock, Num Lock, Scroll Lock

### get_key_state() Method

The `get_key_state()` function returns a `KeyState` struct containing the current pressed/released state of all keyboard keys. It uses the Windows `GetKeyState()` API from `windows-sys` crate.

## Implementation Details

### Files

- **`src/key_state.rs`**: Defines the `KeyState` struct with all keyboard key fields
- **`src/get_key_state.rs`**: Implements the `get_key_state()` function using Windows APIs
- **`src/main.rs`**: Main entry point that demonstrates usage

### Dependencies

- `windows-sys` (v0.52): Provides safe bindings to Windows APIs
  - Feature: `Win32_UI_Input_KeyboardAndMouse`

### Low-Level Windows API Usage

The implementation uses the Windows `GetKeyState()` function which:
- Takes a virtual key code as input
- Returns the state of the specified key
- High-order bit (0x8000) indicates if the key is currently pressed

## Building

This project uses the GNU toolchain for Windows to avoid Visual Studio dependencies:

```powershell
# Switch to GNU toolchain (if needed)
rustup default stable-x86_64-pc-windows-gnu

# Build the project
cargo build

# Run the project
cargo run
```

## Usage Example

```rust
use get_key_state::get_key_state;

fn main() {
    let current_state = get_key_state();
    
    // Check individual keys
    if current_state.a {
        println!("Key 'A' is pressed!");
    }
    
    if current_state.shift {
        println!("Shift is pressed!");
    }
    
    // Print full state (with Debug trait)
    println!("{:#?}", current_state);
}
```

## Notes

- This project requires Windows as it uses Windows-specific APIs
- The GNU toolchain is used to avoid requiring Visual Studio installation
- All key states are checked synchronously when `get_key_state()` is called
- The struct derives `Debug`, `Clone`, and `Copy` traits for convenience

