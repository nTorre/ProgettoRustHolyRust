use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;
use std::sync::Mutex;

use lazy_static::lazy_static;
use rand::Rng;
use strum_macros::EnumIter;

use crate::event::events::Event::Moved;
use crate::event::events::Event::{EnergyConsumed, TileContentUpdated};
use crate::runner::Runnable;
use crate::utils::LibError::*;
use crate::utils::*;
use crate::world::coordinates::Coordinate;
use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::tile::TileType::{DeepWater, ShallowWater, Teleport};
use crate::world::tile::{Content, Tile, TileType};
use crate::world::World;

/// Represents the tools
/// The `tools` trait is used to define the tools.
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::tools;
/// ```
///
/// # Example
/// ```rust
/// use robotics_lib::interface::tools;
///
/// struct Tool;
/// impl tools for Tool {};
/// ```

pub trait Tools {}

/// Direction enum
/// Given to the right functions will move the robot in the given direction, remove the content of a tile
/// and more.
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::Direction;
/// let direction_up= Direction::Up;
/// ```
///
/// # Variants
/// - `Up`: Move the robot up
/// - `Down`: Move the robot down
/// - `Left`: Move the robot left
/// - `Right`: Move the robot right
#[derive(Debug, Clone, Eq, PartialEq, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

lazy_static! {
/// List of coordinates that the robot has seen so far
    static ref PLOT: Mutex<Vec<(usize, usize)>> = Mutex::new(vec![]);
}

/// Represents the world made of ```rust Vec<Vec<Option<Tile>>> ``` and the coordinates of type ```rust (usize, usize)```
type TileMatrix = (Vec<Vec<Option<Tile>>>, (usize, usize));

/// Given the robot, the world and the direction, will move the robot in the given direction. If it moves itself to a teleport tile, it will be activated
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::go;
/// ```
///
/// # Arguments
/// - `robot`: The robot that will be moved
/// - `world`: The world in which the robot is
/// - `direction`: The direction in which the robot will be moved
///
/// # Returns
/// - `Ok`: The view of the robot from is new position and the new position
/// - `Err`: The robot couldn't be moved
///
/// # Errors:
/// - `NoTileTypeProps`: The TileTypeProp of the target cell is not set properly
/// - `OutOfBounds`: The robot couldn't be moved cause it's on the border an the chosen direction is out of bounds
/// - `CannotWalk`: The robot cannot walk on the desired tiletype
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to move in the desired direction
///
/// # Examples
/// ```rust
/// use robotics_lib::interface::{Direction, go};
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::utils::LibError;
/// use robotics_lib::world::World;
///
/// fn go_example(mut world: &mut World, mut robot: &mut impl Runnable, direction: Direction)-> Result<(), LibError> {
///     let updated_view = match go(robot, world, direction) {
///         Ok((view, _)) => view,
///         Err(e) => { return Err(e); }
///     };
///     for row in updated_view.iter(){
///         for col in row.iter() {
///             match col {
///                 | None => print!("default_unknown_tile"),
///                 | Some(tile) => print!("{:?}", tile.content),
///             }
///         }
///     }
///     Ok(())
/// }
/// ```
pub fn go(robot: &mut impl Runnable, world: &mut World, direction: Direction) -> Result<TileMatrix, LibError> {
    go_allowed(robot, world, &direction)?;

    let (row, col) = get_coords_row_col(robot, &direction);

    // Get tiles
    let target_tile = &world.map[row][col];
    let current_tile = &world.map[robot.get_coordinate().get_row()][robot.get_coordinate().get_col()];

    // Init costs
    let mut base_cost = target_tile.tile_type.properties().cost();
    let mut elevation_cost = 0;

    // Get informations that influence the cost
    let environmental_conditions = look_at_sky(world);
    let new_elevation = target_tile.elevation;
    let current_elevation = current_tile.elevation;

    // Calculate cost
    base_cost = calculate_cost_go_with_environment(base_cost, environmental_conditions, target_tile.tile_type);
    // Consider elevation cost only if we are going from a lower tile to a higher tile
    if new_elevation > current_elevation {
        elevation_cost = (new_elevation - current_elevation).pow(2);
    }

    // Update teleport tile informations
    if world.map[row][col].tile_type == Teleport(false) {
        world.map[row][col].tile_type = Teleport(true);
    }

    // Consume energy and then move
    robot.get_energy_mut().consume_energy(base_cost + elevation_cost)?;
    *robot.get_coordinate_mut() = Coordinate::new(row, col);

    // Fire events
    robot.handle_event(EnergyConsumed(base_cost + elevation_cost));
    robot.handle_event(Moved(world.map[row][col].clone(), (row, col)));
    Ok(where_am_i(robot, world))
}

