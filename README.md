# Rust Collection Traits

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
* `Collection`: All collections, excluding associated ones.
  - `AllocatorAwareCollection`: Allocator-aware collections, currently there is only `Vec<T, A>`.
  - `CapacityAwareCollection`: Capacity-aware collections, those you can reserve, create with a given amount of capacity. E.g. `HashSet::with_capacity`
  - `ContiguousCollection`: Contiguous collections, where behaves like a contiguous region of memory. E.g. `Vec::as_slice`
  - `SequentialCollection`: Sequential collections, where you can traverse it in one direction, and modify at the end. E.g. `VecDeque::push_back`
  - `DoubleEndedCollection`: Double-ended collections, where you do sequential-like operations on both ends. E.g. `LinkedList<T>::pop_front`
  - `RandomAccessCollection`: Random access collections, where you can access by indices. E.g. `VecDeque<T>::get`
* `AssociatedCollection`
  - `AssociatedCapacityAwareCollection`: Capacity-aware associated collections.
  - `Set<S>`: Set-like collections, `HashSet<K, S>` and `BTreeSet<K>`
  - `OrderedSe<S>`: Ordered set-like collections, `BTreeSet<K>`
  - `Map<S>`: Map-like collections, `HashMap<K, V, S>` and `BTreeMap<K, V>`
  - `OrderedMap<S>`: Ordered map-like collections, `BTreeMap<K, V>`
* Other miscellaneous operations
  - `Retainable` or `AssocaitedRetainable`: Ability to retain specific elements.
  - `Contains`: Ability to test if an element is in sequence.
  - `DrainFull/DrainRange/{Associated}DrainFilter`: Ability to drain a specific amount of elements without drop the original collection.
  - `Range/RangeMut`: Ability to generate a view of original sequence.

## TODOs

* [x] Rewrite it based on `Mut/Own` marker instead of individual types
  * This requires some non-robust `unreached_unchecked` on default implementation.
* [x] Find a way to constrain generic associated iterator types
  * `ExactSizedIterator`
  * `TrustedRandomAccess`
  * `DoubleEndedIterator`
  * [ ] Still can't propagate requirements to uppermost traits
* [ ] Add more collection-like types, like `&str`, `String`
* [x] Rewrite all to associated types