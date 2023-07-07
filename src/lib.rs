#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::CStr, fmt, mem::transmute};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

///
/// The pixel formats.
///
#[repr(u8)]
pub enum UnimageFormat {
    None = 0,
    RGB = 1,
    RGBA = 2,
}

///
/// A image processor. it can load image, resize it or clip it.
///
pub struct UnimageProcessor {
    ptr: *mut ::std::os::raw::c_void,
}

///
/// Error with message.
///
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
    ///
    /// Create a UnimageProcessor instance.
    ///
    pub fn new() -> UnimageProcessor {
        unsafe {
            UnimageProcessor {
                ptr: unimage_processor_create(),
            }
        }
    }

    ///
    /// Get the instance pointer
    ///
    pub unsafe fn get_instance(&self) -> *mut ::std::os::raw::c_void {
        self.ptr
    }

    ///
    /// Load pixels from a pointer.
    ///
    pub fn load_raw(&mut self, data: *mut u8, width: i32, height: i32, format: UnimageFormat) {
        unsafe { unimage_processor_load_raw(self.ptr, data, width, height, format as u8) }
    }

    ///
    /// Load pixels from a `Vec<u8>`.
    ///
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

    ///
    /// Load pixels from a Slice.
    ///
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

    ///
    /// Load an image from data, if there is something wrong, it will return an error.
    ///
    pub fn load(&mut self, data: *mut u8, len: u32) -> Result<(), UnimageError> {
        let success = unsafe { unimage_processor_load(self.ptr, data, len) };

        if success == 0 {
            Err(UnimageError::from(self.get_error_message()))
        } else {
            Ok(())
        }
    }

    ///
    /// Load an image from data, if there is something wrong, it will return an error.
    ///
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

    ///
    /// Load an image from data, if there is something wrong, it will return an error.
    ///
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

    ///
    /// Get the width of image.
    ///
    pub fn get_width(&self) -> i32 {
        unsafe { unimage_processor_get_width(self.ptr) }
    }

    ///
    /// Get the height of image.
    ///
    pub fn get_height(&self) -> i32 {
        unsafe { unimage_processor_get_height(self.ptr) }
    }

    ///
    /// Get the pixel format of image.
    ///
    pub fn get_format(&self) -> UnimageFormat {
        unsafe { transmute::<u8, UnimageFormat>(unimage_processor_get_format(self.ptr)) }
    }

    ///
    /// Get the last error message.
    ///
    pub fn get_error_message(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(unimage_processor_get_error_message(self.ptr));
            let str = c_str.to_str().unwrap();
            String::from(str)
        }
    }

    ///
    /// Resize the image.
    ///
    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), UnimageError> {
        let success = unsafe { unimage_processor_resize(self.ptr, width, height) };

        if success == 0 {
            Err(UnimageError::from(self.get_error_message()))
        } else {
            Ok(())
        }
    }

    ///
    /// Get the buffer of processor, it will return a pointer that points to pixels data.
    /// If it doesn't load any image, it will return a nullptr.
    ///
    pub unsafe fn get_buffer_as_mut(&mut self) -> *mut u8 {
        unimage_processor_get_buffer(self.ptr)
    }

    ///
    /// Get the buffer of processor, it will return a pointer that points to pixels data.
    /// If it doesn't load any image, it will return a nullptr.
    ///
    pub unsafe fn get_buffer(&self) -> *const u8 {
        unimage_processor_get_buffer(self.ptr)
    }

    ///
    /// Clone the processor instance and the image buffer in it.
    /// If there is something wrong, it will return a error.
    ///
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

    ///
    /// Clip the image.
    ///
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
    ///
    /// Clone the processor instance and the image buffer in it.
    ///
    fn clone(&self) -> Self {
        self.try_clone().unwrap()
    }
}

impl Drop for UnimageProcessor {
    ///
    /// Release the processor instance and the buffer in it.
    ///
    fn drop(&mut self) {
        unsafe { unimage_processor_free(self.ptr) }
    }
}
