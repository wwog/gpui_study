# 第六章：GPUI 高级主题

本章学习 GPUI 的高级特性，包括 Action 系统、异步操作、焦点管理和全局状态。

## 一、Action 系统

Action 是 GPUI 的命令模式实现，用于键盘驱动的 UI。

### 定义简单 Action

```rust
use gpui::actions;

// 使用 actions! 宏定义 Action
actions!(
    my_app,  // 命名空间
    [
        Save,    // 保存
        Undo,    // 撤销
        Redo,    // 重做
    ]
);
```

### 注册 Action 处理器

```rust
impl MyView {
    // Action 处理方法签名
    fn handle_save(&mut self, _: &Save, _window: &mut Window, cx: &mut Context<Self>) {
        // 处理保存
        cx.notify();
    }
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("view")
            .track_focus(&self.focus_handle)
            // 注册 Action 处理器
            .on_action(cx.listener(Self::handle_save))
    }
}
```

### 分发 Action

```rust
// 通过 window 分发 Action
window.dispatch_action(Save.boxed_clone(), cx);

// 或在键盘事件中
.on_key_down(cx.listener(|view, event, window, cx| {
    if event.keystroke.modifiers.platform && event.keystroke.key == "s" {
        window.dispatch_action(Save.boxed_clone(), cx);
    }
}))
```

### 带参数的 Action

```rust
// 需要完整实现 Action trait 或使用 derive 宏
#[derive(Clone, PartialEq, serde::Deserialize, schemars::JsonSchema, Action)]
#[action(namespace = editor)]
pub struct SetValue {
    pub value: i32,
}
```

## 二、异步操作

使用 `cx.spawn()` 在视图上下文中执行异步任务。

### 基本用法

```rust
struct AsyncView {
    loading: bool,
    result: Option<String>,
    _task: Option<Task<()>>,  // 保存任务句柄
}

impl AsyncView {
    fn load_data(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        cx.notify();

        // cx.spawn 启动异步任务
        let task = cx.spawn(async |weak_view, cx| {
            // 模拟网络请求
            cx.background_executor()
                .timer(Duration::from_secs(2))
                .await;

            // 更新视图状态
            let _ = weak_view.update(cx, |view, cx| {
                view.loading = false;
                view.result = Some("数据加载完成".to_string());
                cx.notify();
            });
        });

        // 保存 task，防止被 drop（drop = 取消）
        self._task = Some(task);
    }
}
```

### 重要注意事项

1. **必须保存 Task**：drop Task 会取消异步操作
2. **使用 WeakEntity**：闭包接收弱引用，避免循环引用
3. **使用 update 更新状态**：`weak_view.update(cx, |view, cx| { ... })`
4. **调用 notify**：更新后调用 `cx.notify()` 触发重新渲染

### 取消异步操作

```rust
fn cancel(&mut self) {
    self._task = None;  // drop task 取消操作
}
```

## 三、焦点管理

使用 `FocusHandle` 管理元素焦点，键盘事件依赖焦点系统。

### 创建和存储 FocusHandle

```rust
struct MyView {
    focus_handle: FocusHandle,
}

impl MyView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}
```

### 关联焦点到元素

```rust
div()
    .id("focusable")
    .track_focus(&self.focus_handle)
    .on_key_down(...)  // 现在可以接收键盘事件
```

### 检查和设置焦点

```rust
// 检查是否有焦点
let is_focused = self.focus_handle.is_focused(window);

// 设置焦点
self.focus_handle.focus(window);

// 清除焦点
self.focus_handle.blur(window);
```

### 焦点导航

```rust
// 创建多个焦点句柄
let handles: Vec<FocusHandle> = (0..4)
    .map(|_| cx.focus_handle())
    .collect();

// 切换焦点
fn focus_next(&mut self, window: &mut Window) {
    let next = (self.current + 1) % self.handles.len();
    self.handles[next].focus(window);
}
```

### 焦点事件监听

```rust
// 在 Context 中监听焦点变化
cx.on_focus(&self.focus_handle, window, |view, window, cx| {
    // 获得焦点时
});

cx.on_blur(&self.focus_handle, window, |view, window, cx| {
    // 失去焦点时
});
```

## 四、Global 状态

使用 `Global` trait 管理跨组件共享的全局状态。

### 定义全局状态

```rust
#[derive(Clone)]
struct ThemeState {
    is_dark: bool,
}

impl Global for ThemeState {}
```

### 设置全局状态

```rust
// 在 App 初始化时
cx.set_global(ThemeState { is_dark: false });

// 或在任何组件中
cx.set_global(ThemeState { is_dark: true });
```

### 读取全局状态

```rust
// 安全读取（返回 Option）
let is_dark = cx.try_global::<ThemeState>()
    .map(|t| t.is_dark)
    .unwrap_or(false);

// 假设存在（不存在会 panic）
let is_dark = cx.global::<ThemeState>().is_dark;
```

### 观察全局状态变化

```rust
// 在组件创建时注册观察者
cx.observe_global::<ThemeState>(|view, cx| {
    // 全局状态变化时调用
    cx.notify();
}).detach();
```

## 运行示例

```bash
cargo run -p gpui_advanced
```

示例展示：
- **Action 演示**：按 ↑/↓/R 或数字键控制计数
- **异步演示**：点击加载，观察进度条
- **焦点演示**：点击区域或按方向键导航
- **Global 演示**：切换主题，观察全局状态变化

## API 参考

### Action 相关
```rust
actions!(namespace, [Action1, Action2]);
.on_action(cx.listener(Self::handle_action))
window.dispatch_action(action.boxed_clone(), cx)
```

### 异步相关
```rust
cx.spawn(async |weak_view, cx| { ... })
cx.background_executor().timer(Duration).await
weak_view.update(cx, |view, cx| { ... })
```

### 焦点相关
```rust
cx.focus_handle()
.track_focus(&handle)
handle.focus(window)
handle.is_focused(window)
```

### Global 相关
```rust
impl Global for MyState {}
cx.set_global(state)
cx.try_global::<MyState>()
cx.global::<MyState>()
cx.observe_global::<MyState>(...)
```

## 下一步

恭喜完成 GPUI 学习！现在你可以：
1. 构建自己的 GPUI 应用
2. 阅读 Zed 编辑器源码学习最佳实践
3. 参与 GPUI 社区贡献

