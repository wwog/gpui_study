# 第四章：GPUI 样式系统

本章学习 GPUI 类似 Tailwind CSS 的样式 API，掌握布局、颜色、边框、阴影、文字等样式设置。

## 核心概念

### Styled Trait

GPUI 中的样式通过 `Styled` trait 提供，所有元素（如 `div()`）都实现了这个 trait，允许链式调用样式方法：

```rust
div()
    .flex()           // 显示模式
    .flex_col()       // Flex 方向
    .gap_4()          // 间隙
    .p_4()            // 内边距
    .bg(rgb(0xFFFFFF))// 背景色
    .rounded_lg()     // 圆角
    .shadow_md()      // 阴影
```

## 一、Flexbox 布局

### 显示模式

```rust
.flex()     // 启用 flex 布局（最常用）
.block()    // 块级布局
.grid()     // 网格布局
.hidden()   // 隐藏元素
```

### Flex 方向

```rust
.flex_row()          // 水平方向（默认）
.flex_col()          // 垂直方向
.flex_row_reverse()  // 水平反向
.flex_col_reverse()  // 垂直反向
```

### 主轴对齐 (justify)

```rust
.justify_start()    // 开始对齐
.justify_center()   // 居中对齐
.justify_end()      // 结束对齐
.justify_between()  // 两端对齐，项目之间等距
.justify_around()   // 每个项目两侧等距
```

### 交叉轴对齐 (items)

```rust
.items_start()     // 开始对齐
.items_center()    // 居中对齐
.items_end()       // 结束对齐
.items_baseline()  // 基线对齐
```

### 弹性属性

```rust
.flex_1()       // flex: 1 1 0% - 忽略初始大小，平均分配空间
.flex_auto()    // flex: 1 1 auto - 考虑初始大小
.flex_none()    // flex: 0 0 auto - 不增长不收缩
.flex_grow()    // 允许增长填充空间
.flex_shrink()  // 允许收缩
.flex_wrap()    // 允许换行
```

### 居中技巧

```rust
// 完美居中
div()
    .flex()
    .items_center()
    .justify_center()
    .child("居中内容")
```

## 二、尺寸

### 固定尺寸

```rust
.w(px(100.0))       // 宽度 100 像素
.h(px(50.0))        // 高度 50 像素
.size(px(100.0))    // 宽高都是 100
.min_w(px(50.0))    // 最小宽度
.max_w(px(200.0))   // 最大宽度
.min_h(px(50.0))    // 最小高度
.max_h(px(200.0))   // 最大高度
```

### 相对尺寸

```rust
.w_full()      // 宽度 100%
.h_full()      // 高度 100%
.size_full()   // 宽高都是 100%
```

## 三、间距

### 内边距 (Padding)

```rust
// 四周
.p_1()   // 4px
.p_2()   // 8px
.p_4()   // 16px
.p_8()   // 32px

// 方向
.px_4()  // 水平方向（左右）
.py_2()  // 垂直方向（上下）
.pt_2()  // 顶部
.pb_2()  // 底部
.pl_2()  // 左侧
.pr_2()  // 右侧
```

### 外边距 (Margin)

```rust
.m_4()   // 四周外边距
.mx_4()  // 水平外边距
.my_2()  // 垂直外边距
.mt_2()  // 顶部
.mb_2()  // 底部
.ml_2()  // 左侧
.mr_2()  // 右侧
```

### 间隙 (Gap)

用于 flex/grid 子元素之间的间距：

```rust
.gap_1()   // 4px
.gap_2()   // 8px
.gap_4()   // 16px
.gap_8()   // 32px
```

## 四、颜色

### 颜色函数

```rust
rgb(0xRRGGBB)         // RGB 颜色，如 rgb(0xFF5500)
rgba(0xRRGGBBAA)      // RGBA 颜色，如 rgba(0xFF550080)
hsla(h, s, l, a)      // HSLA 颜色
```

### 应用颜色

```rust
.bg(rgb(0xFFFFFF))              // 背景色
.text_color(rgb(0x000000))      // 文字颜色
.border_color(rgb(0xCCCCCC))    // 边框颜色
```

## 五、边框与圆角

### 边框宽度

```rust
.border_1()   // 1px 边框
.border_2()   // 2px 边框
.border_4()   // 4px 边框
```

### 圆角

```rust
.rounded(px(4.0))  // 自定义圆角
.rounded_md()      // 中等圆角 (6px)
.rounded_lg()      // 大圆角 (8px)
.rounded_xl()      // 超大圆角 (12px)
.rounded_full()    // 完全圆形
```

### 边框样式

```rust
.border_dashed()   // 虚线边框
```

## 六、阴影

```rust
.shadow_sm()   // 小阴影
.shadow_md()   // 中等阴影
.shadow_lg()   // 大阴影
.shadow_xl()   // 超大阴影
```

