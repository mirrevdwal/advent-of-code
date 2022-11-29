use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data/11.txt").expect("Could not read datafile");
    let input = data.lines().next().expect("Datafile is empty").to_string();

    let base = b'a';
    let password = input
        .chars()
        .map(|character| character as u8)
        .collect::<Vec<_>>();

    let (password, password1) = find_next(password, base);
    println!("Part 1: {password1}");

    let (_password, password2) = find_next(password, base);
    println!("Part 2: {password2}");
}

fn find_next(mut password: Vec<u8>, base: u8) -> (Vec<u8>, String) {
    loop {
        password = increment_password(password, base);

        if is_valid(password.clone()) {
            break;
        }
    }

    (
        password.clone(),
        password
            .iter()
            .map(|&number| number as char)
            .collect::<String>(),
    )
}

fn increment_password(mut password: Vec<u8>, base: u8) -> Vec<u8> {
    let mut increment_index = password.len() - 1;

    loop {
        if password[increment_index] == base + 25 {
            password[increment_index] = base;
            increment_index -= 1;
        } else {
            password[increment_index] += 1;
            break;
        }
    }

    password
}

fn is_valid(password: Vec<u8>) -> bool {
    let forbidden_chars = ['i', 'o', 'l'].map(|character| character as u8);
    for forbidden_char in forbidden_chars {
        if password.contains(&forbidden_char) {
            return false;
        }
    }
    if !password
        .windows(3)
        .any(|window| window[0] + 1 == window[1] && window[1] + 1 == window[2])
    {
        return false;
    }

    let pair_count = password
        .windows(2)
        .filter(|window| window[0] == window[1])
        .count();

    let triplet_count = password
        .windows(3)
        .filter(|window| window[0] == window[1] && window[1] == window[2])
        .count();

    if pair_count - triplet_count <= 1 {
        return false;
    }

    true
}
