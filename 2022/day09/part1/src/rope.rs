use std::{collections::HashSet, hash::Hash};

use crate::location::Location;

pub struct Rope {
    head: Location,
    tail: Location,
    visited_locations: HashSet<Location>,
}

impl Rope {
    pub fn new() -> Self {
        let mut visited_locations = HashSet::new();
        visited_locations.insert(Location::new());

        Rope {
            head: Location::new(),
            tail: Location::new(),
            visited_locations,
        }
    }

    pub fn from_start(head: impl Into<Location>, tail: impl Into<Location>) -> Self {
        let tail = tail.into();

        let mut visited_locations = HashSet::new();
        visited_locations.insert(tail);

        Rope {
            head: head.into(),
            tail,
            visited_locations,
        }
    }

    pub fn tail(&self) -> Location {
        self.tail
    }

    pub fn process_command(&mut self, command: &str, nb_steps: i32) {
        match command {
            "R" => self.move_right(nb_steps),
            "L" => self.move_left(nb_steps),
            "U" => self.move_up(nb_steps),
            "D" => self.move_down(nb_steps),
            _ => panic!("Invalid command [{}] encountered", command),
        }
    }

    pub fn get_nb_unique_visited_locations(&self) -> usize {
        self.visited_locations.len()
    }

    fn move_right(&mut self, nb_steps: i32) {
        for _ in 0..nb_steps {
            if self.head.is_diagonal_from(&self.tail)
                && self.head.moves_further_right_from(&self.tail)
            {
                self.tail = self.head.clone();
                self.head.move_right();
            } else {
                self.head.move_right();
                if self.tail.horizontal_distance_between(&self.head) > 1 {
                    self.tail.move_right();
                }
            }
            self.visited_locations.insert(self.tail);
        }
    }

    fn move_left(&mut self, nb_steps: i32) {
        for _ in 0..nb_steps {
            if self.head.is_diagonal_from(&self.tail)
                && self.head.moves_further_left_from(&self.tail)
            {
                self.tail = self.head.clone();
                self.head.move_left();
            } else {
                self.head.move_left();
                if self.tail.horizontal_distance_between(&self.head) > 1 {
                    self.tail.move_left();
                }
            }
            self.visited_locations.insert(self.tail);
        }
    }

    fn move_up(&mut self, nb_steps: i32) {
        for _ in 0..nb_steps {
            if self.head.is_diagonal_from(&self.tail) && self.head.moves_further_up_from(&self.tail)
            {
                self.tail = self.head.clone();
                self.head.move_up();
            } else {
                self.head.move_up();
                if self.tail.vertical_distance_between(&self.head) > 1 {
                    self.tail.move_up();
                }
            }
            self.visited_locations.insert(self.tail);
        }
    }

