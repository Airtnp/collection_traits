use crate::{
    exact_size::ExactSized,
    iter::{Iterable, IterableMut},
};

/// Collections that behave like a contiguous region of memory. (`AsRef<[T]> + AsMut<[T]>`)
pub trait Contiguous<T>: AsRef<[T]> + AsMut<[T]> {
    /// Extracts a immutable slice of `Self`
    fn as_slice(&self) -> &[T];

    /// Extracts a mutable slice of `Self`
    fn as_mut_slice(&mut self) -> &mut [T];
}

/// Collections that behave like a sequence that can access from front to back.
pub trait Sequential<T>: Iterable<T> + IterableMut<T> + ExactSized {
    /// Provides a reference to the back element, or `None` if `self` is empty.
    fn back(&self) -> Option<&T>;

    /// Provides a mutable reference to the back element, or `None` if `self` is empty.
    fn back_mut(&mut self) -> Option<&mut T>;
}

/// Collections that behave like a mutable sequence that can access from front to back.
pub trait OwnedSequential<T>: Sequential<T> {
    /// Moves all elements from `other` into `self`, leaving `other` empty.
    fn append(&mut self, other: &mut Self);

    /// Appends an element to the back of `self`
    fn push_back(&mut self, elt: T);

    /// Removes the last element from `self` and returns it, or `None` if it is empty.
    fn pop_back(&mut self) -> Option<T>;

    /// Removes the element at the given index and returns it.
    fn remove(&mut self, at: Self::SizeType) -> Option<T>;

    /// Splits the collection into two at the given index.
    fn split_off(&mut self, at: Self::SizeType) -> Self;
}

/// Collections that can access sequentially in both ends.
pub trait DoubleEnded<T>: Sequential<T>
// FIXME: how to specify T: 'a here?
// where
//    for<'a> <Self as Iterable<T>>::Iter<'a>: std::iter::DoubleEndedIterator,
//    for<'a> <Self as IterableMut<T>>::IterMut<'a>: std::iter::DoubleEndedIterator,
{
    /// Provides a reference to the front element, or `None` if `self` is empty.
    fn front(&self) -> Option<&T>;

    /// Provides a mutable reference to the front element, or `None` if `self` is empty.
    fn front_mut(&mut self) -> Option<&mut T>;
}

/// Collections that can access and mutate sequentially in both ends.
pub trait OwnedDoubleEnded<T>: DoubleEnded<T> + OwnedSequential<T>
// where
//    for<'a> <Self as Iterable<T>>::Iter<'a>: std::iter::DoubleEndedIterator,
//    for<'a> <Self as IterableMut<T>>::IterMut<'a>: std::iter::DoubleEndedIterator,
{
    /// Prepends an element to `self`.
    fn push_front(&mut self, value: T);

    /// Removes the first element and returns it, or `None` if `self` is empty
    fn pop_front(&mut self) -> Option<T>;
}

/// Collections that are randomly accessible in about O(1) time.
///
/// NOTE: complete definition requires `TrustRandomAccess`.
/// However, adding these constraints will cause any implementation to be impossible:
/// can't specify `T: 'a` in the constraints
pub trait RandomAccess<T>: Sequential<T> + ExactSized
// FIXME: how to specify T: 'a here?
// where
//    for<'a> <Self as Iterable<T>>::Iter<'a>: std::iter::TrustedRandomAccess,
//    for<'a> <Self as IterableMut<T>>::IterMut<'a>: std::iter::TrustedRandomAccess,
{
    /// Rotates the double-ended queue mid places to the left.
    fn rotate_left(&mut self, mid: Self::SizeType);

    /// Rotates the double-ended queue mid places to the right.
    fn rotate_right(&mut self, mid: Self::SizeType);

    /// Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).
    fn partition_point<P: FnMut(&T) -> bool>(&self, pred: P) -> Self::SizeType;

    /// Swaps elements at indices i and j.
    fn swap(&mut self, i: Self::SizeType, j: Self::SizeType);

    /// Provides a reference to the element at the given index.
    fn get(&self, index: Self::SizeType) -> Option<&T>;

    /// Provides a mutable reference to the element at the given index.
    fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut T>;
}

/// Collections that are randomly accessible and owning their contents.
pub trait OwnedRandomAccess<T>:
    RandomAccess<T> + OwnedSequential<T> + std::ops::Index<Self::SizeType> + std::ops::IndexMut<Self::SizeType>
