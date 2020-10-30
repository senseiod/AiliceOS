use core::alloc::Layout;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("\n");
    print!(31;r"
IIIIIIII           III            III     II     OO     IIIIIIIIIIII
II     II         II  II          IIII    II            II
II     II        II    II         II II   II     II     II
II     II       IIIIIIIIII        II  II  II     II     II
IIIIIIII       II        II       II   II II     II     II
II            II          II      II    IIII     II     II
II           II            II     II     III     II     IIIIIIIIIIIII
");
    print!("\n");
    debug!("{}", info);
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort!");
}

#[lang = "oom"]
fn oom(layout: Layout) -> ! {
    panic!("Memory allocation of {} bytes failed", layout.size());
}

#[lang = "eh_personality"]
#[no_mangle]
fn eh_personality() -> ! {
    loop {}
}
