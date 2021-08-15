/// Collections that can return length.
pub trait ExactSized {
    /// Size unit for indexing & length
    type SizeType: std::cmp::PartialEq = usize;

    /// Checks if `Self` is  empty
    fn is_empty(&self) -> bool;

    /// Returns the length of `Self`
    fn len(&self) -> Self::SizeType;
}

mod impls {
    use super::ExactSized;

    macro_rules! exact_sized_impls {
        () => {};
        ([$($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> ExactSized for $t {
                type SizeType = usize;

                fn is_empty(&self) -> bool {
                    <$t>::is_empty(self)
                }

                fn len(&self) -> usize {
                    <$t>::len(self)
                }
            }
            exact_sized_impls!($($tail)*);
        }
    }

    impl<T> ExactSized for &[T] {
        fn is_empty(&self) -> bool {
            <[T]>::is_empty(self)
        }

        fn len(&self) -> usize {
            <[T]>::len(self)
        }
    }

    impl<T> ExactSized for &mut [T] {
        fn is_empty(&self) -> bool {
            <[T]>::is_empty(self)
        }

        fn len(&self) -> usize {
            <[T]>::len(self)
        }
    }

    impl<T, const N: usize> ExactSized for [T; N] {
        fn is_empty(&self) -> bool {
            N == 0
        }

        fn len(&self) -> usize {
            N
        }
    }

    exact_sized_impls!(
        [T, A: std::alloc::Allocator => Vec<T, A>];
        [T => std::collections::VecDeque<T>];
        [T => std::collections::LinkedList<T>];
        [T => std::collections::BTreeSet<T>];
        [T, S: std::hash::BuildHasher => std::collections::HashSet<T, S>];
        [T => std::collections::BinaryHeap<T>];
        [K, V => std::collections::BTreeMap<K, V>];
        [K, V, S: std::hash::BuildHasher => std::collections::HashMap<K, V, S>];
    );
}
