//! Trait crate for abstracting common operations in `std` collections.
#![feature(generic_associated_types)]
#![feature(array_methods)]
#![feature(allocator_api)]
#![feature(associated_type_defaults)]
#![feature(try_reserve)]
#![feature(shrink_to)]
#![feature(trusted_random_access)]
#![feature(min_type_alias_impl_trait)]
#![feature(slice_range)]
#![feature(btree_drain_filter)]
#![feature(drain_filter)]
#![feature(hash_drain_filter)]
#![feature(linked_list_remove)]
#![feature(map_try_insert)]
#![feature(map_first_last)]

pub mod allocator;
pub mod associated;
pub mod capacity;
pub mod exact_size;
pub mod iter;
pub mod misc;
pub mod sequential;

/// Std collections (e.g, `[T; N]`)
///
/// NOTE: not all collections can mutably iterate, e.g., `HashSet<T>`, `BinaryHeap<T>`
pub trait Collection<T>: iter::Iterable<T> + exact_size::ExactSized {}
impl<T, C: iter::Iterable<T> + exact_size::ExactSized> Collection<T> for C {}

/// Std associated collections (e.g., `HashMap<K, V>`)
pub trait AssociatedCollection<K, V>:
    iter::AssociatedIterable<K, V> + iter::AssociatedIterableMut<K, V> + exact_size::ExactSized
{
}
impl<K, V, C: iter::AssociatedIterable<K, V> + iter::AssociatedIterableMut<K, V> + exact_size::ExactSized>
    AssociatedCollection<K, V> for C
{
}

/// Owned std associated collections (e.g., `Vec<T>`)
pub trait OwnedCollection<T>:
    Collection<T> + Extend<T> + IntoIterator<Item = T> + misc::Erasable + misc::EmptyConstructible
{
}
impl<T, C: Collection<T> + Extend<T> + Default + IntoIterator<Item = T> + misc::Erasable + misc::EmptyConstructible>
    OwnedCollection<T> for C
{
}

/// Owned std associated collections (e.g., `BTreeMap<K, V>`)
pub trait OwnedAssociatedCollection<K, V>:
    AssociatedCollection<K, V>
    + Extend<(K, V)>
    + Default
    + IntoIterator<Item = (K, V)>
    + misc::Erasable
    + misc::EmptyConstructible
{
}
impl<
    K,
    V,
    C: AssociatedCollection<K, V>
        + Extend<(K, V)>
        + Default
        + IntoIterator<Item = (K, V)>
        + misc::Erasable
        + misc::EmptyConstructible,
> OwnedAssociatedCollection<K, V> for C
{
}

/// Std contiguous collections (e.g., `[T; N]`, `&mut [T]`)
pub trait ContiguousCollection<T>: Collection<T> + sequential::Contiguous<T> {}
impl<T, C: Collection<T> + sequential::Contiguous<T>> ContiguousCollection<T> for C {}

/// Std sequential collection (e.g., `&[T]`, `LinkedList<T>`)
pub trait SequentialCollection<T>: Collection<T> + sequential::Sequential<T> {}
impl<T, C: Collection<T> + sequential::Sequential<T>> SequentialCollection<T> for C {}

/// Std owned sequential collection (e.g., `Vec<T>`, `VecDeque<T>`)
pub trait OwnedSequentialCollection<T>: OwnedCollection<T> + sequential::OwnedSequential<T> {}
impl<T, C: OwnedCollection<T> + sequential::OwnedSequential<T>> OwnedSequentialCollection<T> for C {}

/// Std doubled-ended sequential collection (e.g. `VecDeque<T>`)
pub trait DoubleEndedCollection<T>: Collection<T> + sequential::DoubleEnded<T> {}
impl<T, C: Collection<T> + sequential::DoubleEnded<T>> DoubleEndedCollection<T> for C {}

