use winapi::um::winuser::MessageBoxW;

fn message_box(content: &str, title: &str, button: u32, icon: u32) -> i32 {
    unsafe {
        let last_result = MessageBoxW(
            std::ptr::null_mut(),
            content.encode_utf16().collect::<Vec<_>>().as_mut_ptr(),
            title.encode_utf16().collect::<Vec<_>>().as_mut_ptr(),
            icon | button,
        );
        last_result
    }
}

fn main() {
    let last_result = message_box("Do you want to continue ?\0", "Test 1/2\0", 0x00000004, 0x00000030);

    if last_result == 6 {
        message_box("You continued the test\0", "Test 2/2\0", 0x000000000, 0x000000040);
    } else {
        message_box("Goodbye\0", "Exit\0", 0x00000003, 0x000000020);
    }
}

