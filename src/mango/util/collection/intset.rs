
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
}

