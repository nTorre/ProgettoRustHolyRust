use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::world::tile::Content;

/// The backpack is used to store the content of the robot
///
/// # Usage
/// ```
/// use robotics_lib::runner::backpack::BackPack;
/// ```
///
/// # Parameters
/// - `size`: The size of the backpack
/// - `contents`: The contents of the backpack
#[derive(Debug)]
pub struct BackPack {
    pub(crate) size: usize,
    pub(crate) contents: HashMap<Content, usize>,
}

impl BackPack {
    /// Creates a new backpack
    ///
    /// # Arguments
    /// size: The size of the backpack
    ///
    pub(crate) fn new(size: usize) -> Self {
        BackPack {
            size,
            contents: Content::iter().map(|content| (content.to_default(), 0)).collect(),
        }
    }

    /// Gets the size of the backpack
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Gets the contents of the backpack
    pub fn get_contents(&self) -> &HashMap<Content, usize> {
        &self.contents
    }
}
