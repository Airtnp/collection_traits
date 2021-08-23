use crate::{
    elem::{AssociatedCollectionTrait, CollectionTrait, Mutable},
    exact_size::ExactSized,
};
use std::ops::RangeBounds;

/// Collections that can iterate as sequence of `&T`.
pub trait Iterable: CollectionTrait {
    // Check: https://internals.rust-lang.org/t/gat-and-lifetime-bounds/12422/3
    /// Immutable iterator type
    type Iter<'a>: Iterator<Item = &'a Self::ElemType>
    where
        Self::ElemType: 'a;

    /// Iterates over immutable reference
    fn iter(&self) -> Self::Iter<'_>;
}

/// Collections that can iterate as sequence of `(&K, &V)`.
pub trait AssociatedIterable: AssociatedCollectionTrait {
    /// Immutable map iterator type
    type Iter<'a>: Iterator<Item = (&'a Self::KeyType, &'a Self::ValueType)>
    where
        Self::KeyType: 'a,
        Self::ValueType: 'a;

    /// Iterates over immutable reference
    fn iter(&self) -> Self::Iter<'_>;
}

/// Collections that can iterate as sequence of `&mut T`.
pub trait IterableMut: CollectionTrait + Mutable {
    /// Mutable iterator type
    type IterMut<'a>: Iterator<Item = &'a mut Self::ElemType>
    where
        Self::ElemType: 'a;

    /// Iterates over mutable reference
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Collections that can iterate as sequence of `(&K, &mut V)`.
pub trait AssociatedIterableMut: AssociatedCollectionTrait + Mutable {
    /// Mutable map iterator type
    type IterMut<'a>: Iterator<Item = (&'a Self::KeyType, &'a mut Self::ValueType)>
    where
        Self::KeyType: 'a,
        Self::ValueType: 'a;

    /// Iterates over mutable reference
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Collections that can return a immutable range of elements
pub trait Range: CollectionTrait + ExactSized {
    type RangeIter<'a>: Iterator<Item = &'a Self::ElemType>
    where
        Self::ElemType: 'a;
    /// Creates an iterator that covers the specified range in the `self`
    fn range<R: std::ops::RangeBounds<Self::SizeType>>(&self, range: R) -> Self::RangeIter<'_>;
}

/// Collections that can return a mutable range of elements
pub trait RangeMut: CollectionTrait + Mutable + ExactSized {
    type RangeIterMut<'a>: Iterator<Item = &'a mut Self::ElemType>
    where
        Self::ElemType: 'a;
    /// Creates an iterator that covers the specified range in the `self`
    fn range_mut<R: std::ops::RangeBounds<Self::SizeType>>(&mut self, range: R) -> Self::RangeIterMut<'_>;
}

/// Collections that can return a immutable range of elements based on key values
pub trait AssociatedRange: AssociatedCollectionTrait {
    type RangeIter<'a>: Iterator<Item = (&'a Self::KeyType, &'a Self::ValueType)>
    where
        Self::KeyType: 'a,
        Self::ValueType: 'a;

    /// Creates an iterator that covers the specified range in the `self`
    ///
    /// NOTE: `{BTreeSet, BTreeMap}::range` are more general.
    fn range<R: std::ops::RangeBounds<Self::KeyType>>(&self, range: R) -> Self::RangeIter<'_>;
}

/// Collections that can return a mutable range of elements based on key values
pub trait AssociatedRangeMut: AssociatedCollectionTrait + Mutable {
    type RangeIterMut<'a>: Iterator<Item = (&'a Self::KeyType, &'a mut Self::ValueType)>
    where
        Self::KeyType: 'a,
        Self::ValueType: 'a;

    /// Creates an iterator that covers the specified range in the `self`
    ///
    /// NOTE: `BTreeMap::range` are more general.
    fn range_mut<R: std::ops::RangeBounds<Self::KeyType>>(&mut self, range: R) -> Self::RangeIterMut<'_>;
}

/// Collections that can drain all elements
pub trait DrainFull: CollectionTrait + Mutable {
    type DrainIter<'a>: Iterator<Item = Self::ElemType>
    where
        Self::ElemType: 'a;

    /// Creates a draining iterator that removes the specified range in `self` and yields the removed items.
    fn drain(&mut self) -> Self::DrainIter<'_>;
}

/// Collections that can drain all elements
pub trait AssociatedDrainFull: AssociatedCollectionTrait + Mutable {
    type DrainIter<'a>: Iterator<Item = (Self::KeyType, Self::ValueType)>
    where
        Self::KeyType: 'a,
        Self::ValueType: 'a;

