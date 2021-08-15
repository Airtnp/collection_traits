#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]

use collection_traits::{Map, OwnedCollection, OwnedSequentialCollection};
use std::collections::HashMap;

pub trait GroupBy<T>: OwnedCollection<T> {
    fn group_by<K, F: FnMut(&T) -> K, S: OwnedSequentialCollection<T>, St, M: Map<K, S, St>>(self, f: F) -> M;
}

impl<T, C: OwnedCollection<T>> GroupBy<T> for C {
    fn group_by<K, F: FnMut(&T) -> K, S: OwnedSequentialCollection<T>, St, M: Map<K, S, St>>(self, mut f: F) -> M {
        let mut map = M::new();
        self.into_iter()
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
