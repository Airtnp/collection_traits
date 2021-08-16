#![allow(unused_variables)]
use crate::{
    elem::{CollectionTrait, Mutable, Owned},
    exact_size::ExactSized,
    iter::Iterable,
};
use std::hint::unreachable_unchecked;

/// Collections that behave like a contiguous region of memory. (`AsRef<[T]> + AsMut<[T]>`)
pub trait Contiguous: CollectionTrait + AsRef<[Self::ElemType]> {
    /// Extracts a immutable slice of `Self`
    fn as_slice(&self) -> &[Self::ElemType];

    /// Extracts a mutable slice of `Self`
    fn as_mut_slice(&mut self) -> &mut [Self::ElemType]
    where
        Self: Mutable,
    {
        unimplemented!()
    }
}

/// Collections that behave like a sequence that can access from front to back.
pub trait Sequential: CollectionTrait + ExactSized {
    /// Provides a reference to the back element, or `None` if `self` is empty.
    fn back(&self) -> Option<&Self::ElemType>;

    /// Provides a mutable reference to the back element, or `None` if `self` is empty.
    fn back_mut(&mut self) -> Option<&mut Self::ElemType>
    where
        Self: Mutable,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Moves all elements from `other` into `self`, leaving `other` empty.
    fn append(&mut self, other: &mut Self)
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Appends an element to the back of `self`
    fn push_back(&mut self, elt: Self::ElemType)
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Removes the last element from `self` and returns it, or `None` if it is empty.
    fn pop_back(&mut self) -> Option<Self::ElemType>
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Removes the element at the given index and returns it.
    fn remove(&mut self, at: Self::SizeType) -> Option<Self::ElemType>
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Splits the collection into two at the given index.
    fn split_off(&mut self, at: Self::SizeType) -> Self
    where
        Self: Owned + Sized,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }
}

/// Collections that can access sequentially in both ends.
pub trait DoubleEnded: Sequential + Iterable {
    /// Provides a reference to the front element, or `None` if `self` is empty.
    fn front(&self) -> Option<&Self::ElemType>;

    /// Provides a mutable reference to the front element, or `None` if `self` is empty.
    fn front_mut(&mut self) -> Option<&mut Self::ElemType>
    where
        Self: Mutable,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Prepends an element to `self`.
    fn push_front(&mut self, value: Self::ElemType)
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Removes the first element and returns it, or `None` if `self` is empty
    fn pop_front(&mut self) -> Option<Self::ElemType>
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }
}

/// Collections that are randomly accessible in about O(1) time.
///
/// NOTE: complete definition requires `TrustRandomAccess`.
pub trait RandomAccess: Sequential + ExactSized {
    /// Rotates the double-ended queue mid places to the left.
    fn rotate_left(&mut self, mid: Self::SizeType)
    where
        Self: Mutable,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Rotates the double-ended queue mid places to the right.
    fn rotate_right(&mut self, mid: Self::SizeType)
    where
        Self: Mutable,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).
    fn partition_point<P: FnMut(&Self::ElemType) -> bool>(&self, pred: P) -> Self::SizeType;

    /// Swaps elements at indices i and j.
    fn swap(&mut self, i: Self::SizeType, j: Self::SizeType)
    where
        Self: Mutable,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Provides a reference to the element at the given index.
    fn get(&self, index: Self::SizeType) -> Option<&Self::ElemType>;

    /// Provides a mutable reference to the element at the given index.
    fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut Self::ElemType>
    where
        Self: Mutable,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Inserts an element at `index` within `self`, shifting all elements with indices greater than or equal to `index` towards the back.
    fn insert(&mut self, index: Self::SizeType, value: Self::ElemType)
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    /// Removes an element from anywhere in `self` and returns it, replacing it with the last element.
    fn swap_remove_back(&mut self, index: Self::SizeType) -> Option<Self::ElemType>
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }
}

mod impls {
    use super::*;

