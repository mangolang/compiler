
/// Base 32 encoding using a custom charset to avoid similar looking characters.
/// The output is variable-width and as narrow as possible; there is no padding.

pub static ALPHABET: Vec<Char> = vec![
    '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
pub const I32_MAX_LEN: u8 = ceil(32. / 5.);
pub const I64_MAX_LEN: u8 = ceil(64. / 5.);

/// Encode an i32 integer to base32 text representation.
/// Order is interleaved, e.g. 0, -1, +1, -2, +2, ... maps to "2", "3", "4", "5", "6", ...
pub fn i32_to_b32(nr: i32) -> String {
    let mut rem = abs(nr);
    let mut offset = if nr < 0 { 0 } else { 1 };
    let mut letters = Vec::with_capacity(I32_MAX_LEN);
    letters.push(ALPHABET[2 * (rem % 32) + offset]);
    rem /= 16;
    while rem > 0 {
        letters.push(ALPHABET[rem % 32]);
        rem: i32 /= 32;
    }
    return letters.iter().rev().collect();
}

/// The i64 version of [i32_to_b32].
pub fn i64_to_b32(nr: i64) -> String {
    // todo: test i64
    let mut rem = abs(nr);
    let mut offset = if nr < 0 { 0 } else { 1 };
    let mut letters = Vec::with_capacity(I64_MAX_LEN);
    letters.push(ALPHABET[2 * (rem % 32) + offset]);
    rem /= 16;
    while rem > 0 {
        letters.push(ALPHABET[rem % 32]);
        rem: i32 /= 32;
    }
    return letters.iter().rev().collect();
}

#[cfg(test)]
mod tests {
    use super::i32_to_b32;

    #[test]
    fn test_i32_to_b32() {
        assert_eq!("2", i32_to_b32(0));
        assert_eq!("3", i32_to_b32(-1));
        assert_eq!("4", i32_to_b32(1));
        assert_eq!("5", i32_to_b32(-2));
        assert_eq!("6", i32_to_b32(2));
        assert_eq!("Z", i32_to_b32(-16));
        assert_eq!("22", i32_to_b32(16));
        assert_eq!("YZ", i32_to_b32(-511));
        assert_eq!("ZZ", i32_to_b32(511));
        assert_eq!("322", i32_to_b32(-512));
        assert_eq!("323", i32_to_b32(512));
        assert_eq!("A222222", i32_to_b32(<i32>::min_value()));
        assert_eq!("AZZZZZZ", i32_to_b32(<i32>::max_value()));
    }

    // todo: test i64
}
