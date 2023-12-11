/// Coordinate struct
/// The `Coordinate` struct is used to define the coordinates of a tile.
///
/// # Usage
/// ```
/// use robotics_lib::world::coordinates::Coordinate;
/// ```
///
/// # Example
/// ```rust
/// use robotics_lib::runner::Robot;
/// use robotics_lib::world::coordinates::Coordinate;
/// let robot = Robot::new();
/// let row = robot.coordinate.get_row();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    /// Creates a new instance of `Coordinate`, called only inside of the common crate
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Coordinate { row, col }
    }

    /// Returns the row of the coordinate
    pub fn get_row(&self) -> usize {
        self.row
    }

    /// Returns the column of the coordinate
    pub fn get_col(&self) -> usize {
        self.col
    }
}
