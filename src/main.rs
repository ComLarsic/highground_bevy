//! A story-driven 2d platformer with rpg-elements, inspired by super paper mario
use std::error::Error;
use libloading::{Library, Symbol};

/// Loads the dynamic library 'libhighground' and calls the entrypoint
/// This is done to simplify potential modding support
fn main() -> Result<(), Box<dyn Error>> {
    // Figure out what the lib name should be
    #[cfg(target_os = "windows")]
    let path = "libhighground.dll";
    #[cfg(target_os = "macos")]
    let path = "libhighground.dylib";
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let path = "liblibhighground.so";
    // Load libhighground
    let lib = unsafe { Library::new(path)? };
    let entrypoint: Symbol<extern fn()> = unsafe { lib.get(b"start")? };
    // Call the entrypoint
    entrypoint();
    return Ok(());
}
