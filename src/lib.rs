use std::mem::ManuallyDrop;

type SmolStrDataPtrInner = Box<str>; // TODO: into ManuallyDrop<String>
const SMOL_STR_DATA_PTR_SIZE: usize = size_of::<SmolStrDataPtrInner>();

union SmolStrData {
    ptr: ManuallyDrop<SmolStrDataPtrInner>,
    inlined: [u8; SMOL_STR_DATA_PTR_SIZE],
}

struct SmolStr {
    data: SmolStrData,
    len_bytes: usize,
}

impl SmolStr {
    fn new() -> Self {
        Self::from("")
    }

    fn is_inlined(&self) -> bool {
        self.len_bytes <= SMOL_STR_DATA_PTR_SIZE
    }

    fn as_str(&self) -> &str {
        if self.is_inlined() {
            unsafe { std::str::from_utf8_unchecked(&self.data.inlined[..self.len_bytes]) }
        } else {
            unsafe { self.data.ptr.as_ref() }
        }
    }
}

impl From<&str> for SmolStr {
    fn from(s: &str) -> Self {
        let len_bytes = s.as_bytes().len();
        let data = match len_bytes {
            ..=SMOL_STR_DATA_PTR_SIZE => {
                let mut inlined = [0; SMOL_STR_DATA_PTR_SIZE];
                inlined[..len_bytes].copy_from_slice(s.as_bytes());
                SmolStrData { inlined }
            },
            _ => SmolStrData {
                ptr: ManuallyDrop::new(s.into())
            },
        };
        SmolStr { data, len_bytes }
    }
}

impl Drop for SmolStr {
    fn drop(&mut self) {
        if !self.is_inlined() {
            unsafe {
                ManuallyDrop::drop(&mut self.data.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(SmolStr::new().as_str(), "");
    }

    #[test]
    fn hii_ho() {
        assert_eq!(SmolStr::from("Hii Ho!").as_str(), "Hii Ho!");
    }

    #[test]
    fn hello_world() {
        assert_eq!(SmolStr::from("Hello World!").as_str(), "Hello World!");
    }
}
