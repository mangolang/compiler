
/// A 'HashSet' that can use specific values for the hash (which is rarely what one would want).
/// This particular implementation uses at most 1 item per bin and grows as needed.
// This is mostly out of interest, it's doubtful that this was a bottleneck.

// Value 16 (4 bits) leads to nicely rounded values: exactly 8 levels for i32, 16 for i64.
const BINS: usize = 16;

#[derive(Debug)]
enum Bin<T, V> {
    // todo: box here?
    SubSet(IntSet<T, V>),
    Exists(T, V),
    Empty,
}

// todo: I need modulo to yield an integer; if floats are Mod+Div it won't work
#[derive(Debug)]
pub struct IntSet<T: Mod + Div, V: Eq> {
//    has_0: bool,
    bins: Vec<Bin<T, U>>,
}

#[derive(Debug)]
impl<T: Mod + Div> IntSet<T, V> {
    /// Create a new empty integer set.
    pub fn new() -> Self {
        let mut bins = Vec::with_capacity(BINS);
        for k in 0 .. BINS {
            bins.append(Bin::Empty);
        }
        IntSet {
//            has_0: false,
            bins: bins,
        }
    }

    /// Insert he value into the set. Returns whether an insert was done.
    pub fn insert_at(&mut self, pos: T, val: V) -> bool {
//        if pos == 0 {
//            if self.has_0 {
//                return false;
//            }
//            self.has_0 = true;
//            return true;
//        }
        let rem = self.bins[pos % BINS];
        match rem {
            Bin::SubSet(ref mut set) => set.insert(pos / BINS),
            Bin::Exists(existing_pos, existing_val) => {
                if pos == existing_pos {
                    return false;
                }
                let mut subset = Bin::SubSet(IntSet::new());
                subset.insert(existing_pos, existing_val);
                subset.insert(pos, val);
                self.bins[pos % BINS] = subset;
                return true;
            },
            Bin::Empty => {
                self.bins[pos % BINS] = Bin::Exists(pos, val);
                return true;
            },
        }
    }

    /// Check whether the set contains the integer key (note that it does not take the value).
    ///
    pub fn contains(&self, pos: T, val: V) -> bool {
//        if val == 0 {
//            return self.has_0;
//        }
        match self.bins[val % BINS] {
            Bin::SubSet(set) => set.contains(val / BINS),
            Bin::Exists(existing_val) => existing_val == val,
            Bin::Empty => false,
        }
    }

    /// Gives the number of elements in the set.
    /// This actively counts the elements, so it is not free.
    pub fn count(&self) -> usize {
        let mut sum = 0;
        for bin in self.bins {
            sum += match bin {
                Bin::SubSet(set) => set.count(),
                Bin::Exists(_) => 1,
                Bin::Empty => 0,
            }
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::IntSet;

    #[test]
    fn test_int_set_basic() {
        let mut set = IntSet::new();
        for k in 0 .. 128 {
            set.insert(k);
        }
        for k in 0 .. 128 {
            assert!(set.contains(k));
        }
        for k in -1024 .. 0 {
            assert!(!set.contains(k));
        }
        for k in 128 .. 1024 {
            assert!(!set.contains(k));
        }
        assert_eq!(set.count(), 128);
    }

    #[test]
    fn test_int_set_repeats() {
        let mut set = IntSet::new();
        for k in 0 .. 128 {
            set.insert(k);
        }
        for k in 0 .. 128 {
            assert!(!set.insert(k));
        }
        assert_eq!(set.count(), 128);
    }

    #[test]
    fn test_int_set_collisions() {
        let mut set = IntSet::new();
        // To have a value in the first bin for 6 levels, the last 24 bits should be 0.
        // To let all the lower levels be created, add at least 6 values.
        for k in 1 .. 9 {
            assert!(set.insert(k * 2**24));
        }
        for k in 1 .. 9 {
            assert!(set.contains(k * 2**24));
        }
        for k in 0 .. 128 {
            assert!(!set.contains(k));
        }
        assert!(!set.contains(2**23));
        assert_eq!(set.count(), 8);
    }
}
