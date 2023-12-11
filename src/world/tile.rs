use std::iter::ExactSizeIterator;
use std::ops::Range;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Number of contents available in the lib
pub(crate) const N_CONTENTS: usize = 16;

/// Represents the properties of a tile type.
/// The `TileTypeProps` struct is used to define the properties of a tile type.
///
/// # Variables
/// - `walk`: A `bool` that indicates whether a robot can walk on the tile type.
/// - `hold`: A `[(Content, bool); N_CONTENTS]` that indicates the list of Content that a tile can contain.
/// - `cost`: An `usize` that indicates the energy cost to pass on the tile type.
///
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TileTypeProps {
    walk: bool,
    hold: [(Content, bool); N_CONTENTS],
    cost: usize,
    // Other properties associated with the tile type
}

impl TileTypeProps {
    pub fn walk(&self) -> bool {
        self.walk
    }
    pub fn hold(&self) -> &[(Content, bool); N_CONTENTS] {
        &self.hold
    }
    pub fn can_hold(&self, content: &Content) -> bool {
        self.hold[content.index()].1
    }
    pub fn cost(&self) -> usize {
        self.cost
    }
}

/// Represents the properties of a tile content.
/// The `ContentProps` struct is used to define the properties of a tile content.
///
/// # Variables
/// - `destroy`: A `bool` that indicates whether a robot can destroy the tile content.
/// - `max`: An `usize` that indicates the maximum return of elements when destroyed.
/// - `store`: A `bool` that indicates whether a robot can store the tile content.
/// - `cost`: An `usize` that indicates the energy cost to interact with the tile content.
/// - `score_weight`: An `i8`, the default value of the relative score given by interaction with each content.
/// - `craft`: A `[(Content, usize); N_CONTENTS]` where each tuple indicates the Content and the quantity of it needed for the craft.
///    if quantity == 0 then the content is not needed for the craft.
///    NOTE: each element is a possible recipe for that content
/// - `disposable`: An `Option<Content>` used in the score module to get the max score achievable in a certain map.
///    If it's not None it means that the Content can be interacted with in certain ways to get more score points
///    ex. you can extinguish a Fire Content with Water Content
///    you can put a Coin in the Bank
///
/// # Usage:
/// ```rust
/// use robotics_lib::world::tile::Content;
/// let max = Content::Rock(0).properties().max();
///
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ContentProps {
    destroy: bool,
    max: usize,
    store: bool,
    cost: usize,
    score_weight: i8,
    disposable: Option<Content>,
    craft: [(Content, usize); N_CONTENTS], // Other properties associated with the tile type
}

impl ContentProps {
    pub fn destroy(&self) -> bool {
        self.destroy
    }
    pub fn max(&self) -> usize {
        self.max
    }
    pub fn store(&self) -> bool {
        self.store
    }
    pub fn cost(&self) -> usize {
        self.cost
    }
    pub fn craft(&self) -> &[(Content, usize); N_CONTENTS] {
        &self.craft
    }
    pub fn score_weight(&self) -> i8 {
        self.score_weight
    }
    pub fn disposable(&self) -> &Option<Content> {
        &self.disposable
    }
}

// TileType

/// Represents the types of tiles in a map.
///
/// This enum defines various tile types that can be used to describe the terrain of individual
/// tiles on a map.
///
/// # Variants
/// - `DeepWater`: Deep water area
/// - `ShallowWater`: Shallow water area
/// - `Sand`: Sand area
/// - `Grass`: Grass area
/// - `Street`: Street area
/// - `Hill`: Hill area
/// - `Mountain`: Mountain area
/// - `Snow`: Snow area
/// - `Lava`: Lava area
/// - `Teleport` : Teleport area, (true) if it has been already discovered by the robot (false) instead. Initially all tiles are false, they will become true if the robot go on top of them
/// - `Wall`: Wall area
///
/// # Usage
/// ```
/// use robotics_lib::world::tile::TileType;
/// let tile = TileType::Grass;
///
/// match tile {
///     TileType::Grass => println!("This tile is covered in grass."),
///     TileType::Street => println!("This tile is part of a street."),
///     _ => {}
/// }
/// ```
///;
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter, Serialize, Deserialize)]
pub enum TileType {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Street,
    Hill,
    Mountain,
    Snow,
    Lava,
    Teleport(bool),
    Wall,
}

