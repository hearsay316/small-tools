// 测试修复后的插件，验证系统进程保护
const { minimizeOtherWindows, hideOtherWindows } = require('./index');

console.log('🔧 测试修复后的窗口管理插件...');
console.log('📋 当前平台:', process.platform);

if (process.platform !== 'win32') {
    console.log('⚠️  警告: 此插件仅在 Windows 平台上工作');
    process.exit(0);
}

console.log('\n🛡️  此版本已添加系统进程保护:');
console.log('   - explorer.exe (Windows 资源管理器)');
console.log('   - dwm.exe (桌面窗口管理器)');
console.log('   - 其他系统关键进程');

console.log('\n🧪 开始测试最小化功能...');
console.log('👀 请观察:');
console.log('   1. 其他应用窗口应该被最小化');
console.log('   2. 桌面右键菜单应该正常工作');
console.log('   3. 任务栏应该正常响应');

try {
    console.log('\n🪟 执行最小化操作...');
    minimizeOtherWindows();
    console.log('✅ 最小化操作完成');
    
    console.log('\n🔍 请测试以下功能是否正常:');
    console.log('   - 桌面右键菜单');
    console.log('   - 任务栏右键菜单');
    console.log('   - 开始菜单');
    console.log('   - Alt+Tab 切换窗口');
    
    
    
} catch (error) {
    console.error('❌ 测试失败:', error.message);
    process.exit(1);
}