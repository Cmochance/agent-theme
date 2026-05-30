# 前端迁移计划：Vanilla JS → TypeScript + Svelte + Tailwind CSS

## 现状分析

### 文件规模

| 文件 | 行数 | 职责 |
|------|------|------|
| `web/index.html` | 117 | 页面结构、所有 DOM 元素 |
| `web/style.css` | 764 | 全部样式（CSS 变量设计系统 + 组件样式） |
| `web/app.js` | 574 | 全部逻辑（状态管理、Tauri 调用、DOM 操作、裁剪器） |

### Tauri Command 清单（前端调用的后端接口）

| Command | 参数 | 返回值 | 调用位置 |
|---------|------|--------|----------|
| `get_config` | 无 | `AppConfig` | refreshStatus |
| `set_enabled` | `{ enabled: bool }` | `AppConfig` | themeEnabledToggle |
| `set_selected_agent` | `{ agent: AgentKind }` | `AppConfig` | setupAgentSelector |
| `get_agent_status` | 无 | `{ running, cdpPort, agentKind }` | refreshStatus |
| `get_all_themes` | 无 | `Vec<Theme>` | loadThemes |
| `apply_theme` | `{ theme_id: string }` | `void` | applyTheme |
| `clear_theme` | 无 | `void` | clearActiveTheme |
| `upload_custom_theme` | `{ bg_base64, preview_base64 }` | `void` | performCrop |
| `delete_custom_theme_cmd` | 无 | `void` | deleteCustomTheme |

### 数据结构

```typescript
interface AppConfig {
  enabled: boolean;
  selectedThemeId: string;
  autoLaunchAgent: boolean;
  activeIdentifier: string | null;
  selectedAgent: 'codex' | 'antigravity';
}

interface Theme {
  id: string;
  displayName: { en: string; zh: string };
  isCustom: boolean;
  background: string;
  preview: string;
  dir: string; // PathBuf 序列化为字符串
}

interface AgentStatus {
  running: boolean;
  cdpPort: number | null;
  agentKind: string;
}
```

### 当前全局状态（app.js 中的 let 变量）

| 变量 | 类型 | 用途 |
|------|------|------|
| `appConfig` | `AppConfig \| null` | 后端配置缓存 |
| `currentThemes` | `Theme[]` | 主题列表 |
| `cropImageSrc` | `string \| null` | 裁剪图片 data URL |
| `cropImageObj` | `HTMLImageElement \| null` | 裁剪图片 DOM 对象 |
| `imageX, imageY` | `number` | 裁剪偏移 |
| `imageScale` | `number` | 裁剪缩放 |
| `isDragging` | `boolean` | 拖拽状态 |
| `startX, startY` | `number` | 拖拽起点 |

### UI 组件拆分（逻辑分区）

1. **AgentSelector** — Codex / Antigravity 切换按钮
2. **StatusBar** — 状态球 + 端口 + 注入开关
3. **ThemeGrid** — 主题卡片网格 + 自定义上传入口
4. **ThemeCard** — 单个主题卡片（预览、应用/删除）
5. **UploadModal** — 上传弹窗（拖拽/浏览 → 裁剪 → 保存）
6. **ImageCropper** — 裁剪器（拖拽定位 + 缩放）

---

## 迁移步骤

### Phase 1：项目基建

**目标**：搭建 Svelte + TypeScript + Tailwind 开发环境，能在浏览器中独立运行。

- [ ] 1.1 安装依赖：`svelte`、`@sveltejs/vite-plugin-svelte`、`vite`、`typescript`、`tailwindcss`、`@tailwindcss/vite`、`@tauri-apps/api`
- [ ] 1.2 创建 `vite.config.ts`，配置 Svelte 插件和 Tailwind 插件
- [ ] 1.3 创建 `tsconfig.json`
- [ ] 1.4 创建 `src/main.ts`（Svelte 入口）和 `src/App.svelte`（根组件）
- [ ] 1.5 创建 `src/app.css`（Tailwind 入口，导入 `@import "tailwindcss"`）
- [ ] 1.6 将 `web/index.html` 改为 Vite 入口 HTML（`index.html` 移到项目根目录），引用 `src/main.ts`
- [ ] 1.7 更新 `package.json` 的 build script 为 `vite build`，输出到 `web/` 目录
- [ ] 1.8 更新 `tauri.conf.json` 的 `frontendDist` 指向 Vite 输出目录
- [ ] 1.9 验证 `npm run dev` 能在浏览器中运行 Svelte 应用