    /// Creates a draining iterator that removes the specified range in `self` and yields the removed items.
    fn drain(&mut self) -> Self::DrainIter<'_>;
}

/// Collections that can drain a range of elements
pub trait DrainRange: CollectionTrait + Mutable + ExactSized {
    type DrainRangeIter<'a>: Iterator<Item = Self::ElemType>
    where
        Self::ElemType: 'a;

    /// Creates a draining iterator that removes the specified range in `self` and yields the removed items.
    fn drain_range<R: RangeBounds<Self::SizeType>>(&mut self, range: R) -> Self::DrainRangeIter<'_>;
}

/// Collections that can drain elements given a filter on mutable reference
pub trait DrainFilter: CollectionTrait + Mutable {
    type DrainFilterIter<'a, F>: Iterator<Item = Self::ElemType>
    where
        Self::ElemType: 'a,
        F: FnMut(&mut Self::ElemType) -> bool + 'a;

    /// Creates a draining iterator that removes the specified range in `self` and yields the removed items.
    ///
    /// FIXME: Use `FnMut(&T) -> bool` because `BTreeSet::drain` requires it. Otherwise, it can be `FnMut(&mut T)`.
    /// Meanwhile, there is no easy way to convert `FnMut(&T) -> bool` to `Fn(&K, &mut ()) -> bool`
    /// and specify it at the associated type part
    fn drain_filter<'a, F: FnMut(&mut Self::ElemType) -> bool + 'a>(
        &'a mut self,
        filter: F,
    ) -> Self::DrainFilterIter<'a, F>;
}

/// Collections that can drain elements given a filter on immutable reference
pub trait AssociatedDrainFilterSet: CollectionTrait + Mutable {
    type DrainFilterIter<'a, F>: Iterator<Item = Self::ElemType>
    where
        Self::ElemType: 'a,
        F: FnMut(&Self::ElemType) -> bool + 'a;

    /// Creates a draining iterator that removes the specified range in `self` and yields the removed items.
    ///
    /// FIXME: Separate `FnMut(&T) -> bool` because `BTreeSet::drain` requires it. Otherwise, it can be `FnMut(&mut T)`.
    /// Meanwhile, there is no easy way to convert `FnMut(&T) -> bool` to `Fn(&K, &mut ()) -> bool`
    /// and specify it at the associated type part/
    fn drain_filter<'a, F: FnMut(&Self::ElemType) -> bool + 'a>(
        &'a mut self,
        filter: F,
    ) -> Self::DrainFilterIter<'a, F>;
}

/// Associated collections that can drain elements given a filter on k-v reference
pub trait AssociatedDrainFilter: AssociatedCollectionTrait + Mutable {
    type DrainFilterIter<'a, F>: Iterator<Item = (Self::KeyType, Self::ValueType)>
    where
        Self::KeyType: 'a,
        Self::ValueType: 'a,
        F: FnMut(&Self::KeyType, &mut Self::ValueType) -> bool + 'a;

    /// Creates a draining iterator that removes the specified range in `self` and yields the removed items.
    fn drain_filter<'a, F: FnMut(&Self::KeyType, &mut Self::ValueType) -> bool + 'a>(
        &'a mut self,
        filter: F,
    ) -> Self::DrainFilterIter<'a, F>;
}

mod impls {
    use super::*;
    use std::ops::RangeBounds;

    impl<T, const N: usize> Iterable for [T; N] {
        type Iter<'a>
        where
            T: 'a,
        = std::slice::Iter<'a, T>;

        fn iter(&self) -> Self::Iter<'_> {
            self.as_slice().iter()
        }
    }

    impl<T> Iterable for &[T] {
        type Iter<'a>
        where
            T: 'a,
        = std::slice::Iter<'a, T>;

        fn iter(&self) -> Self::Iter<'_> {
            <[T]>::iter(self)
        }
    }

    impl<T> Iterable for &mut [T] {
        type Iter<'a>
        where
            T: 'a,
        = std::slice::Iter<'a, T>;