impl TileType {
    pub fn properties(&self) -> &'static TileTypeProps {
        const fn gen_props(tile_type: TileType) -> TileTypeProps {
            match tile_type {
                | TileType::DeepWater => TileTypeProps {
                    cost: 0,
                    walk: false,
                    hold: [
                        (Content::Rock(0), false),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, false),
                        (Content::Coin(0), false),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), true),
                        (Content::None, true),
                        (Content::Fish(0), true),
                        (Content::Market(0), false),
                        (Content::Building, false),
                        (Content::Bush(0), false),
                        (Content::JollyBlock(0), false),
                        (Content::Scarecrow, false),
                    ],
                },
                | TileType::ShallowWater => TileTypeProps {
                    cost: 5,
                    walk: true,
                    hold: [
                        (Content::Rock(0), false),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, false),
                        (Content::Coin(0), false),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), true),
                        (Content::None, true),
                        (Content::Fish(0), true),
                        (Content::Market(0), false),
                        (Content::Building, false),
                        (Content::Bush(0), false),
                        (Content::JollyBlock(0), false),
                        (Content::Scarecrow, true),
                    ],
                },
                | TileType::Sand => TileTypeProps {
                    cost: 3,
                    walk: true,
                    hold: [
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), false),
                        (Content::Building, false),
                        (Content::Bush(0), false),
                        (Content::JollyBlock(0), true),
                        (Content::Scarecrow, true),
                    ],
                },
                | TileType::Grass => TileTypeProps {
                    cost: 1,
                    walk: true,
                    hold: [
                        (Content::Rock(0), true),
                        (Content::Tree(0), true),
                        (Content::Garbage(0), true),
                        (Content::Fire, true),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), true),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), true),
                        (Content::Building, true),
                        (Content::Bush(0), true),
                        (Content::JollyBlock(0), true),
                        (Content::Scarecrow, true),
                    ],
                },
                | TileType::Street => TileTypeProps {
                    cost: 0,
                    walk: true,
                    hold: [
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), true),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), true),
                        (Content::Building, true),
                        (Content::Bush(0), false),
                        (Content::JollyBlock(0), true),
                        (Content::Scarecrow, true),
                    ],
                },
                | TileType::Hill => TileTypeProps {
                    cost: 5,
                    walk: true,
                    hold: [
                        (Content::Rock(0), true),
                        (Content::Tree(0), true),
                        (Content::Garbage(0), true),
                        (Content::Fire, true),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), false),
                        (Content::Building, true),
                        (Content::Bush(0), true),
                        (Content::JollyBlock(0), true),
                        (Content::Scarecrow, true),
                    ],
                },
                | TileType::Mountain => TileTypeProps {
                    cost: 10,
                    walk: true,
                    hold: [
                        (Content::Rock(0), true),
                        (Content::Tree(0), true),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), false),
                        (Content::Building, true),
                        (Content::Bush(0), true),
                        (Content::JollyBlock(0), true),
                        (Content::Scarecrow, true),
                    ],
                },
                | TileType::Snow => TileTypeProps {
                    cost: 3,
                    walk: true,
                    hold: [
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), true),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), false),
                        (Content::Building, true),
                        (Content::Bush(0), true),
                        (Content::JollyBlock(0), true),
                        (Content::Scarecrow, true),
                    ],
                },
                | TileType::Lava => TileTypeProps {
                    cost: 0,
                    walk: false,
                    hold: [
                        (Content::Rock(0), false),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, false),
                        (Content::Coin(0), false),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), false),
                        (Content::Building, false),
                        (Content::Bush(0), false),
                        (Content::JollyBlock(0), false),
                        (Content::Scarecrow, false),
                    ],
                },
                | TileType::Teleport(_) => TileTypeProps {
                    cost: 0,
                    walk: true,
                    hold: [
                        (Content::Rock(0), true),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), true),
                        (Content::Fire, false),
                        (Content::Coin(0), true),
                        (Content::Bin(0..0), true),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), true),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), true),
                        (Content::Building, false),
                        (Content::Bush(0), false),
                        (Content::JollyBlock(0), false),
                        (Content::Scarecrow, false),
                    ],
                },
                | TileType::Wall => TileTypeProps {
                    cost: 0,
                    walk: false,
                    hold: [
                        (Content::Rock(0), false),
                        (Content::Tree(0), false),
                        (Content::Garbage(0), false),
                        (Content::Fire, true),
                        (Content::Coin(0), false),
                        (Content::Bin(0..0), false),
                        (Content::Crate(0..0), false),
                        (Content::Bank(0..0), false),
                        (Content::Water(0), false),
                        (Content::None, true),
                        (Content::Fish(0), false),
                        (Content::Market(0), false),
                        (Content::Building, false),
                        (Content::Bush(0), false),
                        (Content::JollyBlock(0), false),
                        (Content::Scarecrow, false),
                    ],
                },
            }
        }
        match self {
            | TileType::DeepWater => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::DeepWater);
                &TILETYPE_PROPS
            }
            | TileType::ShallowWater => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::ShallowWater);
                &TILETYPE_PROPS
            }
            | TileType::Sand => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Sand);
                &TILETYPE_PROPS
            }
            | TileType::Grass => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Grass);
                &TILETYPE_PROPS
            }
            | TileType::Street => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Street);
                &TILETYPE_PROPS
            }
            | TileType::Hill => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Hill);
                &TILETYPE_PROPS
            }
            | TileType::Mountain => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Mountain);
                &TILETYPE_PROPS
            }
            | TileType::Snow => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Snow);
                &TILETYPE_PROPS
            }
            | TileType::Lava => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Lava);
                &TILETYPE_PROPS
            }
            | TileType::Teleport(_) => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Teleport(false));
                &TILETYPE_PROPS
            }
            | TileType::Wall => {
                static TILETYPE_PROPS: TileTypeProps = gen_props(TileType::Wall);
                &TILETYPE_PROPS
            }
        }
    }
}

