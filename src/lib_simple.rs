#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, BOOL, CloseHandle},
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowThreadProcessId, IsWindowVisible, ShowWindow, SW_MINIMIZE, SW_HIDE, SW_SHOW,
            SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, GetWindowLongW, GWL_EXSTYLE,
            GetForegroundWindow, SetWindowLongW, WS_EX_TOPMOST, SWP_FRAMECHANGED,
            BringWindowToTop, SetForegroundWindow, WS_EX_LAYERED, GWL_STYLE, SWP_SHOWWINDOW, SWP_NOACTIVATE,
            IsWindow, GetClassNameW, WS_EX_TOOLWINDOW,
        },
        System::{
            Threading::{GetCurrentProcessId, OpenProcess, PROCESS_QUERY_INFORMATION},
            ProcessStatus::GetModuleBaseNameW,
        },
    },
};

#[cfg(target_os = "windows")]
use std::{thread, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}}};
#[cfg(target_os = "windows")]
use std::sync::Mutex;

#[cfg(target_os = "windows")]
static TOPMOST_MONITOR: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

#[macro_use]
extern crate napi_derive;

// Windows特定的函数，仅在Windows平台编译
#[cfg(target_os = "windows")]
mod windows_impl {
    use super::*;

    // 检查是否是系统关键进程
    unsafe fn is_system_process(pid: u32) -> bool {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid);
        
        if let Ok(handle) = process_handle {
            let mut process_name = [0u16; 260]; // MAX_PATH
            let result = GetModuleBaseNameW(handle, None, &mut process_name);
            let _ = CloseHandle(handle);
            
            if result > 0 {
                let name = String::from_utf16_lossy(&process_name[..result as usize]).to_lowercase();
                // 保护系统关键进程
                return name == "explorer.exe" || 
                       name == "dwm.exe" || 
                       name == "winlogon.exe" || 
                       name == "csrss.exe" || 
                       name == "lsass.exe" || 
                       name == "services.exe" || 
                       name == "smss.exe" || 
                       name == "wininit.exe" || 
                       name == "svchost.exe" || 
                       name == "taskhost.exe" || 
                       name == "taskhostw.exe" || 
                       name == "sihost.exe" || 
                       name == "shellexperiencehost.exe" || 
                       name == "startmenuexperiencehost.exe" || 
                       name == "searchui.exe" || 
                       name == "cortana.exe" || 
                       name == "runtimebroker.exe" || 
                       name == "applicationframehost.exe" || 
                       name == "systemsettings.exe" || 
                       name == "winstore.app.exe" || 
                       name == "searchapp.exe" || 
                       name == "textinputhost.exe" || 
                       name == "lockapp.exe" || 
                       name == "logonui.exe" || 
                       name == "userinit.exe";
            }
        }
        
