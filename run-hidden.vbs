' VBScript 文件用于完全隐藏控制台窗口运行 Node.js 脚本
Set objShell = CreateObject("WScript.Shell")
Set objFSO = CreateObject("Scripting.FileSystemObject")

' 获取当前脚本所在目录
strScriptPath = objFSO.GetParentFolderName(WScript.ScriptFullName)

' 构建 Node.js 命令
strCommand = "node """ & strScriptPath & "\test-plugin.js"""

' 运行命令，窗口隐藏 (0 = 隐藏窗口)
objShell.Run strCommand, 0, False

' 显示完成消息（可选）
' WScript.Echo "窗口管理插件已在后台运行"