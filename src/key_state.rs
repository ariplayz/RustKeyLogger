#[derive(Debug, Clone, Copy)]
pub struct KeyState {
    // Alphanumeric keys (A-Z)
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
    pub g: bool,
    pub h: bool,
    pub i: bool,
    pub j: bool,
    pub k: bool,
    pub l: bool,
    pub m: bool,
    pub n: bool,
    pub o: bool,
    pub p: bool,
    pub q: bool,
    pub r: bool,
    pub s: bool,
    pub t: bool,
    pub u: bool,
    pub v: bool,
    pub w: bool,
    pub x: bool,
    pub y: bool,
    pub z: bool,

    // Numbers (0-9)
    pub key_0: bool,
    pub key_1: bool,
    pub key_2: bool,
    pub key_3: bool,
    pub key_4: bool,
    pub key_5: bool,
    pub key_6: bool,
    pub key_7: bool,
    pub key_8: bool,
    pub key_9: bool,

    // Function keys
    pub f1: bool,
    pub f2: bool,
    pub f3: bool,
    pub f4: bool,
    pub f5: bool,
    pub f6: bool,
    pub f7: bool,
    pub f8: bool,
    pub f9: bool,
    pub f10: bool,
    pub f11: bool,
    pub f12: bool,

    // Modifier keys
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub win: bool,

    // Navigation keys
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub home: bool,
    pub end: bool,
    pub page_up: bool,
    pub page_down: bool,

    // Special keys
    pub enter: bool,
    pub backspace: bool,
    pub tab: bool,
    pub escape: bool,
    pub delete: bool,
    pub insert: bool,
    pub space: bool,
    pub caps_lock: bool,
    pub num_lock: bool,
    pub scroll_lock: bool,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            a: false, b: false, c: false, d: false, e: false, f: false, g: false, h: false,
            i: false, j: false, k: false, l: false, m: false, n: false, o: false, p: false,
            q: false, r: false, s: false, t: false, u: false, v: false, w: false, x: false,
            y: false, z: false,
            key_0: false, key_1: false, key_2: false, key_3: false, key_4: false,
            key_5: false, key_6: false, key_7: false, key_8: false, key_9: false,
            f1: false, f2: false, f3: false, f4: false, f5: false, f6: false, f7: false,
            f8: false, f9: false, f10: false, f11: false, f12: false,
            shift: false, ctrl: false, alt: false, win: false,
            up: false, down: false, left: false, right: false,
            home: false, end: false, page_up: false, page_down: false,
            enter: false, backspace: false, tab: false, escape: false, delete: false,
            insert: false, space: false, caps_lock: false, num_lock: false, scroll_lock: false,
        }
    }
}