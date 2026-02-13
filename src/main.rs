mod key_state;
mod get_key_state;

use get_key_state::get_key_state;

fn main() {
    println!("Getting current keyboard state...");
    let current_state = get_key_state();
    println!("Current KeyState: {:#?}", current_state);
}
