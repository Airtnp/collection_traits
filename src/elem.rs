use std::hint::unreachable_unchecked;

pub trait CollectionTrait {
    type ElemType;
}

pub trait AssociatedCollectionTrait {
    type KeyType;
    type ValueType;
}

pub trait Mutable: CollectionTrait {}
pub trait Owned: Mutable + IntoIterator<Item = Self::ElemType> {}

pub trait ExtendOwned: CollectionTrait {
    fn extend<T>(&mut self, _: T)
    where
        T: IntoIterator<Item = Self::ElemType>,
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }

    fn extend_one(&mut self, _: Self::ElemType)
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }
    fn extend_reserve(&mut self, _: usize)
    where
        Self: Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }
}

impl<V: CollectionTrait + Extend<V::ElemType> + Owned> ExtendOwned for V {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Self::ElemType>,
        Self: Owned,
    {
        self.extend(iter)
    }

    fn extend_one(&mut self, item: Self::ElemType)
    where
        Self: Owned,
    {
        self.extend_one(item)
    }
    fn extend_reserve(&mut self, additional: usize)
    where
        Self: Owned,
    {
        self.extend_reserve(additional)
    }
}

pub trait IntoIteratorOwned: CollectionTrait {
    type Item = Self::ElemType;
    fn into_iter_owned(self) -> <Self as IntoIterator>::IntoIter
    where
        Self: Sized + Owned,
    {
        // SAFETY: Constrained by trait requirements
        unsafe { unreachable_unchecked() }
    }
}

impl<T: CollectionTrait + Owned> IntoIteratorOwned for T {
    fn into_iter_owned(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

mod impls {
    use super::*;

    impl CollectionTrait for &str {
        type ElemType = char;
    }

    impl ExtendOwned for &str {}
    impl IntoIteratorOwned for &str {}

    impl CollectionTrait for String {
        type ElemType = char;
    }

    impl<T> CollectionTrait for &[T] {
        type ElemType = T;
    }

    impl<T> ExtendOwned for &[T] {}
    impl<T> IntoIteratorOwned for &[T] {}

    impl<T> CollectionTrait for &mut [T] {
        type ElemType = T;
    }

    impl<T> ExtendOwned for &mut [T] {}
    impl<T> IntoIteratorOwned for &mut [T] {}

    impl<T> Mutable for &mut [T] {}
    impl<T, const N: usize> Mutable for [T; N] {}
    impl Mutable for String {}
    impl<T, A: std::alloc::Allocator> Mutable for Vec<T, A> {}
    impl<T> Mutable for std::collections::VecDeque<T> {}
    impl<T> Mutable for std::collections::LinkedList<T> {}
    impl<T> Mutable for std::collections::BinaryHeap<T> {}
    impl<T> Mutable for std::collections::BTreeSet<T> {}
    impl<T, S> Mutable for std::collections::HashSet<T, S> {}
    impl<K, V> Mutable for std::collections::BTreeMap<K, V> {}
    impl<K, V, S> Mutable for std::collections::HashMap<K, V, S> {}

    impl<T, A: std::alloc::Allocator> Owned for Vec<T, A> {}
    impl<T> Owned for std::collections::VecDeque<T> {}
    impl<T> Owned for std::collections::LinkedList<T> {}
    impl<T> Owned for std::collections::BinaryHeap<T> {}
    impl<T> Owned for std::collections::BTreeSet<T> {}
    impl<T, S> Owned for std::collections::HashSet<T, S> {}
    impl<K, V> Owned for std::collections::BTreeMap<K, V> {}
    impl<K, V, S> Owned for std::collections::HashMap<K, V, S> {}

    impl<T, const N: usize> CollectionTrait for [T; N] {
        type ElemType = T;
    }

    impl<T, const N: usize> ExtendOwned for [T; N] {}
    impl<T, const N: usize> IntoIteratorOwned for [T; N] {}

    impl<T, A: std::alloc::Allocator> CollectionTrait for Vec<T, A> {
        type ElemType = T;
    }

    impl<T> CollectionTrait for std::collections::VecDeque<T> {
        type ElemType = T;
    }

    impl<T> CollectionTrait for std::collections::LinkedList<T> {
        type ElemType = T;
    }

    impl<T, S> CollectionTrait for std::collections::HashSet<T, S> {
        type ElemType = T;
    }

    impl<T> CollectionTrait for std::collections::BTreeSet<T> {
        type ElemType = T;
    }

    impl<T> CollectionTrait for std::collections::BinaryHeap<T> {
        type ElemType = T;
    }

    impl<K, V, S> CollectionTrait for std::collections::HashMap<K, V, S> {
        type ElemType = (K, V);
    }

    impl<K, V> CollectionTrait for std::collections::BTreeMap<K, V> {
        type ElemType = (K, V);
    }

    impl<K, S> AssociatedCollectionTrait for std::collections::HashSet<K, S> {
        type KeyType = K;
        type ValueType = ();
    }

    impl<K> AssociatedCollectionTrait for std::collections::BTreeSet<K> {
        type KeyType = K;
        type ValueType = ();
    }

    impl<K, V, S> AssociatedCollectionTrait for std::collections::HashMap<K, V, S> {
        type KeyType = K;
        type ValueType = V;
    }

    impl<K, V> AssociatedCollectionTrait for std::collections::BTreeMap<K, V> {
        type KeyType = K;
        type ValueType = V;
    }
}
