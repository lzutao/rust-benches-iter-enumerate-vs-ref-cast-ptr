pub fn enumerate(needle: u16, haystack: &[u16]) -> Option<usize> {
    for (i, c) in haystack.iter().enumerate() {
        if *c == needle {
            return Some(i);
        }
    }
    None
}

pub fn ref_cast_ptr(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    for c in haystack {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    None
}
