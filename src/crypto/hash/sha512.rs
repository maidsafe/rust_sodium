//! `SHA-512`.
//!
//! There has been considerable degradation of public confidence in the
//! security conjectures for many hash functions, including `SHA-512`.
//! However, for the moment, there do not appear to be alternatives that
//! inspire satisfactory levels of confidence. One can hope that NIST's
//! SHA-3 competition will improve the situation.

use ffi::{crypto_hash_sha512, crypto_hash_sha512_BYTES};

hash_module!(crypto_hash_sha512, crypto_hash_sha512_BYTES as usize, 128);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_vector_1() {
        // corresponding to tests/hash.c, tests/hash2.cpp,
        // tests/hash3.c and tests/hash4.cpp from NaCl
        assert!(::init());
        let x = [0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0xa];
        let h_expected = [0x24, 0xf9, 0x50, 0xaa, 0xc7, 0xb9, 0xea, 0x9b, 0x3c, 0xb7, 0x28, 0x22,
                          0x8a, 0x0c, 0x82, 0xb6, 0x7c, 0x39, 0xe9, 0x6b, 0x4b, 0x34, 0x47, 0x98,
                          0x87, 0x0d, 0x5d, 0xae, 0xe9, 0x3e, 0x3a, 0xe5, 0x93, 0x1b, 0xaa, 0xe8,
                          0xc7, 0xca, 0xcf, 0xea, 0x4b, 0x62, 0x94, 0x52, 0xc3, 0x80, 0x26, 0xa8,
                          0x1d, 0x13, 0x8b, 0xc7, 0xaa, 0xd1, 0xaf, 0x3e, 0xf7, 0xbf, 0xd5, 0xec,
                          0x64, 0x6d, 0x6c, 0x28];
        let Digest(h) = hash(&x);
        assert!(h[..] == h_expected[..]);
    }

    fn test_nist_vector(filename: &str) {
        use rustc_serialize::hex::FromHex;
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        assert!(::init());
        let mut r = BufReader::new(unwrap!(File::open(filename)));
        let mut line = String::new();
        loop {
            line.clear();
            let _ = unwrap!(r.read_line(&mut line));
            if line.is_empty() {
                break;
            }
            let starts_with_len = line.starts_with("Len = ");
            if starts_with_len {
                let len: usize = unwrap!(line[6..].trim().parse());
                line.clear();
                let _ = unwrap!(r.read_line(&mut line));
                let rawmsg = unwrap!(line[6..].from_hex());
                let msg = &rawmsg[..len / 8];
                line.clear();
                let _ = unwrap!(r.read_line(&mut line));
                let md = unwrap!(line[5..].from_hex());
                let Digest(digest) = hash(msg);
                assert!(digest[..] == md[..]);
            }
        }
    }

    #[test]
    fn test_vectors_nist_short() {
        test_nist_vector("testvectors/SHA512ShortMsg.rsp");
    }

    #[test]
    fn test_vectors_nist_long() {
        test_nist_vector("testvectors/SHA512LongMsg.rsp");
    }
}
