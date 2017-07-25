// randombytes.h

lazy_static! {
    static ref INIT_RESULT: Mutex<Option<i32>> = Mutex::new(None);
    static ref RANDOM_BYTES_IMPL: Mutex<RandomBytesImpl> = Mutex::new(RandomBytesImpl::default());
}

thread_local!(static RNG: Rc<RefCell<XorShiftRng>> =
    Rc::new(RefCell::new(XorShiftRng::from_seed(unwrap!(RANDOM_BYTES_IMPL.lock()).seed))));

struct RandomBytesImpl {
    function_pointers: randombytes_implementation,
    name: CString,
    seed: [u32; 4],
}

impl Default for RandomBytesImpl {
    fn default() -> RandomBytesImpl {
        let seed = [
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
        ];
        RandomBytesImpl {
            function_pointers: randombytes_implementation::default(),
            name: unwrap!(CString::new("Rust XorShiftRng")),
            seed: seed,
        }
    }
}

#[repr(C)]
pub struct randombytes_implementation {
    implementation_name: extern "C" fn() -> *const c_char,
    random: extern "C" fn() -> uint32_t,
    stir: Option<extern "C" fn()>,
    uniform: Option<extern "C" fn(upper_bound: uint32_t) -> uint32_t>,
    buf: extern "C" fn(buf: *mut c_void, size: size_t),
    close: Option<extern "C" fn() -> c_int>,
}

impl Default for randombytes_implementation {
    fn default() -> randombytes_implementation {
        randombytes_implementation {
            implementation_name: implementation_name,
            random: random,
            stir: None,
            uniform: None,
            buf: buf,
            close: None,
        }
    }
}

extern "C" fn implementation_name() -> *const c_char {
    unwrap!(RANDOM_BYTES_IMPL.lock()).name.as_ptr()
}

extern "C" fn random() -> uint32_t {
    RNG.with(|rng| rng.borrow_mut().gen())
}

#[cfg_attr(feature = "cargo-clippy", allow(cast_possible_wrap))]
extern "C" fn buf(buf: *mut c_void, size: size_t) {
    unsafe {
        let ptr = buf as *mut u8;
        let rng_ptr = RNG.with(|rng| rng.clone());
        let rng = &mut *rng_ptr.borrow_mut();
        for i in 0..size {
            *ptr.offset(i as isize) = rng.gen();
        }
    }
}

/// Sets [libsodium's `randombytes_implementation`][1] to use a
/// [Rust `Rng` implementation](../rand/trait.Rng.html) and initialises libsodium.
///
/// This allows a seeded PRNG to be used for example, which can be helpful in test scenarios when
/// predictable results may be preferred.
///
/// This function is safe to call multiple times concurrently from different threads.  It will
/// either always return `Ok` or will always return `Err`.
///
/// The error will contain either `-1` or `1`.  If the error is `-1`, the initialisation of
/// libsodium has failed.  If the error is `1`, libsodium has been successfully initialised
/// elsewhere (e.g. via [`rust_sodium::init()`][2]) but this means that our attempt to apply this
/// seeded RNG to libsodium has not been actioned.
///
/// Each sodiumoxide function which uses the random generator in a new thread will cause a new
/// thread-local instance of the PRNG to be created.  Each such instance will be seeded with the
/// same value, meaning for example that two newly-spawned threads calling `box_::gen_keypair()`
/// will generate identical keys.
///
/// [1]: https://download.libsodium.org/doc/advanced/custom_rng.html
/// [2]: https://docs.rs/rust_sodium/*/rust_sodium/fn.init.html
pub fn init_with_rng<T: Rng>(rng: &mut T) -> Result<(), i32> {
    let mut init_result = &mut *unwrap!(INIT_RESULT.lock());
    if let Some(ref existing_result) = *init_result {
        return if *existing_result == 0 {
            Ok(())
        } else {
            Err(*existing_result)
        };
    }
    let mut sodium_result;
    let seed = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
    {
        let random_bytes = &mut *unwrap!(RANDOM_BYTES_IMPL.lock());
        random_bytes.seed = seed;
        sodium_result =
            unsafe { randombytes_set_implementation(&mut random_bytes.function_pointers) };
    }
    if sodium_result == 0 {
        sodium_result = unsafe { sodium_init() };
    }
    // Since `sodium_init()` makes a call to `buf()`, reset the thread-local `RNG` so that it yields
    // consistent results with calls from new threads.
    RNG.with(|rng| *rng.borrow_mut() = XorShiftRng::from_seed(seed));
    *init_result = Some(sodium_result);
    match sodium_result {
        0 => Ok(()),
        result => Err(result),
    }
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_seeded_init_with_rng() {
    use std::thread::Builder;
    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
    unwrap!(init_with_rng(&mut rng));

    // Initialise again - should succeed.
    unwrap!(init_with_rng(&mut rng));

    let expected_public_key = [116, 196, 172, 118, 77, 124, 253, 254, 156, 51, 141, 193, 20, 160,
                               227, 232, 231, 20, 24, 151, 207, 45, 202, 250, 85, 96, 206, 144,
                               170, 185, 192, 101];
    let expected_private_key = [24, 74, 130, 137, 89, 75, 193, 8, 153, 136, 7, 141, 220, 198, 207,
                                232, 228, 74, 189, 36, 9, 209, 239, 95, 69, 207, 163, 2, 37, 237,
                                255, 64];
    let mut public_key = [0u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES];
    let mut private_key = [0u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES];
    unsafe {
        assert_eq!(crypto_box_curve25519xsalsa20poly1305_keypair(public_key.as_mut_ptr(),
                                                                 private_key.as_mut_ptr()),
                   0);
    }
    assert_eq!(expected_public_key, public_key);
    assert_eq!(expected_private_key, private_key);

    let child1 = unwrap!(Builder::new()
                             .name("child1".to_string())
                             .spawn(move || {
        let mut public_key = [0u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES];
        let mut private_key = [0u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES];
        unsafe {
            assert_eq!(crypto_box_curve25519xsalsa20poly1305_keypair(public_key.as_mut_ptr(),
                                                                     private_key.as_mut_ptr()),
                       0);
        }
        assert_eq!(expected_public_key, public_key);
        assert_eq!(expected_private_key, private_key);
    }));
    let child2 = unwrap!(Builder::new()
                             .name("child2".to_string())
                             .spawn(move || {
        let mut public_key = [0u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES];
        let mut private_key = [0u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES];
        unsafe {
            assert_eq!(crypto_box_curve25519xsalsa20poly1305_keypair(public_key.as_mut_ptr(),
                                                                     private_key.as_mut_ptr()),
                       0);
        }
        assert_eq!(expected_public_key, public_key);
        assert_eq!(expected_private_key, private_key);
    }));
    unwrap!(child1.join());
    unwrap!(child2.join());
}
