fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    let v = Vec::from(a);
    (a, v)
}

fn main() {
    let a = [10, 20, 30, 40]; // Array
    let a1 = a.to_vec();
    let mut a2 = Vec::from(a);
    let a3 = vec![10, 20, 30, 40];
    a2[0] = 11;
    let t1 = a1 == a2;
    let t2 = a1 == a3;

    println!("{t1}, {t2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
