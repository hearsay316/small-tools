const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');

// 导入原生模块
let nativeModule;
try {
  nativeModule = require('./test5.node');
  console.log('✅ Native module loaded successfully');
} catch (error) {
  console.warn('⚠️ Native module not available:', error.message);
  // 创建模拟函数以防止应用崩溃
  nativeModule = {
    minimizeOtherWindows: () => Promise.resolve('Native module not available'),
    hideOtherWindows: () => Promise.resolve('Native module not available'),
    setCurrentWindowTopmost: () => Promise.resolve('Native module not available'),
    removeCurrentWindowTopmost: () => Promise.resolve('Native module not available'),
    setCurrentWindowTopmostAggressive: () => Promise.resolve('Native module not available'),
    setCurrentWindowTopmostUltimate: () => Promise.resolve('Native module not available'),
    setCurrentWindowTopmostSuperUltimate: () => Promise.resolve('Native module not available'),
    startTopmostMonitor: () => Promise.resolve('Native module not available'),
    stopTopmostMonitor: () => Promise.resolve('Native module not available'),
    showOtherWindows: () => Promise.resolve('Native module not available'),
    getWindowCount: () => Promise.resolve(0),
    blockWordLookupPopups: () => Promise.resolve('Native module not available'),
    startAntiPopupMonitor: () => Promise.resolve('Native module not available'),
    stopAntiPopupMonitor: () => Promise.resolve('Native module not available'),
    startSuperAntiPopupMode: () => Promise.resolve('Native module not available')
  };
}

let mainWindow;

// 创建主窗口
function createWindow() {
  console.log('🚀 Creating main window...');
  
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      enableRemoteModule: false,
      preload: path.join(__dirname, 'preload.js')
    },
    title: 'Window Manager Tool',
    icon: path.join(__dirname, 'assets', 'icon.png') // 可选图标
  });

  // 加载 HTML 文件
  mainWindow.loadFile('example.html');

  // 开发模式下打开开发者工具
  if (process.argv.includes('--enable-logging')) {
    mainWindow.webContents.openDevTools();
  }

  // 窗口关闭事件
  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  console.log('✅ Main window created successfully');
}

// 应用准备就绪
app.whenReady().then(() => {
  console.log('🎯 Electron app ready, creating window...');
  createWindow();

  // macOS 特殊处理
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

// 所有窗口关闭时退出应用（除了 macOS）
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

// IPC 处理器 - 窗口管理功能
console.log('🔧 Setting up IPC handlers...');

ipcMain.handle('minimize-other-windows', async () => {
  console.log('📨 IPC: minimize-other-windows called');
  try {
    const result = await nativeModule.minimizeOtherWindows();
    console.log('✅ minimize-other-windows result:', result);
    return result;
  } catch (error) {
    console.error('❌ minimize-other-windows error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('hide-other-windows', async () => {
  console.log('📨 IPC: hide-other-windows called');
  try {
    const result = await nativeModule.hideOtherWindows();
    console.log('✅ hide-other-windows result:', result);
    return result;
  } catch (error) {
    console.error('❌ hide-other-windows error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('set-current-window-topmost', async () => {
  console.log('📨 IPC: set-current-window-topmost called');
  try {
    const result = await nativeModule.setCurrentWindowTopmost();
    console.log('✅ set-current-window-topmost result:', result);
    return result;
  } catch (error) {
    console.error('❌ set-current-window-topmost error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('remove-current-window-topmost', async () => {
  console.log('📨 IPC: remove-current-window-topmost called');
  try {
    const result = await nativeModule.removeCurrentWindowTopmost();
    console.log('✅ remove-current-window-topmost result:', result);
    return result;
  } catch (error) {
    console.error('❌ remove-current-window-topmost error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('set-current-window-topmost-aggressive', async () => {
  console.log('📨 IPC: set-current-window-topmost-aggressive called');
  try {
    const result = await nativeModule.setCurrentWindowTopmostAggressive();
    console.log('✅ set-current-window-topmost-aggressive result:', result);
    return result;
  } catch (error) {
    console.error('❌ set-current-window-topmost-aggressive error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('set-current-window-topmost-ultimate', async () => {
  console.log('📨 IPC: set-current-window-topmost-ultimate called');
  try {
    const result = await nativeModule.setCurrentWindowTopmostUltimate();
    console.log('✅ set-current-window-topmost-ultimate result:', result);
    return result;
  } catch (error) {
    console.error('❌ set-current-window-topmost-ultimate error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('set-current-window-topmost-super-ultimate', async () => {
  console.log('📨 IPC: set-current-window-topmost-super-ultimate called');
  try {
    const result = await nativeModule.setCurrentWindowTopmostSuperUltimate();
    console.log('✅ set-current-window-topmost-super-ultimate result:', result);
    return result;
  } catch (error) {
    console.error('❌ set-current-window-topmost-super-ultimate error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('start-topmost-monitor', async () => {
  console.log('📨 IPC: start-topmost-monitor called');
  try {
    const result = await nativeModule.startTopmostMonitor();
    console.log('✅ start-topmost-monitor result:', result);
    return result;
  } catch (error) {
    console.error('❌ start-topmost-monitor error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('stop-topmost-monitor', async () => {
  console.log('📨 IPC: stop-topmost-monitor called');
  try {
    const result = await nativeModule.stopTopmostMonitor();
    console.log('✅ stop-topmost-monitor result:', result);
    return result;
  } catch (error) {
    console.error('❌ stop-topmost-monitor error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('get-window-count', async () => {
  console.log('📨 IPC: get-window-count called');
  try {
    const result = await nativeModule.getWindowCount();
    console.log('✅ get-window-count result:', result);
    return result;
  } catch (error) {
    console.error('❌ get-window-count error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('block-word-lookup-popups', async () => {
  console.log('📨 IPC: block-word-lookup-popups called');
  try {
    const result = await nativeModule.blockWordLookupPopups();
    console.log('✅ block-word-lookup-popups result:', result);
    return result;
  } catch (error) {
    console.error('❌ block-word-lookup-popups error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('start-anti-popup-monitor', async () => {
  console.log('📨 IPC: start-anti-popup-monitor called');
  try {
    const result = await nativeModule.startAntiPopupMonitor();
    console.log('✅ start-anti-popup-monitor result:', result);
    return result;
  } catch (error) {
    console.error('❌ start-anti-popup-monitor error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('stop-anti-popup-monitor', async () => {
  console.log('📨 IPC: stop-anti-popup-monitor called');
  try {
    const result = await nativeModule.stopAntiPopupMonitor();
    console.log('✅ stop-anti-popup-monitor result:', result);
    return result;
  } catch (error) {
    console.error('❌ stop-anti-popup-monitor error:', error);
    return { error: error.message };
  }
});

ipcMain.handle('start-super-anti-popup-mode', async () => {
  console.log('📨 IPC: start-super-anti-popup-mode called');
  try {
    const result = await nativeModule.startSuperAntiPopupMode();
    console.log('✅ start-super-anti-popup-mode result:', result);
    return result;
  } catch (error) {
    console.error('❌ start-super-anti-popup-mode error:', error);
    return { error: error.message };
  }
});

console.log('✅ All IPC handlers registered successfully');

// 应用错误处理
process.on('uncaughtException', (error) => {
  console.error('❌ Uncaught Exception:', error);
});

process.on('unhandledRejection', (reason, promise) => {
  console.error('❌ Unhandled Rejection at:', promise, 'reason:', reason);
});