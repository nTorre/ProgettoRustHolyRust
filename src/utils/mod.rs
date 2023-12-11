use std::cmp::min;
use std::sync::Mutex;

use crate::event::events::Event::{AddedToBackpack, RemovedFromBackpack};
use crate::interface::Direction;
use crate::runner::Runnable;
use crate::utils::LibError::{
    CannotWalk, NoContent, NotEnoughEnergy, NotEnoughSpace, OperationNotAllowed, OutOfBounds,
};
use crate::world::coordinates::Coordinate;
use crate::world::environmental_conditions::{DayTime, EnvironmentalConditions, WeatherType};
use crate::world::tile::Content;
use crate::world::tile::TileType;
use crate::world::tile::TileType::Teleport;
use crate::world::World;

/// It contains all the errors that can be returned by the library
///
/// # Variants
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to do the action
/// - `OutOfBounds`: The robot couldn't be moved
/// - `NoContent`: The content doesn't exist
/// - `NotEnoughSpace(usize)`: The backpack doesn't have enough space
/// - `CannotDestroy`: The content cannot be destroyed by the robot
/// - `CannotWalk`: The robot cannot walk on the desired tile
/// - `WrongContentUsed`: Called the put interface with the wrong content
/// - `NotEnoughContentProvided`,
/// - `OperationNotAllowed`: The operation does not fall within the available actions
/// - `NotCraftable`,
/// - `NoMoreDiscovery`: The available discoverable tiles aren't enough (default: 10% of the world's size),
/// - `EmptyForecast`,
/// - `NotEnoughContentInBackPack`,
/// - `WorldIsNotASquare`,
/// - `TeleportIsTrueOnGeneration`,
/// - `ContentValueIsHigherThanMax`,
/// - `WronContentNotAllowedOnTilegHour`,
/// - `MustDestroyContentFirst`: To complete an operation on a tile the destruction of its content is needed,
///
/// # Examples
///
/// ```rust
/// use robotics_lib::utils::LibError;
/// fn catch_error(error: LibError) {
///     match error {
///         | LibError::NotEnoughEnergy => println!("Not enough energy"),
///         | LibError::OutOfBounds => println!("Out of bounds"),
///         | LibError::NoContent => println!("No content"),
///         | LibError::NotEnoughSpace(remainder) => println!("Not enough space: {}", remainder),
///         | LibError::CannotDestroy => println!("Cannot destroy"),
///         | LibError::NotCraftable => println!("Can't craft this item"),
///         | LibError::NoMoreDiscovery => println!("Not enough discoverable tiles"), ///
///         _ => {}
///     }
/// }
/// ```
///
/// # Remarks
/// - The errors are returned by the functions of the library
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LibError {
    NotEnoughEnergy,
    OutOfBounds,
    NoContent,
    NotEnoughSpace(usize),
    CannotDestroy,
    CannotWalk,
    WrongContentUsed,
    NotEnoughContentProvided,
    OperationNotAllowed,
    NotCraftable,
    NoMoreDiscovery,
    EmptyForecast,
    WrongHour,
    NotEnoughContentInBackPack,
    WorldIsNotASquare,
    TeleportIsTrueOnGeneration,
    ContentValueIsHigherThanMax,
    ContentNotAllowedOnTile,
    MustDestroyContentFirst, //other
}

/// This function is used to check if the robot can go in the direction passed as argument
///
/// # Arguments
/// - robot: The robot that has to move
/// - direction: The direction the robot wants to go
///
/// # Returns
/// Result<bool, Liberror>
/// bool: the robot can go in the direction passed as argument
///
/// # Errors:
/// - `NoTileTypeProps`: The TileTypeProp of the target cell is not set properly
/// - `OutOfBounds`: The robot couldn't be moved cause it's on the border an the chosen direction is out of bounds
/// - `CannotWalk`: The robot cannot walk on the desired tiletype
///
/// # Examples
///
/// ```rust
/// use robotics_lib::interface::Direction;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::utils::{go_allowed, LibError};
/// use robotics_lib::world::World;
/// fn go_allowed_example(robot: &impl Runnable, world: &World, direction: &Direction) -> Result<(), LibError>{
///     go_allowed(robot, world, direction)?;
///     // move the robot
///     Ok(())
/// }
///
/// ```
pub fn go_allowed(robot: &impl Runnable, world: &World, direction: &Direction) -> Result<(), LibError> {
    let (robot_row, robot_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());

    in_bounds(robot, world, direction)?;

    let walk = match direction {
        | Direction::Up => world.map[robot_row - 1][robot_col].tile_type.properties().walk(),
        | Direction::Down => world.map[robot_row + 1][robot_col].tile_type.properties().walk(),
        | Direction::Left => world.map[robot_row][robot_col - 1].tile_type.properties().walk(),
        | Direction::Right => world.map[robot_row][robot_col + 1].tile_type.properties().walk(),
    };

    if !walk {
        return Err(CannotWalk);
    }

    Ok(())
}