/// Given the robot, the world and the coordinate of a teleport tile, will move the robot in the given tile
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::teleport;
/// ```
///
/// # Arguments
/// - `robot`: The robot that will be teleported
/// - `world`: The world in which the robot is
/// - `coordinates`: The coordinate in which the robot will be teleported
///
/// # Returns
/// - `Ok`: The view of the robot from is new position and the new position
/// - `Err`: The robot couldn't be moved
///
/// # Errors:
/// - `OperationNotAllowed`: The robot isn't in a teleport tile or it's trying to teleport itself in a tile which isn't a teleport tile too
/// - `OutOfBounds`: The robot couldn't be teleported because the coordinate given is out of bound
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to use the teleport
pub fn teleport(
    robot: &mut impl Runnable,
    world: &mut World,
    coordinates: (usize, usize),
) -> Result<TileMatrix, LibError> {
    const TELEPORT_COST: usize = 30;
    let coordinate = Coordinate::new(coordinates.0, coordinates.1);
    match teleport_allowed(robot, world, &coordinate) {
        | Ok(_) => {
            robot.get_energy_mut().consume_energy(TELEPORT_COST)?;
            robot.handle_event(EnergyConsumed(TELEPORT_COST));
            *robot.get_coordinate_mut() = coordinate;
            Ok(where_am_i(robot, world))
        }
        | Err(e) => Err(e),
    }
}

/// Given the robot, the world and the direction, will destroy the content of the tile in the given direction
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::destroy;
/// ```
///
/// # Arguments
/// - `robot`: The robot that will be moved
/// - `world`: The world in which the robot is
/// - `direction`: The direction in which will be destroyed the content
///
/// # Returns
/// - `Ok`: The content that was destroyed and the quantity of the content that was destroyed
/// - `Err`: The content couldn't be destroyed
///
/// # Errors
/// - `OutOfBounds`: The content couldn't be destroyed
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to destroy the content
/// - `NoContentProp`: The content doesn't have a property
/// - `NoContent`: The content doesn't exist
/// - `NotEnoughSpacei32)`: The backpack of the robot doesn't have enough space to store the content
///
/// # Examples
/// ```rust
/// use robotics_lib::interface::{destroy, Direction};
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::tile::Content;
/// use robotics_lib::world::World;
/// fn destroy_example(mut world: &mut World, mut robot: &mut impl Runnable, direction: Direction) {
///     match destroy(robot, world, direction){
///         Ok(quantity) => {
///             print!("{:?} quantity of the content has been added to your backpack", quantity);
///         }
///         Err(e) => {
///             print!("{:?}", e);}
///     }
/// }
/// ```
///
/// # Remarks
/// - The content that was destroyed is returned
/// - The quantity of the content that was destroyed is returned
/// - The content that was destroyed is removed from the world
/// - Destroying a content will add it to the backpack of the robot
/// - If the content quantity is more than the free space in the backpack, the content will be added to the backpack until it's full
pub fn destroy(robot: &mut impl Runnable, world: &mut World, direction: Direction) -> Result<usize, LibError> {
    let mut rng = rand::thread_rng();

    in_bounds(robot, world, &direction)?;

    let (target_row, target_col) = get_coords_row_col(robot, &direction);

    let tiletype = &world.map[target_row][target_col].tile_type;
    let mut content = &world.map[target_row][target_col].content;

    let mut value = 0;
    let mut cost = content.properties().cost();
    let water = &Content::Water(0);

    if [ShallowWater, DeepWater].contains(tiletype) && *content == Content::None {
        value = rng.gen_range(0..water.properties().max());
        cost = water.properties().cost();
        content = water;
    }

    if *content == Content::Fire {
        value = content.properties().max();
    }

    if ![ShallowWater, DeepWater].contains(tiletype) && !can_destroy(world, (target_row, target_col))? {
        return Err(CannotDestroy);
    }

    if value == 0 {
        value = content.get_value().0.unwrap();
    }

    if robot.get_energy().has_enough_energy(cost) {
        let amt = add_to_backpack(robot, content.to_default(), value)?;
        world.score_counter.add_score_destroy(&content.to_default(), amt);

        robot.get_energy_mut().consume_energy(cost)?;
        robot.handle_event(EnergyConsumed(cost));

        world.map[target_row][target_col].content = Content::None;
        robot.handle_event(TileContentUpdated(
            world.map[target_row][target_col].clone(),
            (target_row, target_col),
        ));

        Ok(amt)
    } else {
        Err(NotEnoughEnergy)
    }
}

