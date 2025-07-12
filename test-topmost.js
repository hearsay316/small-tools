const { setCurrentWindowTopmost } = require('./index.js');

console.log('测试将当前窗口设置为最顶层...');

try {
    const result = setCurrentWindowTopmost();
    
    if (result) {
        console.log('✅ 成功将当前窗口设置为最顶层！');
        console.log('窗口现在应该显示在所有其他窗口之上，包括划词软件。');
    } else {
        console.log('❌ 设置窗口为最顶层失败');
        console.log('可能的原因：');
        console.log('- 当前进程没有可见的主窗口');
        console.log('- 权限不足');
        console.log('- 系统限制');
    }
} catch (error) {
    console.error('❌ 调用过程中发生错误:', error.message);
}

console.log('\n提示：');
console.log('- 如果成功，当前终端窗口应该会置顶显示');
console.log('- 可以尝试打开其他应用程序验证层级效果');
console.log('- 要取消置顶效果，可以手动点击其他窗口或重启应用');