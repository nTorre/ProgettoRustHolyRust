use std::fmt;
use std::fmt::{Display, Formatter};

use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::tile::{Content, Tile};

/// Represent an [Event] that occurred in a Robot.
///
/// # Example
/// You can listen to the events in the `handle_event` function of the [crate::runner::Runnable] trait
/// ```rust
///  use robotics_lib::energy::Energy;
///  use robotics_lib::event::events::Event;
///  use robotics_lib::runner::{Robot, Runnable};
///  use robotics_lib::runner::backpack::BackPack;
///  use robotics_lib::world::coordinates::Coordinate;
///  use robotics_lib::world::World;
///
///  struct MyRobot(Robot);
///
///  impl Runnable for MyRobot {
///     fn process_tick(&mut self, world: &mut World) {
///         // your processing
///     }
///
///     fn handle_event(&mut self, event: Event) {
///         // consume the event in your GUI
///         println!("{:?}", event);
///     }
///
///     fn get_energy(&self) -> &Energy {
///         &self.0.energy
///     }
///     fn get_energy_mut(&mut self) -> &mut Energy {
///         &mut self.0.energy
///     }
///
///     fn get_coordinate(&self) -> &Coordinate {
///         &self.0.coordinate
///     }
///     fn get_coordinate_mut(&mut self) -> &mut Coordinate {
///         &mut self.0.coordinate
///     }
///
///     fn get_backpack(&self) -> &BackPack {
///         &self.0.backpack
///     }
///     fn get_backpack_mut(&mut self) -> &mut BackPack {
///         &mut self.0.backpack
///     }
///  }
/// ```

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    /// Robot has been initialized and its lifecycle has started
    Ready,
    /// Robot has ended its lifecycle
    Terminated,
    /// [Event] fired when time of the day changes, contains the new [EnvironmentalConditions]
    TimeChanged(EnvironmentalConditions),

    /// [Event] fired when the day changes, contains the new [EnvironmentalConditions]
    DayChanged(EnvironmentalConditions),

    /// [Event] fired when energy gets recharged, contains the recharge amount
    EnergyRecharged(usize),

    /// [Event] fired when energy is consumed, contains the consumed amount
    EnergyConsumed(usize),

    /// [Event] fired when the robot moves to new coordinates
    ///
    /// This [Event] contains the [Tile] to which the robot moved and the coordinates
    Moved(Tile, (usize, usize)),

    /// [Event] fired when a tile content gets updated.
    ///
    /// This [Event] contains the [Tile] of the updated content and the coordinates
    TileContentUpdated(Tile, (usize, usize)),

    /// [Event] fired when a [Content] is added to the backpack, also contains the amount of content added
    AddedToBackpack(Content, usize),

    /// [Event] fired when a [Content] is removed from the backpack, also contains the amount of content removed
    RemovedFromBackpack(Content, usize),
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            | Event::Ready => write!(f, "Robot is ready!"),
            | Event::Terminated => write!(f, "Robot has been terminated!"),
            | Event::TimeChanged(env) => write!(f, "Time changed, new environmental conditions: {:?}", env),
            | Event::DayChanged(env) => write!(f, "Day changed, new environmental conditions: {:?}", env),
            | Event::EnergyRecharged(recharge) => write!(f, "Recharged with {} energy", recharge),
            | Event::EnergyConsumed(consumed) => write!(f, "Consumed {} energy", consumed),
            | Event::Moved(tile, (row, col)) => {
                write!(f, "Moved to coordinates ({}, {}) with tile {:?}", row, col, tile)
            }
            | Event::TileContentUpdated(tile, (row, col)) => {
                write!(f, "Tile content updated at coords ({}, {}) to {:?}", row, col, tile)
            }
            | Event::AddedToBackpack(content, amount) => {
                write!(f, "Added {} amount of {:?} to backpack", amount, content)
            }
            | Event::RemovedFromBackpack(content, amount) => {
                write!(f, "Removed {} amount of {:?} from backpack", amount, content)
            }
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Event: {}", self)
    }
}
