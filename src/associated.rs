use crate::{
    iter::Iterable,
    misc::{AssociatedContains, Contains},
};

/// Associated collections as sets.
///
/// NOTE: Ignore all `T: Borrow<Q>` since we do not have way to parameterize bounds.
pub trait AssociatedSet<K>: Contains<K> + Iterable<K> {
    /// Returns true if `self` has no elements in common with `other`
    fn is_disjoint(&self, other: &Self) -> bool;

    /// Returns true if `self` is a subset of another.
    fn is_subset(&self, other: &Self) -> bool;

    /// Returns true if `self` is a superset of another.
    fn is_superset(&self, other: &Self) -> bool;

    /// Removes and returns the value in `self`, if any, that is equal to the given one.
    fn take(&mut self, value: &K) -> Option<K>;

    /// Adds a value to the set.
    fn insert(&mut self, value: K) -> bool;

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    fn get(&self, value: &K) -> Option<&K>;

    /// Removes a value from `self`. Returns whether the value was present in `self`.
    fn remove(&mut self, value: &K) -> bool;

    /// Adds a value to `self`, replacing the existing value, if any, that is equal to the given one. Returns the replaced value.
    fn replace(&mut self, value: K) -> Option<K>;
}

/// Associated sets operations
///
/// FIXME: Here we manually add `S` to handle lifetime requirements of `HashSet<K, S>`
pub trait AssociatedSetOperation<K, S>: AssociatedSet<K> {
    type DifferenceIter<'a>: Iterator<Item = &'a K>
    where
        K: 'a,
        S: 'a;

    type IntersectionIter<'a>: Iterator<Item = &'a K>
    where
        K: 'a,
        S: 'a;

    type SymmetricDifferenceIter<'a>: Iterator<Item = &'a K>
    where
        K: 'a,
        S: 'a;

    type UnionIter<'a>: Iterator<Item = &'a K>
    where
        K: 'a,
        S: 'a;

    /// Visits the values representing the difference.
    fn difference<'a>(&'a self, other: &'a Self) -> Self::DifferenceIter<'a>;

    /// Visits the values representing the Intersection.
    fn intersection<'a>(&'a self, other: &'a Self) -> Self::IntersectionIter<'a>;

    /// Visits the values representing the symmetric difference.
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Self::SymmetricDifferenceIter<'a>;

    /// Visits the values representing the union.
    fn union<'a>(&'a self, other: &'a Self) -> Self::UnionIter<'a>;
}

/// Associated collections as ordered sets
pub trait AssociatedSetOrd<K>: AssociatedSet<K>
// where
//     for<'a> <Self as Iterable<K>>::Iter<'a>: std::iter::DoubleEndedIterator
{
    /// Moves all elements from `other` into `self`, leaving `other` empty.
    fn append(&mut self, other: &mut Self);

    /// Returns a reference to the first value in `self`, if any.
    fn first(&self) -> Option<&K>;

    /// Removes the first value from `self` and returns it, if any.
    fn pop_first(&mut self) -> Option<K>;

    /// Returns a reference to the last value in `self`, if any.
    fn last(&self) -> Option<&K>;

    /// Removes the last value from `self` and returns it, if any.
    fn pop_last(&mut self) -> Option<K>;

    /// Splits the collection into two at the given key.
    fn split_off(&mut self, key: &K) -> Self;
}

/// Associated collections as key-value maps
pub trait AssociatedMap<K, V>: AssociatedContains<K> {
    /// Inserts a key-value pair into `self`.
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Removes a key from `self`, returning the value at the key if the key was previously in the map.
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Removes a key from `self`, returning the value at the key if the key was previously in the map.
    fn remove_entry(&mut self, key: &K) -> Option<(K, V)>;

    /// Returns a reference to the value corresponding to the key.
    fn get(&self, key: &K) -> Option<&V>;

    /// Returns a mutable reference to the value corresponding to the key.
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    /// Returns the key-value pair corresponding to the supplied key.
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;
}

/// Associated k-v maps operations
///
/// FIXME: Here we manually add `S` to handle lifetime requirements of `HashMap<K, V, S>`
pub trait AssociatedMapIter<K, V, S>: AssociatedMap<K, V> {
    /// Try insert error type.
    ///
    /// Ignore `std::error::Error` because that only applies when `K: Debug, V: Debug`
    type TryInsertError<'a>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    type KeyIter<'a>: Iterator<Item = &'a K>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    type ValueIter<'a>: Iterator<Item = &'a V>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    type ValueIterMut<'a>: Iterator<Item = &'a mut V>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    /// Tries to insert a key-value pair into `self`, and returns a mutable reference to the value in the entry.
    fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, Self::TryInsertError<'_>>;