pub fn in_bounds(robot: &impl Runnable, world: &World, direction: &Direction) -> Result<(), LibError> {
    let (robot_row, robot_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());

    let near_borders = match direction {
        | Direction::Up => robot_row == 0,
        | Direction::Down => robot_row == world.dimension - 1,
        | Direction::Left => robot_col == 0,
        | Direction::Right => robot_col == world.dimension - 1,
    };

    if near_borders {
        return Err(OutOfBounds);
    }

    Ok(())
}

/// This function is used to check if the robot can go in the direction passed as argument
///
/// # Arguments
///
/// - robot: The robot that has to move
/// - row_col: (row, col) coordinates
///
/// returns: bool
///
/// # Examples
///
/// ```
/// use robotics_lib::interface::Direction;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// fn go_allowed_example(world: &World, row_col:(usize,usize)) {
///     if robotics_lib::utils::go_allowed_row_col(world, row_col) {
///         print!("Go allowed");
///     } else {
///         print!("Go not allowed");
///     }
/// }
///
/// ```
pub fn go_allowed_row_col(world: &World, row_col: (usize, usize)) -> bool {
    let (row, col) = row_col;
    row < world.dimension && col < world.dimension
}

/// This function returns the coordinates of the direction respect to the position of the robot
///
/// # Arguments
/// - robot: The robot
/// - direction: The direction of which you want to know the coordinates
///
/// # Returns
/// (usize, usize): The coordinates of the direction respect to the position of the robot
///
pub(crate) fn get_coords_row_col(robot: &impl Runnable, direction: &Direction) -> (usize, usize) {
    let robot_row = robot.get_coordinate().get_row();
    let robot_col = robot.get_coordinate().get_col();
    match direction {
        | Direction::Up => (robot_row - 1, robot_col),
        | Direction::Down => (robot_row + 1, robot_col),
        | Direction::Left => (robot_row, robot_col - 1),
        | Direction::Right => (robot_row, robot_col + 1),
    }
}

