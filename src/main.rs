fn main() {
    println!("Hello, Mouse!\n");

    let mut delta = 10;
    loop {
        println!(". ");
        std::thread::sleep(std::time::Duration::from_secs(1));
        delta = -delta;
        let mut pos = get_cursor_pos();
        pos.x += delta;
        set_cursor_pos(pos.x, pos.y);
    }
}

//  //  //  //  //  //  //  //
use windows::Win32::UI::WindowsAndMessaging::*;
pub use windows::Win32::Foundation::POINT;

pub fn get_cursor_pos() -> POINT {
    let mut pos = POINT::default();
    unsafe { let _ = GetCursorPos(&mut pos); };
    pos
}
pub fn set_cursor_pos(x: i32, y: i32) {
    unsafe { let _ = SetCursorPos(x, y); };
}
