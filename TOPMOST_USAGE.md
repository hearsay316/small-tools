# 窗口置顶功能使用指南

## 功能概述

本项目提供了多层级的窗口置顶功能，专门用于对抗划词软件：

1. **`setCurrentWindowTopmost()`** - 基础置顶功能
2. **`setCurrentWindowTopmostAggressive()`** - 激进置顶模式，超强对抗划词软件
3. **`setCurrentWindowTopmostUltimate()`** - 终极置顶模式，专门对抗纳米AI等顽固应用
4. **`startTopmostMonitor()`** - 启动持续监控，每100ms刷新置顶状态
5. **`stopTopmostMonitor()`** - 停止持续监控
6. **`removeCurrentWindowTopmost()`** - 取消窗口置顶状态，恢复正常层级

## 技术实现

### 多层级置顶策略

#### 基础置顶模式
- 扩展样式设置：添加 `WS_EX_TOPMOST` 扩展样式
- 多次 SetWindowPos 调用：使用 `HWND_TOPMOST` 标志
- 窗口激活：调用 `BringWindowToTop` 和 `SetForegroundWindow`

#### 激进置顶模式（推荐用于对抗划词软件）
- **增强扩展样式**：同时设置 `WS_EX_TOPMOST` 和 `WS_EX_LAYERED`
- **连续强制置顶**：10次连续调用 SetWindowPos，每次间隔2ms
- **交替激活策略**：交替使用不同的窗口激活方法
- **最终确认**：确保置顶状态完全生效

#### 终极置顶模式（专门对抗纳米AI等顽固应用）
- **超级激进策略**：5轮 × 50次操作的多轮置顶策略
- **极短间隔操作**：1毫秒间隔实现几乎无延迟的连续操作
- **持续重设样式**：持续重新设置扩展样式防止被覆盖
- **超级确认阶段**：最终进行20次超级确认
- **纳秒级延迟**：使用纳秒级延迟实现最强置顶效果

#### 持续监控模式（终极解决方案）
- **后台线程监控**：每100ms检查并刷新置顶状态
- **自动修复**：当检测到置顶状态被破坏时自动恢复
- **智能停止**：当窗口关闭时自动停止监控
- **资源安全**：使用原子操作确保线程安全

### 取消置顶功能

- 移除 `WS_EX_TOPMOST` 扩展样式
- 使用 `HWND_NOTOPMOST` 标志恢复正常层级
- 应用 `SWP_FRAMECHANGED` 确保样式更新生效

## 使用方法

### 1. Electron 应用测试（推荐）

这是最佳的测试方式，因为 Electron 应用有实际的窗口界面：

```bash
# 方法1：使用批处理文件
double-click start-electron.bat

# 方法2：直接命令行
npx electron example.js
```

**操作步骤：**
1. 启动 Electron 应用
2. **推荐测试顺序**：
   - 先尝试 "🚀 激进置顶模式" 按钮
   - 如果仍被划词软件覆盖，尝试 "💥 终极置顶模式 (对抗纳米AI)" 按钮
   - 如果还是无效，点击 "🔄 启动持续监控"
   - 测试完成后点击 "⏹️ 停止监控" 和 "📌 取消窗口置顶"
3. 打开划词软件，尝试划词
4. 观察划词软件是否能覆盖应用窗口
5. 使用相应的取消按钮恢复正常状态

### 2. Node.js 命令行测试

虽然命令行环境没有可见窗口，但可以测试 API 调用：

```bash
# 基础测试
node test-topmost.js

# 高级交互测试
node test-topmost-advanced.js
```

### 3. 编程调用

```javascript
const { setCurrentWindowTopmost, removeCurrentWindowTopmost } = require('./index');

// 设置置顶
const success1 = setCurrentWindowTopmost();
if (success1) {
    console.log('窗口已置顶');
}

// 取消置顶
const success2 = removeCurrentWindowTopmost();
if (success2) {
    console.log('已取消置顶');
}
```

## 快捷键支持

在 Electron 应用中，我们提供了便捷的快捷键：

- **Ctrl+T** (或 Cmd+T): 基础窗口置顶
- **Ctrl+Alt+T** (或 Cmd+Alt+T): 激进置顶模式
- **Ctrl+Shift+U** (或 Cmd+Shift+U): 终极置顶模式（对抗纳米AI）
- **Ctrl+M** (或 Cmd+M): 启动持续监控
- **Ctrl+Shift+M** (或 Cmd+Shift+M): 停止监控
- **Ctrl+Shift+T** (或 Cmd+Shift+T): 取消窗口置顶

