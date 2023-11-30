// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

struct SingleGen<T>(T);

fn main() {
    let mut shopping_list: Vec<SingleGen<String>> = Vec::new();
}
