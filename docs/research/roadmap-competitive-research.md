# Agent Theme 优化提升方案

> 基于同类产品与开源项目的竞品调研，为 agent-theme 制定的改进路线图。
> 调研时间：2026-06-13

---

## 一、竞品全景

### 1. 直接竞品：Codex Desktop 主题工具

| 项目 | 技术方案 | 特点 | 状态 |
|---|---|---|---|
| **[jstxn/codex-themes](https://github.com/jstxn/codex-themes)** | Shell 启动器 + Node.js 脚本注入 CSS | 轻量 CLI 工具，JSON 主题格式，自定义调色板 | ⚠️ 已归档，因 Codex 官方支持主题 |
| **[rzh0504/codex-themes-fix](https://github.com/rzh0504/codex-themes-fix)** | jstxn 的 fork，扩展 Windows 支持 | 修复了部分兼容性问题，跨平台 Shell/PowerShell 启动器 | 维护中 |
| **agent-theme（本项目）** | 独立 Tauri v2 桌面应用 + CDP 注入 | GUI 操作、背景图 + 磨砂玻璃、自定义裁剪、11 套内置主题 | 活跃开发中 |

**关键发现：** OpenAI 于 2026 年 3 月在 Codex Desktop 中内置了官方主题系统（`codex-theme-v1` JSON 格式），支持基础主题选择、每通道颜色控制、字体覆盖。这对所有第三方 Codex 主题工具构成了直接竞争压力。

### 2. 相邻领域：AI 聊天应用换肤工具

| 项目 | 目标应用 | 技术方案 | 亮点 |
|---|---|---|---|
| **[StylerGPT](https://stylergpt.com/)** | ChatGPT Web | Chrome 扩展 | 壁纸、字体、对比度、间距、代码块主题、Prompt 管理器、对话文件夹、本地导出；4.7/5 评分 |
| **[ChatGPT-Color-Tweak](https://github.com/LuisGot/ChatGPT-Color-Tweak)** | ChatGPT Web | WXT + TypeScript 浏览器扩展 | CSS 变量注入、实时预览、浏览器 storage 持久化 |
| **[custom-chatgpt-themes](https://github.com/sbukkap/custom-chatgpt-themes)** | ChatGPT Web | Chrome 扩展 (Manifest V3) | 预设主题 + 颜色拾取器自定义、chrome.storage.sync 跨设备同步 |
| **[Claude Desktop Custom Themes](https://github.com/patrickjaja/claude-desktop-bin/tree/master/themes)** | Claude Desktop (Electron) | 修改 app.asar + CSS 变量覆盖 | HSL 三元组变量体系、多窗口主题一致性 |
| **[TweakCC](https://claudelog.com/claude-code-mcps/tweakcc/)** | Claude Code (TUI) | React + Ink 终端交互 | 终端内主题创建器、实时预览 |

### 3. 同类模式参考：桌面应用 CSS 注入生态

| 项目 | 目标应用 | 模式 | 规模 |
|---|---|---|---|
| **[BetterDiscord](https://betterdiscord.app/)** | Discord | 客户端 Mod + CSS 主题 | 数千套社区主题，最成熟的桌面应用换肤生态 |
| **Vencord / Equicord** | Discord | BetterDiscord 替代品 | 跨平台、插件 + 主题库、透明窗口效果 |
| **[Custom CSS and JS Loader](https://marketplace.visualstudio.com/items?itemName=be5invis.vscode-custom-css)** | VS Code | 扩展注入 | VS Code 扩展商店最流行的外观定制方案 |
| **[Seelen UI](https://seelen.io/apps/seelen-ui)** | Windows 桌面 | Rust + Tauri | 任务栏替换、窗口平铺、动态壁纸、桌面小部件 |

---

## 二、核心挑战与机遇

### 威胁
1. **Codex 官方主题已上线** — 颜色/字体定制已被内置，第三方工具需要提供官方不覆盖的差异化价值
2. **竞品技术门槛低** — codex-themes 用一个 Shell 脚本就实现了基本换肤，用户迁移成本极低

### 机遇
1. **官方主题不支持背景图** — agent-theme 的角色壁纸 + 磨砂玻璃是独有的视觉卖点
2. **官方主题不支持多代理切换** — Codex + Antigravity 双代理支持是独有功能
3. **AI 应用多样化** — Claude Desktop、Cursor、Windsurf 等产品都在用 Electron，换肤需求广泛存在
4. **社区主题生态空白** — 目前没有任何 AI 聊天工具有 BetterDiscord 级别的主题社区

---

## 三、优化提升方案

### P0 — 生存线（解决核心竞争力问题）

#### 1. 定位升级：从「Codex 换肤工具」到「AI 桌面应用主题引擎」

**现状：** 仅支持 Codex Desktop 和 Antigravity 两个目标。

**改进：**
- 新增目标支持：Claude Desktop（Electron，修改 `app.asar` 的 CSS 变量）、Cursor / Windsurf（VS Code 系，可复用 CDP 注入逻辑）、ChatGPT Desktop（如有 Electron 版）
- 将「代理类型」选择器扩展为「应用目标」管理面板，每个目标独立配置注入策略
- 品牌叙事从「Codex 的皮肤伴侣」转向「AI 工作空间的主题引擎」

**参考：** BetterDiscord 的成功在于它不只是「Discord 皮肤」，而是整个 Discord 客户端的增强框架。

#### 2. 背景图动态效果

**现状：** 仅支持静态 `.jpg` 背景图。

**改进：**
- 支持 **视频背景**（`.mp4` / `.webm`），类似 Wallpaper Engine 的桌面动态壁纸
- 支持 **CSS 动画背景**（粒子效果、渐变流动、星空等），通过 CDP 注入 `@keyframes` 动画
- 新增「呼吸光晕」效果：根据聊天消息流动产生微妙的光效变化
- 背景图支持 **视差滚动**（parallax），鼠标移动时背景微移，增强空间感

**参考：** Seelen UI 的动态壁纸系统、Wallpaper Engine 的视频壁纸方案。

#### 3. 主题在线分享市场

**现状：** 11 套内置主题 + 用户自定义上传，无分享机制。

**改进：**
- 建立 **主题仓库**（GitHub Pages 或独立网站），类似 BetterDiscord 的 [themes.betterdiscord.app](https://betterdiscord.app/themes)
- 定义 `agent-theme-v1` 主题包格式（JSON 配置 + 预览图 + 背景资源 + CSS 附加样式）
- 应用内集成 **主题浏览器**，支持一键安装社区主题
- 提交机制：用户创建的主题可导出为主题包文件，提交到 GitHub 仓库的 PR
- 热门主题排行榜、标签分类（二次元、风景、极简、赛博朋克等）

**参考：** BetterDiscord 主题生态、Vencord 主题库 [themes.equicord.org](https://themes.equicord.org/)。

### P1 — 差异化（建立护城河）

#### 4. 可视化主题编辑器

**现状：** 自定义主题仅支持上传背景图 + 裁剪。

**改进：**
- 内置 **实时可视化编辑器**：拖拽颜色拾取器调整每个 CSS 变量，实时预览代理界面变化
- 编辑器面板分组：玻璃效果（模糊度、透明度、边框）、配色方案（文字、强调色、表面色）、背景控制（位置、缩放、滤镜）
- **吸色工具**：从背景图中自动提取主色调，一键生成匹配的配色方案
- **AI 生成主题**：输入文字描述（如「赛博朋克夜间城市」），AI 自动生成配色 + 推荐背景图
- 导出为 `agent-theme-v1` JSON，方便分享

**参考：** StylerGPT 的颜色拾取器、[bdeditor.dev](https://bdeditor.dev/)（BetterDiscord 主题在线编辑器）、ChatGPT-Color-Tweak 的实时 CSS 变量注入。

#### 5. 窗口透明与系统级视觉融合

**现状：** agent-theme 自身的窗口使用标准 Tauri 窗口外观。

**改进：**
- 使用 [tauri-apps/window-vibrancy](https://github.com/tauri-apps/window-vibrancy) 为伴侣应用自身窗口添加 **macOS vibrancy（NSVisualEffectView）** 效果
- 支持 **透明窗口模式**：伴侣面板半透明悬浮，与桌面壁纸自然融合
- 支持 **菜单栏模式**（Menu Bar Only）：伴侣缩小为菜单栏图标 + 下拉面板，减少桌面占用
- 圆角窗口 + 无边框设计，更像原生 macOS 工具

**参考：** window-vibrancy 对 macOS `NSVisualEffectView` 的封装、Vencord 的透明窗口主题。

#### 6. CDP 注入健壮性与智能重连

**现状：** 通过 `Page.addScriptToEvaluateOnNewDocument` 注入，代理重启后丢失。

**改进：**
- 实现 **CDP 连接池** 和 **指数退避重连**，避免端口漂移时注入失败
- **增量注入**：只在主题变更时重新注入，而非每次全量注入
- **注入状态可视化**：在 UI 中实时显示 CDP 连接状态、注入是否生效、上次注入时间
- **注入失败自动恢复**：当检测到页面刷新或导航时，自动重新注入当前主题
- 支持 `Page.addScriptToEvaluateOnNewDocument` + `Runtime.evaluate` 双通道，前者用于持久注入，后者用于即时样式更新（无需刷新即可生效）

**参考：** Claude Desktop 主题方案通过修改 `app.asar` 实现持久化（不会因重启丢失），这是 CDP 注入的天然弱点，需要更好的恢复机制来弥补。

### P2 — 生态扩展（规模化增长）

#### 7. 跨平台支持

**现状：** 仅支持 macOS，`agent.rs` 硬编码 macOS 路径和进程检测逻辑。

**改进：**
- **Windows 适配**：
  - 进程检测改用 `wmic` / `tasklist` 或 Windows API
  - 配置路径改用 `%APPDATA%\agent-theme\`
  - WebView2 的 CDP 端口发现逻辑（Edge WebView2 的 DevToolsActivePort 路径不同）
- **Linux 适配**：
  - 进程检测改用 `/proc` 或 `pgrep`
  - 配置路径改用 `~/.config/agent-theme/`
  - WebKitGTK 的调试端口发现
- 构建抽象层：将平台相关逻辑抽到 `platform/` 模块，使用 `#[cfg(target_os)]` 条件编译

**参考：** rzh0504/codex-themes-fix 已实现跨平台（macOS + Windows）的 Shell/PowerShell 启动器。

#### 8. 主题自动调度

**改进：**
- **时间调度**：白天自动切换到明亮主题，夜间切换到暗色主题
- **代理联动**：检测到当前代理状态变化时自动切换对应主题配置
- **快捷键**：全局热键快速切换主题或开关主题注入
- **情景模式**：为不同工作场景（编码、写作、休息）预设主题组合

#### 9. 分发与更新

**现状：** 通过 GitHub Releases 手动下载 `.dmg`，无自动更新。

**改进：**
- **Homebrew Cask**：`brew install --cask agent-theme`，方便 macOS 用户安装
- **Tauri 内置自动更新**：利用 Tauri v2 的 [updater plugin](https://v2.tauri.app/plugin/updater/)，配合 GitHub Releases 实现静默下载 + 一键安装
- **macOS 公证（Notarization）**：完成 Apple 公证流程，消除首次打开的 Gatekeeper 弹窗
- **Sparkle 框架集成**（macOS 原生更新方案）作为备选

**参考：** StylerGPT 通过 Chrome Web Store 分发、BetterDiscord 有独立安装器和自动更新。

### P3 — 锦上添花（体验打磨）

#### 10. 主题音效系统

- 为每个主题配置 **环境音效**（雨声、咖啡厅、白噪音等）
- 切换主题时同时切换音效氛围
- 音量控制和独立开关

#### 11. 智能配色推荐

- 上传背景图后，使用颜色提取算法（如 `color-thief`）自动推荐匹配的配色方案
- 提供 3-5 个风格预设（柔和、高对比、霓虹、复古等）

#### 12. 主题预览增强

- **3D 预览**：以透视角度展示主题在代理界面中的效果
- **预览视频**：为主题生成短视频展示动画效果
- **实时对比**：分屏对比两个主题的效果

#### 13. 本地主题配置同步

- 通过 iCloud / OneDrive 同步 `~/.codex/agent-theme/` 配置目录
- 支持导出/导入完整配置包（所有主题 + 偏好设置）

---

## 四、技术架构建议

### 当前架构优势
- Tauri v2 + Rust 后端 = 极小的二进制体积（~5MB vs Electron 的 ~150MB）
- CDP 注入方案安全可逆，不修改代理源码
- Svelte + Tailwind 前端 = 快速的 UI 响应

### 建议改进的技术栈

| 领域 | 当前 | 建议 | 理由 |
|---|---|---|---|
| 主题包格式 | 自定义 `theme.json` | 对齐 `codex-theme-v1` + 扩展字段 | 兼容官方格式，降低用户迁移成本 |
| CDP 库 | 手写 WebSocket | `chromiumoxide` crate | 更成熟的 CDP 客户端，自动重连、类型安全 |
| 配色提取 | 无 | `color-thief-rust` 或 `palette` crate | 自动从背景图提取主色调 |
| 窗口效果 | 标准窗口 | `window-vibrancy` | macOS vibrancy、Windows acrylic/mica |
| 自动更新 | 无 | Tauri updater plugin | 官方方案，安全签名验证 |
| 跨平台路径 | 硬编码 | `dirs` crate | 标准化各平台配置路径 |

### 代码质量改进

1. **错误处理**：`theme.rs`（70KB）是项目最大的文件，建议拆分为 `theme/discovery.rs`、`theme/css.rs`、`theme/custom.rs` 等子模块
2. **配置迁移**：当 `AppConfig` 结构变更时，实现版本化迁移逻辑，保证旧用户升级不丢配置
3. **测试覆盖**：当前无单元测试，建议为 CSS 生成、主题发现、配置读写添加测试
4. **CI 增强**：在 CI 中添加 `cargo test`、前端单元测试（Vitest）、以及 Tauri 构建产物的 smoke test

---

## 五、优先级排序建议

```
紧急度高 ─────────────────────────────────── 紧急度低

  P0-1 多应用支持    P0-3 主题市场    P1-5 窗口透明    P2-8 自动调度    P3-10 音效
  P0-2 动态背景      P1-4 编辑器      P1-6 CDP 健壮    P2-9 分发更新    P3-11 配色推荐
                                                            P2-7 跨平台      P3-12 预览增强
                                                                               P3-13 配置同步
```

**建议执行顺序：**
1. **短期（1-2 周）**：P1-6 CDP 健壮性改进（提升现有体验）、P3-11 智能配色推荐（低成本高感知）
2. **中期（1-2 月）**：P0-2 动态背景、P1-4 可视化编辑器、P0-3 主题市场的基础架构
3. **长期（3-6 月）**：P0-1 多应用支持、P2-7 跨平台、P2-9 分发与自动更新

---

## 六、总结

agent-theme 在 AI 桌面应用换肤领域目前是 **唯一的 Tauri 原生 GUI 方案**，与 codex-themes 的 CLI 方案和各种浏览器扩展方案形成了明显的技术差异化。面对 Codex 官方主题系统的上线，核心应对策略是：

1. **做官方做不到的事**：角色壁纸、磨砂玻璃、动态背景、多代理切换 —— 这些是官方不太会做的重视觉功能
2. **扩大目标范围**：从 Codex 专属工具升级为 AI 工作空间主题引擎，覆盖更多 AI 应用
3. **建立社区飞轮**：主题市场 + 社区创作 = 用户粘性 + 内容壁垒
4. **打磨体验细节**：自动配色、编辑器、音效、自动调度 —— 让「换肤」从功能变成享受
