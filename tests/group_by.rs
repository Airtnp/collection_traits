use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std_collection_traits::{elem::Owned, Collection, Map, SequentialCollection};

pub trait GroupMapBy: Collection + Owned {
    fn group_map_by<
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

impl<C: Collection + Owned> GroupMapBy for C {
    fn group_map_by<
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
    // requires strict HRTB checking here for closure type...
    /*
    = note: closure with signature `fn(&'2 u32) -> bool` must implement `FnOnce<(&'1 u32,)>`, for any lifetime `'1`...
    = note: ...but it actually implements `FnOnce<(&'2 u32,)>`, for some specific lifetime `'2`
      */
    // let f = |v| v % 2 == 0;

    let v = vec![1u32, 2, 3, 4, 5, 6];
    let output: BTreeMap<_, Vec<_>> = v.group_map_by(|v| v % 2 == 0);
    assert_eq!(*output.get(&true).unwrap(), vec![2, 4, 6]);
    assert_eq!(*output.get(&false).unwrap(), vec![1, 3, 5]);

    let v = VecDeque::from(vec![1u32, 2, 3, 4, 5, 6]);
    let output: HashMap<_, LinkedList<_>> = v.group_map_by(|v| v % 2 == 0);
    assert_eq!(*output.get(&true).unwrap().iter().copied().collect::<Vec<u32>>(), vec![
        2u32, 4, 6
    ]);
    assert_eq!(
        *output.get(&false).unwrap().iter().copied().collect::<Vec<u32>>(),
        vec![1u32, 3, 5]
    );
}
