use crate::{
    interface::{craft, discover_tiles, look_at_sky, teleport, where_am_i},
    world::score::ScoreCounter,
};

use super::*;

// struct InterfaceRobot(Robot);

const INTERFACE_WORLD_SIZE: usize = 3;

// This will test the go interface for cost based on elevation and tiletype:
/**************************************************************************
*  MAP:
*    ______________________________________
*   |            |            |            |
*   |   Street   | Shallow W. |  DeepWater |
*   |    3 el    |   2 el     |    1 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Grass   |    Sand    |    Hill    |
*   |    3 el    |    2 el    |    4 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |   Lava     |    Snow    |  Mountain  |
*   |   3 el     |    7 el    |    9 el    |
*   |____________|____________|____________|
*
*
*/
fn generate_map_go_interface() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile {
            tile_type: Street,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: ShallowWater,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: DeepWater,
            content: Content::None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: Grass,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: Sand,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: Hill,
            content: Content::None,
            elevation: 4,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: Lava,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: Snow,
            content: Content::None,
            elevation: 7,
        },
        Tile {
            tile_type: Mountain,
            content: Content::None,
            elevation: 9,
        },
    ]);
    map
}

#[test]
fn go_interface_test() {
    let mut robot = TestRobot(Robot::new());
    let map = generate_map_go_interface();
    let score_counter = ScoreCounter::new(1.0, &map, None);
    let mut world = World {
        map,
        dimension: INTERFACE_WORLD_SIZE,
        discoverable: INTERFACE_WORLD_SIZE / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter,
    };

    let result: Result<(Vec<Vec<Option<Tile>>>, (usize, usize)), LibError> =
        go(&mut robot, &mut world, Direction::Right);

    // Coordinate checks
    // did it move in the right position?
    assert_eq!(result.as_ref().unwrap().1, (0, 1));
    // check if the coordinate in the robot are changed
    assert_eq!(robot.get_coordinate(), &Coordinate::new(0, 1));

    let _ = go(&mut robot, &mut world, Direction::Right);
    robot.get_energy_mut().recharge_energy(100);

    // check the robot view
    let robot_should_view = vec![
        vec![None, None, None],
        vec![
            Some(Tile {
                tile_type: Street,
                content: Content::None,
                elevation: 3,
            }),
            Some(Tile {
                tile_type: ShallowWater,
                content: Content::None,
                elevation: 2,
            }),
            Some(Tile {
                tile_type: DeepWater,
                content: Content::None,
                elevation: 1,
            }),
        ],
        vec![
            Some(Tile {
                tile_type: Grass,
                content: Content::None,
                elevation: 3,
            }),
            Some(Tile {
                tile_type: Sand,
                content: Content::None,
                elevation: 2,
            }),
            Some(Tile {
                tile_type: Hill,
                content: Content::None,
                elevation: 4,
            }),
        ],
    ];
    assert_eq!(result.as_ref().unwrap().0, robot_should_view);

    // Trying to go up from (0, 1) will result in OutOfBounds lib error
    let result = go(&mut robot, &mut world, Direction::Up);
    assert_eq!(result, Err(OutOfBounds));
    // Check if robot is in same position
    assert_eq!(robot.get_coordinate(), &Coordinate::new(0, 1));

    // check for energy consumption with same elevation and new coordinate

    let _ = go(&mut robot, &mut world, Direction::Down);
    let _should_cost = TileType::properties(&Sand).cost();
    assert_eq!(robot.get_coordinate(), &Coordinate::new(1, 1));

    // The robot is *not* allowed to go in deep water
    // Go to the deep water
    let _ = go(&mut robot, &mut world, Direction::Down);
    let _ = go(&mut robot, &mut world, Direction::Right);
    let _ = go(&mut robot, &mut world, Direction::Up);

    let result = go(&mut robot, &mut world, Direction::Up);
    // println!("{:#?}", world.map);
    println!("Going up, target: {:?}", robot.get_coordinate()); // did not go up
    assert_eq!(result, Err(CannotWalk));

    // But it will be blocked there!!!
    // let result = go(&mut robot, &mut world, Direction::Down);
    // assert_eq!(Err(CannotWalk), result);

    // Let's move out the robot from the deep water and make him consume all energy
    *robot.get_energy_mut() = Energy::new(50);
    *robot.get_coordinate_mut() = Coordinate::new(1, 1);
    let _ = go(&mut robot, &mut world, Direction::Down);
    let _ = go(&mut robot, &mut world, Direction::Up);
    let result = go(&mut robot, &mut world, Direction::Down);
    assert_eq!(result, Err(NotEnoughEnergy));
}

// This will test the teleport interface:
/**************************************************************************
*  MAP:
*    ______________________________________
*   |            |            |            |
*   | Teleport(V)| Teleport(F)|  DeepWater |
*   |    3 el    |   2 el     |    1 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |    Grass   |    Sand    |    Hill    |
*   |    3 el    |    2 el    |    4 el    |
*   |____________|____________|____________|
*   |            |            |            |
*   |   Lava     |    Snow    | Teleport(V)|
*   |   3 el     |    7 el    |    9 el    |
*   |____________|____________|____________|
*
*
*/
fn generate_map_teleport_interface() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile {
            tile_type: Teleport(true),
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: Teleport(false),
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: DeepWater,
            content: Content::None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: Grass,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: Sand,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: Hill,
            content: Content::None,
            elevation: 4,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: Lava,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: Snow,
            content: Content::None,
            elevation: 7,
        },
        Tile {
            tile_type: Teleport(true),
            content: Content::None,
            elevation: 9,
        },
    ]);
    map
}

