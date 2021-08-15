use crate::capacity::CapacityAware;

/// Types that have allocator.
pub trait AllocatorAware {
    /// Allocator type
    type Allocator: std::alloc::Allocator;

    /// Returns the allocator associated with `Self`
    fn allocator(&self) -> &Self::Allocator;
}

pub trait AllocatorConstructible: AllocatorAware {
    /// Construct `Self` without data, given an allocator.
    fn new_in(alloc: <Self as AllocatorAware>::Allocator) -> Self;
}

pub trait AllocatorCapacityConstructible: AllocatorAware + CapacityAware + AllocatorConstructible {
    /// Construct `Self` without data, given capacity and an allocator.
    fn with_capacity_in(capacity: Self::CapacityType, alloc: Self::Allocator) -> Self;
}

mod impls {
    use super::*;

    macro_rules! allocator_impls {
        () => {};
        ([@Alloc $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AllocatorAware for $t {
                type Allocator = A;

                fn allocator(&self) -> &Self::Allocator {
                    <$t>::allocator(self)
                }
            }
            allocator_impls!($($tail)*);
        };
        ([@AllocCtor $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AllocatorConstructible for $t {
                fn new_in(alloc: Self::Allocator) -> Self {
                    <$t>::new_in(alloc)
                }
            }
            allocator_impls!($($tail)*);
        };
        ([@AllocCapCtor $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> AllocatorCapacityConstructible for $t {
                fn with_capacity_in(capacity: Self::CapacityType, alloc: Self::Allocator) -> Self {
                    <$t>::with_capacity_in(capacity, alloc)
                }
            }
            allocator_impls!($($tail)*);
        };
    }

    allocator_impls!(
        [@Alloc T, A: std::alloc::Allocator => Vec<T, A>];
        [@AllocCtor T, A: std::alloc::Allocator => Vec<T, A>];
        [@AllocCapCtor T, A: std::alloc::Allocator => Vec<T, A>];
    );
}
