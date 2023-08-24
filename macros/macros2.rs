// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

macro_rules! tmp {
    () => {
        my_macro!();
    };
}
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    tmp!();
}
