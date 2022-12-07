use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

struct FileEntry {
    path: PathBuf,
    size: usize,
}

fn main() {
    let data = parse_data("data/07.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let files = get_files(data);

    let mut directories: HashMap<&Path, usize> = HashMap::new();
    for file in &files {
        for ancestor in file.path.ancestors().skip(1) {
            *directories.entry(ancestor).or_insert(0) += file.size
        }
    }

    directories
        .iter()
        .filter_map(|(_dir, size)| (*size <= 100_000).then_some(size))
        .sum()
}

fn part_two(data: &str) -> usize {
    let total_size = 70_000_000;
    let needed = 30_000_000;

    let files = get_files(data);

    let mut directories: HashMap<&Path, usize> = HashMap::new();
    for file in &files {
        for ancestor in file.path.ancestors().skip(1) {
            *directories.entry(ancestor).or_insert(0) += file.size
        }
    }

    let used = directories
        .get(&Path::new("/"))
        .expect("Could not find size of root dir");
    let free = total_size - used;

    *directories
        .iter()
        .filter_map(|(_dir, size)| (*size >= needed - free).then_some(size))
        .min()
        .expect("Could not find best directory to delete")
}

fn get_files(input: &str) -> Vec<FileEntry> {
    let mut files: Vec<FileEntry> = Vec::new();
    let mut current_path = PathBuf::new();

    for line in input.lines() {
        let mut segments = line.split_whitespace();

        match segments.next() {
            Some("$") => match segments.next().expect("Nothing found after '$'") {
                "cd" => match segments.next().expect("Could not find directory name") {
                    ".." => {
                        current_path.pop();
                    }
                    "/" => current_path = PathBuf::from("/"),
                    dir_name => current_path.push(dir_name),
                },
                "ls" => {}
                _ => panic!("Unexpected segment after '$'"),
            },
            Some("dir") => {}
            Some(num_str) if num_str.chars().all(|character| character.is_ascii_digit()) => files
                .push(FileEntry {
                    path: current_path.join(segments.next().expect("Could not find file name")),
                    size: num_str.parse::<usize>().unwrap(),
                }),
            None => {
                break;
            }
            _ => panic!("Unexpected first segment"),
        }
    }

    files
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/07-example.txt");
        assert_eq!(95437, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/07-example.txt");
        assert_eq!(24933642, part_two(&data));
    }
}
