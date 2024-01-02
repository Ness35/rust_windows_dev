use std::ptr;
use std::ffi::c_void;
use std::io::Result;

macro_rules! okay 
{
    ($($arg:tt)*) => (println!("[+] {}", $($arg)*));
}
macro_rules! info
{
    ($($arg:tt)*) => (println!("[i] {}", $($arg)*));
}
macro_rules! warn
{
    ($($arg:tt)*) => (println!("[!] {}", $($arg)*));
}
fn main() -> Result<()> {
    // Example usage of macros
    okay!("This is an okay message!");
    warn!("This is a warning message!");
    info!("This is an informational message!");

    Ok(())
}