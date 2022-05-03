use crate::MAX_BACKTRACE_ADRESSES;
use core::arch::asm;

/// Get an array of backtrace addresses.
///
/// This needs `force-frame-pointers` enabled.
pub fn backtrace() -> [Option<usize>; MAX_BACKTRACE_ADRESSES] {
    let fp = unsafe {
        let mut _tmp: u32;
        asm!("mv {0}, x8", out(reg) _tmp);
        _tmp
    };

    backtrace_internal(fp, 2)
}

pub(crate) fn backtrace_internal(
    fp: u32,
    suppress: i32,
) -> [Option<usize>; MAX_BACKTRACE_ADRESSES] {
    let mut result = [None; 10];
    let mut index = 0;

    let mut fp = fp;
    let mut suppress = suppress;
    let mut old_address = 0;
    loop {
        unsafe {
            let address = (fp as *const u32).offset(-1).read_volatile(); // RA/PC
            fp = (fp as *const u32).offset(-2).read_volatile(); // next FP

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