// where
//    for<'a> <Self as Iterable<T>>::Iter<'a>: std::iter::TrustedRandomAccess,
//    for<'a> <Self as IterableMut<T>>::IterMut<'a>: std::iter::TrustedRandomAccess,
{
    /// Inserts an element at `index` within `self`, shifting all elements with indices greater than or equal to `index` towards the back.
    fn insert(&mut self, index: Self::SizeType, value: T);

    /// Removes an element from anywhere in `self` and returns it, replacing it with the last element.
    fn swap_remove_back(&mut self, index: Self::SizeType) -> Option<T>;
}

mod impls {
    use super::*;

    macro_rules! sequential_impls {
        () => {};
        ([@Contiguous $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> Contiguous<T> for $t {
                fn as_slice(&self) -> &[T] {
                    <$t>::as_slice(self)
                }

                fn as_mut_slice(&mut self) -> &mut [T] {
                    <$t>::as_mut_slice(self)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@Sequential $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> Sequential<T> for $t {
                fn back(&self) -> Option<&T> {
                    <$t>::back(self)
                }

                fn back_mut(&mut self) -> Option<&mut T> {
                    <$t>::back_mut(self)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@OwnedSequential $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> OwnedSequential<T> for $t {
                fn append(&mut self, other: &mut Self) {
                    <$t>::append(self, other)
                }

                fn push_back(&mut self, elt: T) {
                    <$t>::push_back(self, elt)
                }

                fn pop_back(&mut self) -> Option<T> {
                    <$t>::pop_back(self)
                }

                fn remove(&mut self, at: Self::SizeType) -> Option<T> {
                    <$t>::remove(self, at)
                }

                fn split_off(&mut self, at: Self::SizeType) -> Self {
                    <$t>::split_off(self, at)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@DoubleEnded $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> DoubleEnded<T> for $t {
                fn front(&self) -> Option<&T> {
                    <$t>::front(self)
                }

                fn front_mut(&mut self) -> Option<&mut T> {
                    <$t>::front_mut(self)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@OwnedDoubleEnded $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> OwnedDoubleEnded<T> for $t {
                fn push_front(&mut self, value: T) {
                    <$t>::push_front(self, value)
                }

                fn pop_front(&mut self) -> Option<T> {
                    <$t>::pop_front(self)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@RandomAccess $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> RandomAccess<T> for $t {
                fn rotate_left(&mut self, mid: Self::SizeType) {
                    <$t>::rotate_left(self, mid)
                }

                fn rotate_right(&mut self, mid: Self::SizeType) {
                    <$t>::rotate_right(self, mid)
                }

                fn partition_point<P: FnMut(&T) -> bool>(&self, pred: P) -> Self::SizeType {
                    <$t>::partition_point(self, pred)
                }

                fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
                    <$t>::swap(self, i, j)
                }

                fn get(&self, index: Self::SizeType) -> Option<&T> {
                    <$t>::get(self, index)
                }

                fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut T> {
                    <$t>::get_mut(self, index)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@OwnedRandomAccess $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> OwnedRandomAccess<T> for $t {
                fn insert(&mut self, index: Self::SizeType, value: T) {
                    <$t>::insert(self, index, value)
                }

                fn swap_remove_back(&mut self, index: Self::SizeType) -> Option<T> {
                    <$t>::swap_remove_back(self, index)
                }
            }
            sequential_impls!($($tail)*);
        };
    }

    impl<T> Contiguous<T> for &mut [T] {
        fn as_slice(&self) -> &[T] {
            self
        }

        fn as_mut_slice(&mut self) -> &mut [T] {
            self
        }
    }

    impl<T, const N: usize> Contiguous<T> for [T; N] {
        fn as_slice(&self) -> &[T] {
            <[T; N]>::as_slice(self)
        }

        fn as_mut_slice(&mut self) -> &mut [T] {
            <[T; N]>::as_mut_slice(self)
        }
    }

    impl<T> Sequential<T> for &mut [T] {
        fn back(&self) -> Option<&T> {
            self.last()
        }

        fn back_mut(&mut self) -> Option<&mut T> {
            self.last_mut()
        }
    }

    impl<T, const N: usize> Sequential<T> for [T; N] {
        fn back(&self) -> Option<&T> {
            self.last()
        }

        fn back_mut(&mut self) -> Option<&mut T> {
            self.last_mut()
        }
    }

    impl<T, A: std::alloc::Allocator> Sequential<T> for Vec<T, A> {
        fn back(&self) -> Option<&T> {
            self.as_slice().last()
        }

        fn back_mut(&mut self) -> Option<&mut T> {
            self.as_mut_slice().last_mut()
        }
    }

    impl<T, A: std::alloc::Allocator + std::clone::Clone> OwnedSequential<T> for Vec<T, A> {
        fn append(&mut self, other: &mut Self) {
            self.append(other)
        }

        fn push_back(&mut self, elt: T) {
            self.push(elt)
        }

        fn pop_back(&mut self) -> Option<T> {
            self.pop()
        }

        // `Vec<T>::remove` returns `T` instead of `Option<T>` (`VecDeque<T>` returns `Option<T>`)
        fn remove(&mut self, at: Self::SizeType) -> Option<T> {
            Some(self.remove(at))
        }

        fn split_off(&mut self, at: Self::SizeType) -> Self {
            self.split_off(at)
        }
    }

    impl<T> OwnedSequential<T> for std::collections::LinkedList<T> {
        fn append(&mut self, other: &mut Self) {
            self.append(other)
        }

        fn push_back(&mut self, elt: T) {
            self.push_back(elt)
        }

        fn pop_back(&mut self) -> Option<T> {
            self.pop_back()
        }

        // `LinkedList<T>::remove` returns `T` instead of `Option<T>`
        fn remove(&mut self, at: Self::SizeType) -> Option<T> {
            Some(self.remove(at))
        }

        fn split_off(&mut self, at: Self::SizeType) -> Self {
            self.split_off(at)
        }
    }

    impl<T> RandomAccess<T> for &mut [T] {
        fn rotate_left(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_left(self, mid)
        }

        fn rotate_right(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_right(self, mid)
        }

        fn partition_point<P: FnMut(&T) -> bool>(&self, pred: P) -> Self::SizeType {
            <[T]>::partition_point(self, pred)
        }

        fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
            <[T]>::swap(self, i, j)
        }

        fn get(&self, index: Self::SizeType) -> Option<&T> {
            <[T]>::get(self, index)
        }

        fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut T> {
            <[T]>::get_mut(self, index)
        }
    }

    impl<T, const N: usize> RandomAccess<T> for [T; N] {
        fn rotate_left(&mut self, mid: Self::SizeType) {
            self.as_mut_slice().rotate_left(mid)
        }

        fn rotate_right(&mut self, mid: Self::SizeType) {
            self.as_mut_slice().rotate_right(mid)
        }

        fn partition_point<P: FnMut(&T) -> bool>(&self, pred: P) -> Self::SizeType {
            self.as_slice().partition_point(pred)
        }

        fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
            self.as_mut_slice().swap(i, j)
        }

        fn get(&self, index: Self::SizeType) -> Option<&T> {
            self.as_slice().get(index)
        }

        fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut T> {
            self.as_mut_slice().get_mut(index)
        }
    }

    impl<T, A: std::alloc::Allocator> RandomAccess<T> for Vec<T, A> {
        fn rotate_left(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_left(self, mid)
        }

        fn rotate_right(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_right(self, mid)
        }

        fn partition_point<P: FnMut(&T) -> bool>(&self, pred: P) -> Self::SizeType {
            <[T]>::partition_point(self, pred)
        }

        fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
            <[T]>::swap(self, i, j)
        }

        fn get(&self, index: Self::SizeType) -> Option<&T> {
            <[T]>::get(self, index)
        }

        fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut T> {
            <[T]>::get_mut(self, index)
        }
    }

    impl<T, A: std::alloc::Allocator + std::clone::Clone> OwnedRandomAccess<T> for Vec<T, A> {
        fn insert(&mut self, index: Self::SizeType, value: T) {
            self.insert(index, value)
        }

        fn swap_remove_back(&mut self, index: Self::SizeType) -> Option<T> {
            Some(self.swap_remove(index))
        }
    }

    sequential_impls!(
        [@Contiguous T, A: std::alloc::Allocator => Vec<T, A>];

        [@Sequential T => std::collections::VecDeque<T>];
        [@Sequential T => std::collections::LinkedList<T>];
        [@OwnedSequential T => std::collections::VecDeque<T>];

        [@DoubleEnded T => std::collections::VecDeque<T>];
        [@DoubleEnded T => std::collections::LinkedList<T>];
        [@OwnedDoubleEnded T => std::collections::VecDeque<T>];
        [@OwnedDoubleEnded T => std::collections::LinkedList<T>];

        [@RandomAccess T => std::collections::VecDeque<T>];
        [@OwnedRandomAccess T => std::collections::VecDeque<T>];
    );
}
