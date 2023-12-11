use std::collections::HashMap;
use std::process::exit;

use strum::IntoEnumIterator;

use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::interface::{
    craft, destroy, discover_tiles, go, one_direction_view, put, robot_map, robot_view, teleport, Direction, Tools,
};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Runner;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError;
use robotics_lib::utils::LibError::NoContent;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Content::{Coin, Garbage, Rock, Tree};
use robotics_lib::world::tile::{Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;

/**************************************************************************
*  MAP:
*    ______________________________________
*   |            |            |            |
*   |  Teleport  |   Grass    |   Grass    |
*   |    0 el    |   0 el     |    0 el    |
*   |    None    |  Tree(1)   |    None    |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Grass   |   Grass    |   Grass    |
*   |    0 el    |   0 el     |    0 el    |
*   |    None    |   Tree(1)  |    None    |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Grass   |  DeepWater |   Grass    |
*   |    0 el    |   0 el     |    0 el    |
*   |    None    |    None    |    None    |
*   |____________|____________|____________|
*
*   TEST:
*
*   Starting from (0,0), the woodworker robot will look for a tile with "Content: Tree(_)".
*   Once found, will try to destroy it. If it is in reach, it will succeed. Otherwise it will move towards it.
*   This is done twice, then it moves to the center and checks that the perceived map corresponds to the map above,
*   but with all tiles having "Content: None".
*   Several small tests of the different interfaces are also performed
*/

#[test]
fn test_woodworker_robot() {
    struct WorldGenerator {}

    impl WorldGenerator {
        fn new() -> Self {
            WorldGenerator {}
        }
    }
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
            let mut map: Vec<Vec<Tile>> = Vec::new();

            map.push(Vec::new());
            map[0].push(Tile {
                tile_type: TileType::Teleport(false),
                content: Content::None,
                elevation: 0,
            });
            map[0].push(Tile {
                tile_type: TileType::Grass,
                content: Content::Tree(1),
                elevation: 0,
            });
            map[0].push(Tile {
                tile_type: TileType::Grass,
                content: Content::None,
                elevation: 0,
            });

            map.push(Vec::new());
            map[1].push(Tile {
                tile_type: TileType::Grass,
                content: Content::None,
                elevation: 0,
            });
            map[1].push(Tile {
                tile_type: TileType::Grass,
                content: Content::Tree(1),
                elevation: 0,
            });
            map[1].push(Tile {
                tile_type: TileType::Grass,
                content: Content::None,
                elevation: 0,
            });

            map.push(Vec::new());
            map[2].push(Tile {
                tile_type: TileType::Grass,
                content: Content::None,
                elevation: 0,
            });
            map[2].push(Tile {
                tile_type: TileType::DeepWater,
                content: Content::None,
                elevation: 0,
            });
            map[2].push(Tile {
                tile_type: TileType::Grass,
                content: Content::None,
                elevation: 0,
            });

            let environmental_conditions =
                EnvironmentalConditions::new(&[WeatherType::Sunny, WeatherType::Rainy], 15, 12);
            (map, (0, 0), environmental_conditions.unwrap(), 100.0, None)
        }
    }
    let mut generator: WorldGenerator = WorldGenerator::new();

    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            let view = robot_view(self, world);
            let mut tree_direction: Option<Direction> = None;

            for (i, row) in view.iter().enumerate() {
                for (j, column) in row.iter().enumerate() {
                    if column.is_some() {
                        let tile = column.clone().unwrap();
                        match tile.content {
                            | Content::Tree(_) => {
                                match (i, j) {
                                    | (0, 0) => tree_direction = Some(Direction::Left),
                                    | (0, 1) => tree_direction = Some(Direction::Up),
                                    | (0, 2) => tree_direction = Some(Direction::Right),
                                    | (1, 0) => tree_direction = Some(Direction::Left),
                                    | (1, 2) => tree_direction = Some(Direction::Right),
                                    | (2, 0) => tree_direction = Some(Direction::Left),
                                    | (2, 1) => tree_direction = Some(Direction::Down),
                                    | (2, 2) => tree_direction = Some(Direction::Right),
                                    | (_, _) => {}
                                }
                                break;
                            }
                            | _ => {}
                        }
                    }
                    if tree_direction.is_some() {
                        break;
                    }
                }
                if tree_direction.is_some() {
                    break;
                }
            }

            if tree_direction.is_some() {
                let res = destroy(self, world, tree_direction.clone().unwrap());
                match res {
                    | Ok(_) => {}
                    | Err(_) => {
                        let res = go(self, world, tree_direction.clone().unwrap());
                        if res.is_err() {
                            exit(1)
                        }
                    }
                }
            } else {
                let actual_energy = self.get_energy().get_energy_level();
                assert_eq!(actual_energy, 1000); //We destroyed trees twice and 2 ticks already passed: We should have 1000 - (3*2) + (10*2) energy, but this is over the max value so we should have 1000

                let _ = go(self, world, Direction::Down);
                let perceived_world = robot_view(self, world);

                let mut map: Vec<Vec<Option<Tile>>> = Vec::new();

                map.push(Vec::new());
                map[0].push(Some(Tile {
                    tile_type: TileType::Teleport(true), //checks if spawning on a teleport discovers it
                    content: Content::None,
                    elevation: 0,
                }));
                map[0].push(Some(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                }));
                map[0].push(Some(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                }));

                map.push(Vec::new());
                map[1].push(Some(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                }));
                map[1].push(Some(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                }));
                map[1].push(Some(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                }));

                map.push(Vec::new());
                map[2].push(Some(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                }));
                map[2].push(Some(Tile {
                    tile_type: TileType::DeepWater,
                    content: Content::None,
                    elevation: 0,
                }));
                map[2].push(Some(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                }));
                assert_eq!(perceived_world, map); //The world has been modified as expected

                let wrong_destroy = destroy(self, world, Up);
                assert!(wrong_destroy.is_err()); //We already destroyed the content

                let _ = go(self, world, Right);
                let wrong_go1 = go(self, world, Right);
                assert!(wrong_go1.is_err()); //Can't go out of bounds

                let _ = go(self, world, Down);
                let wrong_go2 = go(self, world, Left);
                assert!(wrong_go2.is_err()); //Can't go into DeepWater
                let pos = (self.get_coordinate().get_row(), self.get_coordinate().get_row());
                assert_eq!(pos, (2, 2)); //Checks that we didn't actually move, even though we received an error

                let backpack = self.get_backpack();
                for item in backpack.get_contents().iter() {
                    if item.0.clone() == Tree(0) {
                        assert_eq!(*item.1, 2); //We should have gotten 2 units of Tree(0)
                    }
                }
                //The quantity of trees received after destroy is random, could still be 0
                let correct_put = put(self, world, Tree(0), 1, Up);
                assert!(correct_put.is_ok()); //We should be able to put a tree in the grass tile with no content
                let check = discover_tiles(self, world, &[(1, 2)]);
                let tile_content = check.clone().unwrap().get(&(1, 2)).unwrap().clone().unwrap().content;
                assert_eq!(tile_content, Tree(1)); //Checks that we actually put a Tree there
                let wrong_put = put(self, world, Tree(0), 1, Left);
                assert!(wrong_put.is_err()); //We can't put a tree in DeepWater

                let mut one_view = one_direction_view(self, world, Right, 5);
                assert_eq!(one_view.unwrap(), Vec::<Vec<Tile>>::new()); //We shouldn't see anything out of bounds

                one_view = one_direction_view(self, world, Up, 5);
                let mut expected_one_view = Vec::new();
                expected_one_view.push(Vec::new());
                expected_one_view[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                expected_one_view[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Tree(1),
                    elevation: 0,
                });
                expected_one_view.push(Vec::new());
                expected_one_view[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                expected_one_view[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                assert_eq!(one_view.unwrap(), expected_one_view);
            }
        }

        fn handle_event(&mut self, event: Event) {
            match event {
                | Event::Terminated => {}
                | _ => {}
            }
        }

        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
    }

    let my_robot = MyRobot(Robot::new());

    struct Tool {}
    impl Tools for Tool {}

    let mut runner = Runner::new(Box::new(my_robot), &mut generator);
    if runner.is_err() {
        exit(1);
    }

    let mut res = Err(LibError::NoContent);
    for _i in 0..4 {
        res = runner.as_mut().unwrap().game_tick();
    }
    assert!(res.is_ok());
}

