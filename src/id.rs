use std::fmt::Display;

use crate::Registry;

pub struct DisplayId<'r, K: Id, V> {
    id: K,
    registry: &'r Registry<K, V>,
}

pub trait Id: Clone + Copy {
    fn from_index(index: usize) -> Self;
    fn to_index(self) -> usize;

    fn display<V>(self, registry: &Registry<Self, V>) -> DisplayId<Self, V> {
        DisplayId { id: self, registry }
    }
}

impl<K: Id, V> Display for DisplayId<'_, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.registry.get_name(self.id) {
            write!(f, "{name}")
        } else {
            write!(f, "unknown:{}", self.id.to_index())
        }
    }
}
