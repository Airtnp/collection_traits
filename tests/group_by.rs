#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]

use collection_traits::{elem::Owned, Collection, Map, SequentialCollection};
use std::collections::HashMap;

pub trait GroupBy: Collection + Owned {
    fn group_by<
        K,
        F: FnMut(&Self::ElemType) -> K,
        S: SequentialCollection<ElemType = Self::ElemType> + Owned,
        St,
        M: Map<St, KeyType = K, ValueType = S> + Owned,
    >(
        self,
        f: F,
    ) -> M;
}

impl<C: Collection + Owned> GroupBy for C {
    fn group_by<
        K,
        F: FnMut(&Self::ElemType) -> K,
        S: SequentialCollection<ElemType = Self::ElemType> + Owned,
        St,
        M: Map<St, KeyType = K, ValueType = S> + Owned,
    >(
        self,
        mut f: F,
    ) -> M {
        let mut map = M::new();
        self.into_iter_owned()
            .map(|item| {
                let key = f(&item);
                if !map.contains_key(&key) {
                    let mut seq = S::new();
                    seq.push_back(item);
                    map.insert(key, seq);
                } else {
                    map.get_mut(&key).unwrap().push_back(item);
                }
                ()
            })
            .for_each(drop);
        map
    }
}

#[test]
fn test_group_by() {
    let v = vec![1u32, 2, 3, 4, 5, 6];
    // requires strict HRTB checking here...
    let f: impl for<'a> FnMut(&u32) -> bool = |v| v % 2 == 0;
    let output: HashMap<_, Vec<_>> = v.group_by(f);
    assert_eq!(*output.get(&true).unwrap(), vec![2, 4, 6]);
    assert_eq!(*output.get(&false).unwrap(), vec![1, 3, 5]);
}
