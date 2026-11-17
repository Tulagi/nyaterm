# 沉浸全屏 Windows x64 便携包

Windows / Linux 默认使用 `Ctrl+F11`，macOS 使用 `Ctrl+Cmd+F11`。
进入时隐藏原生窗口边框、任务栏、应用标题菜单栏、左右活动栏和侧栏、底部面板，
保留会话标签和终端分屏。再次使用相同快捷键恢复进入前的窗口全屏状态和面板布局。
沉浸模式中的 `F11` 回到普通窗口，`Esc` 仍交给终端。
主动打开侧栏、同步输入组、AI 聊天或命令面板的快捷键会先退出沉浸模式。
快捷键可以在设置中修改，沉浸状态不写入用户配置。

## 云端构建

工作流：`.github/workflows/windows-x64-portable.yml`。
向 `codex/immersive-fullscreen` 分支推送提交会自动构建；工作流也提供
`workflow_dispatch`，在它进入仓库默认分支后可以从 Actions 页面手动运行。

1. Ubuntu runner 检查 Rust 格式、全屏状态转换和打包脚本。
2. Windows 2025 runner 安装仓库锁定的 Rust、MSVC/ATL、LLVM、MSYS2 和 NSIS。
3. 复用上游脚本准备 ConPTY、ADB、scrcpy 和 FFmpeg，编译主程序及辅助程序。
4. 运行全屏状态与快捷键测试，校验包内 PE 架构、依赖资源及便携更新兼容性。
5. 上传 `windows_x64_portable` artifact，保留 30 天。

本机不需要安装 Rust、Visual Studio、MSVC、ATL、LLVM 或 Windows SDK。
系统 RDP 类型库由 Windows runner 提供。Debian 无法直接运行当前项目要求的
Windows Native RDP / ATL 构建步骤，因此最终包使用 Windows runner 编译。

artifact 内包含：

- `NyaTerm_<版本>-immersive.<提交前12位>_windows_x64_portable.zip`
- `SHA256SUMS.txt`
- `BUILD-INFO.txt`（源码提交和构建链接）

下载 Actions artifact 后先解开外层 ZIP，再解开 NyaTerm 便携 ZIP，启动其中的
`NyaTerm.exe`。首次建议使用独立目录验证 `Ctrl+F11`，包括退出后面板尺寸、
分屏、任务栏和原先 F11 状态是否恢复。

工作流沿用上游打包契约，会临时生成 NSIS 安装包用于验证，仅上传便携包。
文件名使用自定义预发布标识，因此不构建 MSI，也不会发布 GitHub Release。
GitHub Gist OAuth 客户端 ID 可通过仓库变量 `NYATERM_GITHUB_GIST_CLIENT_ID` 提供；
未配置时不影响终端与沉浸全屏功能。
