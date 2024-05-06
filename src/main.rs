fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
fn add() -> usize {
    1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add() {
        let sum = add();
        assert_eq!(sum, 2);
    }
}
