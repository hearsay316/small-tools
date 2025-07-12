const { hideOtherWindows, showOtherWindows } = require('./index.js');

console.log('测试隐藏和恢复窗口功能...');
console.log('当前时间:', new Date().toLocaleTimeString());

// 隐藏其他窗口
console.log('\n1. 隐藏其他窗口...');
hideOtherWindows();
console.log('✓ 其他窗口已隐藏');

// 等待 5 秒
console.log('\n等待 5 秒后恢复窗口...');
setTimeout(() => {
    console.log('\n2. 恢复隐藏的窗口...');
    showOtherWindows();
    console.log('✓ 窗口已恢复显示');
    
    console.log('\n测试完成！');
    console.log('请检查其他应用程序窗口是否已正常恢复显示。');
}, 5000);

console.log('\n注意：');
console.log('- 在 5 秒内，其他应用程序窗口应该被隐藏');
console.log('- 5 秒后，窗口应该自动恢复显示');
console.log('- 系统关键进程（如桌面、任务栏）不会受到影响');