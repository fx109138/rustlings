// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// I AM NOT DONE

#[warn(unused_macros)]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro();
}
