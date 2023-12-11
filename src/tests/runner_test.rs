#[test]
fn game_tick_test() {
    use super::*;

    let robot = TestRobot(Robot::new());
    let mut dummy_world = TestWorld::init(10);

    let run = Runner::new(Box::new(robot), &mut dummy_world);
    assert_eq!(run.unwrap().game_tick(), Ok(()));
}