/// Given the world, will try to put a content from the robot backpack into a target tile
///
/// # Arguments
/// - `robot`: a mutable reference to the robot (needed to update the backpack)
/// - `world`: a mutable reference to the world (needed to update the map)
/// - `content_in`: the type content that the robot wants to put
/// - `quantity`: the amount of content that the robot wants to put
/// - `direction`: the direction in which the robot wants to put the content (to identify the target tile)
///
/// # Returns
/// A Result containing a `LibError` or the actually quantity robot has put
///
/// # Errors
/// - `WrongContentUsed`: The content provided by the robot doesn't match with the target tile or it is None
/// - `OutOfBounds`: The target tile is outside the map
/// - `NotEnoughEnergy`: The robot hasn't enough energy to put the content in the target tile
/// - `OperationNotAllowed`: the operation is not allowed
///
/// # Usage
///
///```rust
/// use robotics_lib::energy::Energy;
/// use robotics_lib::event::events::Event;
/// use robotics_lib::interface::{Direction, put};
/// use robotics_lib::runner::{Robot, Runnable};
/// use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
/// use robotics_lib::world::tile::{Content, Tile};
/// use robotics_lib::world::World;
/// use robotics_lib::world::world_generator::Generator;
/// use robotics_lib::world::coordinates::Coordinate;
/// use robotics_lib::runner::backpack::BackPack;
///
/// struct MyRobot(Robot);
/// let mut  robot = MyRobot(Robot::new());
/// impl Runnable for MyRobot { //dummy implementation
///    fn process_tick(&mut self, world: &mut World) {
///        let result = put(self, world, Content::Coin(0), 1, Direction::Up);
///    }
///    fn handle_event(&mut self, event: Event) {}
///    fn get_energy(&self) -> &Energy {&self.0.energy}
///    fn get_energy_mut(&mut self) -> &mut Energy {&mut self.0.energy}
///    fn get_coordinate(&self) -> &Coordinate {&self.0.coordinate}
///    fn get_coordinate_mut(&mut self) -> &mut Coordinate {&mut self.0.coordinate}
///    fn get_backpack(&self) -> &BackPack {&self.0.backpack}
///    fn get_backpack_mut(&mut self) -> &mut BackPack {&mut self.0.backpack}
///
/// }
/// ```
///
/// # Behaviour
/// The put interface will :
/// - checks and the about the content (in input)
/// - check if the target tile is inside the map
/// - get the actual content the robot can put (min between the quantity and the content in the backpack)
/// - try to put as much content as it can
/// - charge the robot however much it was able to insert (unit cost logic).
///
/// We have different behaviours depending on the content in input, the content of the target tile and the tile type of the target tile:
///
/// 1. Coin into Bank:
///
///     If the target tile contains a Bank with a range of coin storage, and the content to be put is a Coin, it checks if the robot can store the coins in the bank. If so, it updates the bank content and consumes the required energy.
///<br/><br/>
/// 2. Garbage into Bin:
///
///     Similar to the first case, but for Garbage into a Bin.
///<br/><br/>
/// 3. Tree into Crate:
///
///     Similar to the first case, but for Tree into a Crate.
///<br/><br/>
/// 4. Fire into Tree or Empty Tile:
///
///     If the target tile contains a Tree or is empty, and the content to be put is Fire, it replaces the existing content with Fire.
///<br/><br/>
/// 5. Extinguishing Fire with Water:
///
///     If the target tile contains Fire and the content to be put is Water, it extinguishes the fire.
///<br/><br/>
/// 6. Rock into Grass, Hill, Sand and Snow Tiles:
///
///     If the target tile is grass, hill, sand or snow, and the content to be put is Rock, it replaces the tile type with Street, keeping the content
///     if it is allowed on both the previous tile and on Streets, failing to create a Street otherwise
///<br/><br/>
/// 7. Rock into ShallowWater Tile:
///
///     similar to the eighth case, but costs more rocks
///<br/><br>
/// 8. Rock into DeepWater Tile:
///
///     Similar to the eighth case, but costs more rocks and energy
///<br/><br/>
/// 9. Rock into Lava Tile:
///
///     Similar to the eighth case, but costs more rocks and energy
///<br/><br/>
/// 10. Empty content into Mountain Tile:
///
///     If the target tile is Mountain, and the content to be but is None, it replaces the tile with Street, and gives the robot rocks.
///     Its cost is relative to the number of rocks received
///<br/><br/>
/// 11. Content a" into Tile containing Market
///
///     If the target tile contains a Market and the market still allows for operations it converts the content given into coins for the robot, if the content provided has a value of 0 coins then it returns a `WrongContentUsed` error
/// <br/><br/>
/// 12. "Content a" into Empty Tile:
///
///     If the target tile contains no content, it checks if the tile can hold the "Content a". If so, it updates the tile content to contain "Content a" and consumes the required energy.
///<br/><br/>
/// 13. "Content a" into a Tile containing "Content a" where "Content a" has a (usize) and is not Market:
///
///     If the target tile contains the same content type as what the caller wants to put in and the content type allows for multiple contents of the same type on the same tile, it tries to fill the tile with the amount given to the function stopping before if the max value is reached
///<br/><br/>
/// 14. Default Case:
///
///     If none of the specified patterns match, it returns an error indicating that the operation is not allowed.
///<br/><br/>
/// # Note
/// If the robot doesn’t have enough energy he must call the interface again with a lower amount to complete the put operation.
pub fn put(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
) -> Result<usize, LibError> {
    // check if the target tile is inside the map
    in_bounds(robot, world, &direction)?;
    let (target_row, target_col) = get_coords_row_col(robot, &direction);
    // check if the provided content is not None and the target TileType is mountain
    if content_in == Content::None && world.map[target_row][target_col].tile_type != TileType::Mountain {
        return Err(WrongContentUsed);
    }
    let amount = min(
        min(
            quantity,
            *robot
                .get_backpack()
                .contents
                .get(&content_in.to_default())
                .unwrap_or(&0),
        ),
        content_in.properties().max(),
    );
    let input = (
        &world.map[target_row][target_col].tile_type,
        &world.map[target_row][target_col].content,
        &content_in,
    );
    let put_result = match input {
        | (_, Content::Bank(range), Content::Coin(_)) => {
            let (quantity_to_remove, cost) = can_store(
                robot,
                range.end - range.start,
                &input.2.to_default(),
                &input.1.to_default(),
                quantity,
            )?;
            let removed_quantity = remove_from_backpack(robot, &content_in.to_default(), quantity_to_remove)?;
            world.map[target_row][target_col].content = Content::Bank((range.start + removed_quantity)..range.end);
            robot.get_energy_mut().consume_energy(cost)?;
            robot.handle_event(EnergyConsumed(cost));
            world
                .score_counter
                .add_score_put(&Content::Bank(Range::default()), removed_quantity); // Adds score
            Ok(removed_quantity)
        }
        | (_, Content::Bin(range), Content::Garbage(_)) => {
            let (quantity_to_remove, cost) = can_store(
                robot,
                range.end - range.start,
                &input.2.to_default(),
                &input.1.to_default(),
                quantity,
            )?;
            let removed_quantity = remove_from_backpack(robot, &content_in.to_default(), quantity_to_remove)?;
            world.map[target_row][target_col].content = Content::Bin((range.start + removed_quantity)..range.end);
            robot.get_energy_mut().consume_energy(cost)?;
            robot.handle_event(EnergyConsumed(cost));
            world
                .score_counter
                .add_score_put(&Content::Bin(Range::default()), removed_quantity); // Adds score
            Ok(removed_quantity)
        }
        | (_, Content::Crate(range), Content::Tree(_)) => {
            let (quantity_to_remove, cost) = can_store(
                robot,
                range.end - range.start,
                &input.2.to_default(),
                &input.1.to_default(),
                quantity,
            )?;
            let removed_quantity = remove_from_backpack(robot, &content_in.to_default(), quantity_to_remove)?;
            world.map[target_row][target_col].content = Content::Crate((range.start + removed_quantity)..range.end);
            robot.get_energy_mut().consume_energy(cost)?;
            robot.handle_event(EnergyConsumed(cost));
            world
                .score_counter
                .add_score_put(&Content::Crate(Range::default()), removed_quantity); // Adds score
            Ok(removed_quantity)
        }
        | (_, Content::Tree(_), Content::Fire) | (_, Content::None, Content::Fire) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            // this was always going to be zero as amount would be = 0 as is the max of fire
            // let cost = input.2.properties().cost() * amount;
            // i changed to this if it was intended maybe change the cost of fire?
            let cost = input.2.properties().cost();
            println!("{} with cost {cost} and {amount}", input.2);
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 1)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].content = input.2.to_default();
                Ok(removed_quantity)
            }
        }
        | (_, Content::Market(remaining_op), to_sell) => {
            if *remaining_op < 1 {
                return Err(OperationNotAllowed);
            }
            world.map[target_row][target_col].content = Content::Market(*remaining_op - 1);
            let items_sold = remove_from_backpack(robot, to_sell, quantity)?;
            let coins = match to_sell {
                | Content::Rock(_) => 1,
                | Content::Tree(_) => 2,
                | Content::Fish(_) => 5,
                | _ => 0,
            };
            //returns total amount of coins gained
            if coins == 0 {
                return Err(WrongContentUsed);
            }

            Ok(add_to_backpack(robot, Content::Coin(0), items_sold * coins)?)
        }
        | (TileType::Grass | TileType::Hill | TileType::Sand | TileType::Snow, _, Content::Rock(_)) => {
            // cost = content_cost (rock_cost) * amount (1) * inherit multiplier (1)
            let cost = input.2.properties().cost();
            if !TileType::Street.properties().can_hold(input.1) {
                return Err(LibError::MustDestroyContentFirst);
            }
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 1)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].tile_type = TileType::Street;
                Ok(removed_quantity)
            }
        }
        | (TileType::ShallowWater, _, Content::Rock(_)) => {
            // cost = content_cost (rock_cost) * amount (2) * inherit multiplier (1)
            let cost = input.2.properties().cost() * 2;
            if !TileType::Street.properties().can_hold(input.1) {
                return Err(LibError::MustDestroyContentFirst);
            }
            // amount of material to expend (2)
            if amount < 2 {
                return Err(NotEnoughContentProvided);
            }
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 2)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].tile_type = TileType::Street;
                Ok(removed_quantity)
            }
        }
        | (TileType::DeepWater, _, Content::Rock(_)) => {
            // cost = content_cost (rock_cost) * amount (3) * inherit multiplier (2)
            let cost = input.2.properties().cost() * 3 * 2;
            if !TileType::Street.properties().can_hold(input.1) {
                return Err(LibError::MustDestroyContentFirst);
            }
            // amount of material to expend (3)
            if amount < 3 {
                return Err(NotEnoughContentProvided);
            }
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 3)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].tile_type = TileType::Street;
                Ok(removed_quantity)
            }
        }
        | (TileType::Lava, _, Content::Rock(_)) => {
            // cost = content_cost (rock_cost) * amount (3) * inherit multiplier (3)
            let cost = input.2.properties().cost() * 3 * 3;
            // amount of material to expend (3)
            if amount < 3 {
                return Err(NotEnoughContentProvided);
            }
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 3)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].tile_type = TileType::Street;
                Ok(removed_quantity)
            }
        }
        | (_, Content::None, Content::Rock(_)) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].content = Content::Rock(amount);
                Ok(removed_quantity)
            }
        }
        // Digging through mountain to make a street
        | (TileType::Mountain, _, Content::None) => {
            let mut rng = rand::thread_rng();
            let amount_to_give = rng.gen_range(1..Content::Rock(0).properties().max());
            let cost = Content::Rock(0).properties().cost() * amount_to_give * 4;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let added_quantity = add_to_backpack(robot, Content::Rock(0), amount_to_give)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].tile_type = TileType::Street;
                Ok(added_quantity)
            }
        }
        | (_, Content::Fire, Content::Water(_)) => {
            let cost = input.2.properties().cost();
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 1)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].content = Content::None;
                world.score_counter.add_score_put(&Content::Fire, removed_quantity); // Adds score
                Ok(removed_quantity)
            }
        }
        | (_, Content::Fire, _) => {
            let cost = Content::Water(0).properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                Ok(removed_quantity)
            }
        }
        | (_, Content::None, _) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].content = input.2.to_value(amount);
                Ok(removed_quantity)
            }
        }
        | (_, a, b) => {
            if a.to_default() != b.to_default() {
                return Err(LibError::WrongContentUsed);
            }
            let (value, _range) = a.get_value();
            if value.is_none() {
                return Err(LibError::OperationNotAllowed);
            }
            let amount = min(amount, a.properties().max() - value.unwrap());
            if amount == 0 {
                return Err(LibError::OperationNotAllowed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                robot.handle_event(EnergyConsumed(cost));
                world.map[target_row][target_col].content = input.2.to_value(amount + value.unwrap());
                Ok(removed_quantity)
            }
        }
    };

    if put_result.is_ok() {
        robot.handle_event(TileContentUpdated(
            world.map[target_row][target_col].clone(),
            (target_row, target_col),
        ))
    }

    put_result
}