## 测试场景

### 对抗划词软件测试

1. **准备工作**
   - 确保已安装划词软件（如有道词典、金山词霸等）
   - 启动 Electron 应用

2. **测试步骤**
   - **第一轮测试**：点击 "🚀 激进置顶模式"
   - **第二轮测试**：如果激进模式无效，点击 "🔄 启动持续监控"
   - 选中应用窗口中的一些文字
   - 观察划词软件的弹窗是否出现在应用窗口之下
   - 尝试多次划词，测试持续监控的效果

3. **预期结果**
   - **激进模式成功**：划词软件无法覆盖应用窗口
   - **持续监控成功**：即使划词软件短暂覆盖，也会在100ms内被重新置顶
   - **完全失败**：可能遇到系统级权限限制或特殊的划词软件

### 多窗口环境测试

1. 打开多个应用程序
2. 设置 Electron 应用为置顶
3. 尝试点击其他应用程序
4. 观察 Electron 应用是否始终保持在最前面

## 故障排除

### 常见问题

**Q: 为什么 Node.js 命令行测试总是失败？**
A: 这是正常现象。Node.js 进程通常没有可见的窗口，因此无法设置置顶。请使用 Electron 应用进行实际测试。

**Q: 置顶功能对某些软件无效怎么办？**
A: 某些系统级软件或具有特殊权限的应用可能仍能覆盖置顶窗口。这是 Windows 系统的安全限制。

**Q: 如何确认置顶功能是否生效？**
A: 
1. 设置置顶后，尝试点击其他应用程序
2. 观察当前窗口是否始终保持在最前面
3. 使用划词软件等工具进行测试

**Q: 取消置顶后窗口行为异常？**
A: 重启应用程序通常可以解决此问题。这是因为某些窗口状态需要完全重置。

## 技术细节

### Windows API 调用序列

```rust
// 设置置顶
1. GetForegroundWindow() 或 get_current_process_main_window()
2. SetWindowLongW(hwnd, GWL_EXSTYLE, current_style | WS_EX_TOPMOST)
3. SetWindowPos(hwnd, HWND_TOPMOST, ...)
4. BringWindowToTop(hwnd)
5. SetForegroundWindow(hwnd)

// 取消置顶
1. SetWindowLongW(hwnd, GWL_EXSTYLE, current_style & !WS_EX_TOPMOST)
2. SetWindowPos(hwnd, HWND_NOTOPMOST, ...)
```

### 兼容性

- **操作系统**: Windows 7 及以上
- **权限要求**: 普通用户权限（某些情况下可能需要管理员权限）
- **应用类型**: 支持所有基于 Windows API 的应用程序

## 更新日志

### v1.3.0 (最新)
- 🚀 新增 `setCurrentWindowTopmostUltimate()` 终极置顶模式，专门对抗纳米AI等顽固应用
- 💥 采用超级激进的多轮置顶策略（5轮 × 50次操作）
- ⚡ 使用极短间隔操作（1毫秒间隔）和纳秒级延迟
- 🔄 持续重新设置扩展样式防止被覆盖
- 🎯 超级确认阶段，最终进行20次超级确认
- 🎮 新增快捷键 `Ctrl+Shift+U` 启动终极置顶模式
- 📱 更新Electron应用界面，添加终极置顶按钮

### v1.2.0
- ✅ 新增 `setCurrentWindowTopmostAggressive()` 激进置顶模式
- ✅ 新增 `startTopmostMonitor()` 持续监控功能
- ✅ 新增 `stopTopmostMonitor()` 停止监控功能
- ✅ 多线程后台监控，每100ms刷新置顶状态
- ✅ 增强的扩展样式设置（WS_EX_LAYERED）
- ✅ 连续强制置顶策略（10次连续调用）
- ✅ 新增快捷键支持
- ✅ 完整的 UI 界面更新

### v1.1.0
- ✅ 新增 `setCurrentWindowTopmost()` 基础置顶功能
- ✅ 新增 `removeCurrentWindowTopmost()` 取消置顶功能
- ✅ 增强的多重置顶策略
- ✅ Electron 应用集成
- ✅ 快捷键支持
- ✅ 完整的测试套件

---

**注意**: 窗口置顶功能的效果可能因系统配置、其他运行的软件以及用户权限而有所不同。建议在实际使用环境中进行充分测试。