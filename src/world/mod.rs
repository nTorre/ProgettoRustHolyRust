use std::collections::HashMap;

use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::score::ScoreCounter;
use crate::world::tile::{Content, Tile};

pub mod coordinates;
pub mod tile;
pub mod world_generator;

pub mod environmental_conditions;
pub mod score;

/// Represents the game world.
///
/// The `World` struct is used to define the game world, which includes a map made up of `Tile`
/// instances, the dimension of the map, how many tiles are still discoverable, the enviromental
/// conditions of the world and the score counter.
///
/// # Usage
///
/// ```rust
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// use robotics_lib::world::world_generator::Generator;
/// fn create_world(generator: &mut impl Generator) {
///     // let world = World::new(generator.generate_world());
///     // give the world to functions that need it
/// }
/// ```
///
/// # Fields
/// - `map`: A 2D vector representing the map made up of `Tile` instances.
/// - `dimension`: The dimension of the map (e.g., the side length of a square map).
/// - `discoverable`: The number of discoverable tiles left (default: 30% of the world's dimension)
/// - `environmental_conditions`: The environmental conditions of the world (daytime and weather).
/// - `score_counter`: ScoreCounter struct keeping track of Robot's score.
#[derive(Debug)]
pub struct World {
    pub(crate) map: Vec<Vec<Tile>>,
    pub(crate) dimension: usize,
    pub(crate) discoverable: usize,
    pub(crate) environmental_conditions: EnvironmentalConditions,
    pub(crate) score_counter: ScoreCounter,
}

impl World {
    pub(crate) fn new(
        map: Vec<Vec<Tile>>,
        environmental_conditions: EnvironmentalConditions,
        max_score: f32,
        score_table: Option<HashMap<Content, f32>>,
    ) -> World {
        let dimension = map.len();
        let score_counter = ScoreCounter::new(max_score, &map, score_table);
        World {
            map,
            dimension,
            discoverable: (dimension.pow(2) / 10 + 1) * 3,
            environmental_conditions,
            score_counter,
        }
    }

    /// # Returns
    /// `true` if the day changed, `false` otherwise
    pub(crate) fn advance_time(&mut self) -> bool {
        self.environmental_conditions.tick()
    }

    /// # Returns
    /// 'usize' containing the number of discoverable tiles left
    pub fn get_discoverable(&mut self) -> usize {
        self.discoverable
    }
}
