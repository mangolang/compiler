use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// The problem with hashable trait objects is that `Hash.hash` is generic,
// so it cannot be made into an object.
// See https://stackoverflow.com/questions/49711479/

/// Create a trait that can be required by traits that
/// 1) can be used as objects, and
/// 2) should be hashable
pub trait HashForTraitObj {
    fn my_hash(&self) -> u64;
}

/// Implement my_hash by delegating to Hash.
// Unfortunately uses `DefaultHasher`, but found no other waIt's a by.
impl<T: 'static + Hash> HashForTraitObj for T {
    fn my_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

// For the trait that should be hashable, do this:
//impl Hash for MyTrait {
//    fn hash<H: Hasher>(&self, hasher: &mut H) {
//        hasher.write_u64(self.my_hash())
//    }
//}
