# Rust Collection traits

Abstract common operations from `std` collections. For a working sample, check `tests/group_by.rs`

This crate is experimental. Better a RFC is needed to formally abstract operations (like Cpp Concept/Named requirements).

Collections in `std`:
* slice: `&[T]`, `&mut [T]`
* array: `[T; N]`
* sequence: `Vec<T>`, `VecDeque<T>`, `LinkedList<T>`
* set: `BTreeSet<T>`, `HashSet<T>`
* map: `BTreeMap<T>`, `HashMap<T>`
* misc: `BinaryHeap<T>`

General Categories:
* `Collection<T>`: All collections, excluding associated ones.
  - `OwnedCollection<T>`: Owned collections, like `Vec<T>`
  - `AllocatorAwareCollection<T>`: Allocator-aware collections, currently there is only `Vec<T, A>`.
  - `CapacityAwareCollection<T>`: Capacity-aware collections, those you can reserve, create with a given amount of capacity. E.g. `HashSet::with_capacity`
  - `{Owned}ContiguousCollection<T>`: Contiguous collections, where behaves like a contiguous region of memory. E.g. `Vec::as_slice`
  - `{Owned}SequentialCollection<T>`: Sequential collections, where you can traverse it in one direction, and modify at the end. E.g. `VecDeque::push_back`
  - `{Owned}DoubleEndedCollection<T>`: Double-ended collections, where you do sequential-like operations on both ends. E.g. `LinkedList<T>::pop_front`
  - `{Owned}RandomAccessCollection<T>`: Random access collections, where you can access by indices. E.g. `VecDeque<T>::get`
* `AssociatedCollection<K, V>`
  - `OwnedAssociatedCollection<K, V>`: Owned associated collections.
  - `AssociatedCapacityAwareCollection<K, V>`: Capacity-aware associated collections.
  - `Set<K, S>`: Set-like collections, `HashSet<K, S>` and `BTreeSet<K>`
  - `OrderedSe<K, S>`: Ordered set-like collections, `BTreeSet<K>`
  - `Map<K, V, S>`: Map-like collections, `HashMap<K, V, S>` and `BTreeMap<K, V>`
  - `OrderedMap`: Ordered map-like collections, `BTreeMap<K, V>`
* Other miscellaneous operations
  - `Retainable<T>` or `AssocaitedRetainable<K, V>`: Ability to retain specific elements.
  - `Contains<T>`: Ability to test if an element is in sequence.
  - `DrainFull/DrainRange/{Associated}DrainFilter`: Ability to drain a specific amount of elements without drop the original collection.
  - `Range/RangeMut`: Ability to generate a view of original sequence.

## TODOs

* [x] Rewrite it based on `Mut/Own` marker instead of individual types
  * This requires some non-robust `unreached_unchecked` on default implementation.
* [x] Find a way to constrain generic associated iterator types
  * `ExactSizedIterator`
  * `TrustedRandomAccess`
  * `DoubleEndedIterator`
* [ ] Add more collection-like types, like `&str`, `String`
* [x] Rewrite all to assocaited types