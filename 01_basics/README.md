# 第一章：GPUI 基础概念

欢迎来到 GPUI 学习的第一章！在这一章中，我们将学习 GPUI 应用的基本结构和核心概念。

## 📚 学习目标

- 理解 GPUI 应用的生命周期
- 掌握 `Application`、`Window`、`Context` 三大核心概念
- 学会使用 `Render` trait 构建 UI
- 创建你的第一个 GPUI 应用

## 🎯 核心概念

### 1. Application - 应用入口

`Application` 是每个 GPUI 应用的入口点。它负责：
- 初始化 GPUI 运行时
- 管理应用的生命周期
- 提供创建窗口的能力

```rust
use gpui::Application;

fn main() {
    Application::new().run(|app| {
        // 应用启动时的回调
        // 在这里打开窗口、初始化状态等
    });
}
```

**关键点：**
- `Application::new()` 创建应用实例
- `.run()` 启动事件循环，这个方法会阻塞直到应用退出
- 传入的闭包参数 `app` 是 `AppContext` 类型

### 2. Window - 窗口管理

窗口是 UI 显示的容器。你可以创建多个窗口。

```rust
app.open_window(WindowOptions::default(), |window, cx| {
    // 窗口初始化回调
    // 返回根视图
})
```

**关键点：**
- `WindowOptions` 配置窗口属性（大小、标题等）
- 回调接收 `Window` 和 `AppContext` 参数
- 必须返回一个实现了 `Render` trait 的视图

### 3. Render Trait - 渲染界面

`Render` trait 定义了如何将数据渲染成 UI。

```rust
impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child("Hello World!")
    }
}
```

**关键点：**
- `&mut self` 允许在渲染时访问和修改状态
- 必须返回实现 `IntoElement` 的类型
- 每次状态变化时会重新调用这个方法

### 4. Context (cx) - 上下文系统

`Context` 是 GPUI 中最重要的概念之一，它提供：
- 访问应用状态
- 触发重新渲染
- 创建事件监听器
- 管理生命周期

**常见的 Context 类型：**
- `AppContext` - 应用级上下文
- `Context<T>` - 视图特定的上下文
- `Window` - 窗口上下文

### 5. View - 视图组件

View 是一个普通的 Rust struct，实现 `Render` trait 后就成为了 UI 组件。

```rust
struct MyView {
    count: i32,  // 视图的状态
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(format!("Count: {}", self.count))
    }
}
```

## 📝 代码文件说明

### `main.rs` - Hello World

最简单的 GPUI 应用，显示 "Hello, GPUI!"。

**关键学习点：**
- Application 的创建和启动
- 打开窗口的基本流程
- 最简单的 Render 实现

**运行：**
```bash
cargo run -p gpui_basics
```

### `render.rs` - Render Trait 深入

探索 `Render` trait 的各种用法。

**关键学习点：**
- Render trait 的方法签名
- 返回不同类型的元素
- 条件渲染

### `context.rs` - Context 使用

理解 Context 的作用和使用方法。

**关键学习点：**
- Context 的不同类型
- 如何通过 Context 访问状态
- Context 的生命周期

## 🔍 重要概念详解

### GPUI 的渲染模型

GPUI 使用**混合模式**：
- **保留模式 (Retained Mode)**: 保存 UI 树结构，只在需要时重新渲染
- **即时模式 (Immediate Mode)**: 每帧重新构建 UI

这给我们带来：
- ✅ React 风格的声明式 API
- ✅ 高性能的增量更新
- ✅ 简单的状态管理

### 渲染流程

```
1. Application::new() 
   ↓
2. Application::run(|app| ...)
   ↓
3. app.open_window(...)
   ↓
4. 创建 View 实例
   ↓
5. 调用 View::render()
   ↓
6. 构建 Element 树
   ↓
7. GPU 渲染到屏幕
```

### 为什么需要 cx.notify()?

GPUI 不会自动追踪状态变化。当状态改变时，你需要告诉 GPUI：

```rust
self.count += 1;      // 状态改变
cx.notify();          // 告诉 GPUI 需要重新渲染
```

这种显式通知的好处：
- 🎯 **精确控制** - 你决定何时重新渲染
- ⚡ **性能更好** - 避免不必要的渲染
- 🔍 **更易调试** - 渲染时机清晰可见

## ✅ 测验题目

完成以下练习来检验你的理解：

### 练习 1：Hello GPUI (基础)

创建一个显示 "Hello, GPUI!" 的最小应用。

**要求：**
- 使用 `Application::new()` 和 `.run()`
- 打开一个窗口
- 在窗口中显示文本

**提示：** 参考 `main.rs`

### 练习 2：多行文本 (简单)

修改应用，显示三行文本：
```
Welcome to GPUI
This is line 2
This is line 3
```

**要求：**
- 使用 `div()` 作为容器
- 使用 `.child()` 添加多个文本

### 练习 3：概念解释 (理论)

用自己的话解释：

1. **Application、Window、Context 三者的关系是什么？**
   
   你的答案：

   - Application 管理整个应用的声明周期，启动事件循环，创建窗口等
   - Window 是一个独立的窗口，用于承载视图
   - Context 是视图特定的上下文对象
  
  关系：`Application` → `Window` → `View` → UI 元素
   
2. **Render trait 的 render 方法什么时候被调用？**
   
   你的答案：
  - 视图需要重新渲染时
    1. 视图状态改变时
    2. 父视图重新渲染时
    3. 调用某种绘制？？
    4. 视图被添加到窗口时
   

3. **为什么 render 方法的第一个参数是 `&mut self`？**
   
   你的答案：
  可变是由于视图状态是可以被修改的

### 练习 4：自定义视图 (中等)

创建一个 `WelcomeView` 结构体，包含：
- 字段 `name: String`
- 在 render 中显示 "Welcome, {name}!"

**要求：**
- 定义 struct
- 实现 Render trait
- 在 main 中创建实例

## 🎓 知识检查清单

学完本章后，你应该能够：

- [ ] 解释 GPUI 应用的基本结构
- [ ] 创建一个 Application 并启动
- [ ] 打开一个窗口
- [ ] 实现 Render trait
- [ ] 理解 Context 的基本作用
- [ ] 创建简单的静态 UI

## 📖 扩展阅读

- `IntoElement` trait - 元素转换机制
- `ParentElement` trait - 父子元素关系
- Window 配置选项
- 应用生命周期钩子

## ➡️ 下一章

完成本章学习后，继续学习 [第二章：元素系统](../02_elements/README.md)，我们将深入学习如何构建复杂的 UI 结构。

---

**准备好了吗？让我们开始编写第一个 GPUI 应用！** 🚀
