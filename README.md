# GPUI 系统学习项目

欢迎来到 GPUI 学习项目！这是一个结构化的学习路径，帮助你从零开始系统掌握 GPUI 框架。

## 🚀 快速开始

**第一次使用？** 请先阅读：
- 📖 [快速入门指南](GETTING_STARTED.md) - 环境配置和项目设置
- 📋 [学习计划总览](STUDY_PLAN.md) - 完整的学习路径和时间规划

**马上开始学习：**
```bash
# 1. 验证环境
cargo run --example hello_world

# 2. 开始第一章
cd 01_basics
cargo run
```

## 什么是 GPUI？

GPUI 是一个由 Zed 编辑器团队开发的**高性能、GPU 加速的 UI 框架**，专为 Rust 设计。它采用了混合即时模式和保留模式的渲染方式，既能提供高性能，又能保持开发的便捷性。

### 核心特点
- 🚀 **GPU 加速** - 充分利用现代 GPU 的性能
- 🎨 **声明式 UI** - 类似 React/SwiftUI 的开发体验
- ⚡ **高性能** - 专为响应式、流畅的用户界面设计
- 🦀 **纯 Rust** - 充分利用 Rust 的类型安全和性能优势

## 学习路径

### 🎯 第一章：GPUI 基础概念 (01_basics)

**学习目标：** 理解 GPUI 应用的基本结构和核心概念

#### 核心概念：
1. **Application** - GPUI 应用的入口点
2. **Window** - 窗口管理
3. **Render trait** - 如何渲染 UI
4. **Context (cx)** - 上下文系统
5. **View** - 视图组件

#### 关键知识点：
- `Application::new()` 创建应用
- `.run()` 启动应用生命周期
- `.open_window()` 打开窗口
- `Render` trait 的 `render()` 方法
- `Context` 的作用和使用

#### 测验题目：
- [ ] 创建一个显示 "Hello GPUI" 的最小应用
- [ ] 解释 Application、Window、Context 三者的关系

---

### 🎨 第二章：元素系统 (02_elements)

**学习目标：** 掌握 GPUI 的元素系统和 UI 构建方法

#### 核心概念：
1. **Element trait** - 元素的基础抽象
2. **IntoElement** - 元素转换
3. **ParentElement** - 父元素和子元素
4. **div()** - 最基础的容器元素
5. **元素组合** - 构建复杂 UI

#### 关键知识点：
- `div()` 创建容器
- `.child()` 添加子元素
- `.children()` 添加多个子元素
- 元素链式调用
- 文本渲染

#### 测验题目：
- [ ] 创建一个嵌套的 div 结构
- [ ] 构建一个包含标题和内容的卡片组件

---

### 📊 第三章：状态管理 (03_state_management)

**学习目标：** 理解 GPUI 的响应式状态管理机制

#### 核心概念：
1. **View 的状态** - 在 struct 中保存状态
2. **cx.notify()** - 触发重新渲染
3. **Model** - 共享状态模型
4. **响应式更新** - 状态变化如何触发 UI 更新
5. **Context 的状态管理 API**

#### 关键知识点：
- 在 View 结构体中定义状态字段
- 使用 `cx.notify()` 标记需要重新渲染
- 状态更新的生命周期
- `&mut self` 在 render 方法中的作用

#### 测验题目：
- [ ] 实现一个计数器应用（+1、-1、重置按钮）
- [ ] 解释 `cx.notify()` 的作用和原理

---

### 🎭 第四章：样式系统 (04_styling)

**学习目标：** 掌握 GPUI 的样式和布局系统

#### 核心概念：
1. **Styled trait** - 样式方法
2. **Flexbox 布局** - flex、items_center、justify_center 等
3. **尺寸和间距** - size、padding、margin、gap
4. **颜色系统** - rgb()、预定义颜色
5. **边框和圆角** - border、rounded
6. **伪状态** - hover、active、focus

#### 关键知识点：
- `.flex()` 启用 flexbox
- `.size_full()`, `.w_*()`, `.h_*()` 尺寸设置
- `.bg()`, `.text_color()` 颜色设置
- `.p_*()`, `.m_*()`, `.gap_*()` 间距设置
- `.rounded_*()` 圆角
- `.hover(|style| ...)` 悬停样式

#### 测验题目：
- [ ] 创建一个响应式的按钮组件（带悬停效果）
- [ ] 实现一个三列等宽的卡片布局

---

### ⚡ 第五章：事件处理 (05_events)

**学习目标：** 掌握用户交互和事件处理