#[test]
fn teleport_interface_test() {
    let mut robot = TestRobot(Robot::new());
    let map = generate_map_teleport_interface();
    let score_counter = ScoreCounter::new(1.0, &map, None);
    let mut world = World {
        map,
        dimension: INTERFACE_WORLD_SIZE,
        discoverable: INTERFACE_WORLD_SIZE / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter,
    };

    // check if the attribute is set to true it will teleport like there is no tomorrow
    let result = teleport(&mut robot, &mut world, (2, 2));

    // check the robot view
    let robot_should_view = vec![
        vec![
            Some(Tile {
                tile_type: Sand,
                content: Content::None,
                elevation: 2,
            }),
            Some(Tile {
                tile_type: Hill,
                content: Content::None,
                elevation: 4,
            }),
            None,
        ],
        vec![
            Some(Tile {
                tile_type: Snow,
                content: Content::None,
                elevation: 7,
            }),
            Some(Tile {
                tile_type: Teleport(true),
                content: Content::None,
                elevation: 9,
            }),
            None,
        ],
        vec![None, None, None],
    ];
    assert_eq!(result.unwrap().0, robot_should_view);

    // let's go back
    let result = teleport(&mut robot, &mut world, (0, 0));
    // check the robot view
    let robot_should_view = vec![
        vec![None, None, None],
        vec![
            None,
            Some(Tile {
                tile_type: Teleport(true),
                content: Content::None,
                elevation: 3,
            }),
            Some(Tile {
                tile_type: Teleport(false),
                content: Content::None,
                elevation: 2,
            }),
        ],
        vec![
            None,
            Some(Tile {
                tile_type: Grass,
                content: Content::None,
                elevation: 3,
            }),
            Some(Tile {
                tile_type: Sand,
                content: Content::None,
                elevation: 2,
            }),
        ],
    ];
    assert_eq!(result.unwrap().0, robot_should_view);

    // let's try to teleport on a teleport(false)
    let result = teleport(&mut robot, &mut world, (0, 1));
    assert_eq!(result, Err(OperationNotAllowed));

    // let's make it visible
    let _result = go(&mut robot, &mut world, Direction::Right);
    let _result = go(&mut robot, &mut world, Direction::Left);

    // let's try again
    let result = teleport(&mut robot, &mut world, (0, 1));
    assert_eq!(result.unwrap().1, (0, 1));

    // out of bounds check
    let result = teleport(&mut robot, &mut world, (0, 10));
    assert_eq!(result, Err(OutOfBounds));

    // let's consume all energy
    *robot.get_energy_mut() = Energy::new(30);
    let _result = teleport(&mut robot, &mut world, (2, 2));
    let result = teleport(&mut robot, &mut world, (0, 0));
    assert_eq!(result, Err(NotEnoughEnergy));
}

// This will test the destroy interface:
/**************************************************************************
*  MAP:
*   - every tile has 0 elevation
*    _________________________
*   |            |            |
*   |    Grass   |    Grass   |
*   |   Rock(0)  |   Tree(2)  |
*   |____________|____________|
*   |            |            |
*   |    Grass   |    Sand    |
*   |   Rock(0)  |   Bin(2)   |
*   |____________|____________|
*
*
*/
fn generate_map_destroy_interface() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let elevation = 0;
    map.push(vec![
        Tile {
            tile_type: Grass,
            content: Content::Rock(0),
            elevation,
        },
        Tile {
            tile_type: Grass,
            content: Content::Tree(2),
            elevation,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: Grass,
            content: Content::Rock(2),
            elevation,
        },
        Tile {
            tile_type: Sand,
            content: Content::Bin(0..0),
            elevation,
        },
    ]);
    map
}

fn return_world_for_destroy_test(backpack_size: usize) -> (World, TestRobot) {
    let map = generate_map_destroy_interface();
    let score_counter = ScoreCounter::new(1.0, &map, None);

    let robot = TestRobot(Robot {
        energy: Energy::new(MAX_ENERGY_LEVEL),
        backpack: BackPack::new(backpack_size),
        coordinate: Coordinate::new(0, 0),
    });
    (
        World {
            map,
            dimension: 2,
            discoverable: 2 / 10 + 1,
            environmental_conditions: generate_sunny_weather(),
            score_counter,
        },
        robot,
    )
}

const BACKPACK_SIZE: usize = 20;

#[test]
fn destroy_interface_test_rock() {
    let (mut world, mut robot) = return_world_for_destroy_test(BACKPACK_SIZE);

    assert_eq!(world.map[1][0].content, Rock(2));
    // let's destroy some rock CHYEAHHH 💪
    let result = destroy(&mut robot, &mut world, Direction::Down);

    assert_eq!(result, Ok(2));
    // check the backpack is updated
    assert_eq!(robot.get_backpack().contents.get(&Rock(0)), Some(&2));
    // check energy is updated
    let energy_left = &Energy::new(MAX_ENERGY_LEVEL - Content::properties(&Rock(0)).cost());
    assert_eq!(robot.get_energy(), energy_left);
    // the rock content is disappeared from the world
    assert_eq!(world.map[1][0].content, Content::None);

    // this should return NoContent
    let result = destroy(&mut robot, &mut world, Direction::Down);
    assert_eq!(result, Err(NoContent));
    //this should not cost
    assert_eq!(robot.get_energy(), energy_left);
}

#[test]
fn destroy_interface_test_tree() {
    let (mut world, mut robot) = return_world_for_destroy_test(BACKPACK_SIZE);

    assert_eq!(world.map[0][1].content, Tree(2));
    // let's destroy some 🌳🌳
    let result = destroy(&mut robot, &mut world, Direction::Right);
    assert_eq!(result, Ok(2));
    // check the backpack is updated
    assert_eq!(robot.get_backpack().contents.get(&Tree(0)), Some(&2));
    // check energy is updated
    let energy_left = &Energy::new(MAX_ENERGY_LEVEL - Content::properties(&Tree(0)).cost());
    assert_eq!(robot.get_energy(), energy_left);
    // the 🌳 content is disappeared from the world
    assert_eq!(world.map[0][1].content, Content::None);

    // this should return NoContent
    let result = destroy(&mut robot, &mut world, Direction::Right);
    assert_eq!(result, Err(NoContent));

    //this should not cost
    assert_eq!(robot.get_energy(), energy_left);
}

#[test]
fn destroy_interface_test_not_enough_space() {
    let (mut world, mut robot) = return_world_for_destroy_test(0);

    assert_eq!(world.map[0][1].content, Tree(2));
    // let's destroy some 🌳🌳
    let result = destroy(&mut robot, &mut world, Direction::Right);

    // UUOOPSS I don't have spaceeee
    assert_eq!(result, Err(NotEnoughSpace(0)));

    // energy shouldn't have been subtracted
    let energy_left = &Energy::new(MAX_ENERGY_LEVEL);
    assert_eq!(robot.get_energy(), energy_left);
    // the 🌳 content should still be in the world
    assert_eq!(world.map[0][1].content, Content::Tree(2));
}

#[test]
fn destroy_interface_issue110_out_of_bounds() {
    let (mut world, mut robot) = return_world_for_destroy_test(BACKPACK_SIZE);

    // destroy outside the map should return OutOfBounds
    let result = destroy(&mut robot, &mut world, Direction::Left);
    assert_eq!(result, Err(OutOfBounds));

    // energy should stay the same
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL));
}

