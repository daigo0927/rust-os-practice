#![no_std] // Rustの標準ライブラリにリンクしない
#![no_main] // 全てのRustレベルのエントリポイントを無効にする
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_practice::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;
use rust_os_practice::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os_practice::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    rust_os_practice::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash");
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