/// Given the world, will return the environmental conditions
/// It's used to see the weather conditions and the time of day
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::look_at_sky;
/// // let environmental_conditions = look_at_sky(world);
/// ```
///
/// # Arguments
/// - `world`: The world in which the robot is
///
/// # Returns
/// - `EnvironmentalConditions`: The environmental conditions struct, which you can use as you wish
pub fn look_at_sky(world: &World) -> EnvironmentalConditions {
    world.environmental_conditions.clone()
}

/// Given the world, will return the map, the dimension and the position of the robot
/// It's used for debug purposed
pub fn debug(robot: &impl Runnable, world: &mut World) -> (Vec<Vec<Tile>>, usize, (usize, usize)) {
    (
        world.map.clone(),
        world.dimension,
        (robot.get_coordinate().get_row(), robot.get_coordinate().get_col()),
    )
}

/// Given the world, will return the map of the robot
/// It's used as private map for the robot
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::robot_map;
/// ```
///
/// # Arguments
/// - `world`: The world in which the robot is
///
/// # Returns
/// - `Vec<Vec<Option<Tile>>>`: The map of the robot
///
/// # Examples
/// ```rust
/// use robotics_lib::interface::robot_map;
/// use robotics_lib::world::World;
///
/// fn robot_world_example(mut world: &mut World) {
///     let robot_world=robot_map(&mut world).unwrap();
///     for row in robot_world.iter() {
///         for elem in row.iter() {
///             match elem {
///                 None => println!("No tile"),
///                 Some(tile) => println!("{:?}", tile),
///             }
///         }
///     }
/// }
///
/// ```
///
/// # Remarks
/// - The map of the robot is returned
/// - The map of the robot is a matrix of `Option<Tile>`
pub fn robot_map(world: &World) -> Option<Vec<Vec<Option<Tile>>>> {
    let mut out: Vec<Vec<Option<Tile>>> = vec![vec![None; world.dimension]; world.dimension];
    if let Ok(plot_guard) = PLOT.lock() {
        for (x, y) in plot_guard.iter() {
            out[*x][*y] = Some(world.map[*x][*y].clone());
        }
        Some(out)
    } else {
        None
    }
}