#[test]
fn destroy_interface_test_cannot_destroy() {
    let (mut world, mut robot) = return_world_for_destroy_test(BACKPACK_SIZE);

    let tile_type = world.map[robot.get_coordinate().get_row()][robot.get_coordinate().get_col()].tile_type;
    let _result = go(&mut robot, &mut world, Direction::Right);

    // destroy a bin should return CannotDestroy OK!
    let result = destroy(&mut robot, &mut world, Direction::Down);
    assert_eq!(result, Err(CannotDestroy));

    // energy should stay the same
    let mut cost = tile_type.properties().cost();
    let environmental_conditions = look_at_sky(&world);
    cost = calculate_cost_go_with_environment(cost, environmental_conditions, tile_type.clone());
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));
}

fn generate_map_discover_tiles_interface() -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    // let content = Content::None;
    map.push(vec![
        Tile {
            tile_type: Street,
            content: Content::Tree(3),
            elevation: 3,
        },
        Tile {
            tile_type: ShallowWater,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: DeepWater,
            content: Content::None,
            elevation: 1,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: Grass,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: Sand,
            content: Content::None,
            elevation: 2,
        },
        Tile {
            tile_type: Hill,
            content: Content::None,
            elevation: 4,
        },
    ]);
    map.push(vec![
        Tile {
            tile_type: Lava,
            content: Content::None,
            elevation: 3,
        },
        Tile {
            tile_type: Snow,
            content: Content::None,
            elevation: 7,
        },
        Tile {
            tile_type: Mountain,
            content: Content::None,
            elevation: 9,
        },
    ]);
    map
}

#[test]
fn discover_tiles_interface_test_1() {
    let mut robot = TestRobot(Robot::new());
    let map = generate_map_discover_tiles_interface();
    let mut world = World {
        map,
        dimension: INTERFACE_WORLD_SIZE,
        discoverable: 20,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };

    let to_discovery: Vec<(usize, usize)> = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
        (3, 3),
    ];
    let discovered = discover_tiles(&mut robot, &mut world, &to_discovery);

    let mut expected: HashMap<(usize, usize), Option<Tile>> = HashMap::new();

    expected.insert(
        (0, 0),
        Some(Tile {
            tile_type: TileType::Street,
            content: Content::Tree(3),
            elevation: 3,
        }),
    );
    expected.insert(
        (0, 1),
        Some(Tile {
            tile_type: TileType::ShallowWater,
            content: Content::None,
            elevation: 2,
        }),
    );
    expected.insert(
        (0, 2),
        Some(Tile {
            tile_type: TileType::DeepWater,
            content: Content::None,
            elevation: 1,
        }),
    );
    expected.insert(
        (1, 0),
        Some(Tile {
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 3,
        }),
    );
    expected.insert(
        (1, 1),
        Some(Tile {
            tile_type: TileType::Sand,
            content: Content::None,
            elevation: 2,
        }),
    );
    expected.insert(
        (1, 2),
        Some(Tile {
            tile_type: TileType::Hill,
            content: Content::None,
            elevation: 4,
        }),
    );
    expected.insert(
        (2, 0),
        Some(Tile {
            tile_type: TileType::Lava,
            content: Content::None,
            elevation: 3,
        }),
    );
    expected.insert(
        (2, 1),
        Some(Tile {
            tile_type: TileType::Snow,
            content: Content::None,
            elevation: 7,
        }),
    );
    expected.insert(
        (2, 2),
        Some(Tile {
            tile_type: TileType::Mountain,
            content: Content::None,
            elevation: 9,
        }),
    );
    expected.insert((3, 3), None);

    assert_eq!(discovered.unwrap(), expected);
}

#[test]
fn discover_tiles_interface_test_2() {
    let mut robot = TestRobot(Robot::new());
    let map = generate_map_go_interface();
    let mut world = World {
        map,
        dimension: INTERFACE_WORLD_SIZE,
        discoverable: INTERFACE_WORLD_SIZE / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };

    let to_discovery: Vec<(usize, usize)> = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
        (3, 3),
    ];
    let discovered = discover_tiles(&mut robot, &mut world, &to_discovery);

    assert_eq!(discovered, Err(NoMoreDiscovery))
}

#[test]
fn discover_tiles_interface_test_3() {
    let mut robot = TestRobot(Robot::new());
    let map = generate_map_go_interface();
    let mut world = World {
        map,
        dimension: INTERFACE_WORLD_SIZE,
        discoverable: INTERFACE_WORLD_SIZE / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };

    while robot.get_energy().has_enough_energy(2) {
        let _ = go(&mut robot, &mut world, Direction::Down);
        let _ = go(&mut robot, &mut world, Direction::Up);
    }

    let mut to_discovery: Vec<(usize, usize)> = Vec::new();
    to_discovery.push((0, 0));
    let discovered = discover_tiles(&mut robot, &mut world, &to_discovery);

    assert_eq!(discovered, Err(NotEnoughEnergy))
}

