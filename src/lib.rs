#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::CStr, fmt, mem::transmute};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(u8)]
pub enum UnimageFormat {
    None = 0,
    RGB = 1,
    RGBA = 2,
}

pub struct UnimageProcessor {
    ptr: *mut ::std::os::raw::c_void,
}

#[derive(Debug)]
pub struct UnimageError {
    message: String,
}

impl fmt::Display for UnimageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl UnimageError {
    fn from(err: String) -> UnimageError {
        UnimageError { message: err }
    }
}

impl UnimageProcessor {
    pub fn new() -> UnimageProcessor {
        unsafe {
            UnimageProcessor {
                ptr: unimage_processor_create(),
            }
        }
    }

    pub fn load_raw(&mut self, data: *mut u8, width: i32, height: i32, format: UnimageFormat) {
        unsafe { unimage_processor_load_raw(self.ptr, data, width, height, format as u8) }
    }

    pub fn load_raw_from_vec(
        &mut self,
        data: &mut Vec<u8>,
        width: i32,
        height: i32,
        format: UnimageFormat,
    ) {
        unsafe {
            unimage_processor_load_raw(self.ptr, data.as_mut_ptr(), width, height, format as u8)
        }
    }

    pub fn load_raw_from_slice(
        &mut self,
        data: &mut [u8],
        width: i32,
        height: i32,
        format: UnimageFormat,
    ) {
        unsafe {
            unimage_processor_load_raw(self.ptr, data.as_mut_ptr(), width, height, format as u8)
        }
    }

    pub fn load(&mut self, data: *mut u8, len: u32) -> Result<(), UnimageError> {
        let success = unsafe { unimage_processor_load(self.ptr, data, len) };

        if success == 0 {
            Err(UnimageError::from(self.get_error_message()))
        } else {
            Ok(())
        }
    }

    pub fn load_from_vec(&mut self, data: &mut Vec<u8>) -> Result<(), UnimageError> {
        let success = unsafe {
            unimage_processor_load(self.ptr, data.as_mut_ptr(), data.len().try_into().unwrap())
        };

        if success == 0 {
            Err(UnimageError::from(self.get_error_message()))
        } else {
            Ok(())
        }
    }

    pub fn load_from_slice(&mut self, data: &mut [u8]) -> Result<(), UnimageError> {
        let success = unsafe {
            unimage_processor_load(self.ptr, data.as_mut_ptr(), data.len().try_into().unwrap())
        };

        if success == 0 {
            Err(UnimageError::from(self.get_error_message()))
        } else {
            Ok(())
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { unimage_processor_get_width(self.ptr) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { unimage_processor_get_height(self.ptr) }
    }

    pub fn get_format(&self) -> UnimageFormat {
        unsafe { transmute::<u8, UnimageFormat>(unimage_processor_get_format(self.ptr)) }
    }

    pub fn get_error_message(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(unimage_processor_get_error_message(self.ptr));
            let str = c_str.to_str().unwrap();
            String::from(str)
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), UnimageError> {
        let success = unsafe { unimage_processor_resize(self.ptr, width, height) };

        if success == 0 {
            Err(UnimageError::from(self.get_error_message()))
        } else {
            Ok(())
        }
    }

    pub unsafe fn get_buffer_as_mut(&mut self) -> *mut u8 {
        unimage_processor_get_buffer(self.ptr)
    }

    pub unsafe fn get_buffer(&self) -> *const u8 {
        unimage_processor_get_buffer(self.ptr)
    }

    pub fn try_clone(&self) -> Result<Self, UnimageError> {
        let new = UnimageProcessor::new();
        let success = unsafe { unimage_processor_copy_from(new.ptr, self.ptr) };

        if success == 0 {
            let err = UnimageError::from(self.get_error_message());
            drop(new);
            Err(err)
        } else {
            Ok(new)
        }
    }

    pub fn clip(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), UnimageError> {
        let success = unsafe { unimage_processor_clip(self.ptr, x, y, width, height) };

        if success == 0 {
            Err(UnimageError::from(self.get_error_message()))
        } else {
            Ok(())
        }
    }
}

impl Clone for UnimageProcessor {
    fn clone(&self) -> Self {
        self.try_clone().unwrap()
    }
}

impl Drop for UnimageProcessor {
    fn drop(&mut self) {
        unsafe { unimage_processor_free(self.ptr) }
    }
}
