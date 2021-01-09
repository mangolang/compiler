pub use self::eqfloat::f64eq;

mod eqfloat;

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::mem;

    #[test]
    fn test_add() {
        assert!(
            mem::size_of::<u32>() <= mem::size_of::<usize>(),
            "Some code stores indices as u32, and may assume that every usize fits in u32"
        );
    }
}
