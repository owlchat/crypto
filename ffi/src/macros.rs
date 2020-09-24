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
        $crate::result!(::std::ffi::CStr::from_ptr($ptr).to_str(), $error)
    }};
}

/// A simple macro to match on a result and if there is an error it returns null or your custom error
#[doc(hidden)]
#[macro_export]
macro_rules! result {
    ($result:expr) => {
        $crate::result!($result, std::ptr::null());
    };
    ($result:expr, $error:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => {
                return $error;
            }
        }
    };
}

/// A simple macro to get keystore from a raw pointer
#[doc(hidden)]
#[macro_export]
macro_rules! keystore {
    ($ptr:expr) => {
        $crate::keystore!($ptr, std::ptr::null())
    };

    ($ptr:expr, $error:expr) => {{
        if $ptr.is_null() {
            return $error;
        } else {
            ($ptr as *const keystore::KeyStore).as_ref().unwrap()
        }
    }};
}
