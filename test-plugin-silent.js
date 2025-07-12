// 无控制台窗口的测试脚本
const { spawn } = require('child_process');
const path = require('path');

// 创建一个隐藏控制台窗口的 Node.js 进程
function runSilentTest() {
    const scriptPath = path.join(__dirname, 'test-plugin.js');
    
    const child = spawn('node', [scriptPath], {
        windowsHide: true,  // 隐藏控制台窗口
        stdio: 'pipe'       // 重定向输出
    });
    
    let output = '';
    
    child.stdout.on('data', (data) => {
        output += data.toString();
    });
    
    child.stderr.on('data', (data) => {
        output += data.toString();
    });
    
    child.on('close', (code) => {
        console.log('测试完成，退出码:', code);
        console.log('输出:', output);
    });
    
    child.on('error', (err) => {
        console.error('执行错误:', err);
    });
}

// 如果直接运行此脚本
if (require.main === module) {
    console.log('🤫 正在后台静默运行窗口管理插件测试...');
    runSilentTest();
}

module.exports = { runSilentTest };