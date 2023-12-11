use super::*;

#[test]
fn default() {
    let en = Energy::new(0);

    assert_eq!(en.get_energy_level(), 0);
}

#[test]
fn new_with_energy_more_than_max() {
    let en = Energy::new(MAX_ENERGY_LEVEL + 1);
    assert_eq!(en.get_energy_level(), MAX_ENERGY_LEVEL);
}

#[test]
fn new_to_max() {
    let en = Energy::new(MAX_ENERGY_LEVEL);
    assert_eq!(en.get_energy_level(), MAX_ENERGY_LEVEL);
}

#[test]
fn consume_energy_enough() {
    let mut en = Energy::new(5);
    assert_eq!(en.consume_energy(2), Ok(()));
}

#[test]
fn consume_energy_not_enough() {
    let mut en = Energy::new(5);
    assert_eq!(en.consume_energy(6), Err(NotEnoughEnergy));
}

#[test]
fn cannot_recharge_more_than_max() {
    let mut en = Energy::new(MAX_ENERGY_LEVEL);
    en.recharge_energy(1);
    assert_eq!(en.get_energy_level(), MAX_ENERGY_LEVEL);
}

#[test]
fn recharge_to_max() {
    let mut en = Energy::new(MAX_ENERGY_LEVEL - 1);
    en.recharge_energy(1);
    assert_eq!(en.get_energy_level(), MAX_ENERGY_LEVEL);
}

#[test]
fn recharge_to_random() {
    let mut en = Energy::default();
    let mut rng = rand::thread_rng();
    let rnd_number = rng.gen_range(0..MAX_ENERGY_LEVEL);
    en.recharge_energy(rnd_number);
    assert_eq!(en.get_energy_level(), rnd_number);
}
