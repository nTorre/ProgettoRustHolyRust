use crate::event::events::Event;
use crate::event::events::Event::{DayChanged, EnergyRecharged, Ready, TimeChanged};

use crate::runner::backpack::BackPack;
use crate::utils::LibError;
use crate::world::coordinates::Coordinate;
use crate::world::tile::TileType::Teleport;
use crate::world::world_generator::{check_world, Generator};
use crate::world::World;

use super::energy::{Energy, MAX_ENERGY_LEVEL};

pub mod backpack;

/// Represents the robot:
/// - `energy`: The energy level of the robot.
/// - `coordinate`: The coordinate of the robot, updated after each move.
/// - `backpack`: The backpack of the robot, updated after each action.
///
/// # Usage
/// ```
/// use robotics_lib::runner::{Robot};
/// struct MyRobot{
///     robot: Robot,
/// };
///
/// let r = MyRobot {
///     robot: Robot::new(),
/// };
/// ```

pub struct Robot {
    pub energy: Energy,
    pub coordinate: Coordinate,
    pub backpack: BackPack,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            energy: Energy::new(MAX_ENERGY_LEVEL),
            coordinate: Coordinate::new(0, 0),
            backpack: BackPack::new(0),
        }
    }
}

impl Default for Robot {
    //impl Default for *Clippy*
    fn default() -> Self {
        Self::new()
    }
}

/// Container for everything needed to run the robot.
///
/// ## Fields
/// - `robot`: A pointer to an implementation of the `Runnable` trait.
/// - `world`: Reppresents the game world.
///
/// # Usage
/// ```
/// # use std::collections::HashMap;
/// use robotics_lib::energy::Energy;
/// # use robotics_lib::event::events::Event;
/// # use robotics_lib::runner::{Robot, Runnable};
/// # use robotics_lib::runner::backpack::BackPack;
/// # use robotics_lib::world::coordinates::Coordinate;
/// # use robotics_lib::world::World;
/// # use robotics_lib::world::world_generator::Generator;
/// # use robotics_lib::world::tile::{Content, Tile};
/// # use robotics_lib::interface::Tools;
/// # use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
/// use robotics_lib::runner::Runner;
///
/// fn foo(){
///  struct WorldGenerator {size: usize};
///  impl WorldGenerator {
/// #    fn init(size: usize) -> Self {
/// #        WorldGenerator { size }
/// #    }
///  }
///
///  struct MyRobot(Robot);
///  impl Runnable for MyRobot{
/// #    fn process_tick(&mut self, world: &mut World) {
/// #        // do something
/// #    }
/// #    fn handle_event(&mut self, event: Event) {
/// #        // react to this event in your GUI
/// #    }
/// #    fn get_energy(&self) -> &Energy {
/// #        &self.0.energy
/// #    }
/// #    fn get_energy_mut(&mut self) -> &mut Energy {
/// #        &mut self.0.energy
/// #    }
/// #    fn get_coordinate(&self) -> &Coordinate {
/// #       &self.0.coordinate
/// #    }
/// #    fn get_coordinate_mut(&mut self) -> &mut Coordinate{
/// #        &mut self.0.coordinate
/// #    }
/// #    fn get_backpack(&self) -> &BackPack {
/// #        &self.0.backpack
/// #    }
/// #    fn get_backpack_mut(&mut self) -> &mut BackPack {
/// #        &mut self.0.backpack
/// #    }
///  }
///  impl Generator for WorldGenerator {
/// #  fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>){todo!()}
///  };
///  
///  let mut robot = MyRobot(Robot::new());
///  let mut generator = WorldGenerator::init(2);
///
///  struct Tool;
///  impl Tools for Tool {};
///  let tools = vec![Tool];
///
///  let run = Runner::new(Box::new(robot), &mut generator).unwrap();
/// }
/// ```
pub struct Runner {
    robot: Box<dyn Runnable>,
    world: World,
}

