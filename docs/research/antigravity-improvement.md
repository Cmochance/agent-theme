# Antigravity 换肤伴侣 (agent-theme) 优化提升方案

本项目 `agent-theme` 是一款基于 Rust (Tauri v2) + Svelte 构建的无侵入式样式注入工具，其主要通过 Chrome DevTools Protocol (CDP) 调试端口向 Codex Desktop、Antigravity 等代理应用的 WebView 中动态注入 CSS 样式，以此实现磨砂玻璃与定制角色背景的视觉主题效果。

为了进一步拓宽项目的功能边界、提升运行效率并改善用户体验，本文在调研了市面上相似的主题定制及样式注入开源项目的基础上，提出了如下优化与提升方案。

---

## 一、 相似项目与竞品分析

在开源社区中，针对桌面混合应用（基于 Electron 或 CEF/Chromium 的客户端）进行主题定制与样式注入的项目主要有以下几类：

| 开源项目 | 注入原理 | 主题管理方式 | 主要特点与借鉴点 |
| :--- | :--- | :--- | :--- |
| **Spicetify-cli** | 通过 CLI 对 Spotify 客户端的 App 包进行二进制补丁（Patching），注入特定入口 JS。 | 自定义 `color.ini` (配置颜色变量) 与 `user.css`，支持高度模块化和变量化。 | 支持一键扩展（Extensions）和 App 插件；提供 `watch` 模式实时热重载 CSS 变更。 |
| **BetterDiscord / Vencord** | 拦截或挂载到 Discord 客户端的渲染进程/Webpack 模块中。 | 通过特定的 Themes 文件夹读取本地 CSS 文件，支持在线导入。 | 内置 Monaco Editor，提供实时的 CSS 自定义与热重载；通过系统窗口透明化实现原生的毛玻璃穿透效果。 |
| **BeautifulDiscord** | 修改客户端的启动 JS 脚本以加载外部 CSS 文件。 | 监控一个本地 CSS 文件。 | 极其轻量，专注于 CSS 注入，不提供 JS 插件功能，保持客户端的稳定和高性能。 |
| **vscode-background** | 修改 VS Code 的核心 CSS 文件以注入背景图。 | 在配置文件中指定本地图片路径并自动转为 base64。 | 专为 VS Code 内部结构定制，提供不透明度、毛玻璃程度和位置配置。 |

### 本项目的核心优势
- **无侵入性 (Non-invasive)**：不修改目标应用的二进制文件或安装包（不需要重包或 Patching），依靠标准的 CDP 协议进行“软注入”，升级主应用不会导致签名失效或文件损毁。
- **跨应用适配**：通过不同的 CSS 选择器与适配规则，同时支持 Codex、Antigravity 和 Linear 等多种基于 Chromium 的桌面客户端。

---

## 二、 当前架构局限与痛点

1. **Base64 图片注入带来的性能与内存压力**：
   目前 `agent-theme` 将背景图片转为 Base64 编码，再嵌入到 JS 中通过 CDP 的 `Runtime.evaluate` 传输并注入。当图片文件过大时，会导致 CDP 传输的 JS 字符串极其庞大，带来内存开销，甚至可能导致目标应用在注入瞬间卡死或渲染抖动。
2. **轮询机制（Polling）开销**：
   当前检测目标应用进程状态是通过高频定时轮询，这在一定程度上占用了 CPU 资源，并且从目标应用启动到 `agent-theme` 捕获到进程并完成注入存在明显的时延。
3. **局限于单平台**：
   `agent.rs` 目前硬编码了 macOS 的 `Application Support` 目录，对于 Windows (`%APPDATA%`) 和 Linux (`~/.config/`) 的用户无法开箱即用。
4. **缺乏实时调试与高阶配置**：
   目前只提供了静态预设主题和通过弹窗上传主题，用户在上传后难以微调 CSS 的细节（如毛玻璃模糊度、特定元素的文字颜色等），缺乏实时预览和修改的沙盒环境。

---

## 三、 核心优化与提升方案

