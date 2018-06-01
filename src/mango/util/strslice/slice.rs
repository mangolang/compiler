/// Take a character-based slice of a string (as opposed to the default byte-slice).
/// Allows negative indices to slice from the end (but start must be before end).
/// This may not be very fast.
pub fn charslice<S: Into<String>>(text: S, start: isize, end: isize) -> String {
    let stext = text.into();
    let from: usize;
    let length: usize;
    if start < 0 {
        // LATER: may remove this check and just default to 0 in the future.
        assert!(
            -start as usize <= stext.len(),
            "charslice: if 'start' is negative, the magnitude may not exceed the length"
        );
        // TODO: off by one?
        from = (stext.len() as isize + start) as usize;
    } else {
        from = start as usize;
    }
    if end < 0 {
        // LATER: may remove this check and just default to 0 in the future.
        assert!(
            -end as usize <= stext.len(),
            "charslice: if 'end' is negative, the magnitude may not exceed the length"
        );
        // TODO: off by one?
        let new_end = (stext.len() as isize + end) as usize;
        assert!(
            new_end >= from,
            "charslice: 'start' may not be before 'end' (end was positive)"
        );
        length = new_end - from;
    } else {
        assert!(
            end >= from as isize,
            "charslice: 'start' may not be before 'end' (end was positive)"
        );
        length = end as usize - from;
    }
    stext.chars().skip(from).take(length).collect()
}

pub fn charslicefrom<S: Into<String>>(text: S, start: isize) -> String {
    let stext = text.into();
    let len = stext.len() as isize;
    charslice(stext, start, len)
}

pub fn charsliceto<S: Into<String>>(text: S, end: isize) -> String {
    charslice(text, 0, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice() {
        assert_eq!(42isize as usize, 42usize);
        assert_eq!("你好", charslice("你好!", 0, 2));
        assert_eq!("!", charslicefrom("你好!", 2));
        assert_eq!("你好", charsliceto("你好!", 2));
        // TODO: test negative values
    }
}
