pub mod wgenerator {
    use rand::Rng;
    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;
    use robotics_lib::interface::Tools;
    use robotics_lib::interface::{craft, debug, destroy, go, look_at_sky, teleport, Direction};
    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::runner::{Robot, Runnable, Runner};
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
    use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
    use robotics_lib::world::tile::Content::{Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree, Water};
    use robotics_lib::world::tile::TileType::{DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport};
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use robotics_lib::world::world_generator::Generator;
    use std::collections::HashMap;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use strum::IntoEnumIterator;

    pub struct WorldGenerator {
        size: usize,
    }

    impl WorldGenerator {
        pub fn init(size: usize) -> Self {
            WorldGenerator { size }
        }
    }


// ...

    impl Generator for WorldGenerator {
        fn gen(
            &mut self,
        ) -> (
            Vec<Vec<Tile>>,
            (usize, usize),
            EnvironmentalConditions,
            f32,
            Option<HashMap<Content, f32>>,
        ) {
            let mut rng = StdRng::from_entropy();  // Usa un seme casuale diverso ad ogni esecuzione
            let mut map: Vec<Vec<Tile>> = Vec::new();

            for _i in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for _j in 0..self.size {
                    let tile_type = Grass;//match rng.gen_range(0..10) {
                    /*  0..=1 => DeepWater,
                      2..=3 => ShallowWater,
                      4..=5 => Sand,
                      6..=7 => Grass,
                      8 => Street,
                      9 => Lava,
                      _ => Grass,
                  };*/

                    let content = match tile_type {
                        Lava => Content::None,
                        Grass => {
                            match rng.gen_range(0..7) {
                                0 => Rock(0),
                                1 => Tree(2),
                                2 => Garbage(2),
                                3 => Fire,
                                4 => Coin(2),
                                5 => Bin(2..3),
                                6 => Crate(2..3),
                                _ => Content::None,
                            }
                        }
                        Street => {
                            match rng.gen_range(0..3) {
                                0 => Coin(2),
                                1 => Bin(2..3),
                                2 => Market(20),
                                _ => Content::None,
                            }
                        }
                        _ => Content::None,
                    };

                    row.push(Tile {
                        tile_type,
                        content,
                        elevation: 0,
                    });
                }
                map.push(row);
            }

            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();
            let max_score = rng.gen::<f32>();

            (map, (0, 5), environmental_conditions, max_score, None)
        }
    }
}
