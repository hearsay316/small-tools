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

use std::{thread, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use std::sync::Mutex;

static TOPMOST_MONITOR: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

#[macro_use]
extern crate napi_derive;

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

#[napi]
pub fn minimize_other_windows() {
    let current_pid = unsafe { GetCurrentProcessId() };

    unsafe {
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(current_pid as isize));
    }
}

#[napi]
pub fn hide_other_windows() {
    let current_pid = unsafe { GetCurrentProcessId() };

    unsafe {
        let _ = EnumWindows(Some(enum_windows_proc_hide), LPARAM(current_pid as isize));
    }
}

#[napi]
pub fn show_other_windows() {
    let current_pid = unsafe { GetCurrentProcessId() };

    unsafe {
        let _ = EnumWindows(Some(enum_windows_proc_show), LPARAM(current_pid as isize));
    }
}

// 获取当前进程的主窗口
unsafe fn get_current_process_main_window() -> Option<HWND> {
    let _current_pid = GetCurrentProcessId();
    let mut main_window: Option<HWND> = None;
    
    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let target_pid = lparam.0 as u32;
        let mut window_pid = 0;
        
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
        
        if window_pid == target_pid && IsWindowVisible(hwnd).as_bool() {
            // 检查是否是主窗口（不是工具窗口）
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            if (ex_style & WS_EX_TOOLWINDOW.0 as i32) == 0 {
                let result_ptr = lparam.0 as *mut Option<HWND>;
                *result_ptr = Some(hwnd);
                return BOOL::from(false); // 停止枚举
            }
        }
        
        BOOL::from(true) // 继续枚举
    }
    
    let _ = EnumWindows(
        Some(enum_proc), 
        LPARAM(&mut main_window as *mut Option<HWND> as isize)
    );
    
    main_window
}

// 强制设置窗口为最顶层的增强版本
unsafe fn set_window_always_on_top(hwnd: HWND) -> bool {
    // 1. 设置最高优先级的扩展样式
    let current_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
    let new_ex_style = current_ex_style | WS_EX_TOPMOST.0 as i32 | WS_EX_LAYERED.0 as i32;
    SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
    
    // 2. 设置窗口为系统级置顶
    let current_style = GetWindowLongW(hwnd, GWL_STYLE);
    SetWindowLongW(hwnd, GWL_STYLE, current_style);
    
    // 3. 多次强制设置为最顶层，使用不同的方法
    for _i in 0..5 {
        // 使用 HWND_TOPMOST
        let _ = SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED | SWP_SHOWWINDOW
        );
        
        // 强制激活和前置
        let _ = BringWindowToTop(hwnd);
        let _ = SetForegroundWindow(hwnd);
        
        thread::sleep(Duration::from_millis(20));
    }
    
    // 4. 最后一次确保置顶
    let _ = SetWindowPos(
        hwnd,
        HWND_TOPMOST,
        0, 0, 0, 0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
    );
    
    true
}

#[napi]
pub fn set_current_window_topmost_aggressive() -> bool {
    unsafe {
        // 首先尝试获取前台窗口
        let hwnd = GetForegroundWindow();
        
        if hwnd.0 != 0 {
            // 检查前台窗口是否属于当前进程
            let current_pid = GetCurrentProcessId();
            let mut window_pid = 0;
            let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
            
            if window_pid == current_pid {
                return set_window_ultra_topmost(hwnd);
            }
        }
        
        // 如果前台窗口不属于当前进程，尝试获取当前进程的主窗口
        if let Some(main_hwnd) = get_current_process_main_window() {
            return set_window_ultra_topmost(main_hwnd);
        }
        
        false
    }
}

// 终极置顶模式 - 专门对抗纳米AI
#[napi]
pub fn set_current_window_topmost_ultimate() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            if let Some(main_hwnd) = get_current_process_main_window() {
                return set_window_ultimate_topmost(main_hwnd);
            }
            return false;
        }
        
        let current_pid = GetCurrentProcessId();
        let mut window_pid = 0;
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
        
        if window_pid == current_pid {
            return set_window_ultimate_topmost(hwnd);
        }
        
        if let Some(main_hwnd) = get_current_process_main_window() {
            return set_window_ultimate_topmost(main_hwnd);
        }
        
        false
    }
}