// This will test the put interface :
/**************************************************************************
*  MAP:
*    ______________________________________
*   |            |            |            |
*   |  TileType  |     🤖     |  TileType  |
*   |  content   |  content   |  content   |
*   |____________|____________|____________|
*   |            |            |            |
*   |  TileType  |  TileType  |  TileType  |
*   |  content   | (receiver) |  content   |
*   |____________|____________|____________|
*   |            |            |            |
*   |  TileType  |  TileType  |  TileType  |
*   |  content   |  content   |  content   |
*   |____________|____________|____________|
*
*   - content_in_backpack: a vec with the content to put in the backpack initially
*   - backpack_size
*   - content_that_receive: the content that will receive stuff from
*     the put operation it will be in (1, 1)
*   - content_to_destroy: content in all the other tile (not (1, 1))
*   - robot start position -> (0,1)
*/
fn generate_map_robot(
    content_in_backpack: Vec<(Content, usize)>,
    backpack_size: usize,
    content_that_receive: Content,
    content: Content,
    tile_type: TileType,
    map_size: usize,
) -> (World, TestRobot) {
    let elevation = 0;

    let map = (0..map_size)
        .into_iter()
        .map(|row| {
            (0..map_size)
                .into_iter()
                .map(|col| match (row, col) {
                    | (1, 1) => Tile {
                        tile_type: tile_type.clone(),
                        content: content_that_receive.clone(),
                        elevation,
                    },
                    | (_, _) => Tile {
                        tile_type: tile_type.clone(),
                        content: content.clone(),
                        elevation,
                    },
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    let robot = TestRobot(Robot {
        energy: Energy::new(MAX_ENERGY_LEVEL),
        backpack: generate_backpack(content_in_backpack, backpack_size),
        coordinate: Coordinate::new(0, 1),
    });
    let score_counter = ScoreCounter::new(1.0, &map, None);
    (
        World {
            map,
            dimension: map_size,
            discoverable: 2 / 10 + 1,
            environmental_conditions: generate_sunny_weather(),
            score_counter,
        },
        robot,
    )
}

fn generate_backpack(contents_to_add: Vec<(Content, usize)>, size: usize) -> BackPack {
    BackPack {
        size,
        contents: contents_to_add.into_iter().collect(),
    }
}

#[test]
fn put_interface_test_coin_bank() {
    let (mut world, mut robot) = generate_map_robot(
        vec![(Coin(0), 20)],
        BACKPACK_SIZE,
        Bank(0..11),
        Content::None,
        Street,
        INTERFACE_WORLD_SIZE,
    );
    // Need a tile that can not store Coin
    world.map[0][0].tile_type = ShallowWater;
    let mut cost = 0;
    // Giving money to the bankers 💸
    (1..=2).into_iter().for_each(|i| {
        let result = put(&mut robot, &mut world, Coin(0), 5, Direction::Down);
        // check the quantity given to the bank
        assert_eq!(result, Ok(5));
        // check if from backpack is decreased
        assert_eq!(
            robot.get_backpack().get_contents().get(&Coin(0)),
            Some(&(BACKPACK_SIZE - 5 * i))
        );
        // check the bank content is updated
        assert_eq!(world.map[1][1].content, Bank(5 * i..11));
        // better check this bankers won't charge me extra
        cost += Content::properties(&Bank(0..0)).cost();
        assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));
    });

    // Bank is almost full
    let result = put(&mut robot, &mut world, Coin(0), 2, Direction::Down);
    assert_eq!(result, Ok(1));
    assert_eq!(robot.get_backpack().get_contents().get(&Coin(0)), Some(&9));

    // Bank is full
    let result = put(&mut robot, &mut world, Coin(0), 2, Direction::Down);
    assert_eq!(robot.get_backpack().get_contents().get(&Coin(0)), Some(&9));
    assert_eq!(result, Ok(0));

    // Everybody knows that tossing the Coin on Shallow Water will make Lakshmi, the goddess of wealth, enter their lives
    let result = put(&mut robot, &mut world, Coin(0), 1, Direction::Left);
    // It was a lie
    assert_eq!(result, Err(WrongContentUsed));

    // Let's try to put the extra coin on the Street
    let result = put(&mut robot, &mut world, Coin(0), 9, Direction::Right);
    assert_eq!(robot.get_backpack().get_contents().get(&Coin(0)), Some(&0));
    assert_eq!(result, Ok(9));

    // check if the content of the tile is updated
    assert_eq!(world.map[0][2].content, Coin(9));

    // leaving coins around will consume the cost of Coin
    cost += Content::properties(&Content::Coin(0)).cost();
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));
}

#[test]
fn put_interface_test_garbage_bin() {
    let (mut world, mut robot) = generate_map_robot(
        vec![(Garbage(0), 20)],
        BACKPACK_SIZE,
        Bin(0..19),
        Content::None,
        Street,
        INTERFACE_WORLD_SIZE,
    );

    // Need a tile that can not store Garbage
    world.map[0][0].tile_type = ShallowWater;

    // putting out the 🗑️
    let result = put(&mut robot, &mut world, Garbage(0), 20, Direction::Down);
    // check the quantity
    assert_eq!(result, Ok(19));
    // check if from backpack is decreased
    assert_eq!(
        robot.get_backpack().get_contents().get(&Garbage(0)),
        Some(&(BACKPACK_SIZE - 19))
    );

    // check if the content of the tile is updated
    assert_eq!(world.map[1][1].content, Bin(19..19));

    // check energy
    let mut cost = Content::properties(&Bin(0..0)).cost();
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));

    // Toss the Garbage on Shallow Water
    let result = put(&mut robot, &mut world, Garbage(0), 1, Direction::Left);
    // Oi, mate! Quit chuckin' your rubbish in the river, ya dental flosser!
    assert_eq!(result, Err(WrongContentUsed));

    // Let's try to put the extra garbage on a tile with no content
    let result = put(&mut robot, &mut world, Garbage(0), 9, Direction::Right);
    assert_eq!(robot.get_backpack().get_contents().get(&Garbage(0)), Some(&0));
    assert_eq!(result, Ok(1));

    // leaving garbage around will consume the cost of Garbage
    cost += Content::properties(&Garbage(0)).cost();
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));
}

#[test]
fn put_interface_test_tree_crate() {
    let (mut world, mut robot) = generate_map_robot(
        vec![(Tree(0), 20)],
        BACKPACK_SIZE,
        Crate(0..19),
        Content::None,
        Street,
        INTERFACE_WORLD_SIZE,
    );
    // Need a tile that can store wood!
    world.map[0][0].tile_type = Grass;
    // Storing 🪵
    let result = put(&mut robot, &mut world, Tree(0), 20, Direction::Down);
    // check the quantity
    assert_eq!(result, Ok(19));
    // check if from backpack is decreased
    assert_eq!(
        robot.get_backpack().get_contents().get(&Tree(0)),
        Some(&(BACKPACK_SIZE - 19))
    );

    // check if the content of the tile is updated
    assert_eq!(world.map[1][1].content, Crate(19..19));

    // check energy
    let mut cost = Content::properties(&Crate(0..0)).cost();
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));

    // Let's try to put the extra wood on Street
    let result = put(&mut robot, &mut world, Tree(0), 1, Direction::Right);
    assert_eq!(result, Err(WrongContentUsed));

    // On Grass should work
    let result = put(&mut robot, &mut world, Tree(0), 1, Direction::Left);
    assert_eq!(result, Ok(1));
    assert_eq!(robot.get_backpack().get_contents().get(&Tree(0)), Some(&0));

    // leaving garbage around will consume the cost of Tree
    cost += Content::properties(&Tree(0)).cost();
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));
}

