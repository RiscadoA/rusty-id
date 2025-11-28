use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;

use crate::{Id, Name};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct Registry<K: Id, V> {
    name_to_id: HashMap<Name, K>,
    entries: Vec<(Option<Name>, V)>,
}

impl<K: Id, V> Registry<K, V> {
    pub fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub fn from_entries(entries: Vec<(Option<Name>, V)>) -> Self {
        let mut name_to_id = HashMap::new();
        for (index, (name_opt, _)) in entries.iter().enumerate() {
            if let Some(name) = name_opt {
                name_to_id.insert(name.clone(), Id::from_index(index));
            }
        }

        Self {
            name_to_id,
            entries,
        }
    }

    pub fn contains(&self, name: &Name) -> bool {
        self.name_to_id.contains_key(name)
    }

    pub fn add_named(&mut self, name: Name, entry: V) -> Result<K, Name> {
        self.add_named_with(name, |_| entry)
    }

    pub fn add_named_with(&mut self, name: Name, func: impl FnOnce(K) -> V) -> Result<K, Name> {
        if self.name_to_id.contains_key(&name) {
            return Err(name);
        }

        let id = self.add_with(Some(name.clone()), func);
        self.name_to_id.insert(name, id);
        Ok(id)
    }

    pub fn add_anonymous(&mut self, entry: V) -> K {
        self.add_anonymous_with(|_| entry)
    }

    pub fn add_anonymous_with(&mut self, func: impl FnOnce(K) -> V) -> K {
        self.add_with(None, func)
    }

    fn add_with(&mut self, name: Option<Name>, func: impl FnOnce(K) -> V) -> K {
        let id = Id::from_index(self.entries.len());
        self.entries.push((name, func(id)));
        id
    }

    pub fn find(&self, name: &Name) -> Option<K> {
        self.name_to_id.get(name).copied()
    }

    pub fn get(&self, id: K) -> &V {
        self.entries.get(id.to_index()).map(|(_, v)| v).unwrap()
    }

    pub fn get_name(&self, id: K) -> Option<&Name> {
        self.entries
            .get(id.to_index())
            .and_then(|(n, _)| n.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, Option<&Name>, &V)> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, (n, v))| (Id::from_index(i), n.as_ref(), v))
    }

    pub fn entries(&self) -> &Vec<(Option<Name>, V)> {
        &self.entries
    }
}

impl<K: Id, V> Default for Registry<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Id, V: Clone> Clone for Registry<K, V> {
    fn clone(&self) -> Self {
        Self {
            name_to_id: self.name_to_id.clone(),
            entries: self.entries.clone(),
        }
    }
}
