use std::io;

use windows::Win32::{System::Threading::{SetProcessPriorityBoost, OpenProcess, PROCESS_SET_INFORMATION}, Foundation::{HWND, LPARAM, BOOL}, UI::WindowsAndMessaging::{EnumWindows, IsWindowVisible, GetWindowTextW, GetWindowThreadProcessId}};

fn main() {
    let windows = visible_windows_with_titles();

    for (ix, (_, title)) in windows.iter().enumerate() {
        println!("{ix}: {title}");
    }

    let ix: usize = io::stdin().lines().next()
        .expect("No stdin")
        .expect("Invalid window number")
        .parse().expect("Invalid window number");

    let hwnd = windows.get(ix).expect("Invalid window number").0;

    unsafe {
        let mut pid = 0;
        assert_ne!(GetWindowThreadProcessId(hwnd, Some(&mut pid)), 0, "GetWindowThreadProcessId failed");

        let hprocess = OpenProcess(PROCESS_SET_INFORMATION, false, pid).expect("OpenProcess failed");

        SetProcessPriorityBoost(hprocess, true).expect("SetProcessPriorityBoost failed");
    }

    println!("ok");
}

fn visible_windows_with_titles() -> Vec<(HWND, String)> {
    list_windows()
        .iter()
        .copied()
        .filter(|hwnd| is_window_visible(*hwnd))
        .filter_map(|hwnd| window_title(hwnd).map(|title| (hwnd, title)))
        .collect::<Vec<_>>()
}

fn list_windows() -> Vec<HWND> {
    extern "system" fn add_window(hwnd: HWND, vec_ptr: LPARAM) -> BOOL {
        let vec: &mut Vec<HWND> = unsafe {
            &mut *(vec_ptr.0 as *mut Vec<HWND>)
        };

        vec.push(hwnd);

        true.into()
    }

    let mut windows: Vec<HWND> = Vec::new();

    unsafe {
        EnumWindows(Some(add_window), LPARAM(&mut windows as *mut _ as isize)).expect("EnumWindows failed");
    }
    
    windows
}

fn is_window_visible(hwnd: HWND) -> bool {
    unsafe {
        IsWindowVisible(hwnd).into()
    }
}

fn window_title(hwnd: HWND) -> Option<String> {
    // More characters aren't shown anyway
    const MAX_LEN: usize = 256;

    let mut buffer = [0_u16; MAX_LEN];
    
    let len = unsafe {
        GetWindowTextW(hwnd, &mut buffer)
    } as usize;
    
    if len > 0 {
        Some(String::from_utf16_lossy(&buffer[..len]))
    } else {
        None
    }
}