/// Std owned doubled-ended sequential collection (`VecDeque<T>`, `LinkedList<T>`)
pub trait OwnedDoubleEndedCollection<T>: OwnedCollection<T> + sequential::OwnedDoubleEnded<T> {}
impl<T, C: OwnedCollection<T> + sequential::OwnedDoubleEnded<T>> OwnedDoubleEndedCollection<T> for C {}

/// Std random-accessible sequential collection (e.g., `&mut [T]`, `[T; N]`)
pub trait RandomAccessCollection<T>: Collection<T> + sequential::RandomAccess<T> {}
impl<T, C: Collection<T> + sequential::RandomAccess<T>> RandomAccessCollection<T> for C {}

/// Std owned random-accessible sequential collection (`Vec<T>`, `VecDeque<T>`)
pub trait OwnedRandomAccessCollection<T>: OwnedCollection<T> + sequential::OwnedRandomAccess<T> {}
impl<T, C: OwnedCollection<T> + sequential::OwnedRandomAccess<T>> OwnedRandomAccessCollection<T> for C {}

/// Std sets (`BTreeSet<K>`, `HashSet<K, S>`)
pub trait Set<K, S = ()>:
    OwnedCollection<K> + associated::AssociatedSet<K> + associated::AssociatedSetOperation<K, S>
{
}
impl<K, S, C: OwnedCollection<K> + associated::AssociatedSet<K> + associated::AssociatedSetOperation<K, S>> Set<K, S>
    for C
{
}

/// Std ordered set (`BTreeSet<K>`)
pub trait OrderedSet<K, S = ()>: Set<K, S> + associated::AssociatedSetOrd<K> {}
impl<K, S, C: Set<K, S> + associated::AssociatedSetOrd<K>> OrderedSet<K, S> for C {}

/// Std maps (`BTreeMap<K, V>`, `HashMap<K, V, S>`)
pub trait Map<K, V, S = ()>:
    OwnedAssociatedCollection<K, V> + associated::AssociatedMap<K, V> + associated::AssociatedMapIter<K, V, S>
{
}
impl<
    K,
    V,
    S,
    C: OwnedAssociatedCollection<K, V> + associated::AssociatedMap<K, V> + associated::AssociatedMapIter<K, V, S>,
> Map<K, V, S> for C
{
}

/// Std ordered map (`BTreeMap<K, V>`)
pub trait OrderedMap<K, V, S = ()>: Map<K, V, S> + associated::AssociatedMapOrd<K, V> {}
impl<K, V, S, C: Map<K, V, S> + associated::AssociatedMapOrd<K, V>> OrderedMap<K, V, S> for C {}

/// Std collections that are aware of allocators. (`Vec<T>`)
pub trait AllocatorAwareCollection<T>:
    OwnedCollection<T> + allocator::AllocatorAware + allocator::AllocatorConstructible
{
}
impl<T, C: OwnedCollection<T> + allocator::AllocatorAware + allocator::AllocatorConstructible>
    AllocatorAwareCollection<T> for C
{
}

/// Std collections that are aware of capacities.
pub trait CapacityAwareCollection<T>:
    OwnedCollection<T>
    + capacity::CapacityAware
    + capacity::CapacityConstructible
    + capacity::CapacityShrink
    + capacity::CapacityReserve
{
}
impl<
    T,
    C: OwnedCollection<T>
        + capacity::CapacityAware
        + capacity::CapacityConstructible
        + capacity::CapacityShrink
        + capacity::CapacityReserve,
> CapacityAwareCollection<T> for C
{
}

pub trait AssociatedCapacityAwareCollection<K, V>:
    OwnedAssociatedCollection<K, V>
    + capacity::CapacityAware
    + capacity::CapacityConstructible
    + capacity::CapacityShrink
    + capacity::CapacityReserve
{
}
impl<
    K,
    V,
    C: OwnedAssociatedCollection<K, V>
        + capacity::CapacityAware
        + capacity::CapacityConstructible
        + capacity::CapacityShrink
        + capacity::CapacityReserve,
> AssociatedCapacityAwareCollection<K, V> for C
{
}
