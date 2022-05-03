#![no_std] // Rustの標準ライブラリにリンクしない
#![no_main] // 全てのRustレベルのエントリポイントを無効にする
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_practice::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os_practice::println;

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() {
    // リンカはデフォルトで`_start`と言う名前の関数を探すので、
    // この関数がエントリポイントとなる
    println!("Hello World{}", "!");

    rust_os_practice::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    println!("It dit not crash!");
    rust_os_practice::hlt_loop();
}

// この関数はパニック時に呼ばれる
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os_practice::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_practice::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
