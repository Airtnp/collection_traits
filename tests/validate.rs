#![feature(allocator_api)]
#![feature(associated_type_bounds)]

use std::{
    alloc::Global,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    convert::TryFrom,
};
use std_collection_traits::{elem::Owned, exact_size::ExactSized, *};

#[test]
fn test_collection() {
    fn accept_collection(c: impl Collection) {
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

    fn accept_associated_collection(mut c: impl AssociatedCollection) {
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
    fn accept_owned_collection<U: Collection<ElemType: Default> + Owned + ExactSized<SizeType = usize>>() {
        let mut c = U::new();

        c.extend((0..3).map(|_| Default::default()));
        c.clear();
        c.extend((0..3).map(|_| Default::default()));
        let _ = c.into_iter_owned().collect::<Vec<_>>();
    }

    accept_owned_collection::<Vec<usize>>();
    accept_owned_collection::<VecDeque<usize>>();
    accept_owned_collection::<LinkedList<usize>>();
    accept_owned_collection::<BTreeSet<usize>>();
    accept_owned_collection::<HashSet<usize>>();
    accept_owned_collection::<BinaryHeap<usize>>();

    fn accept_assoc_owned_collection<
        U: AssociatedCollection<KeyType: Default, ValueType: Default> + Owned + ExactSized<SizeType = usize>,
    >() {
        let mut c = U::new();
        let gen = |_| (Default::default(), Default::default());
        c.extend((0..3).map(gen));
        c.clear();
        c.extend((0..3).map(gen));
        let _ = c.into_iter().collect::<Vec<_>>();
    }

    accept_assoc_owned_collection::<BTreeMap<usize, isize>>();
    accept_assoc_owned_collection::<HashMap<usize, isize>>();
}

#[test]
fn test_sequential() {
    fn contiguous<C: ContiguousCollection>() {}
    contiguous::<&mut [usize]>();
    contiguous::<[usize; 42]>();
    contiguous::<Vec<usize>>();

    fn seq<C: SequentialCollection>() {}
    seq::<&mut [usize]>();
    seq::<[usize; 42]>();
    seq::<Vec<usize>>();
    seq::<VecDeque<usize>>();
    seq::<LinkedList<usize>>();

    fn own_seq<C: SequentialCollection + Owned>() {}
    own_seq::<Vec<usize>>();
    own_seq::<VecDeque<usize>>();
    own_seq::<LinkedList<usize>>();

    fn double<C: DoubleEndedCollection>() {}
    double::<VecDeque<usize>>();
    double::<LinkedList<usize>>();

    fn own_double<C: DoubleEndedCollection + Owned>() {}
    own_double::<VecDeque<usize>>();
    own_double::<LinkedList<usize>>();

    fn random<C: RandomAccessCollection>() {}
    random::<&mut [usize]>();
    random::<[usize; 42]>();
    random::<Vec<usize>>();
    random::<VecDeque<usize>>();

    fn own_random<C: RandomAccessCollection + Owned>() {}
    own_random::<Vec<usize>>();
    own_random::<VecDeque<usize>>();
}

#[test]
fn test_set_map() {
    fn set<S, C: Set<S>>() {}
    set::<_, HashSet<usize>>();
    set::<_, BTreeSet<usize>>();

    fn order_set<S, C: OrderedSet<S>>() {}
    order_set::<_, BTreeSet<usize>>();

    fn map<S, C: Map<S>>() {}
    map::<_, HashMap<usize, isize>>();
    map::<_, BTreeMap<usize, isize>>();

    fn order_map<S, C: OrderedMap<S>>() {}
    order_map::<_, BTreeMap<usize, isize>>();
}

#[test]
fn test_allocator() {
    fn allocator<C: AllocatorAwareCollection>() {}
    allocator::<Vec<usize, Global>>();
}

#[test]
fn test_capacity() {
    fn cap<C: CapacityAwareCollection>() {}
    cap::<Vec<usize>>();
    cap::<VecDeque<usize>>();
    // no `reserve` here
    // cap::<usize, BinaryHeap<_>>();
    cap::<HashSet<usize>>();

    fn assoc_cap<C: AssociatedCapacityAwareCollection>() {}
    assoc_cap::<HashMap<usize, isize>>();
}
