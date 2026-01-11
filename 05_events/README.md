# 第五章：GPUI 事件处理

本章学习 GPUI 的事件系统，包括鼠标事件、键盘事件、滚轮事件和焦点管理。

## 事件类型概览

GPUI 支持以下主要事件类型：

### 鼠标事件
| 事件类型 | 说明 |
|---------|------|
| `ClickEvent` | 点击事件（按下+释放） |
| `MouseDownEvent` | 鼠标按下 |
| `MouseUpEvent` | 鼠标释放 |
| `MouseMoveEvent` | 鼠标移动 |
| `ScrollWheelEvent` | 滚轮滚动 |

### 键盘事件
| 事件类型 | 说明 |
|---------|------|
| `KeyDownEvent` | 键盘按下 |
| `KeyUpEvent` | 键盘释放 |

## 一、点击事件 (on_click)

最常用的事件处理方式：

```rust
div()
    .id("my-button")  // ⚠️ 必须有 id
    .on_click(cx.listener(|view, event: &ClickEvent, window, cx| {
        // 获取点击位置
        let position = event.position();
        
        // 获取修饰键状态
        let modifiers = event.modifiers();
        println!("Ctrl: {}, Shift: {}", modifiers.control, modifiers.shift);
        
        // 处理点击...
        cx.notify();  // 触发重新渲染
    }))
```

## 二、鼠标按下/释放事件

精确控制不同鼠标按钮：

```rust
div()
    .id("area")
    // 左键按下
    .on_mouse_down(MouseButton::Left, cx.listener(|view, event: &MouseDownEvent, window, cx| {
        let position = event.position;
        let click_count = event.click_count;  // 双击检测
        cx.notify();
    }))
    // 右键按下
    .on_mouse_down(MouseButton::Right, cx.listener(|view, event, window, cx| {
        // 右键菜单逻辑
    }))
    // 左键释放
    .on_mouse_up(MouseButton::Left, cx.listener(|view, event: &MouseUpEvent, window, cx| {
        // 释放处理
    }))
```

### MouseButton 选项
- `MouseButton::Left` - 左键
- `MouseButton::Right` - 右键
- `MouseButton::Middle` - 中键

## 三、鼠标移动事件

```rust
div()
    .id("track-area")
    .on_mouse_move(cx.listener(|view, event: &MouseMoveEvent, window, cx| {
        let position = event.position;
        let modifiers = event.modifiers;
        
        // 更新鼠标位置状态
        view.mouse_position = Some(position);
        cx.notify();
    }))
```

## 四、点击元素外部

常用于关闭下拉菜单、模态框等：

```rust
div()
    .id("dropdown")
    .on_mouse_down_out(cx.listener(|view, event: &MouseDownEvent, window, cx| {
        // 当在元素外部点击时触发
        view.is_open = false;
        cx.notify();
    }))
```

## 五、滚轮事件

```rust
div()
    .id("scrollable")  // ⚠️ 必须有 id
    .on_scroll_wheel(cx.listener(|view, event: &ScrollWheelEvent, window, cx| {
        // 获取滚动增量（转换为像素）
        let delta = event.delta.pixel_delta(px(20.0));
        
        // delta.x - 水平滚动量
        // delta.y - 垂直滚动量
        view.scroll_offset.x += f32::from(delta.x);
        view.scroll_offset.y += f32::from(delta.y);
        
        cx.notify();
    }))
```

## 六、键盘事件

### 基本用法

```rust
div()
    .id("keyboard-area")
    .track_focus(&focus_handle)  // ⚠️ 需要可聚焦
    // 键盘按下
    .on_key_down(cx.listener(|view, event: &KeyDownEvent, window, cx| {
        let keystroke = &event.keystroke;
        println!("按键: {}", keystroke);
        
        // 检测是否长按
        if !event.is_held {
            // 首次按下
        }
        
        cx.notify();
    }))
    // 键盘释放
    .on_key_up(cx.listener(|view, event: &KeyUpEvent, window, cx| {
        let keystroke = &event.keystroke;
        // 处理释放
    }))
```

### Keystroke 属性

```rust
// 获取按键名称
keystroke.key  // 如 "a", "enter", "space", "backspace"

// 获取修饰键
keystroke.modifiers.control  // Ctrl
keystroke.modifiers.shift    // Shift
keystroke.modifiers.alt      // Alt
keystroke.modifiers.platform // Cmd (Mac) / Ctrl (Windows)
```

