// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

mod clone;
mod ctors;
mod debug;
mod display;
mod drain;
mod eq;
mod from;
mod functions;
mod iterators;
#[cfg(feature = "serde")]
mod serialization;

use crate::Map;

/// A faster alternative of [`std::collections::HashSet`].
///
/// For example, this is how you make a set, which is allocated on stack and is capable of storing
/// up to eight key-values pairs:
///
/// ```
/// let mut m : micromap::Set<u64, 8> = micromap::Set::new();
/// m.insert(1);
/// m.insert(2);
/// # #[cfg(std)]
/// assert_eq!(2, m.len());
/// ```
///
/// It is faster because it doesn't use a hash function at all. It simply keeps
/// all pairs in an array and when it's necessary to find a value, it goes through
/// all pairs comparing the needle with each pair available. Also it is faster
/// because it doesn't use heap. When a [`Set`] is being created, it allocates the necessary
/// space on stack. That's why the maximum size of the set must be provided in
/// compile time.
///
/// It is also faster because it doesn't grow in size. When a [`Set`] is created,
/// its size is fixed on stack. If an attempt is made to insert too many keys
/// into it, it simply panics. Moreover, in the "release" mode it doesn't panic,
/// but its behaviour is undefined. In the "release" mode all boundary checks
/// are disabled, for the sake of higher performance.
#[repr(transparent)]
pub struct Set<T: PartialEq, const N: usize> {
    map: Map<T, (), N>,
}

/// Iterator over the [`Set`].
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct SetIter<'a, T> {
    iter: crate::Keys<'a, T, ()>,
}

/// Into-iterator over the [`Set`].
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct SetIntoIter<T: PartialEq, const N: usize> {
    iter: crate::IntoKeys<T, (), N>,
}

#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct SetDrain<'a, T: PartialEq> {
    iter: crate::Drain<'a, T, ()>,
}
