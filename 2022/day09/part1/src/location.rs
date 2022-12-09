#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Location {
    pub fn new() -> Self {
        Location { x: 0, y: 0 }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn new_from_coordinates(x: i32, y: i32) -> Self {
        Location { x, y }
    }

    pub fn move_right(&mut self) -> Self {
        self.x += 1;
        *self
    }

    pub fn move_left(&mut self) -> Self {
        self.x -= 1;
        *self
    }

    pub fn move_up(&mut self) -> Self {
        self.y += 1;
        *self
    }

    pub fn move_down(&mut self) -> Self {
        self.y -= 1;
        *self
    }

    pub fn horizontal_distance_between(&self, other: &Location) -> i32 {
        (other.x() - self.x()).abs()
    }

    pub fn vertical_distance_between(&self, other: &Location) -> i32 {
        (other.y() - self.y()).abs()
    }

    pub fn is_diagonal_from(&self, other: &Location) -> bool {
        (self.x() != other.x()) || (self.y() != other.y())
    }

    pub fn moves_further_right_from(&self, other: &Location) -> bool {
        other.x() < self.x()
    }

    pub fn moves_further_left_from(&self, other: &Location) -> bool {
        other.x() > self.x()
    }

    pub fn moves_further_up_from(&self, other: &Location) -> bool {
        other.y() < self.y()
    }

    pub fn moves_further_down_from(&self, other: &Location) -> bool {
        other.y() > self.y()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_distance_between_order_irrelevant() {
        let location1 = Location::new_from_coordinates(1, 0);
        let location2 = Location::new_from_coordinates(3, 0);

        assert_eq!(
            location1.horizontal_distance_between(&location2),
            location2.horizontal_distance_between(&location1)
        );
    }

    #[test]
    fn vertical_distance_between_order_irrelevant() {
        let location1 = Location::new_from_coordinates(0, 1);
        let location2 = Location::new_from_coordinates(0, 3);

        assert_eq!(
            location1.vertical_distance_between(&location2),
            location2.vertical_distance_between(&location1)
        );
    }

    #[test]
    fn is_diagonal_when_diagonal_true() {
        let location1 = Location::new_from_coordinates(0, 0);
        let location2 = Location::new_from_coordinates(1, 1);

        assert!(location1.is_diagonal_from(&location2));
    }

    #[test]
    fn is_diagonal_when_not_diagonal_false() {
        let location1 = Location::new_from_coordinates(1, 0);
        let location2 = Location::new_from_coordinates(1, 1);

        assert!(location1.is_diagonal_from(&location2));
    }
}