### 焦点管理

键盘事件需要元素有焦点：

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

impl Focusable for MyView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("focusable")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|view, event, window, cx| {
                // 处理键盘事件
            }))
    }
}
```

## 七、cx.listener() 详解

`cx.listener()` 是一个便捷方法，用于在事件回调中访问视图状态。

### 为什么需要 listener？

事件回调签名是：
```rust
Fn(&Event, &mut Window, &mut App)
```

但我们通常需要访问视图状态，`listener` 会自动处理：
```rust
// listener 返回的闭包签名
Fn(&Event, &mut Window, &mut App)

// 但内部回调可以访问视图
Fn(&mut Self, &Event, &mut Window, &mut Context<Self>)
```

### 不使用 listener 的替代方式

```rust
let weak_view = cx.entity().downgrade();

div().on_click(move |event, window, cx| {
    weak_view.update(cx, |view, cx| {
        // 处理事件
        cx.notify();
    }).ok();
})
```

## 八、伪状态样式

`hover` 和 `active` 不是事件，而是样式方法：

```rust
div()
    .id("button")  // 必须有 id
    .bg(rgb(0x3B82F6))
    .hover(|style| style.bg(rgb(0x2563EB)))    // 鼠标悬停
    .active(|style| style.bg(rgb(0x1D4ED8)))   // 鼠标按下
```

## 九、事件对象属性参考

### ClickEvent
```rust
event.position()   // Point<Pixels> - 点击位置
event.modifiers()  // Modifiers - 修饰键
```

### MouseDownEvent / MouseUpEvent
```rust
event.button       // MouseButton - 哪个按钮
event.position     // Point<Pixels> - 位置
event.modifiers    // Modifiers - 修饰键
event.click_count  // usize - 点击次数（检测双击）
event.first_mouse  // bool - 是否是激活窗口的首次点击
```

### MouseMoveEvent
```rust
event.position     // Point<Pixels> - 当前位置
event.modifiers    // Modifiers - 修饰键
event.pressed_button // Option<MouseButton> - 按住的按钮
```

### KeyDownEvent
```rust
event.keystroke    // Keystroke - 按键信息
event.is_held      // bool - 是否长按重复
```

### Modifiers
```rust
modifiers.control  // bool - Ctrl
modifiers.alt      // bool - Alt
modifiers.shift    // bool - Shift
modifiers.platform // bool - Cmd (Mac) / Ctrl (Windows)
```

## 十、常见模式

### 拖拽实现

```rust
struct DraggableView {
    is_dragging: bool,
    position: Point<Pixels>,
    drag_start: Option<Point<Pixels>>,
}

impl Render for DraggableView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("draggable")
            .on_mouse_down(MouseButton::Left, cx.listener(|view, event, window, cx| {
                view.is_dragging = true;
                view.drag_start = Some(event.position);
                cx.notify();
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|view, event, window, cx| {
                view.is_dragging = false;
                view.drag_start = None;
                cx.notify();
            }))
            .on_mouse_move(cx.listener(|view, event, window, cx| {
                if view.is_dragging {
                    if let Some(start) = view.drag_start {
                        let delta = event.position - start;
                        view.position = view.position + delta;
                        view.drag_start = Some(event.position);
                        cx.notify();
                    }
                }
            }))
    }
}
```

### 快捷键检测

```rust
.on_key_down(cx.listener(|view, event: &KeyDownEvent, window, cx| {
    let key = &event.keystroke;
    
    // Ctrl+S / Cmd+S
    if key.modifiers.platform && key.key == "s" {
        view.save();
        cx.notify();
    }
    
    // Escape
    if key.key == "escape" {
        view.cancel();
        cx.notify();
    }
}))
```

## 运行示例

```bash
cargo run -p gpui_events
```

示例展示：
- 交互式画布（鼠标绘制）
- 点击事件演示
- 键盘事件监听
- 滚轮事件处理
- 悬停状态展示

## 重要注意事项

1. **`on_click` 需要元素有 `id`**
2. **键盘事件需要元素有焦点**
3. **滚轮事件也需要元素有 `id`**
4. **事件处理后通常需要调用 `cx.notify()`**
5. **使用 `cx.listener()` 简化事件回调**

## 下一章预告

第六章将学习 GPUI 的高级主题，包括 Action 系统、异步操作、自定义 Element 等。

