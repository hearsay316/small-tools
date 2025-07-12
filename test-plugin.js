// 简单的测试脚本，用于验证插件功能
const { minimizeOtherWindows, hideOtherWindows } = require('./index');

console.log('🧪 开始测试窗口管理插件...');

try {
    console.log('📋 当前平台:', process.platform);
    console.log('🏗️  当前架构:', process.arch);
    
    if (process.platform !== 'win32') {
        console.log('⚠️  警告: 此插件主要为 Windows 平台设计');
        console.log('   在其他平台上可能无法正常工作');
    }
    
    console.log('\n🔍 正在测试 minimizeOtherWindows 函数...');
    
    // 测试函数是否存在
    if (typeof minimizeOtherWindows === 'function') {
        console.log('✅ 函数导入成功');
        
        // 在 Windows 平台上执行测试
        if (process.platform === 'win32') {
            console.log('🪟 正在执行最小化操作...');
            minimizeOtherWindows();
            console.log('✅ 最小化操作执行完成');
            console.log('   (请检查其他应用窗口是否已被最小化)');
            
            // 等待一下再测试隐藏功能
            console.log('\n⏳ 等待 3 秒后测试隐藏功能...');
            setTimeout(() => {
                console.log('🔍 正在测试 hideOtherWindows 函数...');
                if (typeof hideOtherWindows === 'function') {
                    console.log('🫥 正在执行隐藏操作...');
                    hideOtherWindows();
                    console.log('✅ 隐藏操作执行完成');
                    console.log('   (请检查其他窗口是否已被隐藏，不在任务栏显示)');
                } else {
                    console.log('❌ hideOtherWindows 函数导入失败');
                }
                console.log('\n🎉 所有测试完成!');
            }, 3000);
        } else {
            console.log('⏭️  跳过执行测试 (非 Windows 平台)');
            console.log('\n🎉 测试完成!');
        }
    } else {
        console.log('❌ 函数导入失败');
        console.log('   类型:', typeof minimizeOtherWindows);
        console.log('\n🎉 测试完成!');
    }
    
} catch (error) {
    console.error('❌ 测试过程中发生错误:');
    console.error('   错误信息:', error.message);
    console.error('   错误堆栈:', error.stack);
    process.exit(1);
}