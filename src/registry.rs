use std::collections::HashMap;

use crate::{Id, Name};

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

    pub fn iter(&self) -> impl Iterator<Item = (K, Option<&Name>, &V)> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, (n, v))| (Id::from_index(i), n.as_ref(), v))
    }
}
