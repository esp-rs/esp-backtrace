#![no_std]
#![cfg_attr(target_arch = "xtensa", feature(asm_experimental_arch))]

const MAX_BACKTRACE_ADRESSES: usize = 10;

#[cfg_attr(target_arch = "riscv32", path = "riscv.rs")]
#[cfg_attr(target_arch = "xtensa", path = "xtensa.rs")]
pub mod arch;

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use esp_println::println;

    println!(" ");
    println!(" ");

    if let Some(location) = info.location() {
        let (file, line, column) = (location.file(), location.line(), location.column());
        println!("!! A panic occured in '{file}', at line {line}, column {column}");
    } else {
        println!("!! A panic occured at an unknown location");
    }

    println!(" ");
    println!("{:#?}", info);
    println!(" ");
    println!("Backtrace:");
    println!(" ");

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
    use esp_println::println;

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
    use esp_println::println;

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

// Ensure that the address is in DRAM and that it is 16-byte aligned.
//
// Based loosely on the `esp_stack_ptr_in_dram` function from
// `components/esp_hw_support/include/esp_memory_utils.h` in ESP-IDF.
//
// Address ranges can be found in `components/soc/$CHIP/include/soc/soc.h` as
// `SOC_DRAM_LOW` and `SOC_DRAM_HIGH`.
fn is_valid_ram_address(address: u32) -> bool {
    if (address & 0xF) != 0 {
        return false;
    }

    #[cfg(feature = "esp32")]
    if !(0x3FFA_E000..=0x4000_0000).contains(&address) {
        return false;
    }

    #[cfg(feature = "esp32c2")]
    if !(0x3FCA_0000..=0x3FCE_0000).contains(&address) {
        return false;
    }

    #[cfg(any(feature = "esp32c3", feature = "esp32h2"))]
    if !(0x3FC8_0000..=0x3FCE_0000).contains(&address) {
        return false;
    }

    #[cfg(feature = "esp32c6")]
    if !(0x4080_0000..=0x4088_0000).contains(&address) {
        return false;
    }

    #[cfg(feature = "esp32s2")]
    if !(0x3FFB_0000..=0x4000_0000).contains(&address) {
        return false;
    }

    #[cfg(feature = "esp32s3")]
    if !(0x3FC8_8000..=0x3FD0_0000).contains(&address) {
        return false;
    }

    true
}
