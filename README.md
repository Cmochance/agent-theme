# Agent Theme

> [!NOTE]
> 🎨 **Agent Theme** 是 Codex Desktop / Antigravity 的独立换肤伴侣应用。
> 通过 Chrome DevTools Protocol 向代理的 WebView 注入自定义 CSS，实现磨砂玻璃 + 角色背景的视觉主题效果。
> 内置 5 套精美主题，支持自定义上传裁剪，无需修改代理源码即可一键换肤。

<p align="center">
  <a href="README.md">简体中文</a> |
  <a href="README.en.md">English</a>
</p>

<p align="center">
  <a href="https://github.com/Cmochance/agent-theme/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/Cmochance/agent-theme?style=social"></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/github/license/Cmochance/agent-theme"></a>
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-1.77%2B-orange?logo=rust"></a>
  <a href="https://v2.tauri.app/"><img alt="Tauri" src="https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri"></a>
  <a href="#"><img alt="Platform" src="https://img.shields.io/badge/macOS-000000?logo=apple"></a>
</p>

## 目录

- [简介](#简介)
- [功能特性](#功能特性)
- [内置主题预览](#内置主题预览)
- [快速开始](#快速开始)
- [主题管理](#主题管理)
- [技术架构](#技术架构)
- [开发指南](#开发指南)
- [常见问题](#常见问题)
- [许可证](#许可证)

## 简介

Agent Theme 是一个独立的桌面应用（Tauri v2），它与 Codex Desktop 或 Antigravity 代理配合使用，通过 Chrome DevTools Protocol (CDP) 动态注入 CSS 样式，在不修改代理源码的情况下实现个性化主题。

**原理：** 代理启动时附带 `--remote-debugging-port` 参数暴露 CDP 端口。Agent Theme 通过 WebSocket 连接到该端口，使用 `Page.addScriptToEvaluateOnNewDocument` 注入 JavaScript，在代理页面加载时自动插入背景图和半透明 CSS overlay，实现磨砂玻璃视觉效果。

## 功能特性

- 🎨 **5 套内置主题：** 长离 (Changli)、奈琳 (Nailin)、扎妮 (Zani)、碧蓝航线 (Azur Lane)、纸箱 (Carton)，一键切换
- 🖼️ **自定义主题：** 支持拖拽上传图片、裁剪、保存为自定义主题
- 🔄 **多代理支持：** 同时支持 Codex Desktop 和 Antigravity 两种代理，可自由切换
- 🚀 **受限重启：** 可重启当前选择的 Codex 或 Antigravity，并附加本地调试端口参数
- 🔌 **CDP 注入：** 通过 Chrome DevTools Protocol 注入主题，不修改代理源码，安全可逆
- 📊 **实时状态：** 界面实时显示代理运行状态和 CDP 端口绑定情况
- 💾 **配置持久化：** 所有设置保存到 `~/.codex/agent-theme/config.json`，重启不丢失
- 🔒 **单实例保护：** 防止同时运行多个伴侣窗口导致冲突

## 内置主题预览

| 主题 ID | 中文名 | 英文名 | 预览 |
|---------|--------|--------|------|
| `changli` | 长离 | Changli | ![Changli](themes/changli/preview.jpg) |
| `nailin` | 奈琳 | Nailin | ![Nailin](themes/nailin/preview.jpg) |
| `zani` | 扎妮 | Zani | ![Zani](themes/zani/preview.jpg) |
| `azurlane` | 碧蓝航线 | Azur Lane | ![Azur Lane](themes/azurlane/preview.jpg) |
| `carton` | 纸箱 | Carton | ![Carton](themes/carton/preview.jpg) |

## 快速开始

### 前置条件

- **macOS**（当前仅支持 macOS，Windows/Linux 适配计划中）
- **Codex Desktop** 或 **Antigravity** 已安装

### 安装与运行

1. 从 [Releases](https://github.com/Cmochance/agent-theme/releases) 下载最新版本的 `.dmg` 文件
2. 将 `Agent Theme Companion.app` 拖入 Applications 文件夹
3. 首次打开时，macOS Gatekeeper 可能拦截：右键点击 App → 选择「打开」；或在 `系统设置 → 隐私与安全性` 中点击「仍要打开」
4. 启动 Agent Theme Companion，界面将显示代理运行状态

### 基础使用

1. **选择代理：** 在顶部切换栏选择 Codex 或 Antigravity
2. **准备调试端口：** 如果界面显示 `No debug port`，点击 `Restart App`，伴侣会重启当前代理并附加本地调试端口参数
3. **选择主题：** 在主题网格中点击任意主题卡片即可预览并应用
4. **切换开关：** 「主题开关」控制是否注入主题样式，关闭后恢复代理原始界面

### 本地调试端口要求

如果代理已经在运行但未开启 CDP 端口，Agent Theme 无法注入主题。点击 `Restart App` 后，本应用会结束当前选择的 Codex 或 Antigravity 进程，并用 `--remote-debugging-port=0` 重新启动它。

本应用的进程管理范围仅限已支持的两个代理应用，不会清理代理应用数据目录下的锁文件，也不会修改代理应用包内容。端口可用后，保持「主题开关」开启并点击主题卡片即可注入。

## 主题管理

### 使用内置主题

内置主题存放在应用的 `themes/` 资源目录中，无需额外配置。选择主题后即时生效。

### 创建自定义主题

1. 点击主题网格中的「+」卡片
2. 拖拽图片到上传区域（支持 JPG/PNG，最大 20MB）
3. 使用裁剪工具调整背景区域
4. 点击「保存并应用」，主题将保存到 `~/.codex/agent-theme/themes/` 目录

### 删除自定义主题

在自定义主题卡片上悬停，点击删除按钮。内置主题不可删除。

### 主题文件结构

```json
{
  "id": "changli",
  "displayName": { "zh": "长离 (Changli)", "en": "Changli" },
  "background": "bg.jpg",
  "preview": "preview.jpg",
  "backgroundFit": "cover",
  "backgroundPosition": "center top"
}
```

## 技术架构

```
agent-theme/
├── src-tauri/          # Rust 后端 (Tauri v2)
│   ├── src/
│   │   ├── main.rs     # 应用入口
│   │   ├── lib.rs      # Tauri commands 注册、生命周期
│   │   ├── agent.rs    # 代理进程检测、启动、管理
│   │   ├── cdp.rs      # CDP WebSocket 连接、主题注入/清除
│   │   ├── config.rs   # 配置读写 (AppConfig)
│   │   └── theme.rs    # 主题发现、CSS 生成、自定义主题 CRUD
│   └── Cargo.toml
├── web/                # 前端 (Vanilla JS + esbuild)
│   ├── app.js          # 主逻辑 (状态轮询、主题切换、裁剪 UI)
│   ├── index.html      # 页面结构
│   ├── style.css       # 暗红金玻璃拟态设计系统
│   └── dist/           # 构建产物 (bundle.js)
└── themes/             # 内置主题资源
    ├── changli/        # 长离
    ├── nailin/         # 奈琳
    ├── zani/           # 扎妮
    ├── azurlane/       # 碧蓝航线
    └── carton/         # 纸箱
```

**核心技术：**

- **后端：** Rust + Tauri 2.11.2，使用 `tokio-tungstenite` 进行 CDP WebSocket 通信，`sysinfo` 检测进程状态，`reqwest` 进行 HTTP 探测
- **前端：** 原生 JavaScript + esbuild 打包，通过 `@tauri-apps/api` 调用 Tauri commands
- **注入原理：** 通过 CDP `Page.addScriptToEvaluateOnNewDocument` 在代理页面加载前注入 JavaScript，动态创建 `<style>` 标签和 DOM overlay，实现背景图 + 半透明磨砂效果，保存 `identifier` 以便后续通过 `Page.removeScriptToEvaluateOnNewDocument` 清除

## 开发指南

### 环境要求

- Rust 1.77.2+
- Node.js 20+
- macOS（当前仅 macOS 可完整运行和调试）

### 本地构建

```bash
git clone https://github.com/Cmochance/agent-theme.git
cd agent-theme

cd web
npm install
npm run build
cd ..

cargo install tauri-cli --version "^2"
cargo tauri dev
```

### CI 检查

项目配置了 GitHub Actions CI，包含：
- **Rust Checks:** `cargo fmt --check`、`cargo clippy -D warnings`、`cargo check`
- **Frontend Build:** esbuild 打包验证

### 调试技巧

- 代理的 CDP 端口写入 `~/Library/Application Support/<Agent>/DevToolsActivePort`
- 可通过 `http://127.0.0.1:<port>/json/list` 查看可调试的 WebView 目标列表
- Rust 日志通过 `tauri-plugin-log` 输出到控制台，可在终端运行 `cargo tauri dev` 查看

## 常见问题

### Q: 主题注入后不生效？

检查界面上的 CDP 端口状态。如果显示 `No debug port`，点击 `Restart App` 让当前代理以本地远程调试模式重启。端口可用但仍失败时，界面会显示后端返回的错误信息。

### Q: 更换主题后代理界面没有更新？

主题通过 `Page.addScriptToEvaluateOnNewDocument` 注入，仅在页面**导航或刷新**时生效。切换主题后可以尝试切换代理的标签页或刷新页面。

### Q: 代理崩溃或重启后主题丢失？

CDP 注入的主题在代理重启后会丢失。Agent Theme 会在检测到可用本地调试端口后自动重新注入，保持「主题开关」开启即可自动恢复。

### Q: 支持 Windows / Linux 吗？

当前仅支持 macOS。`agent.rs` 中代理路径和进程检测逻辑依赖 macOS 特定路径（`~/Library/Application Support/`），跨平台适配在计划中。

### Q: 会修改代理的源码或配置文件吗？

不会。主题注入完全通过 CDP 运行时完成，不修改任何代理文件。清除注入后代理恢复原始外观。

## 许可证

MIT License。完整文本见 [LICENSE](LICENSE)。