/**************************************************************************
*  MAP:
*    ______________________________________
*   |            |            |            |
*   |  Teleport  |   Grass    |   Grass    |
*   |    0 el    |   0 el     |    0 el    |
*   |    None    |  Tree(10)  |  Rock(10)  |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Street  |   Street   |   Street   |
*   |    0 el    |   0 el     |    0 el    |
*   |    None    |   None     |    None    |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Grass   |   Grass    |  Teleport  |
*   |    0 el    |   0 el     |    0 el    |
*   |    Bank    |   Market   |    None    |
*   |____________|____________|____________|
*
*/

#[test]
fn test_crafter_robot() {
    struct WorldGenerator {}

    impl WorldGenerator {
        fn new() -> Self {
            WorldGenerator {}
        }
    }
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
            let mut map: Vec<Vec<Tile>> = Vec::new();

            map.push(Vec::new());
            map[0].push(Tile {
                tile_type: TileType::Teleport(false),
                content: Content::None,
                elevation: 0,
            });
            map[0].push(Tile {
                tile_type: TileType::Grass,
                content: Tree(3),
                elevation: 0,
            });
            map[0].push(Tile {
                tile_type: TileType::Grass,
                content: Rock(10),
                elevation: 0,
            });

            map.push(Vec::new());
            map[1].push(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            });
            map[1].push(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            });
            map[1].push(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            });

            map.push(Vec::new());
            map[2].push(Tile {
                tile_type: TileType::Grass,
                content: Content::Bank(0..10),
                elevation: 0,
            });
            map[2].push(Tile {
                tile_type: TileType::Grass,
                content: Content::Market(2),
                elevation: 0,
            });
            map[2].push(Tile {
                tile_type: TileType::Teleport(false),
                content: Content::None,
                elevation: 0,
            });

            let environmental_conditions =
                EnvironmentalConditions::new(&[WeatherType::Sunny, WeatherType::Rainy], 15, 12);
            (map, (2, 2), environmental_conditions.unwrap(), 100.0, None)
        }
    }
    let mut generator: WorldGenerator = WorldGenerator::new();

    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            let mut tp_res = teleport(self, world, (0, 0));
            assert!(tp_res.is_err()); //The teleport in (0,0) has not been discovered
            tp_res = teleport(self, world, (2, 1));
            assert!(tp_res.is_err()); //There is no teleport in (2,1)

            //Destroy Rock(10) in (0,2) and Tree(3) in (0,1)
            let mut res = go(self, world, Up);
            assert!(res.is_ok());
            let mut destroy_res = destroy(self, world, Up);
            assert!(destroy_res.is_ok());
            res = go(self, world, Left);
            assert!(res.is_ok());
            destroy_res = destroy(self, world, Up);
            assert!(destroy_res.is_ok());

            //Reach Teleport in (0,0) and teleport back to (2,2)
            res = go(self, world, Left);
            assert!(res.is_ok());
            res = go(self, world, Up);
            assert!(res.is_ok());
            tp_res = teleport(self, world, (2, 1));
            assert!(tp_res.is_err()); //There is no teleport in this position
            tp_res = teleport(self, world, (2, 2));
            assert!(tp_res.is_ok());
            let pos = (self.get_coordinate().get_row(), self.get_coordinate().get_col());
            assert_eq!(pos, (2, 2));

            //Try crafting 5 units of garbage
            for _i in 0..5usize {
                let craft_res = craft(self, Content::Garbage(0));
                assert!(craft_res.is_ok());
            }
            for item in self.get_backpack().get_contents().iter() {
                if item.0.clone() == Garbage(0) {
                    assert_eq!(*item.1, 5);
                }
            }

            //With 5 garbage, a coin can be crafted
            let craft_res = craft(self, Content::Coin(0));
            assert!(craft_res.is_ok());
            for item in self.get_backpack().get_contents().iter() {
                if item.0.clone() == Coin(0) {
                    assert_eq!(*item.1, 1);
                }
            }

            //Sell what is left at the market
            let backpack = self.get_backpack();
            let mut to_be_sold: Vec<(Content, usize)> = Vec::new();
            for item in backpack.get_contents().iter() {
                if *item.1 > 0 && item.0.clone() != Coin(0) {
                    to_be_sold.push((item.0.clone(), *item.1));
                }
            }
            for item in to_be_sold.iter() {
                let put_res = put(self, world, item.0.clone(), item.1, Left);
                assert!(put_res.is_ok());
            }

            //Go to the bank
            res = go(self, world, Up);
            assert!(res.is_ok());
            res = go(self, world, Left);
            assert!(res.is_ok());
            res = go(self, world, Left);
            assert!(res.is_ok());

            //Deposit all the coins at the bank
            let available_coins = self.get_backpack().get_contents().get(&Coin(0));
            assert!(available_coins.is_some());
            for item in self.get_backpack().get_contents().iter() {
                if item.0.clone() == Coin(0) {
                    assert_eq!(*item.1, *available_coins.unwrap());
                }
            }
            let mut put_res = put(self, world, Coin(0), *available_coins.unwrap(), Down);
            assert!(put_res.is_ok());
            for item in self.get_backpack().get_contents().iter() {
                if item.0.clone() == Coin(0) {
                    assert_eq!(*item.1, 0);
                }
            }

            //Try depositing more coins
            put_res = put(self, world, Coin(0), 1, Down);
            assert!(put_res.is_err());

            //Try crafting random things. None of these should succeed as they either are not craftable or there are not enough contents
            for c in Content::iter() {
                let craft_res = craft(self, c);
                assert!(craft_res.is_err());
            }
        }

        fn handle_event(&mut self, event: Event) {
            match event {
                | Event::Terminated => {}
                | _ => {}
            }
        }

        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
    }

    let my_robot = MyRobot(Robot::new());

    struct Tool {}
    impl Tools for Tool {}

    let mut runner = Runner::new(Box::new(my_robot), &mut generator);
    if runner.is_err() {
        exit(1);
    }

    let res = runner.as_mut().unwrap().game_tick();
    assert!(res.is_ok());
}

