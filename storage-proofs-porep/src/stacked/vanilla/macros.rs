/// Checks that the two passed values are equal. If they are not equal it prints a trace and returns `false`.
macro_rules! check_eq {
    ($left:expr , $right:expr,) => ({
        check_eq!($left, $right)
    });
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    trace!("check failed: `(left == right)`\
                          \n\
                          \n{}\
                          \n",
                           pretty_assertions::Comparison::new(left_val, right_val));
                    return false;
                }
            }
        }
    });
    ($left:expr , $right:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    trace!("check failed: `(left == right)`: {}\
                          \n\
                          \n{}\
                          \n",
                           format_args!($($arg)*),
                           pretty_assertions::Comparison::new(left_val, right_val));
                    return false;
                }
            }
        }
    });
}

/// Checks that the passed in value is true. If they are not equal it prints a trace and returns `false`.
macro_rules! check {
    ($val:expr) => {
        if !$val {
            trace!("expected {:?} to be true", dbg!($val));
            return false;
        }
    };
}

macro_rules! prefetch {
    ($val:expr) => {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        unsafe {
            #[cfg(target_arch = "x86")]
            use std::arch::x86;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64 as x86;

            x86::_mm_prefetch($val, x86::_MM_HINT_T0);
        }

        // Prefetch on AArch64 is an unstable feature in Rust. Hence it can only be enabled if a
        // nightly version of Rust is used. On stable Rust this macro is a no-op.
        #[cfg(all(nightly, target_arch = "aarch64"))]
        unsafe {
            use std::arch::aarch64::{_prefetch, _PREFETCH_LOCALITY3, _PREFETCH_READ};
            _prefetch($val, _PREFETCH_READ, _PREFETCH_LOCALITY3);
        }
        // Make sure that there are not warnings about unused variables.
        #[cfg(all(not(nightly), target_arch = "aarch64"))]
        let _ = ($val);
    };
}

// Used in multicore sdr only
#[allow(unused_macros)]
macro_rules! compress256 {
    ($state:expr, $buf:expr, 1) => {
        let blocks = [*GenericArray::<u8, U64>::from_slice(&$buf[..64])];
        sha2::compress256((&mut $state[..8]).try_into().unwrap(), &blocks[..]);
    };
    ($state:expr, $buf:expr, 2) => {
        let blocks = [
            *GenericArray::<u8, U64>::from_slice(&$buf[..64]),
            *GenericArray::<u8, U64>::from_slice(&$buf[64..128]),
        ];
        sha2::compress256((&mut $state[..8]).try_into().unwrap(), &blocks[..]);
    };
    ($state:expr, $buf:expr, 3) => {
        let blocks = [
            *GenericArray::<u8, U64>::from_slice(&$buf[..64]),
            *GenericArray::<u8, U64>::from_slice(&$buf[64..128]),
            *GenericArray::<u8, U64>::from_slice(&$buf[128..192]),
        ];
        sha2::compress256((&mut $state[..8]).try_into().unwrap(), &blocks[..]);
    };
    ($state:expr, $buf:expr, 5) => {
        let blocks = [
            *GenericArray::<u8, U64>::from_slice(&$buf[..64]),
            *GenericArray::<u8, U64>::from_slice(&$buf[64..128]),
            *GenericArray::<u8, U64>::from_slice(&$buf[128..192]),
            *GenericArray::<u8, U64>::from_slice(&$buf[192..256]),
            *GenericArray::<u8, U64>::from_slice(&$buf[256..320]),
        ];
        sha2::compress256((&mut $state[..8]).try_into().unwrap(), &blocks[..]);
    };
}
