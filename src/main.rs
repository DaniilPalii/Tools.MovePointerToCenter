#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() {
    let screen_center_x = get_primary_screen_width() / 2;
    let screen_center_y = get_primary_screen_height() / 2;

    unsafe {
        set_cursor_position(screen_center_x, screen_center_y);
    }
}

fn get_primary_screen_width() -> i32 {
    unsafe { get_system_metric(0) }
}

fn get_primary_screen_height() -> i32 {
    unsafe { get_system_metric(1) }
}

#[link(name = "user32")]
unsafe extern "system" {
    #[link_name = "GetSystemMetrics"]
    fn get_system_metric(index: i32) -> i32;

    #[link_name = "SetCursorPos"]
    fn set_cursor_position(x: i32, y: i32) -> i32;
}
