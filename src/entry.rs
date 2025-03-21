// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Entry, OccupiedEntry, VacantEntry};
use core::mem;

impl<'a, K: PartialEq, V, const N: usize> Entry<'a, K, V, N> {
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    #[must_use]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, K: PartialEq, V: Default, const N: usize> Entry<'a, K, V, N> {
    pub fn or_default(self) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }
}

impl<'a, K: PartialEq, V, const N: usize> OccupiedEntry<'a, K, V, N> {
    #[must_use]
    pub fn key(&self) -> &K {
        &self.table.item_ref(self.index).0
    }

    #[must_use]
    pub fn remove_entry(self) -> (K, V) {
        self.table.remove_index_read(self.index)
    }

    #[must_use]
    pub fn get(&self) -> &V {
        &self.table.item_ref(self.index).1
    }

    pub fn get_mut(&mut self) -> &mut V {
        self.table.item_mut(self.index)
    }

    #[must_use]
    pub fn into_mut(self) -> &'a mut V {
        self.table.item_mut(self.index)
    }

    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    #[must_use]
    pub fn remove(self) -> V {
        self.table.remove_index_read(self.index).1
    }
}

impl<'a, K: PartialEq, V, const N: usize> VacantEntry<'a, K, V, N> {
    pub const fn key(&self) -> &K {
        &self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn insert(self, value: V) -> &'a mut V {
        let (index, _) = self.table.insert_i(self.key, value);
        self.table.item_mut(index)
    }
}
