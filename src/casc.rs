use std::ffi::CString;
use std::os::raw;
use std::ptr::null_mut;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

extern "C" {
    fn CascOpenStorage(
        storage_path: *const raw::c_char,
        locale_mask: raw::c_uint,
        ptr_storage_handle: *mut *mut raw::c_void,
    ) -> bool;

    fn CascCloseStorage(
        storage_handle: *mut raw::c_void,
    ) -> bool;

    fn CascOpenFile(
        storage_handle: *mut raw::c_void,
        pv_file_name: *const raw::c_void,
        dw_locale_flag: raw::c_uint,
        dw_open_flag: raw::c_uint,
        ptr_file_handle: *mut *mut raw::c_void,
    ) -> bool;

    fn CascSetFilePointer64(
        file_handle: *mut raw::c_void,
        distance_to_move: raw::c_longlong,
        ptr_new_pos: *mut raw::c_ulonglong,
        dw_move_method: raw::c_uint,
    ) -> bool;

    fn CascReadFile(
        file_handle: *mut raw::c_void,
        lp_buffer: *mut raw::c_void,
        dw_to_read: raw::c_uint,
        pdw_read: *mut raw::c_uint,
    ) -> bool;

    fn CascCloseFile(
        file_handle: *mut raw::c_void,
    ) -> bool;

    fn GetCascError() -> u32;
}

pub struct CascStorage {
    handle: *mut raw::c_void,
}

#[derive(Copy, Clone)]
pub struct CascFile {
    handle: *mut raw::c_void,
}

impl CascStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<CascStorage, CascError> {
        let c_path = CString::new(
            path.as_ref().as_os_str().to_str().unwrap()
        ).unwrap();
        let mut handle = null_mut();
        unsafe {
            if !CascOpenStorage(
                c_path.as_ptr(),
                0,
                &mut handle,
            ) {
                return Err(CascError { code: GetCascError() });
            }
        }
        Ok(CascStorage { handle })
    }

    pub fn openFile<P: AsRef<Path>>(&self, path: P) -> Result<CascFile, CascError> {
        let mut handle = null_mut();
        let c_path = CString::new(
            path.as_ref().as_os_str().to_str().unwrap()
        ).unwrap();
        unsafe {
            if !CascOpenFile(
                self.handle,
                c_path.as_ptr() as *const _ as *const raw::c_void,
                0,
                0,
                &mut handle,
            ) {
                return Err(CascError { code: GetCascError() });
            }
        }
        Ok(CascFile { handle })
    }

    pub fn close(&mut self) -> Result<(), CascError> {
        unsafe {
            if !CascCloseStorage(self.handle) {
                return Err(CascError { code: GetCascError() });
            }
        }
        Ok(())
    }
}

impl Seek for CascFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        let mut result = 0;
        let (dist, move_method) = match pos {
            SeekFrom::Start(dist) => (dist as i64, 0),
            SeekFrom::Current(dist) => (dist, 1),
            SeekFrom::End(dist) => (dist, 2),
        };
        unsafe {
            CascSetFilePointer64(
                self.handle,
                dist,
                &mut result,
                move_method,
            );
        }
        Ok(result)
    }
}

impl Read for CascFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut result = 0;
        unsafe {
            CascReadFile(
                self.handle,
                buf as *mut _ as *mut raw::c_void,
                buf.len() as u32,
                &mut result,
            );
        }
        Ok(result as usize)
    }
}

impl CascFile {
    fn close(&mut self) -> Result<(), std::io::Error> {
        unsafe {
            CascCloseFile(self.handle);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CascError {
    code: u32,
}

impl fmt::Display for CascError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CascError: Code {}", self.code)
    }
}

impl Error for CascError {}