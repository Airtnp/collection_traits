use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};
use std_collection_traits::{elem::Owned, Collection, Map, SequentialCollection};

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
    // requires strict HRTB checking here for closure type...

    let v = vec![1u32, 2, 3, 4, 5, 6];
    let output: BTreeMap<_, Vec<_>> = v.group_by(|v| v % 2 == 0);
    assert_eq!(*output.get(&true).unwrap(), vec![2, 4, 6]);
    assert_eq!(*output.get(&false).unwrap(), vec![1, 3, 5]);

    let v = VecDeque::from(vec![1u32, 2, 3, 4, 5, 6]);
    let output: HashMap<_, LinkedList<_>> = v.group_by(|v| v % 2 == 0);
    assert_eq!(*output.get(&true).unwrap().iter().copied().collect::<Vec<u32>>(), vec![
        2u32, 4, 6
    ]);
    assert_eq!(
        *output.get(&false).unwrap().iter().copied().collect::<Vec<u32>>(),
        vec![1u32, 3, 5]
    );
}