        false
    }

    // 检查窗口是否应该被操作（更安全的过滤）
    unsafe fn should_process_window(hwnd: HWND, current_pid: u32) -> bool {
        let mut window_pid = 0;
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
        
        // 跳过当前进程的窗口
        if window_pid == current_pid {
            return false;
        }
        
        // 跳过不可见窗口
        if !IsWindowVisible(hwnd).as_bool() {
            return false;
        }
        
        // 跳过系统关键进程
        if is_system_process(window_pid) {
            return false;
        }
        
        // 检查窗口扩展样式，跳过工具窗口
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        if (ex_style & WS_EX_TOOLWINDOW.0 as i32) != 0 {
            return false;
        }
        
        // 检查窗口类名，跳过系统窗口类
        let mut class_name = [0u16; 256];
        let class_len = GetClassNameW(hwnd, &mut class_name);
        if class_len > 0 {
            let class_str = String::from_utf16_lossy(&class_name[..class_len as usize]);
            // 跳过系统关键窗口类
            if class_str.starts_with("Shell_") || 
               class_str.starts_with("Progman") || 
               class_str.starts_with("WorkerW") || 
               class_str == "Windows.UI.Core.CoreWindow" || 
               class_str == "ApplicationFrameWindow" ||
               class_str == "Windows.UI.Xaml.Controls.Primitives.PopupRoot" {
                return false;
            }
        }
        
        true
    }

    // 遍历所有窗口并最小化非当前窗口
    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let current_pid = lparam.0 as u32;

        if should_process_window(hwnd, current_pid) {
            let _ = ShowWindow(hwnd, SW_MINIMIZE);
        }

        BOOL::from(true) // 继续枚举
    }

    // 遍历所有窗口并隐藏非当前窗口
    unsafe extern "system" fn enum_windows_proc_hide(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let current_pid = lparam.0 as u32;

        if should_process_window(hwnd, current_pid) {
            let _ = ShowWindow(hwnd, SW_HIDE);
        }

        BOOL::from(true) // 继续枚举
    }

    // 遍历所有窗口并显示隐藏的窗口
    unsafe extern "system" fn enum_windows_proc_show(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let current_pid = lparam.0 as u32;
        let mut window_pid = 0;

        let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));

        // 对于恢复窗口，我们只检查基本条件，不检查可见性
        if window_pid != current_pid && !is_system_process(window_pid) {
            // 检查窗口扩展样式，跳过工具窗口
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            if (ex_style & WS_EX_TOOLWINDOW.0 as i32) == 0 {
                // 检查窗口类名，跳过系统窗口类
                let mut class_name = [0u16; 256];
                let class_len = GetClassNameW(hwnd, &mut class_name);
                let mut should_show = true;
                
                if class_len > 0 {
                    let class_str = String::from_utf16_lossy(&class_name[..class_len as usize]);
                    if class_str.starts_with("Shell_") || 
                       class_str.starts_with("Progman") || 
                       class_str.starts_with("WorkerW") || 
                       class_str == "Windows.UI.Core.CoreWindow" || 
                       class_str == "ApplicationFrameWindow" ||
                       class_str == "Windows.UI.Xaml.Controls.Primitives.PopupRoot" {
                        should_show = false;
                    }
                }
                
                if should_show {
                    let _ = ShowWindow(hwnd, SW_SHOW);
                }
            }
        }

        BOOL::from(true) // 继续枚举
    }

    pub unsafe fn minimize_other_windows_impl() {
        let current_pid = GetCurrentProcessId();
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(current_pid as isize));
    }

    pub unsafe fn hide_other_windows_impl() {
        let current_pid = GetCurrentProcessId();
        let _ = EnumWindows(Some(enum_windows_proc_hide), LPARAM(current_pid as isize));
    }

    pub unsafe fn show_other_windows_impl() {
        let current_pid = GetCurrentProcessId();
        let _ = EnumWindows(Some(enum_windows_proc_show), LPARAM(current_pid as isize));
    }

    pub unsafe fn set_current_window_topmost_impl() -> bool {
        let hwnd = GetForegroundWindow();
        
        if hwnd.0 != 0 {
            let current_pid = GetCurrentProcessId();
            let mut window_pid = 0;
            let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
            
            if window_pid == current_pid {
                // 设置置顶
                let current_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                let new_ex_style = current_ex_style | WS_EX_TOPMOST.0 as i32;
                SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
                
                let _ = SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED
                );
                
                let _ = BringWindowToTop(hwnd);
                let _ = SetForegroundWindow(hwnd);
                
                return true;
            }
        }
        
        false
    }

    pub unsafe fn remove_current_window_topmost_impl() -> bool {
        let hwnd = GetForegroundWindow();
        
        if hwnd.0 != 0 {
            let current_pid = GetCurrentProcessId();
            let mut window_pid = 0;
            let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
            
            if window_pid == current_pid {
                // 移除置顶
                let current_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                let new_ex_style = current_ex_style & !(WS_EX_TOPMOST.0 as i32);
                SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
                
                let result = SetWindowPos(
                    hwnd,
                    HWND(-2), // HWND_NOTOPMOST
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED
                );
                return result.is_ok();
            }
        }
        
        false
    }
}

// 导出的API函数
#[napi]
pub fn minimize_other_windows() {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            windows_impl::minimize_other_windows_impl();
        }
    }
}

#[napi]
pub fn hide_other_windows() {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            windows_impl::hide_other_windows_impl();
        }
    }
}

#[napi]
pub fn show_other_windows() {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            windows_impl::show_other_windows_impl();
        }
    }
}

#[napi]
pub fn set_current_window_topmost() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            windows_impl::set_current_window_topmost_impl()
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[napi]
pub fn remove_current_window_topmost() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            windows_impl::remove_current_window_topmost_impl()
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[napi]
pub fn set_current_window_topmost_aggressive() -> bool {
    #[cfg(target_os = "windows")]
    {
        // 激进模式实现 - 简化版本
        unsafe {
            windows_impl::set_current_window_topmost_impl()
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[napi]
pub fn set_current_window_topmost_ultimate() -> bool {
    #[cfg(target_os = "windows")]
    {
        // 终极模式实现 - 简化版本
        unsafe {
            windows_impl::set_current_window_topmost_impl()
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[napi]
pub fn set_current_window_topmost_super_ultimate() -> bool {
    #[cfg(target_os = "windows")]
    {
        // 超级终极模式实现 - 简化版本
        unsafe {
            windows_impl::set_current_window_topmost_impl()
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[napi]
pub fn start_topmost_monitor() -> bool {
    #[cfg(target_os = "windows")]
    {
        // 监控模式 - 简化版本，直接返回true
        true
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[napi]
pub fn stop_topmost_monitor() -> bool {
    #[cfg(target_os = "windows")]
    {
        // 停止监控 - 简化版本，直接返回true
        true
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[napi]
pub fn stop_super_ultimate_topmost() -> bool {
    #[cfg(target_os = "windows")]
    {
        true
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}