**产出**：空白 Svelte 应用通过 Vite 运行，Tailwind 生效。

### Phase 2：Tauri API 类型层

**目标**：将所有后端调用封装为类型安全的函数。

- [ ] 2.1 创建 `src/lib/tauri-commands.ts`，为每个 Tauri command 编写类型化包装函数
- [ ] 2.2 创建 `src/lib/types.ts`，定义 `AppConfig`、`Theme`、`AgentStatus` 接口
- [ ] 2.3 验证类型与 Rust 端的 `serde` 序列化一致（注意 `camelCase` 转换）

**产出**：`src/lib/tauri-commands.ts` + `src/lib/types.ts`，所有 Tauri 调用有类型推导。

### Phase 3：状态管理

**目标**：用 Svelte 的响应式 store 替代全局变量。

- [ ] 3.1 创建 `src/lib/stores.ts`
  - `configStore: Writable<AppConfig | null>` — 替代 `appConfig`
  - `themesStore: Writable<Theme[]>` — 替代 `currentThemes`
  - `statusStore: Writable<AgentStatus | null>` — 替代 refreshStatus 中的 DOM 更新
- [ ] 3.2 创建 `src/lib/actions.ts` — 封装业务操作（applyTheme、clearTheme、deleteTheme、switchAgent、uploadTheme），内部调用 tauri-commands 并更新 store
- [ ] 3.3 创建轮询逻辑 `src/lib/polling.ts` — 15 秒定时刷新 status + config

**产出**：响应式状态层，组件通过 `$store` 读取数据，通过 actions 修改。

### Phase 4：基础布局组件

**目标**：搭建页面骨架，替换 index.html 的静态结构。

- [ ] 4.1 创建 `src/App.svelte` — 页面容器，引入 Tailwind，组合子组件
- [ ] 4.2 创建 `src/lib/components/AgentSelector.svelte` — 两个切换按钮，凸起 3D 效果用 Tailwind 实现
- [ ] 4.3 创建 `src/lib/components/StatusBar.svelte` — 状态球 + 端口 + 注入开关，三个等宽模块
- [ ] 4.4 创建 `src/lib/components/StatusDot.svelte` — 红/黄/绿立体球，根据 status store 自动切换
- [ ] 4.5 创建 `src/lib/components/Toggle.svelte` — 3D 开关，灰/绿状态
- [ ] 4.6 验证页面布局与当前 app 一致

**产出**：页面顶部区域完全由 Svelte 组件构成。

### Phase 5：主题网格和卡片

**目标**：主题列表和交互逻辑迁移。

- [ ] 5.1 创建 `src/lib/components/ThemeGrid.svelte` — 响应式网格布局，从 themesStore 读取数据
- [ ] 5.2 创建 `src/lib/components/ThemeCard.svelte` — 预览图、主题名、应用/删除按钮，`{#if}` 控制自定义主题的删除按钮
- [ ] 5.3 创建 `src/lib/components/UploadCard.svelte` — "Custom Background" 上传入口卡片
- [ ] 5.4 迁移主题操作：点击卡片 → `applyTheme()`、删除按钮 → `deleteCustomTheme()`
- [ ] 5.5 使用 Svelte `transition:fly` 实现卡片出现动画

**产出**：主题网格完全功能，可切换、删除主题。

### Phase 6：上传弹窗和裁剪器

**目标**：最复杂的交互部分迁移。

- [ ] 6.1 创建 `src/lib/components/UploadModal.svelte` — 弹窗容器，`{#if showModal}` 控制显隐
- [ ] 6.2 创建 `src/lib/components/DropZone.svelte` — 拖拽区域 + 文件浏览，处理 `dragover/drop/change` 事件
- [ ] 6.3 创建 `src/lib/components/ImageCropper.svelte` — 裁剪器核心逻辑
  - 用 Svelte 的 `bind:this` 替代 `document.getElementById`
  - 鼠标事件用 `on:mousedown/mousemove/mouseup` 替代 `addEventListener`
  - 裁剪状态用组件内部 `let` 变量（局部状态，不需要全局 store）
