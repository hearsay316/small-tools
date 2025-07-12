const { setCurrentWindowTopmost, removeCurrentWindowTopmost } = require('./index');
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

console.log('=== 窗口置顶功能测试程序 ===');
console.log('注意：此程序在命令行环境中可能无法直接测试窗口置顶功能');
console.log('建议使用 Electron 应用进行实际测试\n');

function showMenu() {
  console.log('\n请选择操作：');
  console.log('1. 测试设置窗口置顶');
  console.log('2. 测试取消窗口置顶');
  console.log('3. 退出程序');
  console.log('\n提示：在 Electron 应用中测试效果更佳');
  
  rl.question('请输入选项 (1-3): ', (answer) => {
    switch(answer.trim()) {
      case '1':
        testSetTopmost();
        break;
      case '2':
        testRemoveTopmost();
        break;
      case '3':
        console.log('程序退出');
        rl.close();
        break;
      default:
        console.log('无效选项，请重新选择');
        showMenu();
        break;
    }
  });
}

function testSetTopmost() {
  console.log('\n正在测试设置窗口置顶...');
  try {
    const result = setCurrentWindowTopmost();
    if (result) {
      console.log('✅ 设置窗口置顶成功！');
      console.log('   如果在有窗口的环境中，当前窗口应该会置于最顶层');
    } else {
      console.log('❌ 设置窗口置顶失败');
      console.log('   可能原因：');
      console.log('   - 当前进程没有可见窗口');
      console.log('   - 权限不足');
      console.log('   - 系统限制');
    }
  } catch (error) {
    console.log('❌ 发生错误:', error.message);
  }
  showMenu();
}

function testRemoveTopmost() {
  console.log('\n正在测试取消窗口置顶...');
  try {
    const result = removeCurrentWindowTopmost();
    if (result) {
      console.log('✅ 取消窗口置顶成功！');
      console.log('   窗口应该恢复到正常层级');
    } else {
      console.log('❌ 取消窗口置顶失败');
      console.log('   可能原因：');
      console.log('   - 当前进程没有可见窗口');
      console.log('   - 窗口本来就不是置顶状态');
      console.log('   - 权限不足');
    }
  } catch (error) {
    console.log('❌ 发生错误:', error.message);
  }
  showMenu();
}

console.log('\n🔧 功能说明：');
console.log('- 设置窗口置顶：将当前窗口设置为最高层级，高于划词软件');
console.log('- 取消窗口置顶：将窗口恢复到正常层级');
console.log('\n💡 最佳测试方式：');
console.log('1. 运行 Electron 应用：npx electron example.js');
console.log('2. 在应用中点击"设置窗口置顶"按钮');
console.log('3. 打开划词软件测试是否能覆盖应用窗口');
console.log('4. 使用"取消窗口置顶"按钮恢复正常状态');

showMenu();