## 七、文字样式

### 字体大小

```rust
.text_xs()     // 0.75rem - 12px
.text_sm()     // 0.875rem - 14px
.text_base()   // 1rem - 16px
.text_lg()     // 1.125rem - 18px
.text_xl()     // 1.25rem - 20px
.text_2xl()    // 1.5rem - 24px
.text_3xl()    // 1.875rem - 30px
```

### 字体粗细

```rust
.font_weight(FontWeight::THIN)
.font_weight(FontWeight::LIGHT)
.font_weight(FontWeight::NORMAL)
.font_weight(FontWeight::MEDIUM)
.font_weight(FontWeight::SEMIBOLD)
.font_weight(FontWeight::BOLD)
.font_weight(FontWeight::EXTRA_BOLD)
```

### 文字装饰

```rust
.italic()         // 斜体
.underline()      // 下划线
.line_through()   // 删除线
.truncate()       // 超出截断显示省略号
```

### 文字对齐

```rust
.text_left()     // 左对齐
.text_center()   // 居中
.text_right()    // 右对齐
```

## 八、伪状态

用于响应鼠标交互的动态样式：

```rust
div()
    .bg(rgb(0x3B82F6))
    .hover(|s| s.bg(rgb(0x2563EB)))    // 鼠标悬停
    .active(|s| s.bg(rgb(0x1D4ED8)))   // 鼠标按下
```

**注意**：使用 `hover`/`active` 需要元素有 `id`：

```rust
div()
    .id("my-button")  // 必须有 id
    .cursor_pointer()
    .hover(|s| s.bg(rgb(0xEEEEEE)))
```

## 九、条件样式

使用 `when` 根据条件应用样式：

```rust
use gpui::prelude::FluentBuilder;  // 需要导入

div()
    .when(is_active, |s| {
        s.bg(rgb(0x3B82F6)).text_color(rgb(0xFFFFFF))
    })
    .when(!is_active, |s| {
        s.bg(rgb(0xE2E8F0)).text_color(rgb(0x374151))
    })
```

### when_some

当 Option 有值时应用样式：

```rust
.when_some(self.color, |s, color| s.bg(color))
```

## 十、溢出与滚动

```rust
.overflow_hidden()     // 隐藏溢出内容
.overflow_x_scroll()   // 水平滚动
.overflow_y_scroll()   // 垂直滚动（最常用）
.overflow_scroll()     // 双向滚动
```

**⚠️ 重要**：滚动相关的方法需要元素有 `id` 才能工作！

```rust
// ❌ 错误 - 无法滚动
div()
    .overflow_y_scroll()
    .child(...)

// ✅ 正确 - 必须有 id
div()
    .id("scrollable-container")
    .overflow_y_scroll()
    .child(...)
```

## 十一、透明度

```rust
.opacity(0.5)   // 50% 透明度
```

## 十二、光标样式

```rust
.cursor_pointer()   // 手型光标
.cursor_default()   // 默认光标
.cursor_text()      // 文本光标
```

## 十三、调试

仅在 debug 构建中可用：

```rust
#[cfg(debug_assertions)]
div().debug()         // 显示元素边框
     .debug_below()   // 显示所有子元素边框
```

## 完整示例

运行本章示例：

```bash
cargo run -p gpui_styling
```

示例展示：
- Flexbox 布局切换（Row/Column/Wrap/Grid）
- 按钮组件库（变体、尺寸、状态）
- 文字样式展示
- 卡片与阴影效果
- 间距与尺寸演示
- 边框与圆角展示

## 常用模式

### 卡片组件

```rust
div()
    .p_4()
    .rounded_lg()
    .bg(rgb(0xFFFFFF))
    .shadow_md()
    .child("卡片内容")
```

### 按钮组件

```rust
div()
    .id("my-btn")
    .px_4()
    .py_2()
    .rounded_md()
    .bg(rgb(0x3B82F6))
    .text_color(rgb(0xFFFFFF))
    .cursor_pointer()
    .hover(|s| s.bg(rgb(0x2563EB)))
    .active(|s| s.bg(rgb(0x1D4ED8)))
    .child("点击我")
```

### 响应式标签

```rust
fn tag(label: &str, is_active: bool) -> impl IntoElement {
    div()
        .px_2()
        .py_1()
        .rounded_md()
        .text_sm()
        .when(is_active, |s| s.bg(rgb(0x3B82F6)).text_color(rgb(0xFFFFFF)))
        .when(!is_active, |s| s.bg(rgb(0xE5E7EB)).text_color(rgb(0x374151)))
        .child(label.to_string())
}
```

## 下一章预告

第五章将学习 GPUI 的事件处理系统，包括鼠标事件、键盘事件、事件传播等。

