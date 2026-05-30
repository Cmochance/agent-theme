# Agent 工作规范 — agent-theme

## 项目概述

agent-theme 是一个独立的 Tauri v2 桌面应用，作为 Codex Desktop / Antigravity 的换肤伴侣。通过 Chrome DevTools Protocol (CDP) 向代理的 WebView 注入自定义 CSS，实现磨砂玻璃 + 角色背景的视觉主题效果。

- 仓库: Cmochance/agent-theme
- 技术栈: Rust (Tauri 2.11) + Vanilla JS (esbuild)
- 平台: macOS 首发
- 许可证: MIT

## 行为规则

1. 仅当用户明确要求时才记录内容到本文档，不得自行添加。
2. 上下文不确定时必须向用户确认，禁止猜测执行。
3. 保持最小改动原则：复用现有结构和模块，不引入不必要的抽象或重构。
4. 不自行 commit/push：除非用户在当前对话中明确要求提交，否则所有改动保持在未提交状态供用户审查。
5. 敏感信息不输出：API key、token、密码等绝不打印到对话中。
6. 使用中文回复：所有对话内容以中文输出，代码注释和标识符保持英文。

## 项目结构

```
agent-theme/
├── src-tauri/              # Rust 后端 (Tauri v2)
│   ├── src/
│   │   ├── main.rs         # 应用入口 (6 行)
│   │   ├── lib.rs          # Tauri commands 注册、生命周期、状态管理
│   │   ├── agent.rs        # 代理进程检测、启动、管理
│   │   ├── cdp.rs          # CDP WebSocket 连接、主题注入/清除
│   │   ├── config.rs       # 配置读写 (AppConfig)
│   │   └── theme.rs        # 主题发现、CSS 生成、自定义主题 CRUD
│   └── Cargo.toml
├── web/                    # 前端 (Vanilla JS + esbuild)
│   ├── app.js              # 主逻辑 (状态轮询、主题切换、裁剪 UI)
│   ├── index.html          # 页面结构
│   ├── style.css           # 暗红金玻璃拟态设计系统
│   └── dist/               # 构建产物 (bundle.js)
├── themes/                 # 内置主题资源 (每套含 bg.jpg + preview.jpg + theme.json)
│   ├── changli/            # 长离
│   ├── nailin/             # 奈琳
│   ├── zani/               # 扎妮
│   ├── azurlane/           # 碧蓝航线
│   └── carton/             # 纸箱
├── README.md               # 中文主文档
└── README.en.md            # 英文文档
```

## 开发工作流

### 本地构建

```bash
cd web && npm install && npm run build && cd ..
cargo tauri dev
```

### CI 检查

CI (.github/workflows/ci.yml) 在 PR 和 push main 时运行：
- Rust: cargo fmt --check、cargo clippy -- -D warnings、cargo check
- 前端: esbuild 打包验证

### 提交与 PR

- 主分支: main，直接在 main 上开发（项目规模小，不使用 worktree 分支策略）
- 分支命名: 使用 codex/ 前缀或描述性分支名
- README 同步: 影响用户可见行为或新增功能时，同一 PR 内同步更新 README.md 和 README.en.md

## 关键约束

### 主题开发

- 每套内置主题必须包含三个文件: bg.jpg（全分辨率背景）、preview.jpg（预览缩略图）、theme.json（元数据配置）
- theme.json 格式固定，包含 id、displayName（中英双语）、background、preview、backgroundFit、backgroundPosition
- 新增主题需同步更新 theme.rs 中的主题发现逻辑

### CDP 注入

- 注入通过 Page.addScriptToEvaluateOnNewDocument 实现，仅在页面导航或刷新时生效
- 注入后保存 identifier 以便后续 Page.removeScriptToEvaluateOnNewDocument 清除
- 代理重启后注入会丢失，需依赖代理启动检测自动重新注入

### 平台限制

- 当前 agent.rs 硬编码 macOS 路径 (~/Library/Application Support/)，跨平台改动需同时处理进程检测和路径逻辑
- tauri.conf.json 中 icons 目录指向 src-tauri/icons/，路径不可随意更改

### 配置持久化

- AppConfig 通过原子写入保存在 ~/.codex/agent-theme/config.json
- 自定义主题保存在 ~/.codex/agent-theme/themes/
- 修改 config 结构体需要向后兼容，或在启动时做迁移处理

## 文档约定

- README 双语: README.md（中文主文档）+ README.en.md（英文），结构完全对应
- Badge 引用指向 Cmochance/agent-theme
- 新功能或行为变更必须同步更新两个 README，不得事后单独发文档 PR

## 常见陷阱

- apply_patch 写文件时必须遵守 V4A 格式：Begin Patch 开头、Add File 每行前缀 +（含空行）、Update File 中 - 行需 byte-exact 匹配
- 不要用 shell 重定向 (cat >、echo >、printf >) 写实际文件内容，这会绕过 diff 审计
- Tauri macOS 构建产物在 src-tauri/target/release/bundle/macos/
- 代理的 CDP 端口号写入 ~/Library/Application Support/<Agent>/DevToolsActivePort，非固定端口