### 1. 静态资源本地服务化 (Local Static Asset Server / Proxy)
- **问题背景**：Base64 字符串极大增加了 CDP 的传输负担。
- **方案设计**：
  - 在 `agent-theme` Rust 后端启动一个极轻量的、仅监听 `127.0.0.1` 环回地址上随机可用端口的 HTTP 静态服务（例如使用 Rust 的 `axum` 或 `warp`，或者直接利用 Tauri v2 内置的本地文件协议代理）。
  - 注入样式时，不再通过 Base64 编码，而是注入如下形式的 CSS：
    ```css
    body {
      background-image: url('http://127.0.0.1:随机端口/themes/changli/bg.jpg') !important;
    }
    ```
  - **收益**：注入的 JS 脚本将从数 MB 骤降到几百字节，显著提升 CDP 注入的速度，消除因大图注入导致的目标窗口卡顿。

### 2. 基于文件监听（File Watcher）的即时零延迟注入
- **问题背景**：定期轮询占用 CPU，且检测存在时间差。
- **方案设计**：
  - 使用 Rust 的 `notify` 库，监控主应用的数据存储目录（如 `~/Library/Application Support/Codex`）。
  - 目标是监控 `DevToolsActivePort` 文件的创建。当目标应用启动、该文件被写入的一瞬间，操作系统会立即触发文件变更事件，`agent-theme` 捕获该事件并提取端口，即可在毫秒级内发起 CDP 连接并注入样式。
  - **收益**：彻底废弃定时 Polling，降低 CPU 开销，并且让用户感知不到任何样式注入延迟（实现应用刚启动时界面就是换肤后的状态）。

### 3. 多平台支持与自适应路径查找 (Multi-Platform Support)
- **方案设计**：
  - 引入 `dirs` 库，并在 `agent.rs` 中利用 Rust 的 `#[cfg(target_os = "...")]` 宏针对 macOS、Windows、Linux 分别做路径匹配：
    - **macOS**: `~/Library/Application Support/<Agent>`
    - **Windows**: `%APPDATA%\<Agent>`
    - **Linux**: `~/.config/<Agent>`
  - 适配各自平台的进程名称（如 Windows 下为 `Codex.exe` 等）。
  - **收益**：让其他操作系统的开发者和用户也能无缝享受到换肤体验。

### 4. 自定义 CSS 实时沙盒与可视化微调 (Live CSS Editor & Fine-tuning)
- **方案设计**：
  - 在 Svelte 前端引入微型编辑器（如 `monaco-editor` 或是轻量级的 `code-jar`），并在 UI 上增设“实时调试”面板。
  - 提供可视化滑动条，供用户实时微调关键 CSS 变量，例如：
    - 背景图毛玻璃模糊度 (`backdrop-filter: blur(Npx)`)
    - 背景图片透明度/遮罩深浅 (`rgba(0,0,0, N)`)
    - 主题主色调及字体强调色 (`--cl-accent` / `--ink`)
  - 每次滑动滑块或修改编辑器内容，均通过 CDP 的 `Runtime.evaluate` 动态更新 WebView 中的自定义 `<style>` 标签。
  - **收益**：提供所见即所得的主题微调体验，释放极客用户自定义主题的创造力。

### 5. 主题配置标准化与社区/外部导入
- **方案设计**：
  - 规范化自定义主题格式，除了当前的 `bg.jpg`、`preview.jpg`、`theme.json` 之外，支持在 `theme.json` 中配置一个自定义的样式重载域。
  - 允许直接通过 URL（如 GitHub Raw 链接）直接导入第三方主题，或者提供“主题市场/广场”的简单展示与一键安装功能。
  - **收益**：丰富主题生态，降低用户分享和获取主题的成本。

---

## 四、 实施路线图推荐

### 第一阶段：性能与基础体验优化（高性价比）
1. **引入 `notify` 文件监听** 替代低效的轮询，降低功耗，达成瞬间注入效果。
2. **引入 `cfg` 适配 Windows 和 Linux 平台路径**，解锁多平台用户。
3. **规范化图片传输**，设计本地资源静态代理服务，干掉 Base64 注入方案。

### 第二阶段：自定义与易用性增强
1. 在 Svelte 端增加 **“样式调节滑块”**（毛玻璃模糊值、暗色遮罩比例）。
2. 提供 **“自定义 CSS 注入沙盒”**，允许玩家手动追加 CSS 补丁，并将其自动保存到用户配置的自定义主题 JSON 中。

### 第三阶段：生态与扩展性建设
1. 推出简易的 **“主题在线导入”** 功能，支持解析远程 URL 的主题压缩包。
2. 沉淀更好的适配器架构，方便开发者通过简单的选择器配置文件添加对其他 WebView 桌面程序（如 Notion, Slack 等）的主题注入支持。
*** End Patch
