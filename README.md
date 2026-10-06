# DeepSeek Desktop

**简体中文** · [English](README.en.md)

一个以轻量化为重点的 DeepSeek 网页桌面客户端。使用 **Tauri 2 + Rust + 系统 WebView** 直接打开 [DeepSeek 官方对话网页](https://chat.deepseek.com/)，保留官方界面，不需要 API Key。

**macOS Apple Silicon 实测：应用本体 2.37 MiB，DMG 安装包 1.80 MiB。**

[![Build desktop packages](https://github.com/zcx960/deepseekchatdesktop/actions/workflows/build.yml/badge.svg)](https://github.com/zcx960/deepseekchatdesktop/actions/workflows/build.yml)

推送到 `main` 或手动运行 GitHub Actions 后，工作流会构建 Linux、Windows、macOS Intel 和 macOS Apple Silicon 安装包，并将产物保存为 Actions Artifacts。

这是社区制作的独立桌面封装，并非 DeepSeek 官方发布的客户端。

## 为什么轻量

| 设计 | 实现 |
| --- | --- |
| 复用系统浏览器引擎 | macOS 使用 WKWebView，Windows 使用 WebView2，Linux 使用 WebKitGTK |
| 精简运行时 | 不捆绑 Chromium 或 Node.js；Node.js 只用于开发和打包 |
| 精简前端 | 直接加载官方网页，没有本地前端框架、前端构建或开发服务器 |
| 精简后台 | 没有本地 AI 服务、后台轮询或系统托盘；关闭主窗口即退出 |
| 精简原生功能 | 使用系统菜单、快捷键与窗口管理，不额外构建一套界面 |
| 优化 Release 体积 | 启用体积优化、LTO、符号裁剪，并移除未使用的 Tauri 命令 |

体积记录来自 **2026-10-06** 的本机 macOS arm64 Release 构建。上述数字不包含系统 WebView、网页缓存或开发依赖；其他平台和后续版本的体积可能不同。

**安装体积不等于运行内存。** 网页内容由 WebView 渲染，实际内存仍取决于官方页面、对话长度和附件。本项目尚未提供完整的内存基准测试。

## 功能

- 使用 DeepSeek 官网账号登录，保留网页提供的对话、历史记录和附件入口。
- 原生菜单：新对话、刷新、前进/后退、缩放、全屏、主窗口置顶。
- 单实例运行：重复启动时聚焦已有窗口。
- 保存主窗口位置、大小和最大化状态；登录状态由 WebView 持久保存。
- 外部引用链接使用系统浏览器打开，官方页面和指定登录域名留在应用内。
- 支持原生复制、粘贴、文件选择，以及网页登录弹窗。

网页功能和更新由 DeepSeek 提供。登录状态与系统浏览器独立。

## 开发运行

需要 **Node.js 20+**、**Rust stable**，以及对应系统的 [Tauri 构建依赖](https://v2.tauri.app/start/prerequisites/)。

```sh
git clone https://github.com/zcx960/deepseekchatdesktop.git
cd deepseekchatdesktop
npm ci
npm run dev
```

启动后在官方页面自行登录，不需要配置 API Key 或本地模型。

macOS 脚本会优先使用已安装的独立 Command Line Tools，只为子进程设置 `DEVELOPER_DIR`，不修改系统的 `xcode-select`。已有 `DEVELOPER_DIR` 设置会被保留。

## 检查与打包

```sh
npm run check
npm run build
```

`npm run check` 执行 Rust 格式检查、测试及 Clippy 严格检查。macOS 只需要 `.app` 时：

```sh
npm run build:app
```

打包输出位于 `src-tauri/target/release/bundle/`。

| 平台 | 浏览器引擎 | 可生成的产物 | 验证状态 |
| --- | --- | --- | --- |
| macOS Apple Silicon | 系统 WKWebView | `.app`、`.dmg` | 已构建并实际启动 |
| Windows | 系统 WebView2 | 安装包 | 尚未实机验证 |
| Linux | WebKitGTK | AppImage、deb、rpm | 尚未实机验证 |

请在对应平台安装依赖并构建。Windows 使用 WebView2 下载引导器，不把完整浏览器运行时装进安装包。本地 macOS 构建没有 Apple Developer 签名和公证。

## 快捷键

macOS 使用 `⌘`，Windows / Linux 使用 `Ctrl`。

| 操作 | 快捷键 |
| --- | --- |
| 新对话 | `⌘/Ctrl + N` |
| 重新加载 | `⌘/Ctrl + R` |
| 在系统浏览器打开当前页面 | `⌘/Ctrl + Shift + B` |
| 后退 / 前进 | `⌘/Ctrl + [` / `⌘/Ctrl + ]` |
| 放大 / 缩小 | `⌘/Ctrl + =` / `⌘/Ctrl + -` |
| 重置缩放 | `⌘/Ctrl + 0` |
| 切换全屏 | `⌘/Ctrl + Shift + F` |
| 关闭窗口 / 退出应用 | `⌘/Ctrl + W` / `⌘/Ctrl + Q` |
| 复制 / 粘贴 / 全选 | 系统标准快捷键 |

“新对话”重新打开官方聊天首页。“主窗口置顶”在“显示”菜单中切换。关闭主窗口会退出整个应用；关闭弹出窗口只关闭该弹窗。

## 网页与本地权限

远程网页没有 Tauri capability，应用不注册自定义 Tauri 命令，也不暴露全局 Tauri API。桌面封装不另行采集或转发账号、对话和文件，这些内容由 DeepSeek 网页处理。

应用内允许 DeepSeek 的 HTTPS 页面，以及明确列出的 Apple、Google、微信登录域名；其他 HTTP(S) 链接交给系统浏览器。拒绝 `file:`、`javascript:` 等导航。所有弹窗使用相同的导航规则。

应用需要联网。无法加载时可重新加载，使用“对话 → 在浏览器中打开”，或查看“帮助 → 服务状态”。第三方登录提供商可能限制嵌入式浏览器，完整授权流程仍需实测。

## 已完成的验证

2026-10-06，在 macOS Apple Silicon 上：

- Rust 格式检查、3 个 URL 边界测试和 Clippy 严格检查通过。
- Release `.app` 和 DMG 构建成功，DMG 校验通过。
- 开发入口正常编译、启动。
- 成品实际显示了官方登录页和登录后的对话界面。
- 重复启动时第二个进程正常退出，原进程继续运行。

消息发送、附件上传、完整 Apple / Google 授权流程，以及 Windows / Linux 原生交互尚未完成验证。

## 项目结构

```text
scripts/                   开发、构建与检查命令
src-tauri/src/main.rs       WebView、弹窗、单实例与应用生命周期
src-tauri/src/menu.rs       原生菜单与快捷键
src-tauri/src/navigation.rs URL 边界与测试
src-tauri/tauri.conf.json   窗口、权限与打包设置
src-tauri/icons/            应用图标与来源说明
```

## 开源许可证

项目原创代码以 [MIT License](LICENSE) 开源。

DeepSeek 名称和鲸鱼图形归其权利人所有，不属于本项目 MIT 许可授权的原创代码；图标来源见 [说明](src-tauri/icons/README.md)。
