
/// A 'HashSet' for integers where the integers are their own hash.
/// This particular implementation uses at most 1 item per bin.
/// It is not currently possible to get the values in the set (though it can theoretically be done).
// This is mostly out of interest, it's doubtful that this was a bottleneck.

// Value 16 (4 bits) leads to nicely rounded values: exactly 8 levels for i32, 16 for i64.
const BINS: usize = 16;

#[derive(Debug)]
enum Bin<T> {
    SubSet(IntSet<T>),
    Exists(T),
    Empty,
}

#[derive(Debug)]
pub struct IntSet<T: Mod + Div> {
    has_0: bool,
    bins: Vec<Bin<T>>,
}

#[derive(Debug)]
impl<T: Mod + Div> IntSet<T> {
    /// Create a new empty integer set.
    pub fn new() -> Self {
        let mut bins = Vec::with_capacity(BINS);
        for k in 0 .. BINS {
            bins.append(Bin::Empty);
        }
        IntSet {
            has_0: false,
            bins: bins,
        }
    }

    /// Insert he value into the set. Returns whether an insert was done.
    pub fn insert(&mut self, val: T) -> bool {
        if val == 0 {
            if self.has_0 {
                return false;
            }
            self.has_0 = true;
            return true;
        }
        let rem = self.bins[val % BINS];
        match rem {
            Bin::SubSet(ref mut set) => set.insert(val / BINS),
            Bin::Exists(existing) => {
                if val == existing {
                    return false;
                }
                let mut subset = Bin::SubSet(IntSet::new());
                subset.insert(existing);
                subset.insert(val);
                self.bins[val % BINS] = subset;
                return true;
            },
            Bin::Empty => {
                self.bins[val % BINS] = Bin::Exists(val);
                return true;
            },
        }
    }

    /// Check whether the set contains the integer value.
    pub fn contains(&self, val: T) -> bool {
        if val == 0 {
            return self.has_0;
        }
        match self.bins[val % BINS] {
            Bin::SubSet(set) => set.contains(val / BINS),
            Bin::Exists(_) => true,
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