#[test]
fn test_robot_map() {
    struct WorldGenerator {}

    impl WorldGenerator {
        fn new() -> Self {
            WorldGenerator {}
        }
    }
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
            let mut map: Vec<Vec<Tile>> = Vec::new();

            map.push(Vec::new());
            map[0].push(Tile {
                tile_type: TileType::Teleport(false),
                content: Content::None,
                elevation: 0,
            });
            map[0].push(Tile {
                tile_type: TileType::Grass,
                content: Tree(3),
                elevation: 0,
            });
            map[0].push(Tile {
                tile_type: TileType::Grass,
                content: Rock(10),
                elevation: 0,
            });

            map.push(Vec::new());
            map[1].push(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            });
            map[1].push(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            });
            map[1].push(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            });

            map.push(Vec::new());
            map[2].push(Tile {
                tile_type: TileType::Grass,
                content: Content::Bank(0..10),
                elevation: 0,
            });
            map[2].push(Tile {
                tile_type: TileType::Grass,
                content: Content::Market(2),
                elevation: 0,
            });
            map[2].push(Tile {
                tile_type: TileType::Teleport(false),
                content: Content::None,
                elevation: 0,
            });

            let environmental_conditions =
                EnvironmentalConditions::new(&[WeatherType::Sunny, WeatherType::Rainy], 15, 12);
            (map, (0, 0), environmental_conditions.unwrap(), 100.0, None)
        }
    }
    let mut generator: WorldGenerator = WorldGenerator::new();

    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            let view = robot_view(self, world);
            let map = robot_map(world).unwrap();

            let mut expected_map: Vec<Vec<Option<Tile>>> = Vec::new();
            for _ in 0..view.len() {
                expected_map.push(Vec::new());
            }

            expected_map[0].push(Some(Tile {
                tile_type: TileType::Teleport(true),
                content: Content::None,
                elevation: 0,
            }));
            expected_map[0].push(Some(Tile {
                tile_type: TileType::Grass,
                content: Tree(3),
                elevation: 0,
            }));
            expected_map[0].push(None);

            expected_map[1].push(Some(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            }));
            expected_map[1].push(Some(Tile {
                tile_type: TileType::Street,
                content: Content::None,
                elevation: 0,
            }));
            expected_map[1].push(None);

            expected_map[2].push(None);
            expected_map[2].push(None);
            expected_map[2].push(None);

            assert_eq!(map, expected_map);
        }

        fn handle_event(&mut self, event: Event) {
            match event {
                | Event::Terminated => {}
                | _ => {}
            }
        }

        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
    }

    let my_robot = MyRobot(Robot::new());

    struct Tool {}

    impl Tools for Tool {}

    let mut runner = Runner::new(Box::new(my_robot), &mut generator);
    if runner.is_err() {
        exit(1);
    }

    let mut res = Err(NoContent);
    res = runner.as_mut().unwrap().game_tick();
    assert_eq!(res.is_ok(), true);
}
