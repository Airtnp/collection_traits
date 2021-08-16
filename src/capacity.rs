use crate::exact_size::ExactSized;

/// Types that have capacity.
pub trait CapacityAware: ExactSized {
    /// Capacity unit
    type CapacityType = Self::SizeType;

    /// Returns the number of elements the `Self` can hold without reallocating.
    fn capacity(&self) -> Self::CapacityType;
}

/// Collections that can construct `Self` with capacity.
pub trait CapacityConstructible: CapacityAware {
    /// Construct `Self` without data, given capacity.
    fn with_capacity(capacity: Self::CapacityType) -> Self;
}

/// Collections that can shrink based on actual elements.
pub trait CapacityShrink: CapacityAware {
    /// Shrinks the capacity of `Self` as much as possible.
    fn shrink_to_fit(&mut self) {}

    /// Shrinks the capacity of `Self` with a lower bound.
    fn shrink_to(&mut self, min_capacity: Self::CapacityType);
}

/// Collections that can reserve capacity.
pub trait CapacityReserve: CapacityAware {
    /// Try reserve errors
    type TryReserveError: std::error::Error;

    /// Reserves capacity for at least additional more elements to be inserted in the given `Self`.
    fn reserve(&mut self, additional: Self::CapacityType);

    /// Tries to reserve capacity for at least additional more elements to be inserted in the given `Self`.
    fn try_reserve(&mut self, additional: Self::CapacityType) -> Result<(), Self::TryReserveError>;
}

/// Collections that can reserve exact capacity.
pub trait CapacityReserveExact: CapacityReserve {
    /// Reserves the minimum capacity for exactly additional more elements to be inserted in the given `Self`.
    fn reserve_exact(&mut self, additional: Self::CapacityType);

    /// Tries to reserve the minimum capacity for exactly additional elements to be inserted in the given `Self`.
    fn try_reserve_exact(&mut self, additional: Self::CapacityType) -> Result<(), Self::TryReserveError>;
}

mod impls {
    use super::*;

    macro_rules! capacity_impls {
        () => {};
        ([@Cap $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> CapacityAware for $t {
                type CapacityType = usize;

                fn capacity(&self) -> Self::CapacityType {
                    <$t>::capacity(self)
                }
            }
            capacity_impls!($($tail)*);
        };
        ([@CapCtor $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> CapacityConstructible for $t {
                fn with_capacity(capacity: Self::CapacityType) -> Self {
                    <$t>::with_capacity(capacity)
                }
            }
            capacity_impls!($($tail)*);
        };
        ([@CapShrink $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> CapacityShrink for $t {
                fn shrink_to_fit(&mut self) {
                    <$t>::shrink_to_fit(self)
                }

                fn shrink_to(&mut self, min_capacity: Self::CapacityType) {
                    <$t>::shrink_to(self, min_capacity)
                }
            }
            capacity_impls!($($tail)*);
        };
        ([@CapReserve $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> CapacityReserve for $t {
                type TryReserveError = std::collections::TryReserveError;

                fn reserve(&mut self, additional: Self::CapacityType) {
                    <$t>::reserve(self, additional)
                }

                fn try_reserve(&mut self, additional: Self::CapacityType) -> Result<(), Self::TryReserveError> {
                    <$t>::try_reserve(self, additional)
                }
            }
            capacity_impls!($($tail)*);
        };
        ([@CapReserveExact $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> CapacityReserveExact for $t {
                fn reserve_exact(&mut self, additional: Self::CapacityType) {
                    <$t>::reserve_exact(self, additional)
                }

                fn try_reserve_exact(&mut self, additional: Self::CapacityType) -> Result<(), Self::TryReserveError> {
                    <$t>::try_reserve_exact(self, additional)
                }
            }
            capacity_impls!($($tail)*);
        };
    }

    capacity_impls!(
        [@Cap T, A: std::alloc::Allocator => Vec<T, A>];
        [@CapCtor T => Vec<T>];
        [@CapShrink T, A: std::alloc::Allocator => Vec<T, A>];
        [@CapReserve T, A: std::alloc::Allocator => Vec<T, A>];
        [@CapReserveExact T, A: std::alloc::Allocator => Vec<T, A>];

        [@Cap T => std::collections::VecDeque<T>];
        [@CapCtor T => std::collections::VecDeque<T>];
        [@CapShrink T => std::collections::VecDeque<T>];
        [@CapReserve T => std::collections::VecDeque<T>];
        [@CapReserveExact T => std::collections::VecDeque<T>];

        [@Cap T => std::collections::BinaryHeap<T>];
        [@CapCtor T: std::cmp::Ord => std::collections::BinaryHeap<T>];
        [@CapShrink T => std::collections::BinaryHeap<T>];
        // BinaryHeap<T> doesn't have `try_reserve`
        // [@CapReserve T => std::collections::BinaryHeap<T>];

        [@Cap T, S: std::hash::BuildHasher => std::collections::HashSet<T, S>];
        [@CapCtor T => std::collections::HashSet<T>];
        [@CapShrink T: std::cmp::Eq | std::hash::Hash, S: std::hash::BuildHasher => std::collections::HashSet<T, S>];
        [@CapReserve T: std::cmp::Eq | std::hash::Hash, S: std::hash::BuildHasher => std::collections::HashSet<T, S>];

        [@Cap K, V, S: std::hash::BuildHasher => std::collections::HashMap<K, V, S>];
        [@CapCtor K, V => std::collections::HashMap<K, V>];
        [@CapShrink K: std::cmp::Eq | std::hash::Hash, V, S: std::hash::BuildHasher => std::collections::HashMap<K, V, S>];
        [@CapReserve K: std::cmp::Eq | std::hash::Hash, V, S: std::hash::BuildHasher => std::collections::HashMap<K, V, S>];
    );
}
