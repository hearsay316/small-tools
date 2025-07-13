const { contextBridge, ipcRenderer } = require('electron');

console.log('🔧 Preload script starting...');

// 错误处理
process.on('uncaughtException', (error) => {
  console.error('❌ Preload uncaught exception:', error);
});

process.on('unhandledRejection', (reason, promise) => {
  console.error('❌ Preload unhandled rejection:', reason);
});

try {
  // 暴露安全的 API 给渲染进程
  contextBridge.exposeInMainWorld('electronAPI', {
    // 窗口管理功能
    minimizeOtherWindows: () => {
      console.log('📞 Calling minimizeOtherWindows');
      return ipcRenderer.invoke('minimize-other-windows');
    },
    hideOtherWindows: () => {
      console.log('📞 Calling hideOtherWindows');
      return ipcRenderer.invoke('hide-other-windows');
    },
    setCurrentWindowTopmost: () => {
      console.log('📞 Calling setCurrentWindowTopmost');
      return ipcRenderer.invoke('set-current-window-topmost');
    },
    removeCurrentWindowTopmost: () => {
      console.log('📞 Calling removeCurrentWindowTopmost');
      return ipcRenderer.invoke('remove-current-window-topmost');
    },
    setCurrentWindowTopmostAggressive: () => {
      console.log('📞 Calling setCurrentWindowTopmostAggressive');
      return ipcRenderer.invoke('set-current-window-topmost-aggressive');
    },
    setCurrentWindowTopmostUltimate: () => {
      console.log('📞 Calling setCurrentWindowTopmostUltimate');
      return ipcRenderer.invoke('set-current-window-topmost-ultimate');
    },
    setCurrentWindowTopmostSuperUltimate: () => {
      console.log('📞 Calling setCurrentWindowTopmostSuperUltimate');
      return ipcRenderer.invoke('set-current-window-topmost-super-ultimate');
    },
    startTopmostMonitor: () => {
      console.log('📞 Calling startTopmostMonitor');
      return ipcRenderer.invoke('start-topmost-monitor');
    },
    stopTopmostMonitor: () => {
      console.log('📞 Calling stopTopmostMonitor');
      return ipcRenderer.invoke('stop-topmost-monitor');
    },
    getWindowCount: () => {
      console.log('📞 Calling getWindowCount');
      return ipcRenderer.invoke('get-window-count');
    },
    // 新增反划词弹框功能
    blockWordLookupPopups: () => {
      console.log('📞 Calling blockWordLookupPopups');
      return ipcRenderer.invoke('block-word-lookup-popups');
    },
    startAntiPopupMonitor: () => {
      console.log('📞 Calling startAntiPopupMonitor');
      return ipcRenderer.invoke('start-anti-popup-monitor');
    },
    stopAntiPopupMonitor: () => {
      console.log('📞 Calling stopAntiPopupMonitor');
      return ipcRenderer.invoke('stop-anti-popup-monitor');
    },
    startSuperAntiPopupMode: () => {
      console.log('📞 Calling startSuperAntiPopupMode');
      return ipcRenderer.invoke('start-super-anti-popup-mode');
    }
  });

  console.log('✅ Context bridge API exposed successfully!');
  
  // 验证 API 是否正确暴露
  setTimeout(() => {
    if (typeof window !== 'undefined' && window.electronAPI) {
      console.log('✅ electronAPI verified in window object');
      console.log('📋 Available methods:', Object.keys(window.electronAPI));
    } else {
      console.error('❌ electronAPI not found in window object');
    }
  }, 100);
  
} catch (error) {
  console.error('❌ Error in preload script:', error);
}

console.log('🏁 Preload script completed');