/// Represents the necessary functionality for a robot to be able to run
/// The `Runnable` trait is used to define the necessary functionality for a robot to be able to run.
///
/// # Usage
/// ```
/// use robotics_lib::runner::{Runnable};
/// ```
///
/// # Example
///
/// ```rust
/// use robotics_lib::energy::Energy;
/// use robotics_lib::event::events::Event;
/// use robotics_lib::runner::{Robot, Runnable};
/// use robotics_lib::runner::backpack::BackPack;
/// use robotics_lib::world::coordinates::Coordinate;
/// use robotics_lib::world::World;
///
/// struct MyRobot(Robot);
/// impl Runnable for MyRobot{
///     fn process_tick(&mut self, world: &mut World) {
///         // do something
///     }
///     fn handle_event(&mut self, event: Event) {
///         // react to this event in your GUI
///     }
///     fn get_energy(&self) -> &Energy {
///         &self.0.energy
///     }
///     fn get_energy_mut(&mut self) -> &mut Energy {
///         &mut self.0.energy
///     }
///     fn get_coordinate(&self) -> &Coordinate {
///        &self.0.coordinate
///     }
///     fn get_coordinate_mut(&mut self) -> &mut Coordinate{
///         &mut self.0.coordinate
///     }
///     fn get_backpack(&self) -> &BackPack {
///         &self.0.backpack
///     }
///     fn get_backpack_mut(&mut self) -> &mut BackPack {
///         &mut self.0.backpack
///     }
/// }
/// ```
pub trait Runnable {
    fn process_tick(&mut self, world: &mut World);
    fn handle_event(&mut self, event: Event);
    fn get_energy(&self) -> &Energy;
    fn get_energy_mut(&mut self) -> &mut Energy;
    fn get_coordinate(&self) -> &Coordinate;
    fn get_coordinate_mut(&mut self) -> &mut Coordinate;
    fn get_backpack(&self) -> &BackPack;
    fn get_backpack_mut(&mut self) -> &mut BackPack;
}

impl Runner {
    /// Initializes the the Runner
    /// The `new` function is used to initialiaze the robots values and to prompt the creation of the world.
    /// Both the robot and the world are stored in the returned instance of the Runner struct.
    ///
    /// # Usage
    /// ```
    /// use robotics_lib::runner::{Runner};
    /// ```
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// use robotics_lib::energy::Energy;
    /// # use robotics_lib::event::events::Event;
    /// # use robotics_lib::runner::{Robot, Runnable};
    /// # use robotics_lib::runner::backpack::BackPack;
    /// # use robotics_lib::world::coordinates::Coordinate;
    /// # use robotics_lib::world::World;
    /// # use robotics_lib::world::world_generator::Generator;
    /// # use robotics_lib::world::tile::{Content, Tile};
    /// # use robotics_lib::interface::Tools;
    /// # use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
    /// use robotics_lib::runner::Runner;
    ///
    /// fn foo(){
    ///  struct WorldGenerator {size: usize};
    ///  impl WorldGenerator {
    /// #    fn init(size: usize) -> Self {
    /// #        WorldGenerator { size }
    /// #    }
    ///  }
    ///
    ///  struct MyRobot(Robot);
    ///  impl Runnable for MyRobot{
    /// #    fn process_tick(&mut self, world: &mut World) {
    /// #        // do something
    /// #    }
    /// #    fn handle_event(&mut self, event: Event) {
    /// #        // react to this event in your GUI
    /// #    }
    /// #    fn get_energy(&self) -> &Energy {
    /// #        &self.0.energy
    /// #    }
    /// #    fn get_energy_mut(&mut self) -> &mut Energy {
    /// #        &mut self.0.energy
    /// #    }
    /// #    fn get_coordinate(&self) -> &Coordinate {
    /// #       &self.0.coordinate
    /// #    }
    /// #    fn get_coordinate_mut(&mut self) -> &mut Coordinate{
    /// #        &mut self.0.coordinate
    /// #    }
    /// #    fn get_backpack(&self) -> &BackPack {
    /// #        &self.0.backpack
    /// #    }
    /// #    fn get_backpack_mut(&mut self) -> &mut BackPack {
    /// #        &mut self.0.backpack
    /// #    }
    ///  }
    ///  impl Generator for WorldGenerator {
    /// #  fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>){todo!()}
    ///  };
    ///  
    ///  let mut robot = MyRobot(Robot::new());
    ///  let mut generator = WorldGenerator::init(2);
    ///
    ///  struct Tool;
    ///  impl Tools for Tool {};
    ///  let tools = vec![Tool];
    ///
    ///
    ///  let  _ = Runner::new( Box::new(robot), &mut generator).unwrap();
    /// }
    /// ```
    ///
    /// # Return
    /// An instance of Runner.
    pub fn new(mut robot: Box<dyn Runnable>, generator: &mut impl Generator) -> Result<Runner, LibError> {
        let (mut map, (robot_x, robot_y), environmental_conditions, max_score, score_table) = generator.gen();

        check_world(&map)?; //check if the world is valid

        *(robot.get_coordinate_mut()) = Coordinate::new(robot_x, robot_y);
        robot.get_backpack_mut().size = 20;

        if let Teleport(value) = map[robot_x][robot_y].tile_type {
            if !value {
                map[robot_x][robot_y].tile_type = Teleport(true);
            }
        }

        robot.handle_event(Ready);

        Ok(Runner {
            robot,
            world: World::new(map, environmental_conditions, max_score, score_table),
        })
    }

