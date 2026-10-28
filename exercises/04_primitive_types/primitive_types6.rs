fn main() {
    // A bit of a naive experiment to confirm that tuple access returns the value and not a
    // reference.
    let mut num = (1, 2, 3);
    let second = num.1;
    num.1 = 4;
    let display = num.1;
    println!("{display}");
    println!("{second}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
