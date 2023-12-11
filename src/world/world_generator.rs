use std::collections::HashMap;

use crate::utils::LibError;
use crate::utils::LibError::*;
use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::tile::TileType::Teleport;
use crate::world::tile::{Content, Tile, TileType};

// ----------------------------------------------------
// WorldGenerator

/// A trait for generating worlds.
///
/// The `WorldGenerator` trait allows the implementation of the `new` function for the given `World` struct.
///
/// # Return
/// - `Vec<Vec<Tile>>` - The generated map of tiles.
/// - `(usize, usize)` - The spawn position of the robot.
/// - `EnvironmentalConditions` - The environmental conditions of the world.
/// - `f32` - Max score of the world.
/// - `Option<HashMap<Content, f32>> - optional score_table used in score.rs. If None is provided, uses default score_table.
///
/// # Usage
///
/// ```
///
/// use std::collections::HashMap;
/// use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
/// use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
/// use robotics_lib::world::tile::{Content, Tile};
/// use robotics_lib::world::world_generator::Generator;
/// pub(crate) struct WorldGenerator {
///     pub(crate) size: usize,
///     // other attributes
/// }
///
/// impl Generator for WorldGenerator {
///     fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
///         let mut map: Vec<Vec<Tile>> = Vec::with_capacity(self.size);
///         let environmental_conditions = EnvironmentalConditions::new(&vec![Sunny, Rainy], 15, 12);
///         // implementation
///         return (map, (0, 0), EnvironmentalConditions::new(&[], 0, 0).unwrap(), 0.0, None);
///     }
/// }
///
/// ```

// to generate the Content of a tile refer to the hashmap of Contentprops for the max n of elems
// f32 at the end is the score?
pub type World = (
    Vec<Vec<Tile>>,
    (usize, usize),
    EnvironmentalConditions,
    f32,
    Option<HashMap<Content, f32>>,
);
pub trait Generator {
    fn gen(&mut self) -> World;
}

/// A function to check if the world is valid.
///
/// # Checks
/// - The world is square.
/// - The teleport in world are false.
/// - All content-enum value is lower or equal to the tiletype-enum max
/// - The content can be held by the tile
pub fn check_world(world: &Vec<Vec<Tile>>) -> Result<(), LibError> {
    for row in world {
        // Check for square world
        if world.len() != row.len() {
            return Err(WorldIsNotASquare);
        };

        for tile in row {
            // check if all the teleport are false
            if let Teleport(value) = tile.tile_type {
                if value {
                    return Err(TeleportIsTrueOnGeneration);
                }
            }
            let value = match &tile.content {
                | Content::Rock(value) => value,
                | Content::Tree(value) => value,
                | Content::Garbage(value) => value,
                | Content::Fire => &0,
                | Content::Coin(value) => value,
                | Content::Bin(value) => &value.end,
                | Content::Crate(value) => &value.end,
                | Content::Bank(value) => &value.end,
                | Content::Water(value) => value,
                | Content::Market(value) => value,
                | Content::Fish(value) => value,
                | Content::Building => &0,
                | Content::Bush(value) => value,
                | Content::JollyBlock(value) => value,
                | Content::Scarecrow => &0,
                | Content::None => &0,
            };

            //all content-enum value is lower or equal to the tiletype-enum max
            let max = &tile.content.world_generator_max();
            if value > max {
                return Err(ContentValueIsHigherThanMax);
            }

            //check if the content can be held by the tile
            if !tile.tile_type.properties().can_hold(&tile.content.to_default()) {
                return Err(ContentNotAllowedOnTile);
            }
        }
    }
    Ok(())
}

/// A function to get the percentage of each TileType in the world.
pub fn get_tiletype_percentage(world: &Vec<Vec<Tile>>) -> HashMap<TileType, f64> {
    let mut percentage_map = HashMap::new();
    let total = (world.len() * world[0].len()) as f64;
    for row in world {
        for tile in row {
            *percentage_map.entry(tile.tile_type).or_insert(0.0) += 1.0;
        }
    }
    for element in percentage_map.iter_mut() {
        *element.1 /= total;
    }
    percentage_map
}

/// A function to get the percentage of each Content in the world.
pub fn get_content_percentage(world: &Vec<Vec<Tile>>) -> HashMap<Content, f64> {
    let mut percentage_map = HashMap::new();
    let total = (world.len() * world[0].len()) as f64;
    for row in world {
        for tile in row {
            *percentage_map.entry(tile.content.to_default()).or_insert(0.0) += 1.0;
        }
    }
    for element in percentage_map.iter_mut() {
        *element.1 /= total;
    }
    percentage_map
}