// 超级终极置顶模式 - 简化版本
#[napi]
pub fn set_current_window_topmost_super_ultimate() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            if let Some(main_hwnd) = get_current_process_main_window() {
                return set_window_super_ultimate_topmost(main_hwnd);
            }
            return false;
        }
        
        let current_pid = GetCurrentProcessId();
        let mut window_pid = 0;
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
        
        if window_pid == current_pid {
            return set_window_super_ultimate_topmost(hwnd);
        }
        
        if let Some(main_hwnd) = get_current_process_main_window() {
            return set_window_super_ultimate_topmost(main_hwnd);
        }
        
        false
    }
}

// 超级终极置顶实现 - 简化版本
unsafe fn set_window_super_ultimate_topmost(hwnd: HWND) -> bool {
    // 1. 设置扩展样式为置顶
    let current_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
    let new_ex_style = current_ex_style | WS_EX_TOPMOST.0 as i32 | WS_EX_LAYERED.0 as i32;
    SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
    
    // 2. 超级激进的多重置顶策略
    for _round in 0..10 {
        // 每轮进行超密集的置顶操作
        for _i in 0..100 {
            // 多种置顶方法同时使用
            let _ = SetWindowPos(
                hwnd,
                HWND(-1), // HWND_TOPMOST
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW
            );
            
            let _ = SetWindowPos(
                hwnd,
                HWND(-1),
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED
            );
            
            // 强制前台显示
            let _ = BringWindowToTop(hwnd);
            let _ = SetForegroundWindow(hwnd);
            
            // 重新设置扩展样式
            SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
        }
        
        // 极短间隔后继续下一轮
        thread::sleep(Duration::from_millis(1));
    }
    
    // 3. 最终确认置顶状态
    for _i in 0..50 {
        let _ = SetWindowPos(
            hwnd,
            HWND(-1),
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW
        );
        let _ = BringWindowToTop(hwnd);
        let _ = SetForegroundWindow(hwnd);
    }
    
    true
}

// 停止超级终极置顶模式 - 简化版本
#[napi]
pub fn stop_super_ultimate_topmost() -> bool {
    // 简化实现，主要用于接口兼容
    true
}

// 系统级别的终极置顶方法
unsafe fn set_window_ultimate_topmost(hwnd: HWND) -> bool {
    // 1. 设置最高级别的扩展样式
    let current_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
    let new_ex_style = current_ex_style | WS_EX_TOPMOST.0 as i32 | WS_EX_LAYERED.0 as i32;
    SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
    
    // 2. 超级激进的多轮置顶策略
    for _round in 0..5 {
        // 每轮进行超密集的置顶操作
        for i in 0..50 {
            // 使用系统级最高优先级
            let _ = SetWindowPos(
                hwnd,
                HWND(-1), // HWND_TOPMOST
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED | SWP_SHOWWINDOW
            );
            
            // 超频繁的激活和前置操作
            if i % 2 == 0 {
                let _ = SetForegroundWindow(hwnd);
                let _ = BringWindowToTop(hwnd);
                
                // 持续重新设置扩展样式
                SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
            }
            
            // 极短间隔，几乎无延迟
            thread::sleep(Duration::from_millis(1));
        }
        
        // 每轮之间的微小停顿
        thread::sleep(Duration::from_millis(5));
    }
    
    // 3. 最终超级确认阶段
    for _ in 0..20 {
        let _ = SetWindowPos(
            hwnd,
            HWND(-1),
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
        );
        
        // 持续强制前置
        let _ = BringWindowToTop(hwnd);
        
        // 无延迟连续操作
        thread::sleep(Duration::from_nanos(500_000));
    }
    
    true
}

#[napi]
pub fn start_topmost_monitor() -> bool {
    unsafe {
        // 获取当前窗口
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            if let Some(main_hwnd) = get_current_process_main_window() {
                return start_monitor_for_window(main_hwnd);
            }
            return false;
        }
        
        let current_pid = GetCurrentProcessId();
        let mut window_pid = 0;
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
        
        if window_pid == current_pid {
            return start_monitor_for_window(hwnd);
        }
        
        if let Some(main_hwnd) = get_current_process_main_window() {
            return start_monitor_for_window(main_hwnd);
        }
        
        false
    }
}

fn start_monitor_for_window(hwnd: HWND) -> bool {
    // 停止之前的监控
    stop_topmost_monitor();
    
    let should_run = Arc::new(AtomicBool::new(true));
    let should_run_clone = should_run.clone();
    
    // 保存监控状态
    if let Ok(mut monitor) = TOPMOST_MONITOR.lock() {
        *monitor = Some(should_run.clone());
    }
    
    // 启动后台线程持续监控
    thread::spawn(move || {
        while should_run_clone.load(Ordering::Relaxed) {
            unsafe {
                // 检查窗口是否仍然存在
                if IsWindow(hwnd).as_bool() {
                    // 强制刷新置顶状态
                    let _ = SetWindowPos(
                        hwnd,
                        HWND(-1), // HWND_TOPMOST
                        0, 0, 0, 0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
                    );
                    
                    // 每隔50ms检查一次，更快对抗纳米AI
                    thread::sleep(Duration::from_millis(50));
                } else {
                    // 窗口已关闭，停止监控
                    break;
                }
            }
        }
    });
    
    true
}

