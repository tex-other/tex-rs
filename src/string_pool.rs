#[cfg(not(target_os = "macos"))]
#[distributed_slice]
pub(crate) static STRPLI: [&'static str] = [..];

#[cfg(target_os = "macos")]
pub(crate) struct strpool_literal(pub(crate) &'static str);

#[cfg(target_os = "macos")]
inventory::collect!(strpool_literal);

static STRPOOL_ITEMS_FROM_256: Lazy<Vec<&'static str>> = Lazy::new(prepare_compiletime_string_pool);

#[cfg(not(target_os = "macos"))]
fn prepare_compiletime_string_pool() -> Vec<&'static str> {
    let mut existing = BTreeSet::new();
    let mut result = vec![];
    for str in STRPLI.iter().cloned() {
        if str.len() == 1 {
            continue;
        }
        if existing.contains(&str) {
            continue;
        }
        result.push(str);
        existing.insert(str);
    }
    result.sort();
    result
}

#[cfg(target_os = "macos")]
fn prepare_compiletime_string_pool() -> Vec<&'static str> {
    let mut existing = BTreeSet::new();
    let mut result = vec![];
    for strpool_literal(str) in inventory::iter::<strpool_literal> {
        let str = *str;
        if str.len() == 1 {
            continue;
        }
        if existing.contains(&str) {
            continue;
        }
        result.push(str);
        existing.insert(str);
    }
    result.sort();
    result
}

#[cfg(not(target_os = "macos"))]
pub(crate) macro strpool_str($s:expr) {{
    #[::linkme::distributed_slice(crate::string_pool::STRPLI)]
    static __: &'static str = $s;

    let v = crate::string_pool::string_pool_index($s);
    debug_assert!(v <= crate::pascal::char::MAX.0 as _);
    crate::section_0038::str_number(crate::pascal::u32_from_m_to_n::new(v as u32))
}}

#[cfg(target_os = "macos")]
pub(crate) macro strpool_str($s:expr) {{
    inventory::submit! {
        crate::string_pool::strpool_literal($s)
    }

    let v = crate::string_pool::string_pool_index($s);
    debug_assert!(v <= crate::pascal::char::MAX.0 as _);
    crate::section_0038::str_number(crate::pascal::u32_from_m_to_n::new(v as u32))
}}

pub(crate) fn string_pool_index(val: &'static str) -> usize {
    if val.len() == 1 {
        return val.as_bytes()[0] as usize;
    }
    for (idx, str) in (256usize..).zip(STRPOOL_ITEMS_FROM_256.iter().cloned()) {
        if val == str {
            return idx;
        }
    }
    unreachable!(
        "Literal `{}` not found in string pool, string pool size is {}.",
        val,
        256 + STRPOOL_ITEMS_FROM_256.len()
    );
}

static POOL_FILE: Lazy<Vec<u8>> = Lazy::new(generate_initial_memory_pool_file);

static CHECKSUM: Lazy<usize> = Lazy::new(generate_checksum);

pub(crate) fn string_pool_checksum() -> usize {
    *CHECKSUM
}

pub(crate) fn generate_initial_memory_pool_file() -> Vec<u8> {
    let mut cursor = io::Cursor::new(vec![]);
    for str in STRPOOL_ITEMS_FROM_256.iter().cloned() {
        assert!(str.len() < 256);
        write!(cursor, "{:02}{}\n", str.len(), str).unwrap();
    }
    write!(cursor, "*{:09}", *CHECKSUM).unwrap();
    use std::io::Write;

    cursor.into_inner()
}

pub(crate) fn generate_checksum() -> usize {
    123456789
}

pub(crate) fn pool_file() -> io::Cursor<&'static [u8]> {
    io::Cursor::new(&*POOL_FILE)
}

use linkme::distributed_slice;
use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::io;
