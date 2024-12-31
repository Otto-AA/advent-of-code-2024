use crate::{
    parser::Direction,
    warehouse::{Field, Point, Warehouse},
};

pub(crate) struct Simulation<'a> {
    pub(crate) warehouse: &'a mut Warehouse,
    pub(crate) robot_position: Point,
}

impl Simulation<'_> {
    pub(crate) fn move_robot(&mut self, direction: &Direction) {
        if self.can_move_in_direction(direction, &self.robot_position) {
            self.move_in_direction(direction, &self.robot_position.clone());
        }
    }

    fn can_move_in_direction(&self, direction: &Direction, start: &Point) -> bool {
        let (point, field) = self
            .warehouse
            .next_in_direction(start, direction)
            .expect("Tried to move outside of the warehouse");
        match field {
            Field::Empty => true,
            Field::Wall => false,
            Field::Box => self.can_move_in_direction(direction, &point),
            Field::WideBox(wide_box_side) => {
                if matches!(direction, Direction::Left | Direction::Right) {
                    self.can_move_in_direction(direction, &point)
                } else {
                    let (_, other_side_point) = wide_box_side.other_side(&point);
                    self.can_move_in_direction(direction, &point)
                        && self.can_move_in_direction(direction, &other_side_point)
                }
            }
            Field::Robot => panic!("Tried to move into a robot. Too many robots?"),
        }
    }

    fn move_in_direction(&mut self, direction: &Direction, from: &Point) {
        let (next_point, next_field) = self
            .warehouse
            .next_in_direction(from, direction)
            .expect("Tried to move outside of the warehouse");
        match next_field {
            Field::Box => self.move_in_direction(direction, &next_point),
            Field::WideBox(wide_box_side) => {
                if matches!(direction, Direction::Left | Direction::Right) {
                    self.move_in_direction(direction, &next_point)
                } else {
                    let (_, other_side_point) = wide_box_side.other_side(&next_point);
                    self.move_in_direction(direction, &next_point);
                    self.move_in_direction(direction, &other_side_point);
                }
            }
            Field::Empty => {}
            Field::Wall => panic!("Tried to move into a wall"),
            Field::Robot => panic!("Tried to move into a robot. Too many robots?"),
        }
        self.warehouse[&next_point] = self.warehouse[from].clone();
        self.warehouse[from] = Field::Empty;
        self.robot_position = next_point;
    }
}
