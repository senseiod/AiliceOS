use core::alloc::Layout;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("\n");
    print!(r"
IIIIIIII           III            III     II     OO     IIIIIIIIIIII
II     II         II  II          IIII    II            II
II     II        II    II         II II   II     II     II
II     II       IIIIIIIIII        II  II  II     II     II
IIIIIIII       II        II       II   II II     II     II
II            II          II      II    IIII     II     II
II           II            II     II     III     II     IIIIIIIIIIIII
");
    print!("\n");
    print!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort");
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory!");
}

#[lang = "eh_personality"]
#[no_mangle]
fn eh_personality() -> ! {
    loop {}
}
