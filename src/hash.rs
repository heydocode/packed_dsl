const fn _hash_str_with_other_hash(contents: &str, other_hash: u64) -> u64 {
    let left = contents.as_bytes();
    let right = other_hash.to_ne_bytes();

    const CONTENTS_MAX_LEN: usize = 10_000;
    const TOTAL_MAX_LEN: usize = CONTENTS_MAX_LEN + size_of::<u64>();
    assert!(
        contents.len() <= CONTENTS_MAX_LEN,
        "hash_with_other_hash: contents len can't be bigger than 10_000"
    );

    // Note that this huge array disappears at runtime.
    // This is required to combine both arrays.
    let bytes: [u8; TOTAL_MAX_LEN] = {
        let mut bytes: [u8; TOTAL_MAX_LEN] = [0; TOTAL_MAX_LEN];
        let (one, two) = bytes.split_at_mut(left.len());
        one.copy_from_slice(&left);
        two.copy_from_slice(&right);
        bytes
    };

    hash(&bytes, left.len() + right.len())
}

/// Source: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
pub const fn hash_str(contents: &str) -> u64 {
    // Valid hashes taken from:
    // https://miniwebtool.com/fnv1a-hash-generator/
    const _: () = {
        assert!(hash_str("test") == 0xf9e6e6ef197c2b25);
        assert!(hash_str("example.com") == 0x576846634e2714c6);
        assert!(hash_str("user@email.com") == 0xd91bc103244ff6e9);
        assert!(hash_str(stringify!(u64)) == 0x4d35d3193e8d66f2);
    };

    let bytes = contents.as_bytes();
    hash(bytes, bytes.len())
}

pub const fn rehash_with_n_hashes(base_hash: u64, other_hashes: &[u64]) -> u64 {
    const CONTENTS_MAX_LEN: usize = 1_000;
    assert!(
        other_hashes.len() <= CONTENTS_MAX_LEN,
        "hash_with_other_hash: other_hashes len can't be bigger than 1_000"
    );

    const SECOND_CONTENTS_MAX_LEN: usize = CONTENTS_MAX_LEN * (size_of::<u64>() / size_of::<u8>());
    const TOTAL_MAX_LEN: usize = SECOND_CONTENTS_MAX_LEN + size_of::<u64>();
    let mut trans_buf = [0u8; SECOND_CONTENTS_MAX_LEN];
    let mut i = 0;
    while i < other_hashes.len() {
        let temp = other_hashes[i].to_le_bytes();
        let mut i2 = 0;
        while i2 < temp.len() {
            trans_buf[(i * 4) + i2] = temp[i2];
            i2 += 1;
        }
        i += 1;
    }

    let base_hash_ar = base_hash.to_le_bytes();

    // Note that this huge array disappears at runtime.
    // This is required to combine both arrays.
    let bytes: [u8; TOTAL_MAX_LEN] = {
        let mut bytes: [u8; TOTAL_MAX_LEN] = [0; TOTAL_MAX_LEN];
        let (one, two) = bytes.split_at_mut(size_of::<u64>());
        one.copy_from_slice(&base_hash_ar);
        two.copy_from_slice(&trans_buf);
        bytes
    };

    hash(&bytes, base_hash_ar.len() + (other_hashes.len() * (size_of::<u64>() / size_of::<u8>())))
}

const fn hash(bytes: &[u8], len: usize) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    let mut i = 0;

    assert!(
        len <= bytes.len(),
        "Provided len should never be greater than the provided bytes' len"
    );

    while i < len {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }

    assert!(
        hash != 0,
        "The hash is equal to zero, meaning that it's unchangeable, please modify your input"
    );

    hash
}
