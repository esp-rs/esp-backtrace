#![no_std]
#![cfg_attr(target_arch = "xtensa", feature(asm_experimental_arch))]

const MAX_BACKTRACE_ADRESSES: usize = 10;

#[cfg_attr(target_arch = "riscv32", path = "riscv.rs")]
#[cfg_attr(target_arch = "xtensa", path = "xtensa.rs")]
pub mod arch;

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use esp_println::*;

    println!("{:?}", info);

    let backtrace = crate::arch::backtrace();
    for e in backtrace {
        if let Some(addr) = e {
            println!("0x{:x}", addr);
        }
    }

    loop {}
}

#[cfg(all(feature = "exception-handler", target_arch = "xtensa"))]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __exception(
    cause: xtensa_lx_rt::exception::ExceptionCause,
    context: xtensa_lx_rt::exception::Context,
) {
    use esp_println::*;
    println!("\n\nException occured {:?} {:x?}", cause, context);

    println!("0x{:x}", crate::arch::sanitize_address(context.PC));
    println!("0x{:x}", crate::arch::sanitize_address(context.A0));

    let backtrace = crate::arch::backtrace_internal(context.A1, 0);
    for e in backtrace {
        if let Some(addr) = e {
            println!("0x{:x}", addr);
        }
    }

    println!("");
    println!("");
    println!("");

    loop {}
}

#[cfg(all(feature = "exception-handler", target_arch = "riscv32"))]
#[export_name = "ExceptionHandler"]
fn exception_handler(context: &arch::TrapFrame) -> ! {
    use esp_println::*;

    let mepc = riscv::register::mepc::read();
    let code = riscv::register::mcause::read().code() & 0xff;
    let mtval = riscv::register::mtval::read();

    let code = match code {
        0 => "Instruction address misaligned",
        1 => "Instruction access fault",
        2 => "Illegal instruction",
        3 => "Breakpoint",
        4 => "Load address misaligned",
        5 => "Load access fault",
        6 => "Store/AMO address misaligned",
        7 => "Store/AMO access fault",
        8 => "Environment call from U-mode",
        9 => "Environment call from S-mode",
        10 => "Reserved",
        11 => "Environment call from M-mode",
        12 => "Instruction page fault",
        13 => "Load page fault",
        14 => "Reserved",
        15 => "Store/AMO page fault",
        _ => "UNKNOWN",
    };
    println!("Exception '{}' mepc={:x}, mtval={:x}", code, mepc, mtval);
    println!("{:x?}", context);

    let backtrace = crate::arch::backtrace_internal(context.s0 as u32, 0);
    for e in backtrace {
        if let Some(addr) = e {
            println!("0x{:x}", addr);
        }
    }
    loop {}
}

fn is_valid_ram_address(address: u32) -> bool {
    #[cfg(feature = "esp32c3")]
    if !(0x38000000..=0x3fffffff).contains(&address) {
        return false;
    }

    #[cfg(feature = "esp32")]
    if !(0x3ff80000..=0x3fffffff).contains(&address) {
        return false;
    }

    #[cfg(feature = "esp32s2")]
    if !(0x3ff9e000..=0x3fffffff).contains(&address) {
        return false;
    }

    #[cfg(feature = "esp32s3")]
    if !(0x3fc80000..=0x3fffffff).contains(&address) {
        return false;
    }

    true
}
