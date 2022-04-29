#![no_std] // Rustの標準ライブラリにリンクしない
#![no_main] // 全てのRustレベルのエントリポイントを無効にする

use core::panic::PanicInfo;

// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hellow World!";

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    // リンカはデフォルトで`_start`と言う名前の関数を探すので、
    // この関数がエントリポイントとなる

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
