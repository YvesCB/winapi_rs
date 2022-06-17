use windows::{
    core::*, 
    Win32::Foundation::*, 
    Win32::Graphics::Gdi::*, 
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*
};
use std::ffi::c_void;

struct Win32OffscreenBuffer {
    info: BITMAPINFO,
    memory: *mut c_void,
    width: i32,
    height: i32,
    pitch: i32,
    bytes_per_pixel: i32,
}

fn win32_resize_dib_section(buffer: Win32OffscreenBuffer, width: i32, height: i32) {
    if buffer.memory {
        VirtualFree(buffer.memory, 0, MEM_RELEASE);
    }
}

fn main() -> Result<()> {
    unsafe {
        let mut perf_count_freq: i64;
        QueryPerformanceFrequency(&perf_count_freq);
        glob_perf_count_freq = perf_count_freq;

        win32_resize_dib_section(&glob_backbuffer, 1280, 720);

        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = "window";

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: PCSTR(b"window\0".as_ptr()),

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            Default::default(), 
            window_class,
            "Demo window ä",
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
            None, None,
            instance,
            std::ptr::null()
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&message);
        }


        Ok(())
    }
}

extern "system" 
fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, std::ptr::null());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
