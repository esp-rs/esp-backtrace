use crate::MAX_BACKTRACE_ADRESSES;
use core::arch::asm;

/// Get an array of backtrace addresses.
///
pub fn backtrace() -> [Option<usize>; MAX_BACKTRACE_ADRESSES] {
    let sp = unsafe {
        let mut _tmp: u32;
        asm!("mov {0}, a1", out(reg) _tmp);
        _tmp
    };

    backtrace_internal(sp, 1)
}

pub(crate) fn sanitize_address(address: u32) -> u32 {
    (address & 0x3fff_ffff) | 0x4000_0000
}

pub(crate) fn backtrace_internal(
    sp: u32,
    suppress: i32,
) -> [Option<usize>; MAX_BACKTRACE_ADRESSES] {
    let mut result = [None; 10];
    let mut index = 0;

    let mut fp = sp;
    let mut suppress = suppress;
    let mut old_address = 0;

    loop {
        unsafe {
            let address = sanitize_address((fp as *const u32).offset(-4).read_volatile()); // RA/PC
            fp = (fp as *const u32).offset(-3).read_volatile(); // next FP

            if old_address == address {
                break;
            }

            old_address = address;

            if address == 0 {
                break;
            }

            if !crate::is_valid_ram_address(fp) {
                break;
            }

            if fp == 0 {
                break;
            }

            if suppress == 0 {
                result[index] = Some(address as usize);
                index += 1;

                if index >= MAX_BACKTRACE_ADRESSES {
                    break;
                }
            } else {
                suppress -= 1;
            }
        }
    }

    result
}
