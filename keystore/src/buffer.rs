use aes_gcm_siv::aead::{self, Buffer};
use core::slice;
use std::ptr::{self, NonNull};

#[derive(Debug)]
#[repr(C)]
pub struct SharedBuffer {
    buf: NonNull<u8>,
    len: usize,
    cap: usize,
}

impl Eq for SharedBuffer {}

impl PartialEq for SharedBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.len.eq(&other.len) && self.as_ref()[..] == other.as_ref()[..]
    }
}

impl SharedBuffer {
    pub fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr() as _
    }

    fn set_len(&mut self, len: usize) {
        // This is safe because:
        //
        // * the slice passed to `drop_in_place` is valid; the `len > self.len`
        //   case avoids creating an invalid slice, and
        // * the `len` of the buffer is shrunk before calling `drop_in_place`,
        //   such that no value will be dropped twice in case `drop_in_place`
        //   were to panic once (if it panics twice, the program aborts).
        unsafe {
            if len > self.len {
                return;
            }
            let remaining_len = self.len - len;
            let s = ptr::slice_from_raw_parts_mut(self.buf.as_ptr().add(len), remaining_len);
            self.len = len;
            ptr::drop_in_place(s);
        }
    }

    fn extend(&mut self, other: &[u8]) {
        unsafe {
            let len = self.len();
            let dst_slice = slice::from_raw_parts_mut(self.buf.as_ptr().add(len), other.len());
            dst_slice.copy_from_slice(other);
            self.len += other.len();
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.buf.as_ptr().add(self.len())))
            }
        }
    }

    unsafe fn from_raw_parts(ptr: *mut u8, len: usize, cap: usize) -> Self {
        Self {
            buf: NonNull::new_unchecked(ptr),
            len,
            cap,
        }
    }
    /// Try to push the value into the buffer.
    /// returns `None` if the value got pushed .. otherwise returns `Some(value)`
    #[cfg(test)]
    pub fn push(&mut self, value: u8) -> Option<u8> {
        // This will panic or abort if we would allocate > isize::MAX bytes
        // or if the length increment would overflow for zero-sized types.
        if self.len == self.cap {
            return Some(value);
        }
        unsafe {
            let end = self.buf.as_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
            None
        }
    }
}

impl AsMut<[u8]> for SharedBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.buf.as_ptr(), self.len) }
    }
}
impl AsRef<[u8]> for SharedBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr(), self.len) }
    }
}

impl Buffer for SharedBuffer {
    fn extend_from_slice(&mut self, other: &[u8]) -> Result<(), aead::Error> {
        if self.len + other.len() <= self.cap {
            self.extend(other);
            Ok(())
        } else {
            Err(aead::Error)
        }
    }

    fn truncate(&mut self, len: usize) {
        self.set_len(len);
    }
}

impl From<Vec<u8>> for SharedBuffer {
    fn from(mut v: Vec<u8>) -> Self {
        let (ptr, len, cap) = (v.as_mut_ptr(), v.len(), v.capacity());
        std::mem::forget(v);
        unsafe { SharedBuffer::from_raw_parts(ptr, len, cap) }
    }
}

#[cfg(test)]
mod test_buffer {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut buffer: SharedBuffer = Vec::with_capacity(32).into();
        for i in 0..32 {
            let v = buffer.push(i);
            assert_eq!(v, None);
        }
        // one push again.
        let v = buffer.push(255);
        assert_eq!(v, Some(255));
        for i in 0..32 {
            let v = buffer.pop();
            assert_eq!(v, Some(31 - i));
        }

        // one more pop
        let v = buffer.pop();
        assert_eq!(v, None);
    }
}
