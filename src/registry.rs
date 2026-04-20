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

pub struct PartialRegistry<K: Id>(Registry<K, ()>);

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

    pub fn try_add_named_with<E>(
        &mut self,
        name: Name,
        func: impl FnOnce(K) -> Result<V, E>,
    ) -> Result<Result<K, E>, Name> {
        if self.name_to_id.contains_key(&name) {
            return Err(name);
        }

        match self.try_add_with(Some(name.clone()), func) {
            Ok(id) => {
                self.name_to_id.insert(name, id);
                Ok(Ok(id))
            }
            Err(e) => Ok(Err(e)),
        }
    }

    pub fn add_anonymous(&mut self, entry: V) -> K {
        self.add_anonymous_with(|_| entry)
    }

    pub fn add_anonymous_with(&mut self, func: impl FnOnce(K) -> V) -> K {
        self.add_with(None, func)
    }

    pub fn try_add_anonymous_with<E>(
        &mut self,
        func: impl FnOnce(K) -> Result<V, E>,
    ) -> Result<K, E> {
        self.try_add_with(None, func)
    }

    fn add_with(&mut self, name: Option<Name>, func: impl FnOnce(K) -> V) -> K {
        let id = Id::from_index(self.entries.len());
        self.entries.push((name, func(id)));
        id
    }

    fn try_add_with<E>(
        &mut self,
        name: Option<Name>,
        func: impl FnOnce(K) -> Result<V, E>,
    ) -> Result<K, E> {
        let id = Id::from_index(self.entries.len());
        let entry = func(id)?;
        self.entries.push((name, entry));
        Ok(id)
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

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K: Id> PartialRegistry<K> {
    pub fn new() -> Self {
        Self(Registry::new())
    }

    pub fn contains(&self, name: &Name) -> bool {
        self.0.contains(name)
    }

    pub fn find(&self, name: &Name) -> Option<K> {
        self.0.find(name)
    }

    pub fn get_name(&self, id: K) -> Option<&Name> {
        self.0.get_name(id)
    }

    pub fn add_named(&mut self, name: Name) -> Result<K, Name> {
        self.0.add_named(name, ())
    }

    pub fn add_anonymous(&mut self) -> K {
        self.0.add_anonymous(())
    }

    pub fn build<V>(self, mut func: impl FnMut(K) -> V) -> Registry<K, V> {
        Registry::from_entries(
            self.0
                .entries
                .into_iter()
                .enumerate()
                .map(move |(i, (n, _))| (n, func(Id::from_index(i))))
                .collect(),
        )
    }

    pub fn try_build<V, E>(
        self,
        mut func: impl FnMut(K) -> Result<V, E>,
    ) -> Result<Registry<K, V>, E> {
        Ok(Registry::from_entries(
            self.0
                .entries
                .into_iter()
                .enumerate()
                .map(move |(i, (n, _))| func(Id::from_index(i)).map(|e| (n, e)))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, Option<&Name>)> {
        self.0
            .entries
            .iter()
            .enumerate()
            .map(|(i, (n, _))| (Id::from_index(i), n.as_ref()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

impl<K: Id> Default for PartialRegistry<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Id> Clone for PartialRegistry<K> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
