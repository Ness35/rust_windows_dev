use winapi::um::winuser::{MessageBoxW, MB_ICONEXCLAMATION, MB_YESNO};

fn message_box(content: &mut String, title: &mut String) -> i32 {
    unsafe {
        let last_result = MessageBoxW(
            std::ptr::null_mut(),
            content.encode_utf16().collect::<Vec<_>>().as_mut_ptr(),
            title.encode_utf16().collect::<Vec<_>>().as_mut_ptr(),
            MB_ICONEXCLAMATION | MB_YESNO,
        );
        last_result
    }
}

fn main() {
    let mut content = "Do you want to continue ?\0".to_string();
    let mut title = "Test 1/2\0".to_string();
    let last_result = message_box(&mut content, &mut title);

    if last_result == 6 {
        content = "You continued the test\0".to_string();
        title = "Test 2/2\0".to_string();
        message_box(&mut content, &mut title);
    } else {
        content = "Goodbye\0".to_string();
        title = "Exit\0".to_string();
        message_box(&mut content, &mut title);
    }
}