#[test]
fn put_interface_test_fire() {
    let (mut world, mut robot) = generate_map_robot(
        vec![(Fire, 20)],
        BACKPACK_SIZE,
        Tree(0),
        Content::None,
        Grass,
        INTERFACE_WORLD_SIZE,
    );
    // Need a tile that won't go on 🔥!
    world.map[0][0].tile_type = Street;

    // 🌳 will burn
    let result = put(&mut robot, &mut world, Fire, 20, Direction::Down);
    // with fire quantity is always 1
    assert_eq!(result, Ok(1));
    // check if from backpack is decreased
    assert_eq!(
        robot.get_backpack().get_contents().get(&Fire),
        Some(&(BACKPACK_SIZE - 1))
    );

    // check if the content of the tile is updated
    assert_eq!(world.map[1][1].content, Fire);

    // check energy
    let mut cost = Content::properties(&Fire).cost();
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));

    // Let's try to put on 🔥 the Street
    let result = put(&mut robot, &mut world, Fire, 1, Direction::Left);
    assert_eq!(result, Err(WrongContentUsed));

    // On Grass will burn
    let result = put(&mut robot, &mut world, Fire, 1, Direction::Right);
    assert_eq!(result, Ok(1));
    assert_eq!(robot.get_backpack().get_contents().get(&Fire), Some(&18));
    assert_eq!(world.map[0][2].content, Fire);

    // burning staff around will consume the cost of Fire
    cost += Content::properties(&Fire).cost();
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));
}

#[test]
fn put_interface_test_fire_extinguish() {
    let (mut world, mut fire_robot) = generate_map_robot(
        vec![(Water(0), 20)],
        BACKPACK_SIZE,
        Fire,
        Fire,
        Grass,
        INTERFACE_WORLD_SIZE,
    );

    // 🚒 🧯
    // Excuse me, good chaps, shall we attend to this fire dispersing water in every conceivable direction?
    (0..3).into_iter().for_each(|_| {
        Direction::iter().for_each(|dir| {
            let _ = put(&mut fire_robot, &mut world, Water(0), 1, dir);
        });
        // Water brigade, assemble!
        let _ = go(&mut fire_robot, &mut world, Direction::Down);
    });

    // Time for checking!
    // total cost is 9 times cost of water plus the cost of moving on Grass on a Sunny Day?
    let cost = Content::properties(&Water(0)).cost() * 9 + TileType::properties(&Grass).cost() * 2;
    assert_eq!(fire_robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));
    // the fireman should be in position (2, 1)
    assert_eq!(fire_robot.get_coordinate(), &Coordinate::new(2, 1));
    // All the tiles should contain? I guess Content::None
    world
        .map
        .iter()
        .for_each(|row| row.iter().for_each(|tile| assert_eq!(tile.content, Content::None)));
}

#[test]
fn put_interface_test_asphalting() {
    let (mut world, mut asphalt_robot) = generate_map_robot(
        vec![(Rock(0), 20)],
        BACKPACK_SIZE,
        Content::Tree(0),
        Content::None,
        Grass,
        INTERFACE_WORLD_SIZE,
    );

    // to check also the hill type
    world.map[0][0].tile_type = Hill;
    world.map[0][0].content = Garbage(0);

    // can asphalt hill, NOW WITH CONTENT!
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Left);
    assert_eq!(world.map[0][0].tile_type, Street);
    assert_eq!(result, Ok(1));

    // look there is a coin on the grass
    world.map[0][2].content = Coin(0);

    // can asphalt grass, NOW WITH CONTENT!
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Right);
    assert_eq!(world.map[0][2].tile_type, Street);
    assert_eq!(result, Ok(1));

    // and it started snowing... just on my left
    world.map[0][2].tile_type = Snow;

    // can asphalt snow, NOW WITH CONTENT!
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Right);
    assert_eq!(world.map[0][2].tile_type, Street);
    assert_eq!(result, Ok(1));

    // and the street on my right is now sand!?
    world.map[0][0].tile_type = Sand;

    // can asphalt sand, NOW WITH CONTENT!
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Left);
    assert_eq!(world.map[0][0].tile_type, Street);
    assert_eq!(result, Ok(1));
    // cannot asphalt!!! there is a tree hugger over there
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Down);
    assert_eq!(world.map[1][1].tile_type, Grass);
    assert_eq!(world.map[1][1].content, Tree(0));
    assert_eq!(result, Err(MustDestroyContentFirst));

    // oh my, the two streets became ShallowWater and Lava, how convenient!
    world.map[0][0].content = Content::None;
    world.map[0][2].content = Content::None;
    world.map[0][0].tile_type = Lava;
    world.map[0][2].tile_type = ShallowWater;

    // on water and lava too!?
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 3, Direction::Left);
    assert_eq!(world.map[0][0].tile_type, Street);
    assert_eq!(result, Ok(3));
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 2, Direction::Right);
    assert_eq!(world.map[0][2].tile_type, Street);
    assert_eq!(result, Ok(2));

    // why does it keep getting deeper?
    world.map[0][2].tile_type = DeepWater;

    // well on DeepWater too, not impressed anymore.
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 3, Direction::Right);
    assert_eq!(world.map[0][2].tile_type, Street);
    assert_eq!(result, Ok(3));

    // test on mountain
    world.map[0][0].tile_type = Mountain;
    let result = put(&mut asphalt_robot, &mut world, Content::None, 1, Direction::Left);
    assert_eq!(world.map[0][0].tile_type, Street);
    assert_eq!(
        *asphalt_robot.get_backpack().get_contents().get(&Rock(0)).unwrap(),
        8 + result.unwrap()
    );
    assert!(matches!(result, Ok(_)));

    // usual energy checks
    // i'm mathing one sec... (1+1+1+1 * rock cost) + (2 * rock cost) +
    // (3 * rock cost * 2 (deep water multiplier)) + (3 * rock_cost * 3 (lava multiplier))
    let mut cost = (Content::properties(&Rock(0)).cost() * 4)
        + (Content::properties(&Rock(0)).cost() * 2)
        + (Content::properties(&Rock(0)).cost() * 3 * 2)
        + (Content::properties(&Rock(0)).cost() * 3 * 3)
        + (Content::properties(&Rock(0)).cost() * result.unwrap() * 4);
    assert_eq!(asphalt_robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - cost));

    // eruption 🌋
    world.map[0][0].tile_type = Lava;
    // you'll need more than that
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Left);
    assert_eq!(world.map[0][0].tile_type, Lava);
    assert_eq!(result, Err(NotEnoughContentProvided));

    // let's try to put a rock on the street
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Right);
    assert_eq!(world.map[0][2].content, Rock(1));
    assert_eq!(result, Ok(1));
    cost += Content::properties(&Rock(0)).cost();

    // Energy Leak ⚡️
    let _ = asphalt_robot.get_energy_mut().consume_energy(MAX_ENERGY_LEVEL - cost);
    world.map[0][2].content = Content::None;
    // let's try to put a rock on the street with no energy
    let result = put(&mut asphalt_robot, &mut world, Rock(0), 1, Direction::Right);
    assert_eq!(world.map[0][2].content, Content::None);
    assert_eq!(result, Err(NotEnoughEnergy));
}

