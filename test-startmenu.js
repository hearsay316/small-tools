const { hideOtherWindows, showOtherWindows } = require('./index.js');

console.log('=== 增强版开始菜单保护测试 ===');
console.log('当前时间:', new Date().toLocaleTimeString());
console.log('\n本次更新包含以下改进：');
console.log('✓ 增强的窗口类型检测');
console.log('✓ 更安全的系统窗口过滤');
console.log('✓ 跳过工具窗口和系统UI窗口');
console.log('✓ 基于窗口类名的智能过滤');

console.log('\n=== 测试步骤 ===');
console.log('1. 隐藏其他应用程序窗口（保护所有系统UI）');
console.log('2. 测试以下系统功能是否正常：');
console.log('   - 开始菜单（点击或按Win键）');
console.log('   - 桌面右键菜单');
console.log('   - 任务栏功能');
console.log('   - 系统托盘');
console.log('   - Alt+Tab 切换');
console.log('   - Ctrl+Alt+Del 安全界面');
console.log('3. 15秒后自动恢复所有窗口');

console.log('\n开始测试...');

// 隐藏其他窗口
console.log('\n[步骤1] 使用增强过滤算法隐藏其他窗口...');
hideOtherWindows();
console.log('✓ 其他应用程序窗口已隐藏（系统UI受保护）');

console.log('\n[重要] 现在请立即测试以下功能：');
console.log('🖱️  点击开始菜单按钮');
console.log('⌨️  按 Win 键打开开始菜单');
console.log('🖱️  右键点击桌面');
console.log('🖱️  点击任务栏上的应用图标');
console.log('🖱️  点击系统托盘图标');
console.log('⌨️  按 Alt+Tab 切换窗口');
console.log('⌨️  按 Ctrl+Shift+Esc 打开任务管理器');

let countdown = 15;
const timer = setInterval(() => {
    if (countdown > 0) {
        console.log(`\n⏰ 还有 ${countdown} 秒自动恢复窗口...`);
        if (countdown === 10) {
            console.log('\n💡 提示：如果开始菜单仍无法使用，可能需要：');
            console.log('   1. 重启 Windows Explorer (Ctrl+Shift+Esc -> 详细信息 -> explorer.exe -> 结束任务 -> 文件 -> 运行新任务 -> explorer.exe)');
            console.log('   2. 重启计算机');
            console.log('   3. 检查是否有其他安全软件干扰');
        }
        countdown--;
    } else {
        clearInterval(timer);
        console.log('\n[步骤2] 恢复所有隐藏的窗口...');
        showOtherWindows();
        console.log('✓ 所有窗口已恢复显示');
        
        console.log('\n=== 测试结果评估 ===');
        console.log('请根据测试期间的体验评估修复效果：');
        console.log('\n✅ 成功：开始菜单和所有系统功能正常工作');
        console.log('❌ 失败：开始菜单或其他系统功能仍有问题');
        
        console.log('\n如果问题仍然存在，建议：');
        console.log('1. 重启系统以完全重置窗口状态');
        console.log('2. 检查是否有杀毒软件或安全工具干扰');
        console.log('3. 在管理员权限下运行测试');
        console.log('4. 考虑使用更保守的窗口操作策略');
        
        console.log('\n测试完成！');
    }
}, 1000);

console.log('\n📋 技术改进说明：');
console.log('- 新增窗口类名检测，跳过 Shell_*, Progman, WorkerW 等系统窗口类');
console.log('- 新增扩展样式检测，跳过工具窗口 (WS_EX_TOOLWINDOW)');
console.log('- 保护 Windows.UI.Core.CoreWindow 等现代UI窗口');
console.log('- 更严格的系统进程保护列表');