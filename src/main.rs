#![cfg_attr(windows, windows_subsystem = "windows")]

use std::process::ExitCode;

fn center_position(width: i32, height: i32) -> Option<(i32, i32)> {
	if width <= 0 || height <= 0 {
		return None;
	}

	Some((width / 2, height / 2))
}

#[cfg(windows)]
mod windows {
	use super::{ExitCode, center_position};
	use std::iter::once;

	const SM_CXSCREEN: i32 = 0;
	const SM_CYSCREEN: i32 = 1;
	const MB_OK: u32 = 0x0000;
	const MB_ICONERROR: u32 = 0x0010;

	#[link(name = "user32")]
	unsafe extern "system" {
		fn GetSystemMetrics(index: i32) -> i32;
		fn SetCursorPos(x: i32, y: i32) -> i32;
		fn MessageBoxW(
			window: *mut std::ffi::c_void,
			text: *const u16,
			caption: *const u16,
			kind: u32,
		) -> i32;
	}

	#[link(name = "kernel32")]
	unsafe extern "system" {
		fn GetLastError() -> u32;
	}

	fn show_error(message: &str) {
		let text: Vec<u16> = message.encode_utf16().chain(once(0)).collect();
		let caption: Vec<u16> = "Move Pointer To Center"
			.encode_utf16()
			.chain(once(0))
			.collect();

		unsafe {
			MessageBoxW(
				std::ptr::null_mut(),
				text.as_ptr(),
				caption.as_ptr(),
				MB_OK | MB_ICONERROR,
			);
		}
	}

	pub(super) fn run() -> ExitCode {
		let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
		let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

		let Some((x, y)) = center_position(width, height) else {
			show_error("Windows returned invalid primary display dimensions.");
			return ExitCode::FAILURE;
		};

		if unsafe { SetCursorPos(x, y) } == 0 {
			let error = unsafe { GetLastError() };
			show_error(&format!(
				"Could not move the pointer (Windows error {error})."
			));
			return ExitCode::FAILURE;
		}

		ExitCode::SUCCESS
	}
}

#[cfg(windows)]
fn main() -> ExitCode {
	windows::run()
}

#[cfg(not(windows))]
fn main() -> ExitCode {
	eprintln!("MovePointerToCenter is supported only on Windows.");
	ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
	use super::center_position;

	#[test]
	fn returns_the_integer_midpoint_of_the_display() {
		assert_eq!(center_position(1920, 1080), Some((960, 540)));
		assert_eq!(center_position(1919, 1079), Some((959, 539)));
	}

	#[test]
	fn rejects_non_positive_display_dimensions() {
		assert_eq!(center_position(0, 1080), None);
		assert_eq!(center_position(1920, 0), None);
		assert_eq!(center_position(-1, 1080), None);
	}
}
