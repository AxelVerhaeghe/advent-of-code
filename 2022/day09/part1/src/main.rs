use rope::Rope;
use util::read_all_lines;

mod location;
mod rope;
mod util;

fn main() {
    let lines = read_all_lines("input.txt");

    let mut rope = Rope::new();

    for line in lines {
        let (command, nb_steps) = extract_command_and_amount(&line);
        rope.process_command(command, nb_steps);
    }

    let total_visited_locations = rope.get_nb_unique_visited_locations();

    println!("The tail visited a total of {total_visited_locations} unique locations");
}

fn extract_command_and_amount(input: &str) -> (&str, i32) {
    let split: Vec<&str> = input.split(" ").collect();

    let command = split.get(0).unwrap();
    let nb_steps: i32 = split
        .get(1)
        .unwrap()
        .parse::<i32>()
        .expect("error parsing 'nb_steps'");

    (command, nb_steps)
}
