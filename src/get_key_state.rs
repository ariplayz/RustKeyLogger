use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use crate::key_state::KeyState;

/// Helper function to check if a key is pressed
/// The high-order bit (0x8000) indicates if the key is currently pressed
fn is_key_pressed(vk_code: i32) -> bool {
    unsafe {
        let state = GetKeyState(vk_code);
        (state & 0x8000u16 as i16) != 0
    }
}

/// Get the current state of all keyboard keys
pub fn get_key_state() -> KeyState {
    let mut state = KeyState::new();

    // Alphanumeric keys (A-Z)
    state.a = is_key_pressed('A' as i32);
    state.b = is_key_pressed('B' as i32);
    state.c = is_key_pressed('C' as i32);
    state.d = is_key_pressed('D' as i32);
    state.e = is_key_pressed('E' as i32);
    state.f = is_key_pressed('F' as i32);
    state.g = is_key_pressed('G' as i32);
    state.h = is_key_pressed('H' as i32);
    state.i = is_key_pressed('I' as i32);
    state.j = is_key_pressed('J' as i32);
    state.k = is_key_pressed('K' as i32);
    state.l = is_key_pressed('L' as i32);
    state.m = is_key_pressed('M' as i32);
    state.n = is_key_pressed('N' as i32);
    state.o = is_key_pressed('O' as i32);
    state.p = is_key_pressed('P' as i32);
    state.q = is_key_pressed('Q' as i32);
    state.r = is_key_pressed('R' as i32);
    state.s = is_key_pressed('S' as i32);
    state.t = is_key_pressed('T' as i32);
    state.u = is_key_pressed('U' as i32);
    state.v = is_key_pressed('V' as i32);
    state.w = is_key_pressed('W' as i32);
    state.x = is_key_pressed('X' as i32);
    state.y = is_key_pressed('Y' as i32);
    state.z = is_key_pressed('Z' as i32);

    // Numbers (0-9)
    state.key_0 = is_key_pressed('0' as i32);
    state.key_1 = is_key_pressed('1' as i32);
    state.key_2 = is_key_pressed('2' as i32);
    state.key_3 = is_key_pressed('3' as i32);
    state.key_4 = is_key_pressed('4' as i32);
    state.key_5 = is_key_pressed('5' as i32);
    state.key_6 = is_key_pressed('6' as i32);
    state.key_7 = is_key_pressed('7' as i32);
    state.key_8 = is_key_pressed('8' as i32);
    state.key_9 = is_key_pressed('9' as i32);

    // Function keys (F1-F12)
    state.f1 = is_key_pressed(VK_F1 as i32);
    state.f2 = is_key_pressed(VK_F2 as i32);
    state.f3 = is_key_pressed(VK_F3 as i32);
    state.f4 = is_key_pressed(VK_F4 as i32);
    state.f5 = is_key_pressed(VK_F5 as i32);
    state.f6 = is_key_pressed(VK_F6 as i32);
    state.f7 = is_key_pressed(VK_F7 as i32);
    state.f8 = is_key_pressed(VK_F8 as i32);
    state.f9 = is_key_pressed(VK_F9 as i32);
    state.f10 = is_key_pressed(VK_F10 as i32);
    state.f11 = is_key_pressed(VK_F11 as i32);
    state.f12 = is_key_pressed(VK_F12 as i32);

    // Modifier keys
    state.shift = is_key_pressed(VK_SHIFT as i32);
    state.ctrl = is_key_pressed(VK_CONTROL as i32);
    state.alt = is_key_pressed(VK_MENU as i32);
    state.win = is_key_pressed(VK_LWIN as i32) || is_key_pressed(VK_RWIN as i32);

    // Navigation keys
    state.up = is_key_pressed(VK_UP as i32);
    state.down = is_key_pressed(VK_DOWN as i32);
    state.left = is_key_pressed(VK_LEFT as i32);
    state.right = is_key_pressed(VK_RIGHT as i32);
    state.home = is_key_pressed(VK_HOME as i32);
    state.end = is_key_pressed(VK_END as i32);
    state.page_up = is_key_pressed(VK_PRIOR as i32);
    state.page_down = is_key_pressed(VK_NEXT as i32);

    // Special keys
    state.enter = is_key_pressed(VK_RETURN as i32);
    state.backspace = is_key_pressed(VK_BACK as i32);
    state.tab = is_key_pressed(VK_TAB as i32);
    state.escape = is_key_pressed(VK_ESCAPE as i32);
    state.delete = is_key_pressed(VK_DELETE as i32);
    state.insert = is_key_pressed(VK_INSERT as i32);
    state.space = is_key_pressed(VK_SPACE as i32);
    state.caps_lock = is_key_pressed(VK_CAPITAL as i32);
    state.num_lock = is_key_pressed(VK_NUMLOCK as i32);
    state.scroll_lock = is_key_pressed(VK_SCROLL as i32);

    state
}