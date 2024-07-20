#[cfg(target_os = "zkvm")]
use core::arch::asm;

/// Call a function in a foreign program.
///
/// `address` is the callee address, `input` is a bytearray to be passed to the
/// callee function, and `len` is the number of bytes to read from the input bytearray.
/// For now there is no return value and no return status code. The caller can assume
/// that, if this function returns, the call was successful.
///
/// See https://github.com/athenavm/athena/issues/5 for more information.
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn call(address: *const u32, input: *const u32, len: usize) {
  #[cfg(target_os = "zkvm")]
  unsafe {
    asm!(
        "ecall",
        in("t0") crate::syscalls::HOST_CALL,
        in("a0") address,
        in("a1") input,
        in("a2") len,
    )
  }

  #[cfg(not(target_os = "zkvm"))]
  unreachable!()
}

/// Read from host storage at the given address and key.
///
/// The output is stored in the `key` pointer.
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn read_storage(key: *mut u32, address: *const u32) {
  #[cfg(target_os = "zkvm")]
  unsafe {
    asm!(
        "ecall",
        in("t0") crate::syscalls::HOST_READ,
        in("a0") key,
        in("a1") address,
    )
  }

  #[cfg(not(target_os = "zkvm"))]
  unreachable!()
}

/// Write to host storage at the given address and key.
///
/// The result status code is stored in the `key` pointer.
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn write_storage(key: *mut u32, address: *const u32, value: *const u32) {
  #[cfg(target_os = "zkvm")]
  unsafe {
    asm!(
        "ecall",
        in("t0") crate::syscalls::HOST_WRITE,
        in("a0") key,
        in("a1") address,
        in("a2") value,
    )
  }

  #[cfg(not(target_os = "zkvm"))]
  unreachable!()
}
