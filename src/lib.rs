#![no_std]
#![cfg_attr(target_arch = "xtensa", feature(asm_experimental_arch))]

const MAX_BACKTRACE_ADDRESSES: usize = 10;

#[cfg(feature = "colors")]
const RESET: &str = "\u{001B}[0m";
#[cfg(feature = "colors")]
const RED: &str = "\u{001B}[31m";

#[cfg_attr(target_arch = "riscv32", path = "riscv.rs")]
#[cfg_attr(target_arch = "xtensa", path = "xtensa.rs")]
pub mod arch;

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use esp_println::println;

    #[cfg(feature = "colors")]
    println!("{}", RED);

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
    #[cfg(target_arch = "riscv32")]
    if backtrace.iter().filter(|e| e.is_some()).count() == 0 {
        println!("No backtrace available - make sure to force frame-pointers. (see https://crates.io/crates/esp-backtrace)");
    }
    for e in backtrace {
        if let Some(addr) = e {
            println!("0x{:x}", addr);
        }
    }

    #[cfg(feature = "colors")]
    println!("{}", RESET);

    halt();
}

#[cfg(all(feature = "exception-handler", target_arch = "xtensa"))]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe fn __user_exception(cause: arch::ExceptionCause, context: arch::Context) {
    use esp_println::println;

    #[cfg(feature = "colors")]
    println!("{}", RED);

    println!("\n\nException occured '{:?}'", cause);
    println!("{:?}", context);

    let backtrace = crate::arch::backtrace_internal(context.A1, 0);
    for e in backtrace {
        if let Some(addr) = e {
            println!("0x{:x}", addr);
        }
    }

    println!("");
    println!("");
    println!("");

    #[cfg(feature = "colors")]
    println!("{}", RESET);

    halt();
}

#[cfg(all(feature = "exception-handler", target_arch = "riscv32"))]
#[export_name = "ExceptionHandler"]
fn exception_handler(context: &arch::TrapFrame) -> ! {
    use esp_println::println;

    let mepc = context.pc;
    let code = context.mcause & 0xff;
    let mtval = context.mtval;

    #[cfg(feature = "colors")]
    println!("{}", RED);

    if code == 14 {
        println!();
        println!(
            "Stack overflow detected at 0x{:x} called by 0x{:x}",
            mepc, context.ra
        );
        println!();
    } else {
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

        println!(
            "Exception '{}' mepc=0x{:08x}, mtval=0x{:08x}",
            code, mepc, mtval
        );
        println!("{:x?}", context);

        let backtrace = crate::arch::backtrace_internal(context.s0 as u32, 0);
        if backtrace.iter().filter(|e| e.is_some()).count() == 0 {
            println!("No backtrace available - make sure to force frame-pointers. (see https://crates.io/crates/esp-backtrace)");
        }
        for e in backtrace {
            if let Some(addr) = e {
                println!("0x{:x}", addr);
            }
        }
    }

    println!("");
    println!("");
    println!("");

    #[cfg(feature = "colors")]
    println!("{}", RESET);

    halt();
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

    #[cfg(feature = "esp32c3")]
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

    #[cfg(feature = "esp32h2")]
    if !(0x4080_0000..=0x4085_0000).contains(&address) {
        return false;
    }

    true
}

#[cfg(any(
    not(any(feature = "esp32", feature = "esp32s3")),
    not(feature = "halt-cores")
))]
#[allow(unused)]
fn halt() -> ! {
    loop {}
}

#[cfg(all(any(feature = "esp32", feature = "esp32s3"), feature = "halt-cores"))]
#[allow(unused)]
fn halt() -> ! {
    #[cfg(feature = "esp32")]
    mod registers {
        pub(crate) const SW_CPU_STALL: u32 = 0x3ff480ac;
        pub(crate) const OPTIONS0: u32 = 0x3ff48000;
    }

    #[cfg(feature = "esp32s3")]
    mod registers {
        pub(crate) const SW_CPU_STALL: u32 = 0x600080bc;
        pub(crate) const OPTIONS0: u32 = 0x60008000;
    }

    let sw_cpu_stall = registers::SW_CPU_STALL as *mut u32;
    let options0 = registers::OPTIONS0 as *mut u32;

    unsafe {
        sw_cpu_stall.write_volatile(
            sw_cpu_stall.read_volatile() & !(0b111111 << 20) & !(0b111111 << 26)
                | (0x21 << 20)
                | (0x21 << 26),
        );
        options0.write_volatile(options0.read_volatile() & !(0b1111) | 0b1010);
    }

    loop {}
}
