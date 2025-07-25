use std::any::type_name;

use crate::Registry;

pub trait Id: Clone + Copy {
    fn from_index(index: usize) -> Self;
    fn to_index(self) -> usize;

    fn display<V>(self, registry: &Registry<Self, V>) -> impl std::fmt::Display + std::fmt::Debug {
        struct Display<'r, K: Id, V> {
            id: K,
            registry: &'r Registry<K, V>,
        }

        impl<K: Id, V> std::fmt::Display for Display<'_, K, V> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if let Some(name) = self.registry.get_name(self.id) {
                    write!(f, "{name}")
                } else {
                    write!(f, "unknown({})", self.id.to_index())
                }
            }
        }

        impl<K: Id, V> std::fmt::Debug for Display<'_, K, V> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let type_name = type_name::<K>();
                if let Some(name) = self.registry.get_name(self.id) {
                    write!(f, "{type_name}({name})")
                } else {
                    write!(f, "{type_name}(unknown({}))", self.id.to_index())
                }
            }
        }

        Display { id: self, registry }
    }
}