#[test]
fn put_interface_test_water_empty_tile() {
    let (mut world, mut water_robot) = generate_map_robot(
        vec![(Water(0), 20)],
        BACKPACK_SIZE,
        Content::Tree(0),
        Content::None,
        Grass,
        INTERFACE_WORLD_SIZE,
    );

    // Cannot put water on empty tiles
    let result = put(&mut water_robot, &mut world, Water(0), 1, Direction::Left);
    assert_eq!(result, Err(WrongContentUsed));
}

#[test]
fn put_destroy_interface_test_water_on_water() {
    let (mut world, mut water_robot) = generate_map_robot(
        vec![(Water(0), 20)],
        BACKPACK_SIZE,
        Content::None,
        Water(0),
        ShallowWater,
        INTERFACE_WORLD_SIZE,
    );

    // To check deep water as well
    world.map[0][0].tile_type = DeepWater;
    let max = Content::properties(&Water(0)).max();
    println!("the max amount property of water is {}", max);

    // From an old italian proverb: "It always rains where it's already wet!"
    // Putting 💦💦💦💦💦💦 into deep water!
    let result = put(&mut water_robot, &mut world, Water(0), 10, Direction::Left);
    assert_eq!(result, Ok(10));

    // Putting 💦💦💦💦💦💦 into shallow water!
    let result = put(&mut water_robot, &mut world, Water(0), 10, Direction::Right);
    assert_eq!(result, Ok(10));

    // Retreive the water from the DeepWater
    let result = destroy(&mut water_robot, &mut world, Direction::Left);
    assert_eq!(result, Ok(10));
    // Oi, mate, if ya strip the water from a deep or shallow water tile, it's gone, innit?
    // Content's gone, but mind ya, the tile still keeps its watery name;
    assert_eq!(world.map[0][0].content, Content::None);
    assert_eq!(world.map[0][0].tile_type, DeepWater);
    // You can put it back if you like!
    let result = put(&mut water_robot, &mut world, Water(0), 10, Direction::Left);
    assert_eq!(result, Ok(10));
    // let's take it back
    let _ = destroy(&mut water_robot, &mut world, Direction::Left);

    // When you nick water from a watery tile with no content you get a random vallue
    let result = destroy(&mut water_robot, &mut world, Direction::Left);
    // you get a random value! Just have a go and see what ya end up with!"
    // a bit hard being deterministic with random values
    if result.is_ok() {
        println!("retreived random amount of water of {}", result.unwrap());
    } else {
        match result.unwrap_err() {
            | NotEnoughSpace(c) => {
                println!("Not enough space! Added only {c}");
                assert_eq!(water_robot.get_backpack().contents.get(&Water(0)), Some(&BACKPACK_SIZE));
            }
            | _ => panic!(),
        }
    }
}

#[test]
fn put_interface_test_throw_content_on_fire() {
    let (mut world, mut robot) = generate_map_robot(
        vec![(Fish(0), 20)],
        BACKPACK_SIZE,
        Fire,
        Fire,
        Grass,
        INTERFACE_WORLD_SIZE,
    );

    // Putting Fish on a Tile with content Fire will destroy the fish
    let mut quantity = 5;
    // Always good to check the max amount that I can put
    let max = Content::properties(&Fish(0)).max();
    quantity = match max < quantity {
        | true => max,
        | false => quantity,
    };
    let result = put(&mut robot, &mut world, Fish(0), quantity, Direction::Left);
    assert_eq!(result, Ok(quantity));
    assert_eq!(
        robot.get_backpack().get_contents().get(&Fish(0)),
        Some(&(BACKPACK_SIZE - quantity))
    );
    assert_eq!(world.map[0][0].content, Fire);
}

#[test]
fn put_interface_test_market() {
    let backpack_size = 200;
    let (quantity_rocks, quantity_tree, quantity_fish) = (10, 10, 10);
    // let mut backpack_items = quantity_fish + quantity_rocks + quantity_tree;
    let content_backpack = vec![
        (Rock(0), quantity_rocks),
        (Tree(0), quantity_tree),
        (Fish(0), quantity_fish),
    ];
    let (mut world, mut robot) = generate_map_robot(
        content_backpack,
        backpack_size,
        Market(3),
        Tree(10),
        Grass,
        INTERFACE_WORLD_SIZE,
    );

    // these are hardcoded in the put interface in the market match
    // might they change? Hope not.
    const ROCK_PRICE: usize = 1;
    const TREE_PRICE: usize = 2;
    const FISH_PRICE: usize = 5;

    // In this case it does not look like there is a limit in the quantity you can dispose

    // Sellin some 🪨
    let result = put(&mut robot, &mut world, Rock(0), quantity_rocks, Direction::Down);
    // this will return the Coins generated not the quantity
    let mut earned_coins = quantity_rocks * ROCK_PRICE;
    let mut total_earned = earned_coins;
    assert_eq!(result, Ok(earned_coins));
    // let's check the backpack
    assert_eq!(robot.get_backpack().contents.get(&Rock(0)), Some(&(0)));
    assert_eq!(robot.get_backpack().contents.get(&Coin(0)), Some(&(earned_coins)));

    // Sellin some 🪵
    let result = put(&mut robot, &mut world, Tree(0), quantity_tree, Direction::Down);
    // this will return the Coins generated not the quantity
    earned_coins = quantity_tree * TREE_PRICE;
    total_earned += earned_coins;
    assert_eq!(result, Ok(earned_coins));
    // let's check the backpack
    assert_eq!(robot.get_backpack().contents.get(&Tree(0)), Some(&(0)));
    assert_eq!(robot.get_backpack().contents.get(&Coin(0)), Some(&(total_earned)));

    // Sellin some 🐟
    let result = put(&mut robot, &mut world, Fish(0), quantity_fish - 1, Direction::Down);
    // this will return the Coins generated not the quantity
    earned_coins = (quantity_fish - 1) * FISH_PRICE;
    total_earned += earned_coins;
    assert_eq!(result, Ok(earned_coins));
    // let's check the backpack
    assert_eq!(robot.get_backpack().contents.get(&Fish(0)), Some(&(1)));
    assert_eq!(robot.get_backpack().contents.get(&Coin(0)), Some(&(total_earned)));

    // no more transaction!!
    let result = put(&mut robot, &mut world, Fish(0), 1, Direction::Down);
    assert_eq!(result, Err(OperationNotAllowed));
}