    /// The `game_tick` method calls all the update functions.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// use robotics_lib::energy::Energy;
    /// # use robotics_lib::event::events::Event;
    /// # use robotics_lib::runner::{Robot, Runnable};
    /// # use robotics_lib::runner::backpack::BackPack;
    /// # use robotics_lib::world::coordinates::Coordinate;
    /// # use robotics_lib::world::World;
    /// # use robotics_lib::world::world_generator::Generator;
    /// # use robotics_lib::world::tile::{Content, Tile};
    /// # use robotics_lib::interface::Tools;
    /// # use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
    /// use robotics_lib::runner::Runner;
    ///
    /// fn foo(){
    ///  struct WorldGenerator {size: usize};
    ///  impl WorldGenerator {
    /// #    fn init(size: usize) -> Self {
    /// #        WorldGenerator { size }
    /// #    }
    ///  }
    ///
    ///  struct MyRobot(Robot);
    ///  impl Runnable for MyRobot{
    /// #    fn process_tick(&mut self, world: &mut World) {
    /// #        // do something
    /// #    }
    /// #    fn handle_event(&mut self, event: Event) {
    /// #        // react to this event in your GUI
    /// #    }
    /// #    fn get_energy(&self) -> &Energy {
    /// #        &self.0.energy
    /// #    }
    /// #    fn get_energy_mut(&mut self) -> &mut Energy {
    /// #        &mut self.0.energy
    /// #    }
    /// #    fn get_coordinate(&self) -> &Coordinate {
    /// #       &self.0.coordinate
    /// #    }
    /// #    fn get_coordinate_mut(&mut self) -> &mut Coordinate{
    /// #        &mut self.0.coordinate
    /// #    }
    /// #    fn get_backpack(&self) -> &BackPack {
    /// #        &self.0.backpack
    /// #    }
    /// #    fn get_backpack_mut(&mut self) -> &mut BackPack {
    /// #        &mut self.0.backpack
    /// #    }
    ///  }
    ///  impl Generator for WorldGenerator {
    /// #  fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>){todo!()}
    ///  };
    ///  
    ///  let mut robot = MyRobot(Robot::new());
    ///  let mut generator = WorldGenerator::init(2);
    ///
    ///  struct Tool;
    ///  impl Tools for Tool {};
    ///  let tools = vec![Tool];
    ///
    ///
    ///  let mut run = Runner::new( Box::new(robot), &mut generator).unwrap();
    ///
    ///   'running : loop{
    ///     run.game_tick();
    ///     //time control here
    ///    }
    /// }
    /// ```
    pub fn game_tick(&mut self) -> Result<(), LibError> {
        //add other update functions here
        if self.world.advance_time() {
            self.robot
                .handle_event(DayChanged(self.world.environmental_conditions.clone()))
        } else {
            self.robot
                .handle_event(TimeChanged(self.world.environmental_conditions.clone()))
        }

        self.robot.process_tick(&mut self.world);

        let energy_to_add = 10;
        self.robot.get_energy_mut().recharge_energy(energy_to_add);
        self.robot.handle_event(EnergyRecharged(energy_to_add));
        Ok(())
    }

    ///Returns an immutable reference to the `robot` field of the `Runner` struct.
    pub fn get_robot(&self) -> &Box<dyn Runnable> {
        &self.robot
    }
}