/// Given the world, will return the area around the robot
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::robot_view;
/// ```
///
/// # Arguments
/// - `world`: The world in which the robot is
///
/// # Returns
/// - `Vec<Vec<Option<Tile>>>`: The area around the robot (3x3)
///
/// # Examples
/// ```rust
/// use robotics_lib::interface::robot_view;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// fn robot_view_example(robot: &impl Runnable, mut world: &mut World) {
///     let robot_view=robot_view(robot, &mut world);
///     for row in robot_view.iter() {
///         for col in row.iter() {
///             match col {
///                 | None => print!("default_unknown_tile"),
///                 | Some(tile) => print!("{:?}", tile.tile_type),
///            }
///         }
///     }
/// }
/// ```
///
/// # Remarks
/// - The area around the robot is returned
/// - The area around the robot is a matrix of `Option<Tile>`
/// - The area around the robot is a 3x3 matrix
/// - The area around the robot is centered on the robot
pub fn robot_view(robot: &impl Runnable, world: &World) -> Vec<Vec<Option<Tile>>> {
    let mut tmp: [[bool; 3]; 3] = [[false; 3]; 3];
    let mut out: Vec<Vec<Option<Tile>>> = vec![vec![None; 3]; 3]; //Matrix of Option <Tile>
    let (robot_row, robot_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());

    if robot_row == 0 {
        tmp[0][0] = true;
        tmp[0][1] = true;
        tmp[0][2] = true;
        out[0][0] = None;
        out[0][1] = None;
        out[0][2] = None;
    }
    if robot_col == 0 {
        tmp[0][0] = true;
        tmp[1][0] = true;
        tmp[2][0] = true;
        out[0][0] = None;
        out[1][0] = None;
        out[2][0] = None;
    }
    if robot_row == world.dimension - 1 {
        tmp[2][0] = true;
        tmp[2][1] = true;
        tmp[2][2] = true;
        out[2][0] = None;
        out[2][1] = None;
        out[2][2] = None;
    }
    if robot_col == world.dimension - 1 {
        tmp[0][2] = true;
        tmp[1][2] = true;
        tmp[2][2] = true;
        out[0][2] = None;
        out[1][2] = None;
        out[2][2] = None;
    }

    tmp.iter().enumerate().for_each(|(i, vector)| {
        vector.iter().enumerate().for_each(|(j, elem)| {
            if !elem {
                let row = robot_row + i - 1;
                let col = robot_col + j - 1;
                out[i][j] = Some(world.map[row][col].clone());

                // add to plot
                add_to_plot(&PLOT, row, col);
            }
        })
    });
    out
}

