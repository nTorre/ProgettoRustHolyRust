use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;
use strum::IntoEnumIterator;

use crate::world::tile::Content::{
    Bank, Bin, Bush, Coin, Crate, Fish, Garbage, JollyBlock, Market, Rock, Scarecrow, Tree, Water,
};
use crate::world::tile::{Content, Tile};

// ----------------------------------------------
// ScoreCounter
///
/// A data structure that keeps track of the Robot's score. Initialized inside the `World` struct with
/// max_score field.
///
/// # Usage
///
/// use robotics_lib::world::score::ScoreCounter;
/// use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
/// use robotics_lib::world::World;
///
/// let map: Vec<Vec<Tile>> = ...;
/// let env_cond: EnvironmentalConditions = ...;
/// let max_score: f32 = 1000.
///
/// let world: World = World::new(map, environmental_conditions, max_score);
///
/// # Fields
///
/// - `score`: An Rc<RefCell<f32>> container, holding the current value of score.
/// - `max_score`: An f32 value of the max score, given during the initialization of the `World`.
/// - `score_table`: A HashMap used inside of the `ScoreCounter` to increment score field.
///
/// # Remarks
///
/// `ScoreCounter` does not operate multi-thread. In case of such usage panics might occur.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ScoreCounter {
    pub(crate) score: Rc<RefCell<f32>>,
    pub(crate) max_score: f32,
    pub(crate) score_table: HashMap<Content, f32>,
}

impl ScoreCounter {
    ///
    /// Creates a new instance of `ScoreCounter`.
    ///
    /// # Arguments
    ///
    /// - `max_score`: An f32 value showing the desired maximum score of the current map given by the user.
    /// - `map`: A &Vec<Vec<Tile>> - map of the current world, passed to the World::new() function.
    /// - `score_table`: An Option<HashMap<Content, f32>> containing a pair of a Content and a relative score_weight assigned by user
    ///
    /// # Returns
    ///
    /// A new `ScoreCounter` instance with provided properties.
    ///
    pub(crate) fn new(
        max_score: f32,
        map: &Vec<Vec<Tile>>,
        score_table: Option<HashMap<Content, f32>>,
    ) -> ScoreCounter {
        ScoreCounter {
            score: Rc::new(RefCell::new(0.)),
            max_score,
            score_table: ScoreCounter::init_score_table(map, max_score, score_table),
        }
    }

