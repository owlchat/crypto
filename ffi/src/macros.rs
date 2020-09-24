/// A macro to convert c_char pointer to rust's str type
#[doc(hidden)]
#[macro_export]
macro_rules! cstr {
    ($ptr:expr, allow_null) => {
        if $ptr.is_null() {
            None
        } else {
            Some($crate::cstr!($ptr))
        }
    };
    ($ptr:expr, allow_null, $error:expr) => {
        if $ptr.is_null() {
            None
        } else {
            Some($crate::cstr!($ptr, $error))
        }
    };
    ($ptr:expr) => {
        $crate::cstr!($ptr, std::ptr::null());
    };
    ($ptr:expr, $error:expr) => {{
        if $ptr.is_null() {
            return $error;
        }
        $crate::unwrap_or_null!(::std::ffi::CStr::from_ptr($ptr).to_str(), $error)
    }};
}

/// A simple macro to match on a result
#[doc(hidden)]
#[macro_export]
macro_rules! result {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                return match e {
                    keystore::KeyStoreError::AeadError(_) => crate::OperationStatus::AeadError,
                    keystore::KeyStoreError::Bip39Error(_) => crate::OperationStatus::Bip39Error,
                    keystore::KeyStoreError::EmptySeed => crate::OperationStatus::KeyStoreHasNoSeed,
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! unwrap_or_null {
    ($result:expr) => {
        $crate::unwrap_or_null!($result, std::ptr::null());
    };

    ($result:expr, $err:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return $err,
        }
    };
}

/// A simple macro to get keystore from a raw pointer
#[doc(hidden)]
#[macro_export]
macro_rules! keystore {
    ($ptr:expr) => {
        $crate::keystore!($ptr, crate::OperationStatus::KeyStoreNotInialized)
    };

    ($ptr:expr, $error:expr) => {{
        if $ptr.is_null() {
            return $error;
        } else {
            ($ptr as *const keystore::KeyStore).as_ref().unwrap()
        }
    }};
}