///Given the: robot, world, direction and distance will return a 3xdirection matrix of Tile
///
/// #Usage
/// ```rust
/// use robotics_lib::interface::one_direction_view;
/// ```
///
/// #Arguments
/// - `robot`: The robot
/// - `world`: The world in which the robot is
/// - `direction`: The direction in which the robot is looking
/// - `distance`: The distance in which the robot is looking
///
/// #Returns
/// - `Vec<Vec<Tile>>`: The tile next to the robot in the given direction and distance
/// - `LibError`: The error that occurred
///
/// #Errors
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to see the tile
///
/// #Examples
///```rust
/// use robotics_lib::interface::one_direction_view;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// use robotics_lib::world::tile::Tile;
/// use robotics_lib::interface::Direction;
/// fn one_direction_view_example(robot: &mut impl Runnable, mut world: &mut World) {
///    let one_direction_view=one_direction_view(robot, &mut world, Direction::Up, 5);
///    match one_direction_view {
///        Ok(up_view) => {
///            println!("{:?}", up_view);
///            println!("The view above is amazing! I can see {} tiles", up_view.len());
///        }
///        Err(e) => {
///            println!("Oh no! Something went wrong how sad:{:?}", e);
///        }
///    }
/// }
/// ```
///
/// #Remarks
/// - The tile next to the robot in the given direction and distance is returned as a matrix of `Tile`
/// - If the distance is greater than the dimension of the map, the matrix will be smaller than the distance
/// - This interface won't consume energy if the distance is 1 otherwise will consume distance*3+1 energy
///
/// #Note
/// You cannot se the tile under the robot with this interface
pub fn one_direction_view(
    robot: &mut impl Runnable,
    world: &World,
    direction: Direction,
    distance: usize,
) -> Result<Vec<Vec<Tile>>, LibError> {
    let (robot_row, robot_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
    let map_dimension = world.dimension;
    let mut out: Vec<Vec<Tile>> = Vec::new();
    match direction {
        | Direction::Up => {
            let tile_to_see = min(distance, robot_row);
            if tile_to_see == 0 {
                return Ok(out);
            }
            let energy_needed = check_price_view(robot, tile_to_see)?;
            let start_index_col: isize = if robot_col == 0 { 0 } else { -1 };
            let ending_index_col = if robot_col == map_dimension - 1 { 0 } else { 1 };
            for i in 1..=tile_to_see {
                let mut row_vec = Vec::new();
                for j in start_index_col..=ending_index_col {
                    let row = robot_row - i;
                    let col = (robot_col as isize + j) as usize;
                    row_vec.push(world.map[row][col].clone());
                    add_to_plot(&PLOT, row, col);
                }
                out.push(row_vec);
            }
            robot.get_energy_mut().consume_energy(energy_needed)?;
            robot.handle_event(EnergyConsumed(energy_needed));
            Ok(out)
        }
        | Direction::Down => {
            let tile_to_see = min(distance, map_dimension - robot_row - 1);
            if tile_to_see == 0 {
                return Ok(out);
            }
            let energy_needed = check_price_view(robot, tile_to_see)?;
            let start_index_col: isize = if robot_col == 0 { 0 } else { -1 };
            let ending_index_col = if robot_col == map_dimension - 1 { 0 } else { 1 };
            for i in 1..=tile_to_see {
                let mut row_vec = Vec::new();
                for j in start_index_col..=ending_index_col {
                    let row = robot_row + i;
                    let col = (robot_col as isize + j) as usize;
                    row_vec.push(world.map[row][col].clone());
                    add_to_plot(&PLOT, row, col);
                }
                out.push(row_vec);
            }
            robot.get_energy_mut().consume_energy(energy_needed)?;
            robot.handle_event(EnergyConsumed(energy_needed));
            Ok(out)
        }
        | Direction::Left => {
            let tile_to_see = min(distance, robot_col);
            if tile_to_see == 0 {
                return Ok(out);
            }
            let energy_needed = check_price_view(robot, tile_to_see)?;
            let start_index_row: isize = if robot_row == 0 { 0 } else { -1 };
            let ending_index_row = if robot_row == map_dimension - 1 { 0 } else { 1 };
            for i in start_index_row..=ending_index_row {
                let mut row_vec = Vec::new();
                for j in 1..=tile_to_see {
                    let row = (robot_row as isize + i) as usize;
                    let col = robot_col - j;
                    row_vec.push(world.map[row][col].clone());
                    add_to_plot(&PLOT, row, col);
                }
                out.push(row_vec);
            }
            robot.get_energy_mut().consume_energy(energy_needed)?;
            robot.handle_event(EnergyConsumed(energy_needed));
            Ok(out)
        }
        | Direction::Right => {
            let tile_to_see = min(distance, map_dimension - robot_col - 1);
            if tile_to_see == 0 {
                return Ok(out);
            }
            let energy_needed = check_price_view(robot, tile_to_see)?;
            let start_index_row: isize = if robot_row == 0 { 0 } else { -1 };
            let ending_index_row = if robot_row == map_dimension - 1 { 0 } else { 1 };
            for i in start_index_row..=ending_index_row {
                let mut row_vec = Vec::new();
                for j in 1..=tile_to_see {
                    let row = (robot_row as isize + i) as usize;
                    let col = robot_col + j;
                    row_vec.push(world.map[row][col].clone());
                    add_to_plot(&PLOT, row, col);
                }
                out.push(row_vec);
            }
            robot.get_energy_mut().consume_energy(energy_needed)?;
            robot.handle_event(EnergyConsumed(energy_needed));
            Ok(out)
        }
    }
}

/// Given the world, will return the area around the robot as a matrix of `Option<Tile>` with the position of the robot
///
/// # Usage
/// ```
/// use robotics_lib::interface::where_am_i;
/// ```
///
/// # Arguments
/// - `world`: The world in which the robot is
/// - `robot`: The robot that is moving around the map
///
/// # Returns
/// - `Vec<Vec<Option<Tile>>>`: The area around the robot (3x3)
/// - `Coordinate`: The position of the robot
///
/// # Examples
/// ```
/// use robotics_lib::interface::where_am_i;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// fn where_am_i_example(robot: &impl Runnable, mut world: &mut World) {
///    let (robot_view, robot_position)=where_am_i(robot, &mut world);
///     for row in robot_view.iter() {
///         for col in row.iter() {
///             match col {
///                 | None => print!("default_unknown_tile"),
///                 | Some(tile) => print!("{:?}", tile.tile_type),
///             }
///         }
///     }
/// }
/// ```
///
pub fn where_am_i(robot: &impl Runnable, world: &World) -> (Vec<Vec<Option<Tile>>>, (usize, usize)) {
    (
        robot_view(robot, world),
        (robot.get_coordinate().get_row(), robot.get_coordinate().get_col()),
    )
}

/// Given the world, will return the amount of score received by the robot.
///
/// # Usage
/// ```
/// use robotics_lib::interface::get_score;
/// ```
///
/// # Arguments
/// - `world`: Targeted world
///
/// # Returns
/// - `f32`: Received score
pub fn get_score(world: &World) -> f32 {
    world.score_counter.get_score()
}

/// Given a content to craft, will attempt to craft it from the contents already present in the backpack
///
/// # Usage
/// ```rust
/// use robotics_lib::interface::craft;
/// ```
/// # Arguments
/// - `robot`: The robot that is moving around the map
/// - `content`: The content you want to craft
///
/// # Returns
/// - `Result<Content, LibError>`: Returns an error if the content to craft is of type None or if there aren't enough materials to craft it
///
/// # Errors
/// - `NotCraftable`: The content can't be crafted or there aren't enough materials to do so
/// - `NotEnoughEnergy`: There isn't enough energy to complete the action
/// - `NotEnoughSpace`: There is not enough space in the backpack to add the item just crafted
///
/// # Examples
/// ```rust
/// use robotics_lib::interface::craft;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// use robotics_lib::world::tile::Content;
///
/// fn craft_example(robot: &mut impl Runnable, mut world: &mut World) {
///    match craft(robot, Content::Garbage(0)){ ///
///         Ok(content) => { println!("{} was crafted successfully", content) }
///         _ => { println!("Item not craftable") }
///     }
/// }
/// ```
///
/// # Remarks
/// - The array of craft for each content contains a possible recipe to craft said item
///     ex. Garbage(0) can be crafted with x amounts of rocks or y amount of something else
/// - The interface checks for the first available recipe (means: the one that uses content the robot has enough of in the backpack) to use to craft said items
/// - For now it's up to the robot to be smart enough to try to craft an item only when it does have enough energy to do so
pub fn craft(robot: &mut impl Runnable, content: Content) -> Result<Content, LibError> {
    match content {
        // if the content to craft given is of type Content::None
        | Content::None => Err(NotCraftable),
        | _ => {
            //get content props for the thing to craft
            for (content_n, quantity) in content.properties().craft() {
                if *quantity != 0 {
                    match remove_from_backpack(robot, content_n, *quantity) {
                        | Ok(value) => {
                            if value != *quantity {
                                // there wasn't enough material in the backpack, the ones removed will be added back
                                add_to_backpack(robot, content_n.to_default(), value)?;
                            }
                            //make the robot pay
                            let cost = content.properties().cost();
                            robot.get_energy_mut().consume_energy(cost)?;
                            robot.handle_event(EnergyConsumed(cost));
                            // there was enough contents to craft
                            add_to_backpack(robot, content.to_default(), 1)?;
                            return Ok(content);
                        }
                        | _ => {}
                    }
                }
            }
            Err(NotCraftable)
        }
    }
}

/// Given a Vec of (x, y) coordinates of the world, the function returns what those tiles are (it discovers them).
/// Discovering each tile costs 3 energy units and it is possible to discover tiles up to 30% of the world's total dimension
///
/// # Usage
/// ```
/// use robotics_lib::interface::discover_tiles;
/// ```
///
/// # Arguments
/// - `robot`: The robot that is moving around the map
/// - `world`: The world in which the robot is
/// - `to_discover`: A vec containing the list of coordinates (A tuple of 2 usize) of the tiles we want to discover
///
/// # Returns
/// - `Result<HashMap<(usize, usize), Option<Tile>>, LibError>`: Returns an error if there isn't enough energy or there aren't enough discoverable tiles left
///
/// # Errors
/// - `NotEnoughEnergy`: There isn't enough energy to complete the action
/// - `NoMoreDiscovery`: There aren0t enough discoverable tiles left to complete the action
///
/// # Examples
/// ```rust
/// use robotics_lib::interface::discover_tiles;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// fn where_discover_tiles_example(robot: &mut impl Runnable, mut world: &mut World) {
///    let to_discover : Vec<(usize, usize)> = vec![(0,0), (1,1), (2,2)]; //List of coordinates of tiles to be discovered
///    let discovered = discover_tiles(robot, world, &to_discover);
/// }
/// ```
pub fn discover_tiles(
    robot: &mut impl Runnable,
    world: &mut World,
    to_discover: &[(usize, usize)],
) -> Result<HashMap<(usize, usize), Option<Tile>>, LibError> {
    let mut return_value: HashMap<(usize, usize), Option<Tile>> = HashMap::new();
    if world.discoverable >= to_discover.len() {
        let energy_needed = to_discover.len() * 3;
        if robot.get_energy().has_enough_energy(energy_needed) {
            world.discoverable -= to_discover.len();
            robot.get_energy_mut().consume_energy(energy_needed)?;
            robot.handle_event(EnergyConsumed(energy_needed));
            for (x, y) in to_discover.iter() {
                if *x < world.map.len() && *y < world.map[*x].len() {
                    let tile = world.map[*x][*y].clone();
                    return_value.insert((*x, *y), Some(tile));
                    add_to_plot(&PLOT, *x, *y);
                } else {
                    return_value.insert((*x, *y), None);
                }
            }
        } else {
            return Err(NotEnoughEnergy);
        }
    } else {
        return Err(NoMoreDiscovery);
    }
    Ok(return_value)
}
