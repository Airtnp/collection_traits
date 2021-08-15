#![feature(allocator_api)]

use collection_traits::{exact_size::ExactSized, *};
use std::{
    alloc::Global,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    convert::TryFrom,
};

#[test]
fn test_collection() {
    fn accept_collection<T>(c: impl Collection<T>) {
        let _ = c.iter().collect::<Vec<_>>();
        let _ = c.len();
        let _ = c.is_empty();
    }

    accept_collection(vec![1, 2, 3].as_mut_slice());
    accept_collection(<[usize; 3]>::try_from(vec![1, 2, 3]).unwrap());
    accept_collection(vec![1, 2, 3]);
    accept_collection(VecDeque::<usize>::new());
    accept_collection(LinkedList::<usize>::new());
    accept_collection(BinaryHeap::<usize>::new());
    accept_collection(BTreeSet::<usize>::new());
    accept_collection(HashSet::<usize>::new());

    fn accept_associated_collection<K, V>(mut c: impl AssociatedCollection<K, V>) {
        let _ = c.iter().collect::<Vec<_>>();
        let _ = c.iter_mut().collect::<Vec<_>>();
        let _ = c.len();
        let _ = c.is_empty();
    }

    accept_associated_collection(BTreeMap::<usize, usize>::new());
    accept_associated_collection(HashMap::<usize, usize>::new());
}

#[test]
fn test_owned_collection() {
    fn accept_owned_collection<T: Default, U: OwnedCollection<T> + ExactSized<SizeType = usize>>() {
        let mut c = U::new();

        c.extend((0..3).map(|_| Default::default()));
        c.clear();
        c.extend((0..3).map(|_| Default::default()));
        let _ = c.into_iter().collect::<Vec<_>>();
    }

    accept_owned_collection::<usize, Vec<_>>();
    accept_owned_collection::<usize, VecDeque<_>>();
    accept_owned_collection::<usize, LinkedList<_>>();
    accept_owned_collection::<usize, BTreeSet<_>>();
    accept_owned_collection::<usize, HashSet<_>>();
    accept_owned_collection::<usize, BinaryHeap<_>>();

    fn accept_assoc_owned_collection<
        K: Default,
        V: Default,
        U: OwnedAssociatedCollection<K, V> + ExactSized<SizeType = usize>,
    >() {
        let mut c = U::new();
        c.extend((0..3).map(|_| (Default::default(), Default::default())));
        c.clear();
        c.extend((0..3).map(|_| (Default::default(), Default::default())));
        let _ = c.into_iter().collect::<Vec<_>>();
    }

    accept_assoc_owned_collection::<usize, usize, BTreeMap<_, _>>();
    accept_assoc_owned_collection::<usize, usize, HashMap<_, _>>();
}

#[test]
fn test_sequential() {
    fn contiguous<T, C: ContiguousCollection<T>>() {}
    contiguous::<usize, &mut [usize]>();
    contiguous::<usize, [usize; 42]>();
    contiguous::<usize, Vec<usize>>();

    fn seq<T, C: SequentialCollection<T>>() {}
    seq::<usize, &mut [usize]>();
    seq::<usize, [usize; 42]>();
    seq::<usize, Vec<usize>>();
    seq::<usize, VecDeque<usize>>();
    seq::<usize, LinkedList<usize>>();

    fn own_seq<T, C: OwnedSequentialCollection<T>>() {}
    own_seq::<usize, Vec<usize>>();
    own_seq::<usize, VecDeque<usize>>();
    own_seq::<usize, LinkedList<usize>>();

    fn double<T, C: DoubleEndedCollection<T>>() {}
    double::<usize, VecDeque<usize>>();
    double::<usize, LinkedList<usize>>();

    fn own_double<T, C: OwnedDoubleEndedCollection<T>>() {}
    own_double::<usize, VecDeque<usize>>();
    own_double::<usize, LinkedList<usize>>();

    fn random<T, C: RandomAccessCollection<T>>() {}
    random::<usize, &mut [usize]>();
    random::<usize, [usize; 42]>();
    random::<usize, Vec<usize>>();
    random::<usize, VecDeque<usize>>();

    fn own_random<T, C: OwnedRandomAccessCollection<T>>() {}
    own_random::<usize, Vec<usize>>();
    own_random::<usize, VecDeque<usize>>();
}

#[test]
fn test_set_map() {
    fn set<K, S, C: Set<K, S>>() {}
    set::<usize, _, HashSet<usize>>();
    set::<usize, _, BTreeSet<usize>>();

    fn order_set<K, S, C: OrderedSet<K, S>>() {}
    order_set::<usize, _, BTreeSet<usize>>();

    fn map<K, V, S, C: Map<K, V, S>>() {}
    map::<usize, isize, _, HashMap<_, _>>();
    map::<usize, isize, _, BTreeMap<_, _>>();

    fn order_map<K, V, S, C: Map<K, V, S>>() {}
    order_map::<usize, isize, _, BTreeMap<_, _>>();
}

#[test]
fn test_allocator() {
    fn allocator<T, C: AllocatorAwareCollection<T>>() {}
    allocator::<usize, Vec<usize, Global>>();
}

#[test]
fn test_capacity() {
    fn cap<T, C: CapacityAwareCollection<T>>() {}
    cap::<usize, Vec<_>>();
    cap::<usize, VecDeque<_>>();
    // no `reserve` here
    // cap::<usize, BinaryHeap<_>>();
    cap::<usize, HashSet<_>>();

    fn assoc_cap<K, V, C: AssociatedCapacityAwareCollection<K, V>>() {}
    assoc_cap::<usize, isize, HashMap<_, _>>();
}