- [ ] 6.4 迁移 `performCrop()` 逻辑到 `ImageCropper.svelte` 中的 `saveCrop()` 函数
- [ ] 6.5 弹窗打开/关闭动画用 `transition:fade`

**产出**：完整上传流程（拖拽 → 裁剪 → 保存）在 Svelte 中运行。

### Phase 7：CSS 迁移到 Tailwind

**目标**：逐步用 Tailwind 替代手写 CSS。

- [ ] 7.1 将 `:root` CSS 变量迁移到 Tailwind `theme.extend.colors` 配置
- [ ] 7.2 组件样式逐一替换：从简单组件开始（StatusDot、Toggle、AgentSelector），再到复杂组件（ThemeCard、UploadModal）
- [ ] 7.3 特殊效果（凸起按钮渐变、凹陷边框、立体球径向渐变）用 Tailwind `bg-linear-to-*` + 自定义 CSS（`<style>` 块）混合方案
- [ ] 7.4 玻璃拟态效果（`backdrop-filter: blur`）用 Tailwind `backdrop-blur-*` 类
- [ ] 7.5 删除 `web/style.css`

**产出**：所有样式由 Tailwind + 组件 scoped style 构成。

### Phase 8：清理和集成

**目标**：移除旧文件，更新构建链。

- [ ] 8.1 删除 `web/app.js`、`web/index.html`、`web/style.css`
- [ ] 8.2 确认 `npm run build` 由 Vite 接管，输出到 `web/` 目录
- [ ] 8.3 更新 `tauri.conf.json` 的 `frontendDist` 指向 Vite 输出目录（如需调整）
- [ ] 8.4 更新 `.gitignore`：忽略 Vite 输出目录，添加 `.svelte-kit/`（如有）
- [ ] 8.5 更新 `.github/workflows/ci.yml`：前端构建改为 `npm install && npm run build`
- [ ] 8.6 更新 `AGENTS.md` 中的前端技术栈描述
- [ ] 8.7 全量测试：`cargo tauri dev` + `cargo tauri build` + 手动验证所有功能

**产出**：迁移完成，旧文件清除，CI 通过。

---

## 目录结构（迁移后）

```
agent-theme/
├── index.html              ← Vite 入口 HTML
├── package.json             ← 含 svelte/vite/tailwind 依赖
├── vite.config.ts           ← Vite + Svelte + Tailwind 配置
├── tsconfig.json
├── src/
│   ├── main.ts              ← Svelte 应用入口
│   ├── App.svelte           ← 根组件
│   ├── app.css              ← Tailwind 入口
│   └── lib/
│       ├── types.ts         ← AppConfig/Theme/AgentStatus 类型
│       ├── tauri-commands.ts ← Tauri command 类型化封装
│       ├── stores.ts        ← Svelte writable stores
│       ├── actions.ts       ← 业务操作封装
│       ├── polling.ts       ← 定时轮询逻辑
│       └── components/
│           ├── AgentSelector.svelte
│           ├── StatusBar.svelte
│           ├── StatusDot.svelte
│           ├── Toggle.svelte
│           ├── ThemeGrid.svelte
│           ├── ThemeCard.svelte
│           ├── UploadCard.svelte
│           ├── UploadModal.svelte
│           ├── DropZone.svelte
│           └── ImageCropper.svelte
├── web/                     ← Vite 构建输出目录（.gitignore 忽略内容）
├── themes/                  ← 主题资源（不变）
└── src-tauri/               ← Rust 后端（不变）
```

## 技术决策记录

| 决策 | 选择 | 理由 |
|------|------|------|
| 构建工具 | Vite | Svelte 官方推荐，HMR 快，Tauri v2 文档示例也用 Vite |
| CSS 方案 | Tailwind + scoped style | 纯 Tailwind 无法覆盖渐变/径向渐变等复杂效果，用 `<style>` 补充 |
| 状态管理 | Svelte stores | 项目规模不需要 Redux/Zustand，原生 store 足够 |
| 路由 | 不需要 | 单页面工具，无路由需求 |
| 裁剪器 | 自研保留 | 当前裁剪逻辑已可用，暂不引入第三方库 |
