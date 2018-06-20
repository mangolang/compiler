use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::Hasher;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::fmt::Debug;
use std::any::Any;

#[derive(Hash, Debug)]
struct A(i32);

#[derive(Hash, Debug)]
struct B {
	val: String,
}

trait MyTrait {
	fn as_any(&self) -> &Any;
	fn my_hash<H: Hasher>(&self, hasher: &mut Hasher);
}

trait AnyHasher {
	fn as_any(&self) -> &Any;
}

impl<H: 'static + Hasher> AnyHasher for H {
	fn as_any(&self) -> &Any {
		self as &Any
	}
}

impl<T: 'static + Hash> MyTrait for T {
	fn as_any(&self) -> &Any {
		self as &Any
	}

	fn my_hash<H>(&self, hasher: &mut AnyHasher) {
		let h = hasher.as_any().downcast_ref::<H>().unwrap();
		self.as_any().downcast_ref::<T>().unwrap().hash(h)
	}
}

impl Hash for MyTrait {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.my_hash(hasher)
	}
}

fn main() {
	let s = RandomState::new();
	let mut hasher = s.build_hasher();

	let x: &MyTrait = &A(1);
	x.hash(&mut hasher);
}
