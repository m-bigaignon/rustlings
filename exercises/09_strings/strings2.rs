// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(word.as_str()) {
        // My first instinct was to find the 'inverse' of the `as_string` coercion
        // used in the previous exercise. End result is the same, this one is more explicit imo
        // but also not as concise.
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