        fn iter(&self) -> Self::Iter<'_> {
            <[T]>::iter(self)
        }
    }

    impl<T, const N: usize> IterableMut for [T; N] {
        type IterMut<'a>
        where
            T: 'a,
        = std::slice::IterMut<'a, T>;

        fn iter_mut(&mut self) -> Self::IterMut<'_> {
            self.as_mut_slice().iter_mut()
        }
    }

    impl<T> IterableMut for &mut [T] {
        type IterMut<'a>
        where
            T: 'a,
        = std::slice::IterMut<'a, T>;

        fn iter_mut(&mut self) -> Self::IterMut<'_> {
            <[T]>::iter_mut(self)
        }
    }

    impl<K> AssociatedIterable for std::collections::HashSet<K> {
        type Iter<'a>
        where
            K: 'a,
        = std::iter::Map<std::collections::hash_set::Iter<'a, K>, impl FnMut(&'a K) -> (&'a K, &'a ())>;

        fn iter(&self) -> Self::Iter<'_> {
            self.iter().map(|v| (v, &()))
        }
    }

    impl<K> AssociatedIterable for std::collections::BTreeSet<K> {
        type Iter<'a>
        where
            K: 'a,
        = std::iter::Map<std::collections::btree_set::Iter<'a, K>, impl FnMut(&'a K) -> (&'a K, &'a ())>;

        fn iter(&self) -> Self::Iter<'_> {
            self.iter().map(|v| (v, &()))
        }
    }

    impl<T> Range for &[T] {
        type RangeIter<'a>
        where
            T: 'a,
        = std::slice::Iter<'a, T>;

        fn range<R: RangeBounds<Self::SizeType>>(&self, range: R) -> Self::RangeIter<'_> {
            self[std::slice::range(range, ..self.len())].iter()
        }
    }

    impl<T> Range for &mut [T] {
        type RangeIter<'a>
        where
            T: 'a,
        = std::slice::Iter<'a, T>;

        fn range<R: RangeBounds<Self::SizeType>>(&self, range: R) -> Self::RangeIter<'_> {
            self[std::slice::range(range, ..self.len())].iter()
        }
    }

    impl<T> RangeMut for &mut [T] {
        type RangeIterMut<'a>
        where
            T: 'a,
        = std::slice::IterMut<'a, T>;

        fn range_mut<R: RangeBounds<Self::SizeType>>(&mut self, range: R) -> Self::RangeIterMut<'_> {
            let len = self.len();
            self[std::slice::range(range, ..len)].iter_mut()
        }
    }

    impl<T, const N: usize> Range for [T; N] {
        type RangeIter<'a>
        where
            T: 'a,
        = std::slice::Iter<'a, T>;

        fn range<R: RangeBounds<Self::SizeType>>(&self, range: R) -> Self::RangeIter<'_> {
            self[std::slice::range(range, ..self.len())].iter()
        }
    }

    impl<T, const N: usize> RangeMut for [T; N] {
        type RangeIterMut<'a>
        where
            T: 'a,
        = std::slice::IterMut<'a, T>;

        fn range_mut<R: RangeBounds<Self::SizeType>>(&mut self, range: R) -> Self::RangeIterMut<'_> {
            let len = self.len();
            self[std::slice::range(range, ..len)].iter_mut()
        }
    }

    impl<T, A: std::alloc::Allocator> Range for Vec<T, A> {
        type RangeIter<'a>
        where
            T: 'a,
        = std::slice::Iter<'a, T>;

        fn range<R: RangeBounds<Self::SizeType>>(&self, range: R) -> Self::RangeIter<'_> {
            self[std::slice::range(range, ..self.len())].iter()
        }
    }

    impl<T, A: std::alloc::Allocator> RangeMut for Vec<T, A> {
        type RangeIterMut<'a>
        where
            T: 'a,
        = std::slice::IterMut<'a, T>;

        fn range_mut<R: RangeBounds<Self::SizeType>>(&mut self, range: R) -> Self::RangeIterMut<'_> {
            let len = self.len();
            self[std::slice::range(range, ..len)].iter_mut()
        }
    }

    /*
    // FIXME: compiler throws incorrect error, maybe due to min_type_alias_impl_trait feature
    //    type parameter `R` is part of concrete type but not used in parameter list for the `impl Trait` type alias
    impl<K: std::cmp::Ord> AssociatedRange<K, ()> for std::collections::BTreeSet<K> {
        type RangeIter<'a>
        where
            K: 'a,
        = std::iter::Map<std::collections::btree_set::Range<'a, K>, impl FnMut(&'a K) -> (&'a K, &'a ())>;

        fn range<R: RangeBounds<K>>(&self, range: R) -> Self::RangeIter<'_> {
            let range = <BTreeSet<K>>::range(self, range);
            range.map(|v| (v, &()))
        }
    }
     */

    macro_rules! iter_impls {
        () => {};
        ([@Delegate $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> Iterable for $t {
                type Iter<'a> where T: 'a = $iter;

                fn iter(&self) -> Self::Iter<'_> {
                    <$t>::iter(self)
                }
            }
            iter_impls!($($tail)*);
        };
        ([@Delegate $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty, $iter_mut: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> Iterable for $t {
                type Iter<'a> where T: 'a = $iter;

                fn iter(&self) -> Self::Iter<'_> {
                    <$t>::iter(self)
                }
            }

            impl<$($args $(: $bound $(+ $others)*)?),*> IterableMut for $t {
                type IterMut<'a> where T: 'a = $iter_mut;

                fn iter_mut(&mut self) -> Self::IterMut<'_> {
                    <$t>::iter_mut(self)
                }
            }
            iter_impls!($($tail)*);
        };
        ([@DelegateMap $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty, $iter_mut: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedIterable for $t {
                type Iter<'a> where K: 'a, V: 'a = $iter;

                fn iter(&self) -> Self::Iter<'_> {
                    <$t>::iter(self)
                }
            }

            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedIterableMut for $t {
                type IterMut<'a> where K: 'a, V: 'a = $iter_mut;

                fn iter_mut(&mut self) -> Self::IterMut<'_> {
                    <$t>::iter_mut(self)
                }
            }
            iter_impls!($($tail)*);
        };
        ([@Slice $t: ty]; $($tail:tt)*) => {
            impl<T, A: std::alloc::Allocator> Iterable for $t {
                type Iter<'a> where T: 'a = std::slice::Iter<'a, T>;

                fn iter(&self) -> Self::Iter<'_> {
                        self.as_slice().iter()
                }
            }

            impl<T, A: std::alloc::Allocator> IterableMut for $t {
                type IterMut<'a> where T: 'a = std::slice::IterMut<'a, T>;

                fn iter_mut(&mut self) -> Self::IterMut<'_> {
                    self.as_mut_slice().iter_mut()
                }
            }
            iter_impls!($($tail)*);
        };
        ([@Range $t: ty, $iter: ty, $iter_mut: ty]; $($tail:tt)*) => {
            impl<T> Range for $t {
                type RangeIter<'a> where T: 'a = $iter;

                fn range<R: std::ops::RangeBounds<Self::SizeType>>(&self, range: R) -> Self::RangeIter<'_> {
                    <$t>::range(self, range)
                }
            }

            impl<T> RangeMut for $t {
                type RangeIterMut<'a> where T: 'a = $iter_mut;

                fn range_mut<R: std::ops::RangeBounds<Self::SizeType>>(&mut self, range: R) -> Self::RangeIterMut<'_> {
                    <$t>::range_mut(self, range)
                }
            }
            iter_impls!($($tail)*);
        };
        ([@AssocRange $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty, $iter_mut: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedRange for $t {
                type RangeIter<'a> where K: 'a, V: 'a = $iter;

                fn range<R: std::ops::RangeBounds<K>>(&self, range: R) -> Self::RangeIter<'_> {
                    <$t>::range(self, range)
                }
            }

            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedRangeMut for $t {
                type RangeIterMut<'a> where K: 'a, V: 'a = $iter_mut;

                fn range_mut<R: std::ops::RangeBounds<K>>(&mut self, range: R) -> Self::RangeIterMut<'_> {
                    <$t>::range_mut(self, range)
                }
            }
            iter_impls!($($tail)*);
        };
        ([@DrainFull $targ: ty => $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> DrainFull for $t {
                type DrainIter<'a> where $($args: 'a),* = $iter;

                fn drain(&mut self) -> Self::DrainIter<'_> {
                    <$t>::drain(self)
                }
            }
            iter_impls!($($tail)*);
        };
        ([@DrainRange $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> DrainRange for $t {
                type DrainRangeIter<'a> where T: 'a = $iter;

                fn drain_range<R: RangeBounds<Self::SizeType>>(&mut self, range: R) -> Self::DrainRangeIter<'_> {
                    <$t>::drain(self, range)
                }

            }
            iter_impls!($($tail)*);
        };
        ([@DrainFilter $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> DrainFilter for $t {
                type DrainFilterIter<'a, F> where T: 'a, F: FnMut(&mut T) -> bool + 'a = $iter;

                fn drain_filter<'a, F: FnMut(&mut T) -> bool + 'a>(&'a mut self, filter: F) -> Self::DrainFilterIter<'a, F> {
                    <$t>::drain_filter(self, filter)
                }

            }
            iter_impls!($($tail)*);
        };
        ([@DrainFilterSet $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedDrainFilterSet for $t {
                type DrainFilterIter<'a, F> where T: 'a, F: FnMut(&T) -> bool + 'a = $iter;

                fn drain_filter<'a, F: FnMut(&T) -> bool + 'a>(&'a mut self, filter: F) -> Self::DrainFilterIter<'a, F> {
                    <$t>::drain_filter(self, filter)
                }

            }
            iter_impls!($($tail)*);
        };
        ([@AssocDrainFilter $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty, $iter: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AssociatedDrainFilter for $t {
                type DrainFilterIter<'a, F> where K: 'a, V: 'a, F: FnMut(&K, &mut V) -> bool + 'a = $iter;

                fn drain_filter<'a, F: FnMut(&K, &mut V) -> bool + 'a>(&'a mut self, filter: F) -> Self::DrainFilterIter<'a, F> {
                    <$t>::drain_filter(self, filter)
                }
            }
            iter_impls!($($tail)*);
        };
    }

    impl<K, V, S> AssociatedDrainFull for std::collections::HashMap<K, V, S> {
        type DrainIter<'a>
        where
            K: 'a,
            V: 'a,
        = std::collections::hash_map::Drain<'a, K, V>;

        fn drain(&mut self) -> Self::DrainIter<'_> {
            self.drain()
        }
    }

    iter_impls!(
        [@Slice Vec<T, A>];

        [@Delegate T => std::collections::VecDeque<T>, std::collections::vec_deque::Iter<'a, T>, std::collections::vec_deque::IterMut<'a, T>];
        [@Delegate T => std::collections::LinkedList<T>, std::collections::linked_list::Iter<'a, T>, std::collections::linked_list::IterMut<'a, T>];
        [@Delegate T => std::collections::BTreeSet<T>, std::collections::btree_set::Iter<'a, T>];
        [@Delegate T, S => std::collections::HashSet<T, S>, std::collections::hash_set::Iter<'a, T>];
        [@Delegate T => std::collections::BinaryHeap<T>, std::collections::binary_heap::Iter<'a, T>];
        [@DelegateMap K, V => std::collections::BTreeMap<K, V>, std::collections::btree_map::Iter<'a, K, V>, std::collections::btree_map::IterMut<'a, K, V>];
        [@DelegateMap K, V, S => std::collections::HashMap<K, V, S>, std::collections::hash_map::Iter<'a, K, V>, std::collections::hash_map::IterMut<'a, K, V>];

        [@Range std::collections::VecDeque<T>, std::collections::vec_deque::Iter<'a, T>, std::collections::vec_deque::IterMut<'a, T>];
        [@AssocRange K: std::cmp::Ord, V => std::collections::BTreeMap<K, V>, std::collections::btree_map::Range<'a, K, V>, std::collections::btree_map::RangeMut<'a, K, V>];

        [@DrainFull T => T: std::cmp::Ord => std::collections::BinaryHeap<T>, std::collections::binary_heap::Drain<'a, T>];
        [@DrainFull T => T: std::cmp::Ord => std::collections::HashSet<T>, std::collections::hash_set::Drain<'a, T>];

        // FIXME: omit allocator here => need to rewrite associated types
        [@DrainRange T => Vec<T>, std::vec::Drain<'a, T>];
        [@DrainRange T => std::collections::VecDeque<T>, std::collections::vec_deque::Drain<'a, T>];

        [@DrainFilter T => Vec<T>, std::vec::DrainFilter<'a, T, F>];
        [@DrainFilter T => std::collections::LinkedList<T>, std::collections::linked_list::DrainFilter<'a, T, F>];
        [@DrainFilterSet T: std::cmp::Ord => std::collections::BTreeSet<T>, std::collections::btree_set::DrainFilter<'a, T, F>];
        [@DrainFilterSet T, S: std::hash::BuildHasher => std::collections::HashSet<T, S>, std::collections::hash_set::DrainFilter<'a, T, F>];

        [@AssocDrainFilter K, V, S: std::hash::BuildHasher => std::collections::HashMap<K, V, S>, std::collections::hash_map::DrainFilter<'a, K, V, F>];
        [@AssocDrainFilter K: std::cmp::Ord, V => std::collections::BTreeMap<K, V>, std::collections::btree_map::DrainFilter<'a, K, V, F>];
    );
}