/// Represents the content of a tile. The (usize) is the max amount of the content that will be given when destroyed or the
/// max amount that can be stored inside it. (Decided by the generator)
///
/// This enum defines various entity types that can be placed in the contents of a tile.
///
/// # Variants
/// - `Rock`: Rock entity
/// - `Tree`: Tree entity
/// - `Garbage`: Garbage entity
/// - `Fire`: Fire entity
/// - `Coin`: Coin entity
/// - `Bin`: Bin entity
/// - `Crate`: Crate entity
/// - `Water`: Water entity
/// - `Bank`: Bank entity
/// - `Market`: Market entity
/// - `Fish`: Fish entity
/// - `None`: None entity
/// - `Building`: Building entity
/// - `Bush`: Bush entity
/// - `JollyBlock`: Generic object entity
/// - `Scarecrow`: Generic living thing entity
/// - `...`
///
/// # Usage
/// ```
/// use robotics_lib::world::tile::Content;
/// let entity = Content::Rock(20);
///
/// match entity {
///     Content::Rock(_) => println!("This is a solid rock in the game world."),
///     Content::Tree(_) => println!("A tall tree stands here."),
///     Content::Garbage(_) => println!("You find a pile of garbage."),
///     Content::JollyBlock(_) => println!("This is a fountain/boat/..."),
///     Content::Scarecrow => println!("This is a duck/pig/..."),
///     _ => {}
/// }
/// ```
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, EnumIter, Display, Serialize, Deserialize)]
pub enum Content {
    Rock(usize),
    Tree(usize),
    Garbage(usize),
    Fire,
    Coin(usize),
    Bin(Range<usize>),
    Crate(Range<usize>),
    Bank(Range<usize>),
    Water(usize),
    Market(usize),
    Fish(usize),
    Building,
    Bush(usize),
    JollyBlock(usize),
    Scarecrow,
    None,
}

