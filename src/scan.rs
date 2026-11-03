//! Byte searches shared by the message rewrites of [`crate::message`] and
//! [`crate::rfc5321::data`].

/// The index of the first byte `hit` matches.
///
/// Sixteen bytes at a time, each block's hits gathered into one mask, which
/// compiles to vector compares on the x86_64 and aarch64 baselines. `hit`
/// must join its tests with `|`: a `matches!` compiles to a branch a byte.
#[inline]
pub(crate) fn position(bytes: &[u8], hit: impl Fn(u8) -> bool) -> Option<usize> {
    let (blocks, tail) = bytes.as_chunks::<16>();

    for (i, block) in blocks.iter().enumerate() {
        let mut mask = 0u16;

        for (j, &b) in block.iter().enumerate() {
            mask |= u16::from(hit(b)) << j;
        }

        if mask != 0 {
            return Some(i * 16 + mask.trailing_zeros() as usize);
        }
    }

    tail.iter()
        .position(|&b| hit(b))
        .map(|j| blocks.len() * 16 + j)
}