    /// Gets an iterator over the keys of `self`.
    fn keys(&self) -> Self::KeyIter<'_>;

    /// Gets an iterator over the values of `self`.
    fn values(&self) -> Self::ValueIter<'_>;

    /// Gets an mutable iterator over the values of `self`.
    fn values_mut(&mut self) -> Self::ValueIterMut<'_>;
}

/// Associated collections that are ordered.
pub trait AssociatedMapOrd<K, V>: AssociatedMap<K, V>
// where
//     for<'a> <Self as Iterable<K>>::Iter<'a>: std::iter::DoubleEndedIterator
//     for<'a> <Self as AssociatedMap<K, V>>::KeyIter<'a>: std::iter::DoubleEndedIterator
//     for<'a> <Self as AssociatedMap<K, V>>::ValueIter<'a>: std::iter::DoubleEndedIterator
//     for<'a> <Self as AssociatedMap<K, V>>::ValueIterMut<'a>: std::iter::DoubleEndedIerator
{
    /// Moves all elements from `other` into `self`, leaving `other` empty.
    fn append(&mut self, other: &mut Self);

    /// Splits the collection into two at the given key.
    fn split_off(&mut self, key: &K) -> Self;

    /// Returns the first key-value pair in the map.
    fn first_key_value(&self) -> Option<(&K, &V)>;

    /// Returns the last key-value pair in the map.
    fn last_key_value(&self) -> Option<(&K, &V)>;

    /// Removes the first value from `self` and returns it, if any.
    fn pop_first(&mut self) -> Option<(K, V)>;

    /// Removes the last value from `self` and returns it, if any.
    fn pop_last(&mut self) -> Option<(K, V)>;
}

mod impls {
    use super::*;

    macro_rules! assoc_impls {
        () => {};
        ([@Set <$($targ: ty),*> => $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedSet<$($targ),*> for $t {
                fn is_disjoint(&self, other: &Self) -> bool {
                    <$t>::is_disjoint(self, other)
                }

                fn is_subset(&self, other: &Self) -> bool {
                    <$t>::is_subset(self, other)
                }

                fn is_superset(&self, other: &Self) -> bool {
                    <$t>::is_superset(self, other)
                }

                fn take(&mut self, value: &K) -> Option<K> {
                    <$t>::take(self, value)
                }

                fn insert(&mut self, value: K) -> bool {
                    <$t>::insert(self, value)
                }

                fn get(&self, value: &K) -> Option<&K> {
                    <$t>::get(self, value)
                }

                fn remove(&mut self, value: &K) -> bool {
                    <$t>::remove(self, value)
                }

                fn replace(&mut self, value: K) -> Option<K> {
                    <$t>::replace(self, value)
                }
            }
            assoc_impls!($($tail)*);
        };
        ([@SetOp <$($targ: ty),*> => $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter_diff: ty, $iter_insect: ty, $iter_sym: ty, $iter_union: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedSetOperation<$($targ),*> for $t {
                type DifferenceIter<'a> where $($args: 'a),* = $iter_diff;

                type IntersectionIter<'a> where $($args: 'a),* = $iter_insect;

                type SymmetricDifferenceIter<'a> where $($args: 'a),* = $iter_sym;

                type UnionIter<'a> where $($args: 'a),* = $iter_union;

                fn difference<'a>(&'a self, other: &'a Self) -> Self::DifferenceIter<'a> {
                    <$t>::difference(self, other)
                }

                fn intersection<'a>(&'a self, other: &'a Self) -> Self::IntersectionIter<'a> {
                    <$t>::intersection(self, other)
                }

                fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Self::SymmetricDifferenceIter<'a> {
                    <$t>::symmetric_difference(self, other)
                }