/// This function is used to check if the robot can be teleported in the coordinates passed as argument
///
/// # Arguments
/// - robot: The robot that has to move
/// - world: The world in which the robot is
/// - coordinates: The coordinates where the robot wants to go
///
///
/// # Returns
/// Result<(), Liberror>
///
/// # Errors:
/// - `OutOfBounds`: The robot couldn't be teleported because the coordinate given is out of bound
/// - `OperationNotAllowed`: The robot isn't in a teleport tile or it's trying to teleport itself in a tile which isn't a teleport tile too
///
pub fn teleport_allowed(robot: &impl Runnable, world: &World, coordinates: &Coordinate) -> Result<(), LibError> {
    let (teleport_row, teleport_col) = (coordinates.get_row(), coordinates.get_col());
    if !go_allowed_row_col(world, (teleport_row, teleport_col)) {
        return Err(OutOfBounds);
    }

    let (robot_row, robot_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
    if world.map[robot_row][robot_col].tile_type != Teleport(true)
        || world.map[teleport_row][teleport_col].tile_type != Teleport(true)
    {
        return Err(OperationNotAllowed);
    }
    Ok(())
}

/// This function returns the capability to destroy or not the content
///
/// # Arguments
/// - world: The world where the robot is
/// - row_col: (row, col) coordinates
///
/// # Returns
/// -Ok(()) if the content can be destroyed <br>
/// -Err(LibError) otherwise
///
/// # Errors
/// -OutOfBounds: The coordinates are out of bounds <br>
/// -CannotDestroy: The content cannot be destroyed
pub(crate) fn can_destroy(world: &World, row_col: (usize, usize)) -> Result<bool, LibError> {
    if !go_allowed_row_col(world, row_col) {
        return Err(LibError::OutOfBounds);
    }
    if world.map[row_col.0][row_col.1].content == Content::None {
        return Err(NoContent);
    }
    Ok(world.map[row_col.0][row_col.1].content.properties().destroy())
}

/// This function let's you put content in the backpack
///
/// # Arguments
/// - robot: The robot to which the contents will be added to
/// - content: The content that has to be put in the backpack
/// - quantity: The quantity of the content that has to be put in the backpack
///
/// # Returns
/// Result<usize, LibError>
///
/// # Errors
/// - NotEnoughSpace: There is not enough space in the backpack
pub(crate) fn add_to_backpack(robot: &mut impl Runnable, content: Content, quantity: usize) -> Result<usize, LibError> {
    let remainder = robot.get_backpack().size - robot.get_backpack().contents.values().sum::<usize>();
    let quantity_to_add = quantity.min(remainder);

    *robot
        .get_backpack_mut()
        .contents
        .entry(content.to_default())
        .or_insert(0) += quantity_to_add;

    robot.handle_event(AddedToBackpack(content, quantity_to_add));

    if remainder >= quantity {
        Ok(quantity_to_add)
    } else {
        Err(NotEnoughSpace(quantity_to_add))
    }
}

/// This function lets you remove some content from the backpack
///
/// # Arguments
/// - robot: The robot from which the contents will be removed
/// - content: The content that has to be removed in the backpack
/// - quantity: The quantity of the content that has to be removed from the backpack
///
/// # Returns
/// The removed quantity
///
/// # Behavior
/// - if the robot doesn't have the content in its backpack, it returns an error
/// - if the robot has less content than the quantity passed as argument, it remove the quantity of the content that the robot has
/// - if the robot has enough content to remove, it removes the quantity passed as argument
pub(crate) fn remove_from_backpack(
    robot: &mut impl Runnable,
    content: &Content,
    quantity: usize,
) -> Result<usize, LibError> {
    let remove_result = match robot.get_backpack_mut().contents.get_mut(&content.to_default()) {
        | None => Err(NoContent),
        | Some(value) => {
            if 0_usize == *value {
                // the robot doesn't have the value in its backpack
                Err(NoContent)
            } else if *value <= quantity {
                // an interface wants to remove more content than the robot actually has
                let tmp = *value;
                *value = 0;
                Ok(tmp)
            } else {
                // the robot has enough content to remove
                *value -= quantity;
                Ok(quantity)
            }
        }
    };

    let result = match remove_result {
        | Ok(u) => u,
        | Err(e) => {
            return Err(e);
        }
    };

    if remove_result.is_ok() {
        robot.handle_event(RemovedFromBackpack(content.clone(), result))
    }

    remove_result
}

/// A function used for the put interface combined with the store `Content`
///
/// # Arguments
/// - `robot`: The robot that has to store the content
/// - `available_space`: The available space in the backpack
/// - `content_in`: The content that has to be stored
/// - `content`: The content that has to be stored
/// - `quantity`: The quantity of the content that has to be stored
///
/// # Returns
/// A tuple containing the quantity to remove and the energy needed
pub(crate) fn can_store(
    robot: &mut impl Runnable,
    available_space: usize,
    content_in: &Content,
    content: &Content,
    quantity: usize,
) -> Result<(usize, usize), LibError> {
    let cost = content.properties().cost();
    let quantity_to_remove = min(
        min(
            available_space,
            *robot.get_backpack().contents.get(&content_in.to_default()).unwrap(),
        ),
        quantity,
    );
    // check if there is enough energy
    if !robot.get_energy().has_enough_energy(cost * quantity_to_remove) {
        return Err(LibError::NotEnoughEnergy);
    }
    Ok((quantity_to_remove, cost * quantity_to_remove))
}

/// A utility function used to keep track of the visited tiles (PLOT)
pub(crate) fn add_to_plot(plot: &Mutex<Vec<(usize, usize)>>, row: usize, col: usize) {
    if let Ok(mut plot_guard) = plot.lock() {
        if !plot_guard.contains(&(row, col)) {
            plot_guard.push((row, col));
        }
    }
}

/// A function which is used to calculate the energy requirements for the monodirectional view
///
/// # Arguments
///
/// - `robot`: The robot that is trying to see
/// - `tile_to_see`: Number of tiles to a specific direction that the robot is trying to see
///
/// # Returns
/// the calculated energy (n * 3) where n is the number of tiles to see
pub(crate) fn check_price_view(robot: &mut impl Runnable, tile_to_see: usize) -> Result<usize, LibError> {
    let energy_needed = if tile_to_see <= 1 { 0_usize } else { tile_to_see * 3 };
    if !robot.get_energy().has_enough_energy(energy_needed) {
        return Err(NotEnoughEnergy);
    }
    Ok(energy_needed)
}

/// A function which is used to calculate the cost of moving to a certain tiletype based on the
/// environmental conditions
///
/// # Arguments
///
/// - `tile_type`: The TileType of the tile the robot is trying to move to
/// - `environmental_conditions`: The `EnvironmentalConditions` struct of the world
/// - `cost`: The cost already calculated by the go interface without taking enviroment into account
///
/// # Returns
/// The calculated cost, which substitutes the one in the go interface
pub fn calculate_cost_go_with_environment(
    mut cost: usize,
    environmental_conditions: EnvironmentalConditions,
    tile_type: TileType,
) -> usize {
    let mut increment = 0.0;

    match (tile_type, environmental_conditions.get_weather_condition()) {
        | (_, WeatherType::Sunny) => {}
        | (_, WeatherType::Rainy) => increment += cost as f64 * 1.1,
        | (_, WeatherType::TropicalMonsoon) => increment += cost as f64 * 2.0,
        | (TileType::Street, WeatherType::TrentinoSnow) => increment += 1.0,
        | (TileType::Street, WeatherType::Foggy) => increment += 1.0,
        | (TileType::Hill, WeatherType::TrentinoSnow) => increment += cost as f64 * 1.6,
        | (TileType::Mountain, WeatherType::TrentinoSnow) => increment += cost as f64 * 1.7,
        | (TileType::Snow, WeatherType::TrentinoSnow) => increment += cost as f64 * 2.0,
        | (_, _) => {}
    }

    match (tile_type, environmental_conditions.get_time_of_day()) {
        | (TileType::Sand, DayTime::Afternoon) => increment += cost as f64 * 1.7,
        | (_, DayTime::Morning) => increment += cost as f64 * 1.1,
        | (_, DayTime::Afternoon) => {}
        | (_, DayTime::Night) => increment += cost as f64 * 1.4,
    }

    increment = increment.ceil();
    cost += increment as usize;
    cost
}
