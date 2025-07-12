# Electron Window Manager Plugin

一个用 Rust 编写的 Electron 插件，用于最小化系统中的其他窗口。

## 功能特性

- 🪟 最小化/隐藏除当前应用外的所有可见窗口
- 🔄 恢复之前隐藏的窗口
- 🛡️ 保护系统关键进程（explorer.exe、dwm.exe 等）
- ⚡ 使用 Rust 编写，性能优异
- 🔒 安全的 Windows API 调用
- 📦 支持多平台编译（主要针对 Windows）

## 安装

```bash
npm install
npm run build
```

## 使用方法

### 使用方法

```javascript
const { minimizeOtherWindows, hideOtherWindows, showOtherWindows } = require('./path/to/test3');

// 最小化其他所有窗口
minimizeOtherWindows();

// 隐藏其他所有窗口
hideOtherWindows();

// 恢复之前隐藏的窗口
showOtherWindows();
```

## 功能说明

### 最小化 vs 隐藏

- **最小化窗口** (`minimizeOtherWindows()`): 窗口会被最小化到任务栏，用户可以通过点击任务栏图标恢复窗口
- **隐藏窗口** (`hideOtherWindows()`): 窗口会完全隐藏，不在任务栏显示，需要调用 `showOtherWindows()` 才能恢复

### 系统保护

为了确保系统稳定性，以下关键进程的窗口不会被影响：

**核心系统进程：**
- `explorer.exe` (桌面和文件管理器)
- `dwm.exe` (桌面窗口管理器)
- `winlogon.exe` (登录进程)
- `csrss.exe` (客户端/服务器运行时)
- `lsass.exe` (本地安全授权)
- `services.exe` (服务控制管理器)
- `smss.exe` (会话管理器)
- `wininit.exe` (Windows 初始化进程)
- `userinit.exe` (用户初始化进程)

**用户界面相关进程：**
- `startmenuexperiencehost.exe` (开始菜单)
- `shellexperiencehost.exe` (Shell 体验主机)
- `searchui.exe` / `searchapp.exe` (搜索功能)
- `cortana.exe` (Cortana 助手)
- `sihost.exe` (Shell 基础设施主机)
- `textinputhost.exe` (文本输入主机)
- `lockapp.exe` (锁屏应用)
- `logonui.exe` (登录界面)

**运行时和服务进程：**
- `svchost.exe` (服务主机进程)
- `taskhost.exe` / `taskhostw.exe` (任务主机)
- `runtimebroker.exe` (运行时代理)
- `applicationframehost.exe` (应用程序框架主机)
- `systemsettings.exe` (系统设置)
- `winstore.app.exe` (Microsoft Store)

### 在 Electron 渲染进程中使用

首先在主进程中注册 IPC 处理器：

```javascript
// main.js
const { ipcMain } = require('electron');
const { minimizeOtherWindows } = require('./path/to/test3');

ipcMain.handle('minimize-other-windows', () => {
  minimizeOtherWindows();
});
```

然后在渲染进程中调用：

```javascript
// renderer.js
const { ipcRenderer } = require('electron');

// 调用最小化功能
async function minimizeOthers() {
  await ipcRenderer.invoke('minimize-other-windows');
}

// 绑定到按钮点击事件
document.getElementById('minimize-btn').addEventListener('click', minimizeOthers);
```

### TypeScript 支持

```typescript
import { minimizeOtherWindows } from './path/to/test3';

// 类型安全的调用
minimizeOtherWindows();
```

## API 文档

### `minimizeOtherWindows()`

最小化除当前进程外的所有可见窗口。

**参数：** 无

**返回值：** `void`

**平台支持：** Windows

**工作原理：**
1. 获取当前进程 ID
2. 枚举系统中所有窗口
3. 检查窗口是否可见且不属于当前进程
4. 对符合条件的窗口调用 `ShowWindow(hwnd, SW_MINIMIZE)`

## 开发

### 构建

```bash
# 开发构建
npm run build:debug

# 发布构建
npm run build
```

### 测试

```bash
# 运行基本测试
npm run test-plugin

# 测试隐藏和恢复窗口功能
npm run test-hide-show

# 静默运行（隐藏控制台窗口）
npm run test-silent

# 使用 VBScript 完全隐藏运行
npm run test-hidden
```

### 隐藏控制台窗口运行

如果你想在不显示控制台窗口的情况下运行插件，可以使用以下方法：

#### 方法 1：使用 VBScript（推荐）
```bash
# 双击运行或在命令行中执行
run-hidden.vbs
```

#### 方法 2：使用批处理文件
```bash
run-silent.bat
```

#### 方法 3：使用 Node.js 静默模式
```bash
node test-plugin-silent.js
```

#### 方法 4：在代码中使用 windowsHide 选项
```javascript
const { spawn } = require('child_process');

const child = spawn('node', ['your-script.js'], {
    windowsHide: true,  // 隐藏控制台窗口
    stdio: 'pipe'
});
```

## 技术细节

- 使用 `napi-rs` 创建 Node.js 原生插件
- 调用 Windows API：`EnumWindows`、`ShowWindow`、`IsWindowVisible`
- 使用 Rust 的 `windows` crate 进行系统调用
- 支持多架构编译（x64、ia32、arm64）

## 注意事项

1. **平台限制**：此插件主要为 Windows 平台设计
2. **权限要求**：某些受保护的系统窗口可能无法被最小化
3. **安全考虑**：控制其他应用程序窗口可能被安全软件标记
4. **性能**：枚举窗口是一个相对快速的操作，但在窗口数量极多时可能有轻微延迟

## 许可证

MIT