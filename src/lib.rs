#![feature(test)]

extern crate test;

pub mod parity_libsecp256k1 {
    use parity_secp256k1::{Error, PublicKey, SecretKey};
    use test::Bencher;

    fn ecmul() -> Result<(), Error> {
        let secret_key = SecretKey::parse(&[
            137, 16, 46, 159, 212, 158, 232, 178, 197, 253, 105, 137, 102, 159, 70, 217, 110, 211,
            254, 82, 216, 4, 105, 171, 102, 252, 54, 190, 114, 91, 11, 69,
        ])?;

        let mut public_key = PublicKey::parse_compressed(&[
            2, 123, 236, 243, 192, 100, 34, 40, 51, 111, 129, 130, 160, 64, 129, 135, 11, 184, 68,
            84, 83, 198, 234, 196, 150, 13, 208, 86, 34, 150, 10, 59, 247,
        ])?;
        public_key.tweak_mul_assign(&secret_key)?;

        assert_eq!(
            public_key,
            PublicKey::parse_compressed(&[
                2, 151, 202, 113, 10, 9, 43, 125, 187, 101, 157, 152, 191, 94, 12, 236, 133, 229,
                16, 233, 221, 52, 150, 183, 243, 61, 110, 8, 152, 132, 99, 49, 189,
            ])?
        );

        Ok(())
    }

    #[test]
    fn test_libsecp256k1() {
        ecmul().unwrap();
    }

    #[bench]
    fn bench_libsecp256k1(b: &mut Bencher) {
        b.iter(|| ecmul().unwrap());
    }
}

pub mod bitcoin_core_libsecp256k1 {
    use bitcoin_core_secp256k1::{
        ffi::types::AlignedType, AllPreallocated, Error, PublicKey, Secp256k1,
    };
    use test::Bencher;

    fn ecmul(secp: &Secp256k1<AllPreallocated<'_>>) -> Result<(), Error> {
        let secret_key = &[
            137, 16, 46, 159, 212, 158, 232, 178, 197, 253, 105, 137, 102, 159, 70, 217, 110, 211,
            254, 82, 216, 4, 105, 171, 102, 252, 54, 190, 114, 91, 11, 69,
        ];

        let mut public_key = PublicKey::from_slice(&[
            2, 123, 236, 243, 192, 100, 34, 40, 51, 111, 129, 130, 160, 64, 129, 135, 11, 184, 68,
            84, 83, 198, 234, 196, 150, 13, 208, 86, 34, 150, 10, 59, 247,
        ])?;
        public_key.mul_assign(&secp, secret_key)?;

        assert_eq!(
            public_key,
            PublicKey::from_slice(&[
                2, 151, 202, 113, 10, 9, 43, 125, 187, 101, 157, 152, 191, 94, 12, 236, 133, 229,
                16, 233, 221, 52, 150, 183, 243, 61, 110, 8, 152, 132, 99, 49, 189,
            ])?
        );

        Ok(())
    }

    #[test]
    fn test_libsecp256k1() {
        let mut buf = vec![AlignedType::zeroed(); Secp256k1::preallocate_size()];
        let secp = Secp256k1::preallocated_new(&mut buf).unwrap();
        ecmul(&secp).unwrap();
    }

    #[bench]
    fn bench_libsecp256k1(b: &mut Bencher) {
        let mut buf = vec![AlignedType::zeroed(); Secp256k1::preallocate_size()];
        let secp = Secp256k1::preallocated_new(&mut buf).unwrap();
        b.iter(|| ecmul(&secp).unwrap());
    }
}

pub mod rust_crypto_libsecp256k1 {
    use rust_crypto_secp256k1::{
        elliptic_curve::{ecdh::diffie_hellman, Error, PublicKey},
        EncodedPoint, Secp256k1, SecretKey,
    };
    use std::ops::Deref;
    use test::Bencher;

    fn ecmul() -> Result<(), Error> {
        let secret_key = SecretKey::from_bytes(&[
            137, 16, 46, 159, 212, 158, 232, 178, 197, 253, 105, 137, 102, 159, 70, 217, 110, 211,
            254, 82, 216, 4, 105, 171, 102, 252, 54, 190, 114, 91, 11, 69,
        ])?;

        let encoded_point = EncodedPoint::from_bytes(&[
            2, 123, 236, 243, 192, 100, 34, 40, 51, 111, 129, 130, 160, 64, 129, 135, 11, 184, 68,
            84, 83, 198, 234, 196, 150, 13, 208, 86, 34, 150, 10, 59, 247,
        ])?;

        let public_key = PublicKey::<Secp256k1>::from_sec1_bytes(encoded_point.as_ref()).unwrap();
        let shared_secret = diffie_hellman(secret_key.secret_scalar(), public_key.as_affine());

        assert_eq!(
            shared_secret.as_bytes().deref(),
            &[
                151, 202, 113, 10, 9, 43, 125, 187, 101, 157, 152, 191, 94, 12, 236, 133, 229, 16,
                233, 221, 52, 150, 183, 243, 61, 110, 8, 152, 132, 99, 49, 189,
            ]
        );

        Ok(())
    }

    #[test]
    fn test_libsecp256k1() {
        ecmul().unwrap();
    }

    #[bench]
    fn bench_libsecp256k1(b: &mut Bencher) {
        b.iter(|| ecmul().unwrap());
    }
}