#### 核心概念：
1. **InteractiveElement** - 可交互元素
2. **事件监听器** - on_click、on_mouse_down 等
3. **cx.listener()** - 创建事件处理器
4. **事件对象** - ClickEvent、MouseDownEvent 等
5. **键盘事件** - 键盘输入处理

#### 关键知识点：
- `.on_click(cx.listener(Self::method))` 点击事件
- `.cursor_pointer()` 鼠标指针样式
- 事件处理函数的签名
- 事件传播和阻止
- 键盘快捷键

#### 测验题目：
- [ ] 实现一个可点击的按钮列表
- [ ] 创建一个支持键盘导航的菜单

---

### 🚀 第六章：高级主题 (06_advanced)

**学习目标：** 探索 GPUI 的高级特性

#### 核心概念：
1. **Action 系统** - 命令和快捷键
2. **焦点管理** - focus、blur
3. **异步操作** - 在 GPUI 中处理异步任务
4. **自定义 Element** - 实现自己的元素类型
5. **性能优化** - 渲染优化技巧

#### 关键知识点：
- 定义和分发 Actions
- 焦点系统和焦点链
- `cx.spawn()` 异步任务
- 实现自定义 Element trait
- 避免不必要的重新渲染

#### 测验题目：
- [ ] 实现一个带全局快捷键的应用
- [ ] 创建一个自定义的进度条组件

---

## 如何使用本项目

### 运行示例

```bash
# 运行 hello world 示例
cargo run --example hello_world

# 运行计数器示例
cargo run --example counter

# 运行 todo list 示例
cargo run --example todo_list
```

### 运行各章节

```bash
# 运行第一章
cargo run -p gpui_basics

# 运行第二章
cargo run -p gpui_elements

# 以此类推...
```

### 学习建议

1. **按顺序学习** - 每章都建立在前一章的基础上
2. **动手实践** - 不要只看代码，自己敲一遍
3. **完成测验** - 每章的测验帮助巩固知识
4. **实验探索** - 尝试修改代码，看看会发生什么
5. **查阅文档** - 遇到问题时查看 [GPUI 文档](https://www.gpui.rs/)

## 学习检查清单

### 第一章 ✅
- [ ] 理解 Application 的作用
- [ ] 能创建并打开窗口
- [ ] 理解 Render trait
- [ ] 了解 Context 的用途
- [ ] 完成第一章测验

### 第二章 ✅
- [ ] 理解 Element 系统
- [ ] 熟练使用 div() 和 child()
- [ ] 能构建嵌套的 UI 结构
- [ ] 完成第二章测验

### 第三章 ✅
- [ ] 理解响应式状态管理
- [ ] 掌握 cx.notify() 的使用
- [ ] 能实现有状态的组件
- [ ] 完成第三章测验

### 第四章 ✅
- [ ] 熟练使用 Flexbox 布局
- [ ] 掌握样式方法
- [ ] 能实现响应式设计
- [ ] 完成第四章测验

### 第五章 ✅
- [ ] 理解事件系统
- [ ] 能处理鼠标和键盘事件
- [ ] 掌握 cx.listener() 的使用
- [ ] 完成第五章测验

### 第六章 ✅
- [ ] 理解 Action 系统
- [ ] 掌握焦点管理
- [ ] 能处理异步操作
- [ ] 完成第六章测验

## 资源链接

- 📚 [GPUI 官方文档](https://www.gpui.rs/)
- 📖 [GPUI Book](https://matinaniss.github.io/gpui-book/)
- 🔧 [Zed 编辑器源码](https://github.com/zed-industries/zed) - GPUI 的最佳实践
- 💬 [Zed Discord 社区](https://discord.gg/zed)

## 📖 详细文档

- **[GETTING_STARTED.md](GETTING_STARTED.md)** - 环境配置、前置要求、常见问题
- **[STUDY_PLAN.md](STUDY_PLAN.md)** - 完整学习计划、时间安排、学习方法
- **[01_basics/README.md](01_basics/README.md)** - 第一章详细说明

## 🎓 下一步

完成所有章节后，你可以：
1. 构建自己的 GPUI 应用
2. 贡献到 Zed 编辑器项目
3. 探索 GPUI 的高级特性
4. 阅读 Zed 的源码学习最佳实践

## 💬 获取帮助

- 遇到问题？查看各章节 README 的常见问题部分
- 需要社区支持？加入 [Zed Discord](https://discord.gg/zed)
- 发现错误？欢迎提交 Issue 或 PR

---

**祝学习愉快！记住：实践是最好的老师。** 🎉

现在就运行 `cargo run --example hello_world` 开始你的 GPUI 之旅吧！