// Kernel driver interface for undetected R/W
// Uses MmCopyVirtualMemory to bypass EAC usermode hooks

use std::ptr;
use std::mem;
use winapi::um::fileapi::*;
use winapi::um::handleapi::*;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winnt::*;
use winapi::shared::minwindef::*;

// IOCTL codes for driver communication
const IOCTL_READ_MEMORY: u32 = 0x222004;
const IOCTL_WRITE_MEMORY: u32 = 0x222008;
const IOCTL_GET_BASE: u32 = 0x22200C;

#[repr(C)]
struct MemoryRequest {
    process_id: u32,
    address: usize,
    buffer: *mut u8,
    size: usize,
}

#[repr(C)]
struct BaseRequest {
    process_id: u32,
    module_name: [u8; 260],
    base_address: usize,
}

pub struct DriverInterface {
    handle: *mut winapi::ctypes::c_void,
    process_id: u32,
}

impl DriverInterface {
    pub fn new(process_id: u32) -> Option<Self> {
        unsafe {
            // Try multiple driver names (obfuscation)
            let device_names = [
                b"\\\\.\\RustDriver\0",
                b"\\\\.\\WinDrv\0",
                b"\\\\.\\SysDriver\0",
            ];
            
            for device_name in &device_names {
                let handle = CreateFileA(
                    device_name.as_ptr() as *const i8,
                    GENERIC_READ | GENERIC_WRITE,
                    0,
                    ptr::null_mut(),
                    OPEN_EXISTING,
                    FILE_ATTRIBUTE_NORMAL,
                    ptr::null_mut(),
                );
                
                if handle != INVALID_HANDLE_VALUE {
                    println!("[+] Driver connected");
                    return Some(DriverInterface {
                        handle,
                        process_id,
                    });
                }
            }
            
            println!("[-] Driver not found");
            println!("[!] Install driver first:");
            println!("    1. Build driver from driver/");
            println!("    2. Load with kdmapper or DSEFix");
            println!("    3. Driver will create device: \\\\.\\RustDriver");
            None
        }
    }
    
    pub fn read<T: Copy>(&self, address: usize) -> Result<T, ()> {
        unsafe {
            let mut buffer: T = mem::zeroed();
            let mut request = MemoryRequest {
                process_id: self.process_id,
                address,
                buffer: &mut buffer as *mut T as *mut u8,
                size: mem::size_of::<T>(),
            };
            
            let mut bytes_returned = 0;
            let result = DeviceIoControl(
                self.handle,
                IOCTL_READ_MEMORY,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut bytes_returned,
                ptr::null_mut(),
            );
            
            if result != 0 {
                Ok(buffer)
            } else {
                Err(())
            }
        }
    }
    
    pub fn write<T: Copy>(&self, address: usize, value: T) -> Result<(), ()> {
        unsafe {
            let mut request = MemoryRequest {
                process_id: self.process_id,
                address,
                buffer: &value as *const T as *mut u8,
                size: mem::size_of::<T>(),
            };
            
            let mut bytes_returned = 0;
            let result = DeviceIoControl(
                self.handle,
                IOCTL_WRITE_MEMORY,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                ptr::null_mut(),
                0,
                &mut bytes_returned,
                ptr::null_mut(),
            );
            
            if result != 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }
    
    pub fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>, ()> {
        let mut buffer = vec![0u8; size];
        unsafe {
            let mut request = MemoryRequest {
                process_id: self.process_id,
                address,
                buffer: buffer.as_mut_ptr(),
                size,
            };
            
            let mut bytes_returned = 0;
            let result = DeviceIoControl(
                self.handle,
                IOCTL_READ_MEMORY,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut bytes_returned,
                ptr::null_mut(),
            );
            
            if result != 0 {
                Ok(buffer)
            } else {
                Err(())
            }
        }
    }
    
    // Get module base via driver (more reliable than usermode)
    pub fn get_module_base(&self, module_name: &str) -> Result<usize, ()> {
        unsafe {
            let mut request = BaseRequest {
                process_id: self.process_id,
                module_name: [0; 260],
                base_address: 0,
            };
            
            // Copy module name
            let bytes = module_name.as_bytes();
            let len = bytes.len().min(259);
            request.module_name[..len].copy_from_slice(&bytes[..len]);
            
            let mut bytes_returned = 0;
            let result = DeviceIoControl(
                self.handle,
                IOCTL_GET_BASE,
                &mut request as *mut _ as *mut _,
                mem::size_of::<BaseRequest>() as u32,
                &mut request as *mut _ as *mut _,
                mem::size_of::<BaseRequest>() as u32,
                &mut bytes_returned,
                ptr::null_mut(),
            );
            
            if result != 0 && request.base_address != 0 {
                Ok(request.base_address)
            } else {
                Err(())
            }
        }
    }
}

impl Drop for DriverInterface {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
