//! Trait crate for abstracting common operations in `std` collections.
#![feature(generic_associated_types)]
#![feature(array_methods)]
#![feature(allocator_api)]
#![feature(associated_type_defaults)]
#![feature(try_reserve)]
#![feature(trusted_random_access)]
#![feature(type_alias_impl_trait)]
#![feature(slice_range)]
#![feature(btree_drain_filter)]
#![feature(drain_filter)]
#![feature(hash_drain_filter)]
#![feature(linked_list_remove)]
#![feature(map_try_insert)]
#![feature(map_first_last)]
#![feature(extend_one)]

use crate::elem::{AssociatedCollectionTrait, CollectionTrait, ExtendOwned, IntoIteratorOwned};

pub mod allocator;
pub mod associated;
pub mod capacity;
pub mod elem;
pub mod exact_size;
pub mod iter;
pub mod misc;
pub mod sequential;

/// Std collections (e.g, `[T; N]`)
///
/// NOTE: not all collections can mutably iterate, e.g., `HashSet<T>`, `BinaryHeap<T>`
pub trait Collection:
    CollectionTrait
    + iter::Iterable
    + exact_size::ExactSized
    + IntoIteratorOwned
    + ExtendOwned
    + misc::EmptyConstructible
    + misc::Erasable
    + HasExactSizeIterator
{
}
impl<
    C: CollectionTrait
        + iter::Iterable
        + exact_size::ExactSized
        + IntoIteratorOwned
        + ExtendOwned
        + misc::EmptyConstructible
        + misc::Erasable
        + HasExactSizeIterator,
> Collection for C
{
}

/// Std associated collections (e.g., `HashMap<K, V>`)
pub trait AssociatedCollection:
    CollectionTrait<ElemType = (Self::KeyType, Self::ValueType)>
    + AssociatedCollectionTrait
    + iter::AssociatedIterable
    + iter::AssociatedIterableMut
    + exact_size::ExactSized
    + IntoIteratorOwned
    + ExtendOwned
    + misc::EmptyConstructible
    + misc::Erasable
    + HasAssociatedExactSizeIterator
{
}
impl<
    C: CollectionTrait<ElemType = (Self::KeyType, Self::ValueType)>
        + AssociatedCollectionTrait
        + iter::AssociatedIterable
        + iter::AssociatedIterableMut
        + exact_size::ExactSized
        + IntoIteratorOwned
        + ExtendOwned
        + misc::EmptyConstructible
        + misc::Erasable
        + HasAssociatedExactSizeIterator,
> AssociatedCollection for C
{
}

pub trait HasExactSizeIterator {}
impl<C: iter::Iterable> HasExactSizeIterator for C where
    for<'a> <C as iter::Iterable>::Iter<'a>: std::iter::ExactSizeIterator
{
}

pub trait HasAssociatedExactSizeIterator {}
impl<C: iter::AssociatedIterable> HasAssociatedExactSizeIterator for C where
    for<'a> <C as iter::AssociatedIterable>::Iter<'a>: std::iter::ExactSizeIterator
{
}

/// Std contiguous collections (e.g., `[T; N]`, `&mut [T]`)
pub trait ContiguousCollection: Collection + sequential::Contiguous {}
impl<C: Collection + sequential::Contiguous> ContiguousCollection for C {}

/// Std sequential collection (e.g., `&[T]`, `LinkedList<T>`)
pub trait SequentialCollection: Collection + sequential::Sequential {}
impl<C: Collection + sequential::Sequential> SequentialCollection for C {}

/// Std doubled-ended sequential collection (e.g. `VecDeque<T>`)
pub trait DoubleEndedCollection: Collection + sequential::DoubleEnded + HasDoubleEndedIterator {}
impl<C: Collection + sequential::DoubleEnded + HasDoubleEndedIterator> DoubleEndedCollection for C {}

pub trait HasDoubleEndedIterator {}
impl<C: iter::Iterable> HasDoubleEndedIterator for C where
    for<'a> <C as iter::Iterable>::Iter<'a>: std::iter::DoubleEndedIterator
{
}

/// Std random-accessible sequential collection (e.g., `&mut [T]`, `[T; N]`)
pub trait RandomAccessCollection: Collection + sequential::RandomAccess + HasRandomAccessIterator {}
impl<C: Collection + sequential::RandomAccess + HasRandomAccessIterator> RandomAccessCollection for C {}

pub trait HasRandomAccessIterator {}
impl<C: iter::Iterable> HasRandomAccessIterator for C where
    for<'a> <C as iter::Iterable>::Iter<'a>: std::iter::TrustedRandomAccess
{
}

/// Std sets (`BTreeSet<K>`, `HashSet<K, S>`)
pub trait Set<S = ()>: Collection + associated::AssociatedSet + associated::AssociatedSetOperation<S> {}
impl<S, C: Collection + associated::AssociatedSet + associated::AssociatedSetOperation<S>> Set<S> for C {}

/// Std ordered set (`BTreeSet<K>`)
pub trait OrderedSet<S = ()>: Set<S> + associated::AssociatedSetOrd {}
impl<S, C: Set<S> + associated::AssociatedSetOrd> OrderedSet<S> for C {}

/// Std maps (`BTreeMap<K, V>`, `HashMap<K, V, S>`)
pub trait Map<S = ()>: AssociatedCollection + associated::AssociatedMap + associated::AssociatedMapIter<S> {}
impl<S, C: AssociatedCollection + associated::AssociatedMap + associated::AssociatedMapIter<S>> Map<S> for C {}

/// Std ordered map (`BTreeMap<K, V>`)
pub trait OrderedMap<S = ()>: Map<S> + associated::AssociatedMapOrd {}
impl<S, C: Map<S> + associated::AssociatedMapOrd> OrderedMap<S> for C {}

/// Std collections that are aware of allocators. (`Vec<T>`)
pub trait AllocatorAwareCollection: Collection + allocator::AllocatorAware + allocator::AllocatorConstructible {}
impl<C: Collection + allocator::AllocatorAware + allocator::AllocatorConstructible> AllocatorAwareCollection for C {}

/// Std collections that are aware of capacities.
pub trait CapacityAwareCollection:
    Collection
    + capacity::CapacityAware
    + capacity::CapacityConstructible
    + capacity::CapacityShrink
    + capacity::CapacityReserve
{
}
impl<
    C: Collection
        + capacity::CapacityAware
        + capacity::CapacityConstructible
        + capacity::CapacityShrink
        + capacity::CapacityReserve,
> CapacityAwareCollection for C
{
}

pub trait AssociatedCapacityAwareCollection:
    AssociatedCollection
    + capacity::CapacityAware
    + capacity::CapacityConstructible
    + capacity::CapacityShrink
    + capacity::CapacityReserve
{
}
impl<
    C: AssociatedCollection
        + capacity::CapacityAware
        + capacity::CapacityConstructible
        + capacity::CapacityShrink
        + capacity::CapacityReserve,
> AssociatedCapacityAwareCollection for C
{
}