    macro_rules! sequential_impls {
        () => {};
        ([@Contiguous $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> Contiguous for $t {
                fn as_slice(&self) -> &[Self::ElemType] {
                    <$t>::as_slice(self)
                }

                fn as_mut_slice(&mut self) -> &mut [Self::ElemType] {
                    <$t>::as_mut_slice(self)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@Sequential $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> Sequential for $t {
                fn back(&self) -> Option<&Self::ElemType> {
                    <$t>::back(self)
                }

                fn back_mut(&mut self) -> Option<&mut Self::ElemType> {
                    <$t>::back_mut(self)
                }

                fn append(&mut self, other: &mut Self) {
                    <$t>::append(self, other)
                }

                fn push_back(&mut self, elt: Self::ElemType) {
                    <$t>::push_back(self, elt)
                }

                fn pop_back(&mut self) -> Option<Self::ElemType> {
                    <$t>::pop_back(self)
                }

                fn remove(&mut self, at: Self::SizeType) -> Option<Self::ElemType> {
                    <$t>::remove(self, at)
                }

                fn split_off(&mut self, at: Self::SizeType) -> Self {
                    <$t>::split_off(self, at)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@DoubleEnded $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> DoubleEnded for $t {
                fn front(&self) -> Option<&Self::ElemType> {
                    <$t>::front(self)
                }

                fn front_mut(&mut self) -> Option<&mut Self::ElemType> {
                    <$t>::front_mut(self)
                }

                fn push_front(&mut self, value: Self::ElemType) {
                    <$t>::push_front(self, value)
                }

                fn pop_front(&mut self) -> Option<Self::ElemType> {
                    <$t>::pop_front(self)
                }
            }
            sequential_impls!($($tail)*);
        };
        ([@RandomAccess $($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> RandomAccess for $t {
                fn rotate_left(&mut self, mid: Self::SizeType) {
                    <$t>::rotate_left(self, mid)
                }

                fn rotate_right(&mut self, mid: Self::SizeType) {
                    <$t>::rotate_right(self, mid)
                }

                fn partition_point<P: FnMut(&Self::ElemType) -> bool>(&self, pred: P) -> Self::SizeType {
                    <$t>::partition_point(self, pred)
                }

                fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
                    <$t>::swap(self, i, j)
                }

                fn get(&self, index: Self::SizeType) -> Option<&Self::ElemType> {
                    <$t>::get(self, index)
                }

                fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut Self::ElemType> {
                    <$t>::get_mut(self, index)
                }

                fn insert(&mut self, index: Self::SizeType, value: Self::ElemType) {
                    <$t>::insert(self, index, value)
                }

                fn swap_remove_back(&mut self, index: Self::SizeType) -> Option<Self::ElemType> {
                    <$t>::swap_remove_back(self, index)
                }
            }
            sequential_impls!($($tail)*);
        };
    }

    impl<T> Contiguous for &[T] {
        fn as_slice(&self) -> &[Self::ElemType] {
            self
        }
    }

    impl<T> Contiguous for &mut [T] {
        fn as_slice(&self) -> &[Self::ElemType] {
            self
        }

        fn as_mut_slice(&mut self) -> &mut [Self::ElemType] {
            self
        }
    }

    impl<T, const N: usize> Contiguous for [T; N] {
        fn as_slice(&self) -> &[Self::ElemType] {
            <[T; N]>::as_slice(self)
        }

        fn as_mut_slice(&mut self) -> &mut [Self::ElemType] {
            <[T; N]>::as_mut_slice(self)
        }
    }

    impl<T> Sequential for &[T] {
        fn back(&self) -> Option<&Self::ElemType> {
            self.last()
        }
    }

    impl<T> Sequential for &mut [T] {
        fn back(&self) -> Option<&Self::ElemType> {
            self.last()
        }

        fn back_mut(&mut self) -> Option<&mut Self::ElemType> {
            self.last_mut()
        }
    }

    impl<T, const N: usize> Sequential for [T; N] {
        fn back(&self) -> Option<&Self::ElemType> {
            self.last()
        }

        fn back_mut(&mut self) -> Option<&mut Self::ElemType> {
            self.last_mut()
        }
    }

    impl<T, A: std::alloc::Allocator + std::clone::Clone> Sequential for Vec<T, A> {
        fn back(&self) -> Option<&Self::ElemType> {
            self.as_slice().last()
        }

        fn back_mut(&mut self) -> Option<&mut Self::ElemType> {
            self.as_mut_slice().last_mut()
        }

        fn append(&mut self, other: &mut Self) {
            self.append(other)
        }

        fn push_back(&mut self, elt: Self::ElemType) {
            self.push(elt)
        }

        fn pop_back(&mut self) -> Option<Self::ElemType> {
            self.pop()
        }

        // `Vec<T>::remove` returns `T` instead of `Option<T>` (`VecDeque<T>` returns `Option<T>`)
        fn remove(&mut self, at: Self::SizeType) -> Option<Self::ElemType> {
            Some(self.remove(at))
        }

        fn split_off(&mut self, at: Self::SizeType) -> Self {
            self.split_off(at)
        }
    }

    impl<T> Sequential for std::collections::LinkedList<T> {
        fn back(&self) -> Option<&Self::ElemType> {
            self.back()
        }

        fn back_mut(&mut self) -> Option<&mut Self::ElemType> {
            self.back_mut()
        }

        fn append(&mut self, other: &mut Self) {
            self.append(other)
        }

        fn push_back(&mut self, elt: Self::ElemType) {
            self.push_back(elt)
        }

        fn pop_back(&mut self) -> Option<Self::ElemType> {
            self.pop_back()
        }

        // `LinkedList<T>::remove` returns `T` instead of `Option<T>`
        fn remove(&mut self, at: Self::SizeType) -> Option<Self::ElemType> {
            Some(self.remove(at))
        }

        fn split_off(&mut self, at: Self::SizeType) -> Self {
            self.split_off(at)
        }
    }

    impl<T> RandomAccess for &mut [T] {
        fn rotate_left(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_left(self, mid)
        }

        fn rotate_right(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_right(self, mid)
        }

        fn partition_point<P: FnMut(&Self::ElemType) -> bool>(&self, pred: P) -> Self::SizeType {
            <[T]>::partition_point(self, pred)
        }

        fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
            <[T]>::swap(self, i, j)
        }

        fn get(&self, index: Self::SizeType) -> Option<&Self::ElemType> {
            <[T]>::get(self, index)
        }

        fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut Self::ElemType> {
            <[T]>::get_mut(self, index)
        }
    }

    impl<T, const N: usize> RandomAccess for [T; N] {
        fn rotate_left(&mut self, mid: Self::SizeType) {
            self.as_mut_slice().rotate_left(mid)
        }

        fn rotate_right(&mut self, mid: Self::SizeType) {
            self.as_mut_slice().rotate_right(mid)
        }

        fn partition_point<P: FnMut(&Self::ElemType) -> bool>(&self, pred: P) -> Self::SizeType {
            self.as_slice().partition_point(pred)
        }

        fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
            self.as_mut_slice().swap(i, j)
        }

        fn get(&self, index: Self::SizeType) -> Option<&T> {
            self.as_slice().get(index)
        }

        fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut Self::ElemType> {
            self.as_mut_slice().get_mut(index)
        }
    }

    impl<T, A: std::alloc::Allocator + std::clone::Clone> RandomAccess for Vec<T, A> {
        fn rotate_left(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_left(self, mid)
        }

        fn rotate_right(&mut self, mid: Self::SizeType) {
            <[T]>::rotate_right(self, mid)
        }

        fn partition_point<P: FnMut(&Self::ElemType) -> bool>(&self, pred: P) -> Self::SizeType {
            <[T]>::partition_point(self, pred)
        }

        fn swap(&mut self, i: Self::SizeType, j: Self::SizeType) {
            <[T]>::swap(self, i, j)
        }

        fn get(&self, index: Self::SizeType) -> Option<&Self::ElemType> {
            <[T]>::get(self, index)
        }

        fn get_mut(&mut self, index: Self::SizeType) -> Option<&mut Self::ElemType> {
            <[T]>::get_mut(self, index)
        }

        fn insert(&mut self, index: Self::SizeType, value: Self::ElemType) {
            self.insert(index, value)
        }

        fn swap_remove_back(&mut self, index: Self::SizeType) -> Option<Self::ElemType> {
            Some(self.swap_remove(index))
        }
    }

    sequential_impls!(
        [@Contiguous T, A: std::alloc::Allocator => Vec<T, A>];

        [@Sequential T => std::collections::VecDeque<T>];

        [@DoubleEnded T => std::collections::VecDeque<T>];
        [@DoubleEnded T => std::collections::LinkedList<T>];

        [@RandomAccess T => std::collections::VecDeque<T>];
    );
}
