// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
