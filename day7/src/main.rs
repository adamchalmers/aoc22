use std::collections::HashMap;

use parse::Line;

mod parse;

fn main() {
    let input = include_bytes!("../input");
    let (_, lines) = parse::parse_lines(input).expect("could not parse input file");
    let dir_sizes = size_of_dirs(lines);
    println!("q1: {}", q1(&dir_sizes));
    println!("q2: {}", q2(&dir_sizes));
}

/// Given the parsed input, find the size of every directory.
/// Order of the returned vec is arbitrary.
fn size_of_dirs(lines: Vec<Line>) -> Vec<usize> {
    let mut filesizes: HashMap<Vec<String>, usize> = Default::default();
    let mut cwd = Vec::new(); // cwd = current working directory
    for line in lines {
        match line {
            Line::Cd(dir) => match dir.as_ref() {
                ".." => {
                    cwd.pop();
                }
                "/" => {
                    cwd = Vec::new();
                }
                normal_dir => cwd.push(normal_dir.to_owned()),
            },
            Line::LsFile(size, name) => {
                let mut absolute_path = cwd.clone();
                absolute_path.push(name);
                filesizes.insert(absolute_path, size as _);
            }
            Line::Other => {}
        }
    }

    // Track the size of each directory.
    let mut dir_sizes: HashMap<Vec<String>, usize> = Default::default();

    // Loop over every file. Add its size to the size of each parent directory.
    for (absolute_filepath, size) in filesizes {
        for i in 0..absolute_filepath.len() {
            let dir = absolute_filepath[0..i].to_vec();
            *dir_sizes.entry(dir).or_insert(0) += size;
        }
    }

    dir_sizes.values().copied().collect()
}

/// Find all of the directories with a total size of at most 100000. What is the sum of the total
/// sizes of those directories?
fn q1(dir_sizes: &[usize]) -> usize {
    const MAX_TOTAL_SIZE: usize = 100_000;
    dir_sizes
        .iter()
        .filter(|size| size <= &&MAX_TOTAL_SIZE)
        .sum()
}

/// Find the smallest directory that, if deleted, would free up enough space on the filesystem to
/// run the update. What is the total size of that directory?
fn q2(dir_sizes: &[usize]) -> usize {
    let bytes_in_use = *dir_sizes.iter().max().unwrap();
    const DISK_SIZE: usize = 70000000;
    let bytes_unused = DISK_SIZE - bytes_in_use;
    const BYTES_REQUIRED: usize = 30000000;
    let bytes_to_delete = BYTES_REQUIRED - bytes_unused;
    *dir_sizes
        .iter()
        .filter(|dir_size| dir_size >= &&bytes_to_delete)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_answer() {
        let input = include_bytes!("../input");
        let (_, lines) = parse::parse_lines(input).expect("could not parse input file");
        let dir_sizes = size_of_dirs(lines);
        assert_eq!(1390824, q1(&dir_sizes));
        assert_eq!(7490863, q2(&dir_sizes));
    }
}