                fn union<'a>(&'a self, other: &'a Self) -> Self::UnionIter<'_> {
                    <$t>::union(self, other)
                }
            }
            assoc_impls!($($tail)*);
        };
        ([@OrdSet <$($targ: ty),*> => $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedSetOrd<$($targ),*> for $t {
                fn append(&mut self, other: &mut Self) {
                    <$t>::append(self, other)
                }

                fn first(&self) -> Option<&K> {
                    <$t>::first(self)
                }

                fn pop_first(&mut self) -> Option<K> {
                    <$t>::pop_first(self)
                }

                fn last(&self) -> Option<&K> {
                    <$t>::last(self)
                }

                fn pop_last(&mut self) -> Option<K> {
                    <$t>::pop_last(self)
                }

                fn split_off(&mut self, key: &K) -> Self {
                    <$t>::split_off(self, key)
                }
            }
            assoc_impls!($($tail)*);
        };
        ([@MapIter <$($targ: ty),*> => $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $err: ty, $iter_key: ty, $iter_value: ty, $iter_value_mut: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedMapIter<$($targ),*> for $t {
                type TryInsertError<'a> where $($args: 'a),* = $err;

                type KeyIter<'a> where $($args: 'a),* = $iter_key;

                type ValueIter<'a> where $($args: 'a),* = $iter_value;

                type ValueIterMut<'a> where $($args: 'a),* = $iter_value_mut;

                fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, Self::TryInsertError<'_>> {
                    <$t>::try_insert(self, key, value)
                }

                fn keys(&self) -> Self::KeyIter<'_> {
                    <$t>::keys(self)
                }

                fn values(&self) -> Self::ValueIter<'_> {
                    <$t>::values(self)
                }

                fn values_mut(&mut self) -> Self::ValueIterMut<'_> {
                    <$t>::values_mut(self)
                }
            }
            assoc_impls!($($tail)*);
        };
        ([@Map <$($targ: ty),*> => $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedMap<$($targ),*> for $t {
                fn insert(&mut self, key: K, value: V) -> Option<V> {
                    <$t>::insert(self, key, value)
                }

                fn remove(&mut self, key: &K) -> Option<V> {
                    <$t>::remove(self, key)
                }

                fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
                    <$t>::remove_entry(self, key)
                }

                fn get(&self, key: &K) -> Option<&V> {
                    <$t>::get(self, key)
                }

                fn get_mut(&mut self, key: &K) -> Option<&mut V> {
                    <$t>::get_mut(self, key)
                }

                fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
                    <$t>::get_key_value(self, key)
                }
            }
            assoc_impls!($($tail)*);
        };
        ([@OrdMap <$($targ: ty),*> => $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedMapOrd<$($targ),*> for $t {
                fn append(&mut self, other: &mut Self) {
                    <$t>::append(self, other)
                }

                fn first_key_value(&self) -> Option<(&K, &V)> {
                    <$t>::first_key_value(self)
                }

                fn pop_first(&mut self) -> Option<(K, V)> {
                    <$t>::pop_first(self)
                }

                fn last_key_value(&self) -> Option<(&K, &V)> {
                    <$t>::last_key_value(self)
                }

                fn pop_last(&mut self) -> Option<(K, V)> {
                    <$t>::pop_last(self)
                }

                fn split_off(&mut self, key: &K) -> Self {
                    <$t>::split_off(self, key)
                }
            }
            assoc_impls!($($tail)*);
        };
    }

    assoc_impls!(
        [@Set <K> => K: std::cmp::Eq | std::hash::Hash, S: std::hash::BuildHasher =>
            std::collections::HashSet<K, S>];
        [@Set <K> => K: std::cmp::Ord => std::collections::BTreeSet<K>];
        [@SetOp <K, S> => K: std::cmp::Eq | std::hash::Hash, S: std::hash::BuildHasher =>
            std::collections::HashSet<K, S>,
            std::collections::hash_set::Difference<'a, K, S>,
            std::collections::hash_set::Intersection<'a, K, S>,
            std::collections::hash_set::SymmetricDifference<'a, K, S>,
            std::collections::hash_set::Union<'a, K, S>
        ];
        [@SetOp <K, ()> => K: std::cmp::Ord =>
            std::collections::BTreeSet<K>,
            std::collections::btree_set::Difference<'a, K>,
            std::collections::btree_set::Intersection<'a, K>,
            std::collections::btree_set::SymmetricDifference<'a, K>,
            std::collections::btree_set::Union<'a, K>
        ];
        [@OrdSet <K> => K: std::cmp::Ord => std::collections::BTreeSet<K>];

        [@Map <K, V> => K: std::cmp::Eq | std::hash::Hash, V, S: std::hash::BuildHasher =>
            std::collections::HashMap<K, V, S>
        ];
        [@Map <K, V> => K: std::cmp::Ord, V => std::collections::BTreeMap<K, V>];
        [@MapIter <K, V, S> => K: std::cmp::Eq | std::hash::Hash, V, S: std::hash::BuildHasher =>
            std::collections::HashMap<K, V, S>,
            std::collections::hash_map::OccupiedError<'a, K, V>,
            std::collections::hash_map::Keys<'a, K, V>,
            std::collections::hash_map::Values<'a, K, V>,
            std::collections::hash_map::ValuesMut<'a, K, V>
        ];
        [@MapIter <K, V, ()> => K: std::cmp::Ord, V =>
            std::collections::BTreeMap<K, V>,
            std::collections::btree_map::OccupiedError<'a, K, V>,
            std::collections::btree_map::Keys<'a, K, V>,
            std::collections::btree_map::Values<'a, K, V>,
            std::collections::btree_map::ValuesMut<'a, K, V>
        ];

        [@OrdMap <K, V> => K: std::cmp::Ord, V => std::collections::BTreeMap<K, V>];
    );
}
