

#[doc(hidden)]
pub use crate::ops::Bound;

pub use alloc_crate::collections::{binary_heap, btree_map, btree_set};

pub use alloc_crate::collections::{linked_list, vec_deque};

pub use alloc_crate::collections::{BTreeMap, BTreeSet, BinaryHeap};

pub use alloc_crate::collections::{LinkedList, VecDeque};


pub use self::hash_map::HashMap;

pub use self::hash_set::HashSet;


pub use alloc_crate::collections::TryReserveError;
pub use alloc_crate::collections::TryReserveErrorKind;

//pub use alloc_crate::collections::TryReserveErrorKind;

mod hash;


pub mod hash_map {
    //! A hash map implemented with quadratic probing and SIMD lookup.
    pub use super::hash::map::*;
}

pub mod hash_set {
    //! A hash set implemented as a `HashMap` where the value is `()`.
    pub use super::hash::set::*;
}