// This will test the put interface :
/**************************************************************************
*  To facilitate the testing I will use the following data to check if the view
*  return the right cells:
*   - Rock(val) => val = row index
*   - elevation = col index
*
*  MAP GENERATED WITH SIZE 3
*    ______________________________________
*   |            |            |            |
*   |   Rock(0)  |   Rock(0)  |   Rock(0)  |
*   |    el:0    |    el:1    |    el:2    |
*   |____________|____________|____________|
*   |            |            |            |
*   |   Rock(1)  |   Rock(1)  |   Rock(1)  |
*   |    el:0    |    el:1    |    el:2    |
*   |____________|____________|____________|
*   |            |            |            |
*   |   Rock(2)  |   Rock(2)  |   Rock(2)  |
*   |    el:0    |    el:1    |    el:2    |
*   |____________|____________|____________|
*/

fn generate_map_and_robot_for_one_direction(map_size: usize, initial_position: (usize, usize)) -> (World, TestRobot) {
    let map = (0..map_size)
        .into_iter()
        .map(|row| {
            (0..map_size)
                .into_iter()
                .map(|col| Tile {
                    tile_type: Grass,
                    content: Rock(row),
                    elevation: col,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let robot = TestRobot(Robot {
        energy: Energy::new(MAX_ENERGY_LEVEL),
        backpack: generate_backpack(vec![], BACKPACK_SIZE),
        coordinate: Coordinate::new(initial_position.0, initial_position.1),
    });
    let score_counter = ScoreCounter::new(1.0, &map, None);
    (
        World {
            map,
            dimension: map_size,
            discoverable: 2 / 10 + 1,
            environmental_conditions: generate_sunny_weather(),
            score_counter,
        },
        robot,
    )
}

#[test]
fn one_direction_view_cost_and_discovered_tiles() {
    let (mut world, mut robot) = generate_map_and_robot_for_one_direction(20, (9, 9));
    let mut should_cost = 0;
    (1..4).into_iter().for_each(|distance| {
        Direction::iter().for_each(|dir| {
            let one_direction_view = one_direction_view(&mut robot, &mut world, dir.clone(), distance);
            let view = one_direction_view.unwrap();
            if distance > 1 {
                should_cost += distance * 3
            }
            // Check that the quantity of tiles are right.
            assert_eq!(view.len() * view[0].len(), distance * 3);
            // Check the cost
            assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL - should_cost));
        })
    });

    // Energy consumption until here 60
    //let's set the energy left to 40 so to simulate a 100 max energy
    *robot.get_energy_mut() = Energy::new(40);
    // Check the result view looking up!
    let result = one_direction_view(&mut robot, &mut world, Direction::Up, 4);
    let should_view = (5..=8)
        .into_iter()
        .rev()
        .map(|row| {
            (8..=10)
                .into_iter()
                .map(|col| Tile {
                    tile_type: Grass,
                    content: Rock(row),
                    elevation: col,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(result, Ok(should_view));

    // Check the result view looking left!
    let result = one_direction_view(&mut robot, &mut world, Direction::Left, 4);
    let should_view = (8..=10)
        .into_iter()
        .map(|row| {
            (5..=8)
                .into_iter()
                .rev()
                .map(|col| Tile {
                    tile_type: Grass,
                    content: Rock(row),
                    elevation: col,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(result, Ok(should_view));

    // If you look in the same direction it will still cost
    let _ = one_direction_view(&mut robot, &mut world, Direction::Up, 4);
    // 60 - (3 * 4) * 4 times = 108 > 100 NotEnoughEnergy
    let one_direction_view = one_direction_view(&mut robot, &mut world, Direction::Up, 4);
    assert_eq!(one_direction_view, Err(NotEnoughEnergy));
}

#[test]
fn one_direction_view_test_check_view_left() {
    const WORLD_SIZE: usize = 3;

    let (mut world, mut robot) = generate_map_and_robot_for_one_direction(WORLD_SIZE, (0, 0));
    // let's look on our left from (0,0) it should return nothing!
    let mut should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Left, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our left from (2,0) it should return nothing!
    *robot.get_coordinate_mut() = Coordinate::new(WORLD_SIZE - 1, 0);
    should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Left, 1);
    assert_eq!(one_result, Ok(should_view));

    // robot in position (0, 2)
    *robot.get_coordinate_mut() = Coordinate::new(0, WORLD_SIZE - 1);
    should_view = vec![
        vec![Tile {
            tile_type: Grass,
            // row
            content: Rock(0),
            // col
            elevation: 1,
        }],
        vec![Tile {
            tile_type: Grass,
            // row
            content: Rock(1),
            // col
            elevation: 1,
        }],
    ];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Left, 1);
    assert_eq!(one_result, Ok(should_view));

    // robot in position (2, 2)
    *robot.get_coordinate_mut() = Coordinate::new(WORLD_SIZE - 1, WORLD_SIZE - 1);
    should_view = vec![
        vec![Tile {
            tile_type: Grass,
            // row
            content: Rock(1),
            // col
            elevation: 1,
        }],
        vec![Tile {
            tile_type: Grass,
            // row
            content: Rock(2),
            // col
            elevation: 1,
        }],
    ];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Left, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our right from the middle
    *robot.get_coordinate_mut() = Coordinate::new(WORLD_SIZE / 2, WORLD_SIZE / 2);
    should_view = vec![
        vec![Tile {
            tile_type: Grass,
            content: Rock(0),
            elevation: 0,
        }],
        vec![Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 0,
        }],
        vec![Tile {
            tile_type: Grass,
            content: Rock(2),
            elevation: 0,
        }],
    ];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Left, 1);
    assert_eq!(one_result, Ok(should_view));

    // All this operation with distance 1 should be free
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL));
}

#[test]
fn one_direction_view_test_check_view_right() {
    const WORLD_SIZE: usize = 3;

    // let's look on our right from (0,0) with distance 1
    let (mut world, mut robot) = generate_map_and_robot_for_one_direction(WORLD_SIZE, (0, 0));
    let should_view = vec![
        vec![Tile {
            tile_type: Grass,
            content: Rock(0),
            elevation: 1,
        }],
        vec![Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 1,
        }],
    ];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Right, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our right from (2, 0) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(2, 0);
    let mut should_view = vec![
        vec![Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 1,
        }],
        vec![Tile {
            tile_type: Grass,
            content: Rock(2),
            elevation: 1,
        }],
    ];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Right, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our right from (0,2) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(0, 2);
    // should be an empty view we are in the right border
    should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Right, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our right from (2,2) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(2, 2);
    // should be an empty view we are in the right border
    should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Right, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our right from the middle
    *robot.get_coordinate_mut() = Coordinate::new(1, 1);
    should_view = vec![
        vec![Tile {
            tile_type: Grass,
            content: Rock(0),
            elevation: 2,
        }],
        vec![Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 2,
        }],
        vec![Tile {
            tile_type: Grass,
            content: Rock(2),
            elevation: 2,
        }],
    ];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Right, 1);

    assert_eq!(one_result, Ok(should_view));
    // All this operation are free
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL));
}

