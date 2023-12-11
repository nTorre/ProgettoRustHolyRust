use super::*;

#[test]
fn can_not_go_up_and_left_from_tile_zero_zero() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::None, 4),
        dimension: 4,
        discoverable: 4 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };

    let robot = TestRobot(Robot::new());
    assert_eq!(go_allowed(&robot, &world, &Direction::Up), Err(LibError::OutOfBounds));
    assert_eq!(go_allowed(&robot, &world, &Direction::Left), Err(LibError::OutOfBounds));
}

#[test]
fn can_not_move_anywhere_if_world_size_is_one() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::None, 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };

    // Assuming the Robot::new method will set (0, 0) as coordinates.
    let robot = TestRobot(Robot::new());
    assert_eq!(go_allowed(&robot, &world, &Direction::Up), Err(LibError::OutOfBounds));
    assert_eq!(go_allowed(&robot, &world, &Direction::Down), Err(LibError::OutOfBounds));
    assert_eq!(
        go_allowed(&robot, &world, &Direction::Right),
        Err(LibError::OutOfBounds)
    );
    assert_eq!(go_allowed(&robot, &world, &Direction::Left), Err(LibError::OutOfBounds));
}

#[test]
fn can_move_down_and_right_from_tile_zero_zero() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::None, 4),
        dimension: 4,
        discoverable: 4 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };

    // Assuming the Robot::new method will set (0, 0) as coordinates.
    let robot = TestRobot(Robot::new());

    assert_eq!(go_allowed(&robot, &world, &Direction::Down), Ok(()));
    assert_eq!(go_allowed(&robot, &world, &Direction::Right), Ok(()));
}

#[test]
fn can_not_move_on_deep_water_tile() {
    let world = World {
        map: generate_map_of_type_and_content(DeepWater, Content::None, 4),
        dimension: 4,
        discoverable: 4 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    let robot = TestRobot(Robot::new());
    assert_eq!(go_allowed(&robot, &world, &Direction::Down), Err(LibError::CannotWalk));
}

#[test]
fn get_direction_coordinates() {
    // Assuming the Robot::new method will set (0, 0) as coordinates.
    let robot = TestRobot(Robot {
        energy: Energy::new(0),
        coordinate: Coordinate::new(1, 1),
        backpack: BackPack::new(0),
    });
    assert_eq!(get_coords_row_col(&robot, &Direction::Down), (2, 1));
    assert_eq!(get_coords_row_col(&robot, &Direction::Left), (1, 0));
    assert_eq!(get_coords_row_col(&robot, &Direction::Right), (1, 2));
    assert_eq!(get_coords_row_col(&robot, &Direction::Up), (0, 1));
}

#[test]
fn a_tree_will_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Tree(0), 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn a_bin_will_not_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Bin(0..0), 1),
        dimension: 1,
        discoverable: 4 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(false));
}

#[test]
fn a_bank_will_not_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Bin(0..0), 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(false));
}

#[test]
fn a_crate_will_not_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Crate(0..0), 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(false));
}

#[test]
fn a_coin_will_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Coin(0), 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn fire_will_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Fire, 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn garbage_will_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Garbage(0), 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn none_will_not_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::None, 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Err(NoContent));
}

#[test]
fn water_will_be_destroyed() {
    let world = World {
        map: generate_map_of_type_and_content(Grass, Content::Water(0), 1),
        dimension: 1,
        discoverable: 1 / 10 + 1,
        environmental_conditions: generate_sunny_weather(),
        score_counter: Default::default(),
    };
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn add_tree_to_backpack_with_exact_space() {
    let mut robot = TestRobot(Robot {
        energy: Energy::default(),
        coordinate: Coordinate::new(0, 0),
        backpack: BackPack::new(10),
    });
    assert_eq!(add_to_backpack(&mut robot, Content::Tree(0), 10), Ok(10));
}

#[test]
fn add_various_content_to_backpack_untill_full() {
    let mut robot = TestRobot(Robot {
        energy: Energy::default(),
        coordinate: Coordinate::new(0, 0),
        backpack: BackPack::new(50),
    });
    assert_eq!(add_to_backpack(&mut robot, Content::Tree(0), 10), Ok(10));
    assert_eq!(add_to_backpack(&mut robot, Content::Coin(0), 10), Ok(10));
    assert_eq!(add_to_backpack(&mut robot, Content::Water(0), 30), Ok(30));
    assert_eq!(
        add_to_backpack(&mut robot, Content::Garbage(0), 30),
        Err(NotEnoughSpace(0))
    );
}

#[test]
fn add_tree_to_backpack_with_not_enough_space() {
    let mut robot = TestRobot(Robot {
        energy: Energy::default(),
        coordinate: Coordinate::new(0, 0),
        backpack: BackPack::new(6),
    });
    assert_eq!(
        add_to_backpack(&mut robot, Content::Tree(0), 10),
        Err(LibError::NotEnoughSpace(6))
    );
}