#[napi]
pub fn stop_topmost_monitor() -> bool {
    if let Ok(mut monitor) = TOPMOST_MONITOR.lock() {
        if let Some(should_run) = monitor.take() {
            should_run.store(false, Ordering::Relaxed);
            return true;
        }
    }
    false
}

// 超级激进的置顶方法 - 针对纳米AI等顽固应用
unsafe fn set_window_ultra_topmost(hwnd: HWND) -> bool {
    // 1. 设置所有可能的置顶样式
    let current_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
    let new_ex_style = current_ex_style | WS_EX_TOPMOST.0 as i32 | WS_EX_LAYERED.0 as i32;
    SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
    
    // 2. 超级激进的连续置顶策略
    for _round in 0..3 {
        // 每轮进行更密集的置顶操作
        for i in 0..20 {
            // 使用最高优先级，强制置顶
            let _ = SetWindowPos(
                hwnd,
                HWND(-1), // HWND_TOPMOST
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED | SWP_SHOWWINDOW
            );
            
            // 更频繁的激活操作
            if i % 3 == 0 {
                let _ = SetForegroundWindow(hwnd);
                let _ = BringWindowToTop(hwnd);
                
                // 重新设置扩展样式确保不被覆盖
                SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
            }
            
            // 更短的间隔
            thread::sleep(Duration::from_millis(2));
        }
        
        // 每轮之间稍作停顿
        thread::sleep(Duration::from_millis(10));
    }
    
    // 3. 最终多次确认
    for _ in 0..5 {
        let _ = SetWindowPos(
            hwnd,
            HWND(-1),
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
        );
        thread::sleep(Duration::from_millis(1));
    }
    
    true
}

#[napi]
pub fn set_current_window_topmost() -> bool {
    unsafe {
        // 首先尝试获取前台窗口
        let hwnd = GetForegroundWindow();
        
        if hwnd.0 != 0 {
            // 检查前台窗口是否属于当前进程
            let current_pid = GetCurrentProcessId();
            let mut window_pid = 0;
            let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
            
            if window_pid == current_pid {
                return set_window_always_on_top(hwnd);
            }
        }
        
        // 如果前台窗口不属于当前进程，尝试获取当前进程的主窗口
        if let Some(main_hwnd) = get_current_process_main_window() {
            return set_window_always_on_top(main_hwnd);
        }
        
        false
    }
}

#[napi]
pub fn remove_current_window_topmost() -> bool {
    unsafe {
        // 首先尝试获取前台窗口
        let hwnd = GetForegroundWindow();
        
        if hwnd.0 != 0 {
            // 检查前台窗口是否属于当前进程
            let current_pid = GetCurrentProcessId();
            let mut window_pid = 0;
            let _ = GetWindowThreadProcessId(hwnd, Some(&mut window_pid));
            
            if window_pid == current_pid {
                // 移除扩展样式中的 TOPMOST
                let current_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                let new_ex_style = current_ex_style & !(WS_EX_TOPMOST.0 as i32);
                SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);
                
                // 使用 SetWindowPos 移除 TOPMOST 状态
                let result = SetWindowPos(
                    hwnd,
                    HWND(-2), // HWND_NOTOPMOST
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED
                );
                return result.is_ok();
            }
        }
        
        // 如果前台窗口不属于当前进程，尝试获取当前进程的主窗口
        if let Some(main_hwnd) = get_current_process_main_window() {
            // 移除扩展样式中的 TOPMOST
            let current_ex_style = GetWindowLongW(main_hwnd, GWL_EXSTYLE);
            let new_ex_style = current_ex_style & !(WS_EX_TOPMOST.0 as i32);
            SetWindowLongW(main_hwnd, GWL_EXSTYLE, new_ex_style);
            
            // 使用 SetWindowPos 移除 TOPMOST 状态
            let result = SetWindowPos(
                main_hwnd,
                HWND(-2), // HWND_NOTOPMOST
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED
            );
            return result.is_ok();
        }
        
        false
    }
}