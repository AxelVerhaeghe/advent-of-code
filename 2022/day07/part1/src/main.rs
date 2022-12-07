use file_system::FileSystem;

use util::read_all_lines;

mod directory;
mod file;
mod file_system;
mod instruction;
mod util;

fn main() {
    let lines = read_all_lines("./input.txt");

    let mut fs = FileSystem::new();
    fs.process_scan(lines);

    let total_sizes = fs.get_all_total_sizes();

    let result: i32 = total_sizes
        .iter()
        .filter(|&(_, &size)| size <= 100000)
        .map(|entry| entry.1)
        .sum();

    println!("Total sum: {result}");
}