    fn move_down(&mut self, nb_steps: i32) {
        for _ in 0..nb_steps {
            if self.head.is_diagonal_from(&self.tail)
                && self.head.moves_further_down_from(&self.tail)
            {
                self.tail = self.head.clone();
                self.head.move_down();
            } else {
                self.head.move_down();
                if self.tail.vertical_distance_between(&self.head) > 1 {
                    self.tail.move_down();
                }
            }
            self.visited_locations.insert(self.tail);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_starts_head_and_tail_at_origin() {
        let origin = Location::new();

        let result = Rope::new();

        assert_eq!(result.head, origin);
        assert_eq!(result.tail, origin);
    }
}

#[cfg(test)]
mod tests_move_right {
    use super::*;

    #[test]
    fn move_by_one_when_at_same_starting_position_tail_doesnt_move() {
        let mut rope = Rope::new();
        let initial_tail = rope.tail.clone();

        rope.move_right(1);

        assert_eq!(rope.tail, initial_tail);
    }

    #[test]
    fn move_by_one_head_x_one_higher() {
        let mut rope = Rope::new();

        rope.move_right(1);

        assert_eq!(rope.head, Location::new_from_coordinates(1, 0));
    }

    #[test]
    fn move_right_by_two_head_two_higher() {
        let mut rope = Rope::new();

        rope.move_right(2);

        assert_eq!(rope.head, Location::new_from_coordinates(2, 0));
    }

    #[test]
    fn move_by_two_initial_position_equal_tail_moves_right() {
        let mut rope = Rope::new();

        rope.move_right(2);

        assert_eq!(rope.tail, Location::new_from_coordinates(1, 0));
    }

    #[test]
    fn when_diagonal_and_moves_further_tail_follows() {
        let initial_head = Location::new_from_coordinates(1, 1);
        let initial_tail = Location::new_from_coordinates(0, 0);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_right(1);

        assert_eq!(rope.tail, Location::new_from_coordinates(1, 1));
    }

    #[test]
    fn when_diagonal_and_moves_closer_tail_doesnt_move() {
        let initial_head = Location::new_from_coordinates(0, 0);
        let initial_tail = Location::new_from_coordinates(1, 1);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_right(1);

        assert_eq!(rope.tail, initial_tail);
    }
}

#[cfg(test)]
mod tests_move_left {
    use super::*;

    #[test]
    fn move_by_one_when_at_same_starting_position_tail_doesnt_move() {
        let mut rope = Rope::new();
        let initial_tail = rope.tail.clone();

        rope.move_left(1);

        assert_eq!(rope.tail, initial_tail);
    }

    #[test]
    fn move_by_one_head_x_one_lower() {
        let mut rope = Rope::new();

        rope.move_left(1);

        assert_eq!(rope.head, Location::new_from_coordinates(-1, 0));
    }

    #[test]
    fn move_by_two_head_two_lower() {
        let mut rope = Rope::new();

        rope.move_left(2);

        assert_eq!(rope.head, Location::new_from_coordinates(-2, 0));
    }

    #[test]
    fn move_by_two_initial_position_equal_tail_moves_left() {
        let mut rope = Rope::new();
        let expected = rope.tail.clone().move_left();

        rope.move_left(2);

        assert_eq!(rope.tail, expected);
    }

    #[test]
    fn when_diagonal_and_moves_further_tail_follows() {
        let initial_head = Location::new_from_coordinates(0, 0);
        let initial_tail = Location::new_from_coordinates(1, 1);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_left(1);

        assert_eq!(rope.tail, Location::new_from_coordinates(0, 0));
    }

    #[test]
    fn when_diagonal_and_moves_closer_tail_doesnt_move() {
        let initial_head = Location::new_from_coordinates(1, 1);
        let initial_tail = Location::new_from_coordinates(0, 0);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_left(1);

        assert_eq!(rope.tail, initial_tail);
    }
}

#[cfg(test)]
mod tests_move_up {
    use super::*;

    #[test]
    fn move_up_by_one_when_at_same_starting_position_tail_doesnt_move() {
        let mut rope = Rope::new();
        let initial_tail = rope.tail.clone();

        rope.move_up(1);

        assert_eq!(rope.tail, initial_tail);
    }

    #[test]
    fn move_up_by_one_head_y_one_higher() {
        let mut rope = Rope::new();

        rope.move_up(1);

        assert_eq!(rope.head, Location::new_from_coordinates(0, 1));
    }

    #[test]
    fn move_up_by_two_head_two_higher() {
        let mut rope = Rope::new();

        rope.move_up(2);

        assert_eq!(rope.head, Location::new_from_coordinates(0, 2));
    }

    #[test]
    fn move_up_by_two_initial_position_equal_tail_moves_up() {
        let mut rope = Rope::new();

        rope.move_up(2);

        assert_eq!(rope.tail, Location::new_from_coordinates(0, 1));
    }

    #[test]
    fn when_diagonal_and_moves_further_tail_follows() {
        let initial_head = Location::new_from_coordinates(1, 1);
        let initial_tail = Location::new_from_coordinates(0, 0);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_up(1);

        assert_eq!(rope.tail, Location::new_from_coordinates(1, 1));
    }

    #[test]
    fn when_diagonal_and_moves_closer_tail_doesnt_move() {
        let initial_head = Location::new_from_coordinates(0, 0);
        let initial_tail = Location::new_from_coordinates(1, 1);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_up(1);

        assert_eq!(rope.tail, initial_tail);
    }
}

#[cfg(test)]
mod tests_move_down {
    use super::*;

    #[test]
    fn move_down_by_one_when_at_same_starting_position_tail_doesnt_move() {
        let mut rope = Rope::new();
        let initial_tail = rope.tail.clone();

        rope.move_down(1);

        assert_eq!(rope.tail, initial_tail);
    }

    #[test]
    fn move_down_by_one_head_y_one_lower() {
        let mut rope = Rope::new();

        rope.move_down(1);

        assert_eq!(rope.head, Location::new_from_coordinates(0, -1));
    }

    #[test]
    fn move_down_by_two_head_two_lower() {
        let mut rope = Rope::new();

        rope.move_down(2);

        assert_eq!(rope.head, Location::new_from_coordinates(0, -2));
    }

    #[test]
    fn move_down_by_two_initial_position_equal_tail_moves_down() {
        let mut rope = Rope::new();

        rope.move_down(2);

        assert_eq!(rope.tail, Location::new_from_coordinates(0, -1));
    }

    #[test]
    fn when_diagonal_and_moves_further_tail_follows() {
        let initial_head = Location::new_from_coordinates(0, 0);
        let initial_tail = Location::new_from_coordinates(1, 1);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_down(1);

        assert_eq!(rope.tail, Location::new_from_coordinates(0, 0));
    }

    #[test]
    fn when_diagonal_and_moves_closer_tail_doesnt_move() {
        let initial_head = Location::new_from_coordinates(1, 1);
        let initial_tail = Location::new_from_coordinates(0, 0);

        let mut rope = Rope::from_start(initial_head, initial_tail);

        rope.move_down(1);

        assert_eq!(rope.tail, initial_tail);
    }
}
