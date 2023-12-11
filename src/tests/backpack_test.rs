use super::*;

#[test]
fn backpack_unit_test_default() {
    // const DEFAULT_VAL: usize = 0;
    // const DEFAULT_RANGE: Range<usize> = 0..0;
    const TEST_SIZE: usize = 10;
    let backpack = BackPack::new(TEST_SIZE);
    let content = Content::iter()
        .into_iter()
        .map(|c| (c, 0_usize))
        .collect::<HashMap<_, _>>();

    assert_eq!(backpack.get_size(), TEST_SIZE);

    assert_eq!(backpack.get_contents().len(), content.len());

    backpack.get_contents().iter().for_each(|(key, value)| {
        assert_eq!(Some(value), content.get(key));
    })
}