impl Content {
    pub fn world_generator_max(&self) -> usize {
        // returns the max decided by the world generator for each content
        match self {
            | Content::Rock(value) => *value,
            | Content::Tree(value) => *value,
            | Content::Garbage(value) => *value,
            | Content::Fire => 0,
            | Content::Coin(value) => *value,
            | Content::Bin(value) => value.end,
            | Content::Crate(value) => value.end,
            | Content::Bank(value) => value.end,
            | Content::Water(value) => *value,
            | Content::Market(value) => *value,
            | Content::Fish(value) => *value,
            | Content::Building => 0,
            | Content::Bush(value) => *value,
            | Content::JollyBlock(value) => *value,
            | Content::Scarecrow => 0,
            | Content::None => 0,
        }
    }
    pub fn index(&self) -> usize {
        match self {
            | Content::Rock(_) => 0,
            | Content::Tree(_) => 1,
            | Content::Garbage(_) => 2,
            | Content::Fire => 3,
            | Content::Coin(_) => 4,
            | Content::Bin(_) => 5,
            | Content::Crate(_) => 6,
            | Content::Bank(_) => 7,
            | Content::Water(_) => 8,
            | Content::None => 9,
            | Content::Fish(_) => 10,
            | Content::Market(_) => 11,
            | Content::Building => 12,
            | Content::Bush(_) => 13,
            | Content::JollyBlock(_) => 14,
            | Content::Scarecrow => 15,
        }
    }
    pub fn to_default(&self) -> Self {
        match self {
            | Content::Coin(_) => Content::Coin(0),
            | Content::Garbage(_) => Content::Garbage(0),
            | Content::Water(_) => Content::Water(0),
            | Content::Rock(_) => Content::Rock(0),
            | Content::Tree(_) => Content::Tree(0),
            | Content::Market(_) => Content::Market(0),
            | Content::Fish(_) => Content::Fish(0),
            | Content::Bin(_) => Content::Bin(0..0),
            | Content::Bank(_) => Content::Bank(0..0),
            | Content::Crate(_) => Content::Crate(0..0),
            | Content::Fire => Content::Fire,
            | Content::Scarecrow => Content::Scarecrow,
            | Content::Building => Content::Building,
            | Content::None => Content::None,
            | Content::Bush(_) => Content::Bush(0),
            | Content::JollyBlock(_) => Content::JollyBlock(0),
        }
    }
    /// Function to get the instantiated value of a content
    ///
    /// Returns either the range or value of the content
    pub fn get_value(&self) -> (Option<usize>, Option<Range<usize>>) {
        match self {
            | Content::Rock(value) => (Some(*value), None),
            | Content::Tree(value) => (Some(*value), None),
            | Content::Garbage(value) => (Some(*value), None),
            | Content::Fire => (Some(1), None),
            | Content::Coin(value) => (Some(*value), None),
            | Content::Bin(value) => (None, Some(value.clone())),
            | Content::Crate(value) => (None, Some(value.clone())),
            | Content::Bank(value) => (None, Some(value.clone())),
            | Content::Water(value) => (Some(*value), None),
            | Content::Market(value) => (Some(*value), None),
            | Content::Fish(value) => (Some(*value), None),
            | Content::None => (None, None),
            | Content::Building => (None, None),
            | Content::Bush(value) => (Some(*value), None),
            | Content::JollyBlock(value) => (Some(*value), None),
            | Content::Scarecrow => (None, None),
        }
    }
    /// Function to instantiate a content given a value
    ///
    /// # Arguments
    /// - `value`: the value to instantiate the content with
    ///
    /// # Remarks
    /// in the case of `Storables` the range will be 0..value
    pub fn to_value(&self, value: usize) -> Self {
        match self {
            | Content::Coin(_) => Content::Coin(value),
            | Content::Garbage(_) => Content::Garbage(value),
            | Content::Water(_) => Content::Water(value),
            | Content::Rock(_) => Content::Rock(value),
            | Content::Tree(_) => Content::Tree(value),
            | Content::Market(_) => Content::Market(value),
            | Content::Fish(_) => Content::Fish(value),
            | Content::Bin(_) => Content::Bin(0..value),
            | Content::Bank(_) => Content::Bank(0..value),
            | Content::Crate(_) => Content::Crate(0..value),
            | Content::None => Content::None,
            | Content::Fire => Content::Fire,
            | Content::Building => Content::Building,
            | Content::Bush(_) => Content::Bush(value),
            | Content::JollyBlock(_) => Content::JollyBlock(value),
            | Content::Scarecrow => Content::Scarecrow,
        }
    }
    pub fn properties(&self) -> &'static ContentProps {
        const fn gen_props(content: Content) -> ContentProps {
            // default array for a non-craftable content
            let not_craftable: [(Content, usize); N_CONTENTS] = [
                (Content::Rock(0), 0),
                (Content::Tree(0), 0),
                (Content::Garbage(0), 0),
                (Content::Fire, 0),
                (Content::Coin(0), 0),
                (Content::Bin(0..0), 0),
                (Content::Crate(0..0), 0),
                (Content::Bank(0..0), 0),
                (Content::Water(0), 0),
                (Content::None, 0),
                (Content::Fish(0), 0),
                (Content::Market(0), 0),
                (Content::Building, 0),
                (Content::Bush(0), 0),
                (Content::JollyBlock(0), 0),
                (Content::Scarecrow, 0),
            ];

            match content {
                | Content::Rock(_) => ContentProps {
                    destroy: true,
                    max: 4,
                    store: false,
                    cost: 1,
                    craft: not_craftable,
                    score_weight: 1,
                    disposable: None,
                },
                | Content::Tree(_) => ContentProps {
                    destroy: true,
                    max: 5,
                    store: false,
                    cost: 3,
                    craft: not_craftable,
                    score_weight: 3,
                    disposable: None,
                },
                | Content::Garbage(_) => ContentProps {
                    destroy: true,
                    max: 3,
                    store: false,
                    cost: 4,
                    craft: [
                        (Content::Rock(0), 3),
                        (Content::Tree(0), 1),
                        (Content::Garbage(0), 0),
                        (Content::Fire, 0),
                        (Content::Coin(0), 0),
                        (Content::Bin(0..0), 0),
                        (Content::Crate(0..0), 0),
                        (Content::Bank(0..0), 0),
                        (Content::Water(0), 0),
                        (Content::None, 0),
                        (Content::Fish(0), 1),
                        (Content::Market(0), 0),
                        (Content::Building, 0),
                        (Content::Bush(0), 0),
                        (Content::JollyBlock(0), 0),
                        (Content::Scarecrow, 0),
                    ],
                    score_weight: 2,
                    disposable: None,
                },
                | Content::Fire => ContentProps {
                    destroy: true,
                    max: 1,
                    store: false,
                    cost: 5,
                    craft: not_craftable,
                    score_weight: 10,
                    disposable: Some(Content::Water(0)),
                },
                | Content::Coin(_) => ContentProps {
                    destroy: true,
                    max: 10,
                    store: true,
                    cost: 0,
                    craft: [
                        (Content::Rock(0), 0),
                        (Content::Tree(0), 0),
                        (Content::Garbage(0), 5),
                        (Content::Fire, 0),
                        (Content::Coin(0), 0),
                        (Content::Bin(0..0), 0),
                        (Content::Crate(0..0), 0),
                        (Content::Bank(0..0), 0),
                        (Content::Water(0), 0),
                        (Content::None, 0),
                        (Content::Fish(0), 0),
                        (Content::Market(0), 0),
                        (Content::Building, 0),
                        (Content::Bush(0), 0),
                        (Content::JollyBlock(0), 0),
                        (Content::Scarecrow, 0),
                    ],
                    score_weight: 2,
                    disposable: None,
                },
                | Content::Bin(_) => ContentProps {
                    destroy: false,
                    max: 10,
                    store: true,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 10,
                    disposable: Some(Content::Garbage(0)),
                },
                | Content::Crate(_) => ContentProps {
                    destroy: false,
                    max: 20,
                    store: true,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 7,
                    disposable: Some(Content::Tree(0)),
                },
                | Content::Bank(_) => ContentProps {
                    destroy: false,
                    max: 50,
                    store: true,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 10,
                    disposable: Some(Content::Coin(0)),
                },
                | Content::Water(_) => ContentProps {
                    destroy: true,
                    max: 20,
                    store: true,
                    cost: 3,
                    craft: not_craftable,
                    score_weight: 2,
                    disposable: None,
                },
                | Content::None => ContentProps {
                    destroy: false,
                    max: 0,
                    store: false,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 0,
                    disposable: None,
                },
                | Content::Fish(_) => ContentProps {
                    destroy: true,
                    max: 3,
                    store: true,
                    cost: 1,
                    craft: not_craftable,
                    score_weight: 1,
                    disposable: None,
                },
                | Content::Market(_) => ContentProps {
                    destroy: false,
                    max: 20,
                    store: false,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 5,
                    // disposable here is set to just Fish,
                    // but you can dispose also Tree and Rocks
                    disposable: Some(Content::Fish(0)),
                },
                | Content::Building => ContentProps {
                    destroy: false,
                    max: 0,
                    store: false,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 0,
                    disposable: None,
                },
                | Content::Bush(_) => ContentProps {
                    destroy: true,
                    max: 10,
                    store: true,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 1,
                    disposable: None,
                },
                | Content::JollyBlock(_) => ContentProps {
                    destroy: true,
                    max: 2,
                    store: true,
                    cost: 2,
                    craft: [
                        (Content::Rock(0), 2),
                        (Content::Tree(0), 2),
                        (Content::Garbage(0), 2),
                        (Content::Fire, 0),
                        (Content::Coin(0), 2),
                        (Content::Bin(0..0), 0),
                        (Content::Crate(0..0), 0),
                        (Content::Bank(0..0), 0),
                        (Content::Water(0), 0),
                        (Content::None, 0),
                        (Content::Fish(0), 2),
                        (Content::Market(0), 0),
                        (Content::Building, 0),
                        (Content::Scarecrow, 2),
                        (Content::JollyBlock(0), 0),
                        (Content::Bush(0), 2),
                    ],
                    score_weight: 2,
                    disposable: None,
                },
                | Content::Scarecrow => ContentProps {
                    destroy: false,
                    max: 0,
                    store: false,
                    cost: 0,
                    craft: not_craftable,
                    score_weight: 2,
                    disposable: None,
                },
            }
        }
        match self {
            | Content::Rock(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Rock(0));
                &CONTENT_PROPS
            }
            | Content::Tree(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Tree(0));
                &CONTENT_PROPS
            }
            | Content::Garbage(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Garbage(0));
                &CONTENT_PROPS
            }
            | Content::Fire => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Fire);
                &CONTENT_PROPS
            }
            | Content::Coin(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Coin(0));
                &CONTENT_PROPS
            }
            | Content::Bin(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Bin(0..0));
                &CONTENT_PROPS
            }
            | Content::Crate(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Crate(0..0));
                &CONTENT_PROPS
            }
            | Content::Bank(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Bank(0..0));
                &CONTENT_PROPS
            }
            | Content::Water(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Water(0));
                &CONTENT_PROPS
            }
            | Content::None => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::None);
                &CONTENT_PROPS
            }
            | Content::Fish(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Fish(0));
                &CONTENT_PROPS
            }
            | Content::Market(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Market(0));
                &CONTENT_PROPS
            }
            | Content::Bush(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Bush(0));
                &CONTENT_PROPS
            }
            | Content::Building => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Building);
                &CONTENT_PROPS
            }
            | Content::Scarecrow => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::Scarecrow);
                &CONTENT_PROPS
            }
            | Content::JollyBlock(_) => {
                static CONTENT_PROPS: ContentProps = gen_props(Content::JollyBlock(0));
                &CONTENT_PROPS
            }
        }
    }
}

/// Represents a tile in the game world.
///
/// The `Tile` struct is used to define individual tiles within the game world. Each tile can have
/// a specific `TileType` that describes the terrain or content of the tile and may contain a
/// `Content` representing a static object on that tile. Each tile has an elevation, which is a
/// positive integer including zero.
///
///
/// # Fields
///
/// - `tile_type`: A `TileType`.
/// - `content`: A `Content`
/// - `elevation`: Elevation of the tile
///
///
/// # Usage
///
/// ```
/// // Create a new tile with a grassy type and no content.
/// use robotics_lib::world::tile::{Content, Tile, TileType};
///
/// // Create a tile representing a street with a garbage pile.
/// let street_tile = Tile {
///     tile_type: TileType::Grass,
///     content: Content::Garbage(2),
///     elevation: 20
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub content: Content,
    pub elevation: usize,
}
