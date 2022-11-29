fn main() {
    let row = 2947;
    let column = 3029;

    let index = get_index(row, column);

    let mut value: u128 = 20151125;
    for _i in 1..index {
        value = (value * 252533) % 33554393;
    }

    println!("Part 1: {}", value);
}

fn get_index(row: usize, column: usize) -> usize {
    let total = row + column - 2;
    total * (total + 1) / 2 + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(get_index(1, 1), 1);
        assert_eq!(get_index(2, 4), 14);
        assert_eq!(get_index(4, 3), 18);
    }
}