    /// Initializes a score table.
    ///
    /// # Arguments
    ///
    /// - `world`: A &Vec<Vec<Tile>> - map of the current world.
    /// - `max_score`: An f32 value showing the desired maximum score of the current map.
    ///
    /// # Returns
    ///
    /// A new HashMap<Content, f32> mapping interaction with a given `Content` to the amount of score
    /// that should be given by performing it.
    ///
    /// # Panics
    ///
    /// 1. If the given map contains no tiles (is empty).
    /// 2. If the given max_score is 0.
    ///
    // max_score = colAmount1 * (weight1 + dispWeight1) + colAmount2 * (weight2 + dispWeight2) + ...
    pub(crate) fn init_score_table(
        world: &Vec<Vec<Tile>>,
        max_score: f32,
        table: Option<HashMap<Content, f32>>,
    ) -> HashMap<Content, f32> {
        if world.is_empty() {
            panic!("The world map is empty.")
        }

        if max_score == 0. {
            panic!("Given max score is 0.")
        }

        // Calculates the amount of a given Content that can be crafted
        let craftable_amount = |content: &Content, collectables: &mut HashMap<Content, u32>| -> u32 {
            let mut craftable_amount: u32 = 0;
            for craft in content.properties().craft() {
                if craft.1 != 0 && collectables.contains_key(&craft.0) {
                    craftable_amount += collectables.get(&craft.0).unwrap() / craft.1 as u32;
                }
            }
            craftable_amount
        };

        // Subtracts the materials used for crafting from collectables HashMap
        let subtract_craft_materials = |content: &Content, amount: u32, collectables: &mut HashMap<Content, u32>| {
            let mut left_to_craft = amount as i32;
            for craft in content.properties().craft() {
                if craft.1 != 0 && collectables.contains_key(&craft.0) {
                    let crafted = *collectables.get(&craft.0).unwrap() / craft.1 as u32;
                    if crafted as i32 >= left_to_craft {
                        *collectables.get_mut(&craft.0).unwrap() -= left_to_craft as u32;
                        return;
                    } else {
                        *collectables.get_mut(&craft.0).unwrap() -= crafted;
                        left_to_craft -= crafted as i32;
                    }
                }
            }
        };

        // Initializes HashMaps to hold the amount of each Content
        let mut collectables: HashMap<Content, u32> = HashMap::new();

        // And the space needed to dispose them
        // The key is the Content that can be disposed, the value is a tuple of the Content where it can be disposed
        // and the amount of space there is to dispose it
        let mut disposables: HashMap<Content, (Content, u32)> = HashMap::new();

        // And a placeholder for the score table
        let mut score_table: HashMap<Content, f32> = HashMap::new();

        // Iterates over Content and compiles a list of dispose stations with corresponding content
        for content in Content::iter() {
            match content.properties().disposable() {
                | None => {
                    collectables.insert(content, 0);
                }
                | Some(x) => {
                    disposables.insert(x.to_default(), (content, 0));
                }
            }
        }

        // Iterates over the map, summing up the amount of each Content
        for row in world {
            for tile in row {
                if tile.content.properties().score_weight() != 0 {
                    match tile.content.clone() {
                        | Coin(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | Garbage(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | Rock(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | Tree(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | Water(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | Fish(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | Bush(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | JollyBlock(x) => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += x as u32;
                        }
                        | Scarecrow => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += 1;
                        }
                        | Crate(x) => {
                            disposables
                                .entry(tile.content.properties().disposable().clone().unwrap().to_default())
                                .or_insert((Crate(0..1).to_default(), 0))
                                .1 += (x.end - x.start) as u32;
                        }
                        | Bin(x) => {
                            disposables
                                .entry(tile.content.properties().disposable().clone().unwrap().to_default())
                                .or_insert((Bin(0..1).to_default(), 0))
                                .1 += (x.end - x.start) as u32;
                        }
                        | Bank(x) => {
                            disposables
                                .entry(tile.content.properties().disposable().clone().unwrap().to_default())
                                .or_insert((Bank(0..1).to_default(), 0))
                                .1 += (x.end - x.start) as u32;
                        }
                        | Market(x) => {
                            disposables
                                .entry(tile.content.properties().disposable().clone().unwrap().to_default())
                                .or_insert((Market(1).to_default(), 0))
                                .1 += x as u32;
                        }
                        | _other => {
                            *collectables.entry(tile.content.to_default()).or_insert(0) += 0;
                        }
                    }
                }
            }
        }

        let mut sum_score: f32 = 0.;

        match table {
            | None => {
                // Add the score for contents you can't dispose, stored in collectables HashMap
                for content in collectables.keys() {
                    sum_score +=
                        *collectables.get(content).unwrap() as f32 * content.properties().score_weight() as f32;
                }

                // Order disposables according to score_weight
                let mut disposables_ordered: Vec<(Content, (Content, u32))> = disposables.clone().into_iter().collect();
                disposables_ordered.sort_by(|x, y| {
                    y.1 .0
                        .properties()
                        .score_weight()
                        .cmp(&x.1 .0.properties().score_weight())
                });

                // Sums up the total score that could possibly be got on the map

                for content in disposables_ordered {
                    if disposables.contains_key(&content.0) {
                        // Checks if there is enough space to dispose everything in the world
                        if disposables.get(&content.0).unwrap().1 <= *collectables.get(&content.0).unwrap() {
                            // If there is, just add the score for all of it
                            sum_score += disposables.get(&content.0).unwrap().1 as f32
                                * disposables.get(&content.0).unwrap().0.properties().score_weight() as f32;
                        } else {
                            // If there isn't, check how much you gotta craft
                            let craftable_content_count = craftable_amount(&content.0, &mut collectables);
                            // Initialize a variable to hold the total amount of content
                            let total_content_count = craftable_content_count + collectables.get(&content.0).unwrap();

                            // If there is enough space to dispose everything you can craft, just add the score for all of it
                            // and subtract the materials used for crafting from collectables HashMap
                            if total_content_count < disposables.get(&content.0).unwrap().1 {
                                sum_score += total_content_count as f32
                                    * disposables.get(&content.0).unwrap().0.properties().score_weight() as f32;
                                subtract_craft_materials(&content.0, craftable_content_count, &mut collectables);
                            } else {
                                // If there isn't enough space to dispose everything you can craft, add the score for what is disposable
                                // and subtract the materials used for crafting from collectables HashMap
                                sum_score += disposables.get(&content.0).unwrap().0.properties().score_weight() as f32
                                    * disposables.get(&content.0).unwrap().1 as f32;
                                subtract_craft_materials(
                                    &content.0,
                                    disposables.get(&content.0).unwrap().1 - collectables.get(&content.0).unwrap(),
                                    &mut collectables,
                                );
                            }
                        }
                    }
                }

                // Initializes and calculates the multiplication factor needed to normalize the score
                let score_mult = max_score / sum_score;

                // Compiles a score table, containing each content and the amount of score you get from an
                // interaction with it
                for content in collectables.keys() {
                    if disposables.contains_key(content) {
                        score_table.insert(
                            disposables.get(content).unwrap().clone().0,
                            disposables.get(content).unwrap().0.properties().score_weight() as f32 * score_mult,
                        );
                    }
                    score_table.insert(content.clone(), content.properties().score_weight() as f32 * score_mult);
                }

                score_table
            }
            | Some(raw_table) => {
                // Default all the Contents in the table
                let mut table: HashMap<Content, f32> = HashMap::new();

                for content in raw_table.keys() {
                    table.insert(content.to_default(), *raw_table.get(content).unwrap());
                }

                // Add all the Contents that are not in the table, setting score to 0
                for content in Content::iter() {
                    table.entry(content).or_insert(0.);
                }

                // Add the score for contents you can't dispose, stored in collectables HashMap
                for content in collectables.keys() {
                    sum_score += *collectables.get(content).unwrap() as f32 * *table.get(content).unwrap();
                }

                // Order disposables according to score_weight given in the custom score table
                let mut disposables_ordered: Vec<(Content, (Content, u32))> = disposables.clone().into_iter().collect();
                disposables_ordered.sort_by(|x, y| {
                    table
                        .get(&y.1 .0)
                        .unwrap()
                        .partial_cmp(table.get(&x.1 .0).unwrap())
                        .unwrap()
                });

                // Sums up the total score that could possible be got on the map

                // This part is identical to the one in the None case, except for the score_weight used being the one
                // given in the custom score table

                for content in disposables_ordered {
                    if disposables.contains_key(&content.0) {
                        // Checks if there is enough space to dispose everything in the world
                        if disposables.get(&content.0).unwrap().1 <= *collectables.get(&content.0).unwrap() {
                            // If there is, just add the score for all of it
                            sum_score +=
                                disposables.get(&content.0).unwrap().1 as f32 * *table.get(&content.1 .0).unwrap();
                        } else {
                            // If there isn't, check how much you gotta craft
                            let craftable_content_count = craftable_amount(&content.0, &mut collectables);
                            // Initialize a variable to hold the total amount of content
                            let total_content_count = craftable_content_count + collectables.get(&content.0).unwrap();

                            // If there is enough space to dispose everything you can craft, just add the score for all of it
                            // and subtract the materials used for crafting from collectables HashMap
                            if total_content_count < disposables.get(&content.0).unwrap().1 {
                                sum_score += total_content_count as f32 * *table.get(&content.1 .0).unwrap();
                                subtract_craft_materials(&content.0, craftable_content_count, &mut collectables);
                            } else {
                                // If there isn't enough space to dispose everything you can craft, add the score for what is disposable
                                // and subtract the materials used for crafting from collectables HashMap
                                sum_score +=
                                    *table.get(&content.1 .0).unwrap() * disposables.get(&content.0).unwrap().1 as f32;
                                subtract_craft_materials(
                                    &content.0,
                                    disposables.get(&content.0).unwrap().1 - collectables.get(&content.0).unwrap(),
                                    &mut collectables,
                                );
                            }
                        }
                    }
                }

                // Initializes and calculates the multiplication factor needed to normalize the score
                let score_mult = max_score / sum_score;

                let mut score_table = HashMap::new();

                // Collects Contents with normalized score
                for content in table {
                    score_table.insert(content.0, content.1 * score_mult);
                }

                score_table
            }
        }
    }

    /// "Add_score" function used inside of `destroy` interface
    ///
    /// Automatically increases the score according to the object being destroyed and it's quantity.
    ///
    /// This function does not perform error handling as should be called inside the `destroy` interface after all the necessary checks.
    ///
    /// # Arguments
    ///
    /// - `object`: Object being destroyed.
    /// - `quantity`: Quantity of the object in that tile (usize of the `Content` enum).
    ///
    pub(crate) fn add_score_destroy(&self, object: &Content, quantity: usize) {
        self.add_score_flat(*self.score_table.get(&object.to_default()).unwrap() * quantity as f32);
    }

    /// "Add_score" function used inside of `put` interface
    ///
    /// Automatically increases the score according to the object being put and it's quantity.
    ///
    /// This function does not perform error handling as should be called inside the `put` interface after all the necessary checks.
    ///
    /// # Arguments
    ///
    /// - `dispose`: Object being disposed.
    /// - `quantity`: Quantity of the object being disposed.
    ///
    pub(crate) fn add_score_put(&self, dispose: &Content, quantity: usize) {
        // if !self.score_table.contains_key(&dispose) {
        //     return Result::Err(MissingScoreTableEntry);
        // }
        self.add_score_flat(*self.score_table.get(&dispose.to_default()).unwrap() * quantity as f32);
    }

    /// "Add_score" function used primarily inside of the `ScoreStruct` itself.
    ///
    /// Increases the score by the given flat f32 value.
    ///
    /// Being a primarily helper function, it is public inside the crate for the sake of testing and
    /// other special cases where it might be needed.
    ///
    /// # Arguments
    ///
    /// - `value`: An f32 to be added to the score.
    ///
    pub(crate) fn add_score_flat(&self, value: f32) {
        self.score.clone().replace(self.score.clone().take() + value);
    }

    /// A getter function.
    ///
    /// # Returns
    ///
    /// A result of an f32.
    ///
    pub(crate) fn get_score(&self) -> f32 {
        *self.score.clone().borrow()
    }
}

impl Default for ScoreCounter {
    fn default() -> Self {
        ScoreCounter {
            score: Rc::new(RefCell::new(0.)),
            max_score: 1.,
            score_table: HashMap::new(),
        }
    }
}
