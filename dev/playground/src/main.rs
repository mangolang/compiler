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
	fn my_hash(&self, hasher: &mut Hasher);
}

trait AnyHasher {
	fn as_any(&self) -> &Any;
}

impl<H: 'static + Hasher> AnyHasher for H {
	fn as_any(&self) -> &Any {
		self as &Any
	}
}

// TODO: but now I want this not for everything
impl<T: 'static + Hash, H: 'static + Hasher> MyTrait for T {
	fn as_any(&self) -> &Any {
		self as &Any
	}

	fn my_hash<H>(&self, hasher: &mut AnyHasher) {
		let h = hasher.as_any().downcast_ref::<H>().unwrap();
		self.as_any().downcast_ref::<T>().unwrap().hash(h)
	}
}

//impl MyTrait for A {}
//impl MyTrait for B {}

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


//trait PreS: Debug {}
//
//trait HasherAsAny {
//	fn as_any(&self) -> &Any;
//}
//
//trait PostS {
//	fn as_any(&self) -> &Any;
//
//	fn _hash<H: Hasher>(&self, hasher: H);
//}
//
//impl<T: 'static + Hasher> HasherAsAny for T {
//	fn as_any(&self) -> &Any {
//		self as &Any
//	}
//}
//
//impl<T: 'static + PreS> PostS for T {
//	fn as_any(&self) -> &Any {
//		self as &Any
//	}
//
//	fn _hash<H: Hasher>(&self, hasher: H) {
//		self.as_any().downcast_ref::<T>().hash(hasher)
//	}
//}
//
//impl PreS for A {}
//
//impl PreS for B {}
//
//impl Hash for PostS {
//	fn hash(&self, hasher: &mut HasherAsAny) {
//		self._hash(hasher.as_any().downcast_ref::<T>())
//	}
//}
//
//fn main() {
//	let x: &PostS = &A(1);
//	let m = HashMap::new();
//	m.insert(x, 0);
//}