#[test]
fn one_direction_view_test_check_view_up() {
    const WORLD_SIZE: usize = 3;

    // let's look up from (0,0) with distance 1
    let (mut world, mut robot) = generate_map_and_robot_for_one_direction(WORLD_SIZE, (0, 0));
    let mut should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Up, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our up from (2, 0) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(2, 0);
    should_view = vec![vec![
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 0,
        },
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 1,
        },
    ]];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Up, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our right from (0,2) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(0, 2);
    should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Up, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our right from (2,2) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(2, 2);
    should_view = vec![vec![
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 2,
        },
    ]];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Up, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look up from the middle
    *robot.get_coordinate_mut() = Coordinate::new(1, 1);
    should_view = vec![vec![
        Tile {
            tile_type: Grass,
            content: Rock(0),
            elevation: 0,
        },
        Tile {
            tile_type: Grass,
            content: Rock(0),
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: Rock(0),
            elevation: 2,
        },
    ]];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Up, 1);

    assert_eq!(one_result, Ok(should_view));
    // All this operation are free
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL));
}

#[test]
fn one_direction_view_test_check_view_down() {
    const WORLD_SIZE: usize = 3;

    // let's look down from (0,0) with distance 1
    let (mut world, mut robot) = generate_map_and_robot_for_one_direction(WORLD_SIZE, (0, 0));
    let mut should_view = vec![vec![
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 0,
        },
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 1,
        },
    ]];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Down, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our down from (2, 0) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(2, 0);
    should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Down, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on our down from (0,2) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(0, 2);
    // should be an empty view we are in the right border
    should_view = vec![vec![
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: Rock(1),
            elevation: 2,
        },
    ]];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Down, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look on down from (2,2) with distance 1
    *robot.get_coordinate_mut() = Coordinate::new(2, 2);
    should_view = vec![];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Down, 1);
    assert_eq!(one_result, Ok(should_view));

    // let's look down from the middle
    *robot.get_coordinate_mut() = Coordinate::new(1, 1);
    should_view = vec![vec![
        Tile {
            tile_type: Grass,
            content: Rock(2),
            elevation: 0,
        },
        Tile {
            tile_type: Grass,
            content: Rock(2),
            elevation: 1,
        },
        Tile {
            tile_type: Grass,
            content: Rock(2),
            elevation: 2,
        },
    ]];
    let one_result = one_direction_view(&mut robot, &mut world, Direction::Down, 1);

    assert_eq!(one_result, Ok(should_view));
    // All this operation are free
    assert_eq!(robot.get_energy(), &Energy::new(MAX_ENERGY_LEVEL));
}

#[test]
fn where_am_i_test() {
    let (world, robot) = generate_map_robot(vec![], 0, Content::None, Content::None, Grass, 9);
    let tile = Tile {
        tile_type: Grass,
        content: Content::None,
        elevation: 0,
    };
    let should_view = vec![
        vec![None, None, None],
        vec![Some(tile.clone()), Some(tile.clone()), Some(tile.clone())],
        vec![Some(tile.clone()), Some(tile.clone()), Some(tile.clone())],
    ];
    assert_eq!(where_am_i(&robot, &world), (should_view, (0, 1)));
}

#[test]
fn craft_interface_test() {
    let mut robot = TestRobot(Robot {
        energy: Energy::new(MAX_ENERGY_LEVEL),
        backpack: generate_backpack(vec![(Rock(0), 15)], BACKPACK_SIZE),
        coordinate: Coordinate::new(0, 0),
    });
    // Tree is not craftable
    let mut result = craft(&mut robot, Tree(0));
    assert_eq!(result, Err(NotCraftable));

    // I have 15 rocks, let's make some garbage!
    (0..5).into_iter().for_each(|_| {
        result = craft(&mut robot, Garbage(0));
    });
    // if succeded will return Garbage(0)
    assert_eq!(result, Ok(Garbage(0)));
    // check if added in backpack, with 15 rocks we should make 5 Garbage!
    assert_eq!(robot.get_backpack().get_contents().get(&Garbage(0)), Some(&5));
    // check if the rocks are removed from backpack
    assert_eq!(robot.get_backpack().get_contents().get(&Rock(0)), Some(&0));
    // check if it costed energy
    let mut energy_left = MAX_ENERGY_LEVEL - Content::properties(&Garbage(0)).cost() * 5;
    assert_eq!(robot.get_energy(), &Energy::new(energy_left));

    // if not enough resourses should return NotCraftable
    let result = craft(&mut robot, Garbage(0));
    assert_eq!(result, Err(NotCraftable));

    // with garbage we can make money
    let result = craft(&mut robot, Coin(0));
    // if succeded will return Coin(0)
    assert_eq!(result, Ok(Coin(0)));
    // check if Coin added in backpack
    assert_eq!(robot.get_backpack().get_contents().get(&Coin(0)), Some(&1));
    // check if fish removed from backpack
    assert_eq!(robot.get_backpack().get_contents().get(&Garbage(0)), Some(&0));
    // check if it costed energy actually coin is made for freeeee
    energy_left -= Content::properties(&Coin(0)).cost();
    assert_eq!(robot.get_energy(), &Energy::new(energy_left));

    // energy leak
    *robot.get_energy_mut() = Energy::new(0);
    // let's add the content we need to make Garbage
    *robot.get_backpack_mut() = generate_backpack(vec![(Rock(0), 3)], BACKPACK_SIZE);
    let result = craft(&mut robot, Garbage(0));
    assert_eq!(result, Err(NotEnoughEnergy));

    // For how are the recipe now, this function won't return Err(NotEnoughSpace)
    // so I could not test against that.
}

#[test]
#[ignore]
fn destroy_fire_will_cause_panic() {
    let (mut world, mut robot) = generate_map_robot(vec![], 20, Fire, Fire, Grass, 3);

    // destroy fire will cause the destroy interface to panic!
    let _result = destroy(&mut robot, &mut world, Direction::Right);
}
