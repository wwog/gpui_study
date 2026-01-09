# 创建子视图的所有方式 - 完整指南

## 🎯 核心问题

**"除了 cx.new() 还有什么方式创建子视图？"**

答案是：**8 种方式**！让我们逐一详解。

---

## 📚 方式对比总览

| 方式 | 语法示例 | 有状态 | 适用场景 | 推荐度 |
|------|---------|--------|---------|--------|
| 1. cx.new() | `cx.new(\|_\| View{})` | ✅ | 可复用组件 | ⭐⭐⭐⭐⭐ |
| 2. 内联 div | `div().child("text")` | ❌ | 简单布局 | ⭐⭐⭐ |
| 3. RenderOnce | `impl RenderOnce` | ❌ | 一次性组件 | ⭐⭐⭐⭐ |
| 4. 函数返回 | `fn create() -> Div` | ❌ | 工具函数 | ⭐⭐⭐⭐ |
| 5. 字符串 | `.child("hello")` | ❌ | 纯文本 | ⭐⭐⭐⭐⭐ |
| 6. 条件渲染 | `if x { a } else { b }` | - | 动态显示 | ⭐⭐⭐⭐ |
| 7. 迭代器 | `.children(iter)` | - | 列表数据 | ⭐⭐⭐⭐⭐ |
| 8. 嵌套 View | `View { child: View }` | ✅ | 复杂应用 | ⭐⭐⭐⭐ |

---

## 方式 1: cx.new() 创建 View<T> ⭐⭐⭐⭐⭐

### 特点
- **有状态** - 可以保存和修改数据
- **独立更新** - 调用 `cx.notify()` 触发重新渲染
- **生命周期管理** - GPUI 自动管理内存
- **最常用** - 正式项目的首选方式

### 代码示例

```rust
// 1. 定义视图结构体
struct Counter {
    count: i32,
}

// 2. 实现 Render trait
impl Render for Counter {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(format!("Count: {}", self.count))
    }
}

// 3. 在父组件中使用 cx.new() 创建
impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(cx.new(|_| Counter { count: 0 }))    // ← 关键代码
            .child(cx.new(|_| Counter { count: 10 }))   // 创建多个实例
    }
}
```

### 何时使用
- ✅ 需要独立状态的组件
- ✅ 可复用的 UI 组件
- ✅ 需要响应用户交互
- ✅ 正式项目中的所有组件

### 核心 API

```rust
cx.new(|_cx| ViewStruct {
    field1: value1,
    field2: value2,
})
```

**返回值**: `View<ViewStruct>` (实现了 `IntoElement`)

---

## 方式 2: 内联 div() ⭐⭐⭐

### 特点
- **无需结构体** - 直接创建 UI
- **简单直观** - 代码量少
- **无状态** - 不能保存数据
- **不可复用** - 每次都要重写

### 代码示例

```rust
impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            // 直接创建 div，无需额外组件
            .child(
                div()
                    .p_4()
                    .bg(rgb(0xE0F2FE))
                    .rounded_lg()
                    .child("这是内联创建的元素")
            )
            .child(
                div()
                    .p_4()
                    .bg(rgb(0xFEE2E2))
                    .child("另一个内联元素")
            )
    }
}
```

### 何时使用
- ✅ 快速原型开发
- ✅ 简单的容器和布局
- ✅ 一次性的 UI 元素
- ❌ 避免用于可复用组件

---

## 方式 3: RenderOnce trait ⭐⭐⭐⭐

### 特点
- **一次性渲染** - 消耗 `self` 而不是 `&mut self`
- **性能更好** - 不需要保留状态
- **无法更新** - 渲染后即销毁
- **类似函数组件** - 像 React 的函数组件

### 代码示例

```rust
// 1. 定义结构体
struct Badge {
    text: String,
    color: Hsla,
}

// 2. 实现 RenderOnce（注意：self 而不是 &mut self）
impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .px_3()
            .py_1()
            .bg(self.color)
            .rounded_full()
            .text_sm()
            .child(self.text)  // 注意：self.text 被 move，不是借用
    }
}

// 3. 使用（直接传入实例）
impl Render for App {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(Badge {
                text: "新".to_string(),
                color: rgb(0xEF4444).into(),
            })
    }
}
```

### Render vs RenderOnce

```rust
// Render - 可以多次渲染
impl Render for MyView {
    fn render(&mut self, ...) -> impl IntoElement { ... }
    //         ^^^^^^^^^ 可变借用，可以修改状态
}

// RenderOnce - 只渲染一次
impl RenderOnce for MyComponent {
    fn render(self, ...) -> impl IntoElement { ... }
    //        ^^^^ 消耗自身，渲染后销毁
}
```

### 何时使用
- ✅ 纯展示型组件（无交互）
- ✅ 徽章、标签等小组件
- ✅ 性能敏感场景
- ❌ 需要状态更新的组件

---

## 方式 4: 函数返回元素 ⭐⭐⭐⭐

### 特点
- **函数式** - 简洁的函数定义
- **可复用** - 可以多次调用
- **可参数化** - 接受参数定制行为
- **无状态** - 不保存数据

### 代码示例

```rust
// 1. 定义返回元素的函数
fn create_button(label: String, color: Hsla) -> impl IntoElement {
    div()
        .px_4()
        .py_2()
        .bg(color)
        .rounded_md()
        .cursor_pointer()
        .child(label)
}

fn create_card(title: String, content: String) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .p_6()
        .bg(rgb(0xFFFFFF))
        .rounded_lg()
        .shadow_lg()
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .child(title)
        )
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x6B7280))
                .child(content)
        )
}

// 2. 使用这些函数
impl Render for App {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_4()
            .child(create_button("确定".to_string(), rgb(0x10B981).into()))
            .child(create_button("取消".to_string(), rgb(0xEF4444).into()))
            .child(create_card(
                "标题".to_string(),
                "这是卡片内容".to_string()
            ))
    }
}
```

### 何时使用
- ✅ 可复用的 UI 片段
- ✅ 工具函数库
- ✅ 需要参数定制的组件
- ✅ 构建组件库

### 与 RenderOnce 的区别

```rust
// 函数方式 - 更简单
fn create_label(text: String) -> impl IntoElement {
    div().child(text)
}

// RenderOnce - 更结构化
struct Label { text: String }
impl RenderOnce for Label {
    fn render(self, ...) -> impl IntoElement {
        div().child(self.text)
    }
}

// 使用对比
.child(create_label("Hello".to_string()))     // 函数方式
.child(Label { text: "Hello".to_string() })   // RenderOnce 方式
```

---

## 方式 5: 字符串和基本类型 ⭐⭐⭐⭐⭐

### 特点
- **最简单** - 直接传字符串
- **自动转换** - 实现了 `IntoElement`
- **高性能** - 无额外开销
- **最常用** - 显示文本的首选

### 支持的类型

```rust
// &str - 字符串切片
.child("Hello, GPUI!")

// String - 拥有所有权的字符串
.child(String::from("World"))

// 格式化字符串
.child(format!("Count: {}", 42))

// 可以显示的类型（通过 .to_string()）
.child(self.count.to_string())  // i32 → String
.child(self.price.to_string())  // f64 → String
```

### 代码示例

```rust
impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            // 直接传字符串字面量
            .child("欢迎使用 GPUI")
            // String 类型
            .child(String::from("这是第二行"))
            // 格式化字符串
            .child(format!("当前计数: {}", self.count))
            // 数字转字符串
            .child(self.count.to_string())
            // 复杂格式化
            .child(format!("用户: {} | 得分: {:.2}", self.name, self.score))
    }
}
```

### 何时使用
- ✅ 显示文本内容
- ✅ 显示数字、日期等
- ✅ 任何需要文本的地方

---

## 方式 6: 条件渲染 ⭐⭐⭐⭐

### 特点
- **动态显示** - 根据条件选择不同内容
- **类似三元运算符** - `if { a } else { b }`
- **返回相同类型** - 两个分支必须返回同类型

### 代码示例

```rust
impl Render for UserView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            // 方式 1: if-else 表达式
            .child(if self.is_logged_in {
                div()
                    .bg(rgb(0xD1FAE5))
                    .child("已登录 ✓")
            } else {
                div()
                    .bg(rgb(0xFEE2E2))
                    .child("未登录 ✗")
            })
            
            // 方式 2: match 表达式
            .child(match self.status {
                Status::Active => div().bg(rgb(0x10B981)).child("活跃"),
                Status::Idle => div().bg(rgb(0xF59E0B)).child("空闲"),
                Status::Offline => div().bg(rgb(0x6B7280)).child("离线"),
            })
            
            // 方式 3: 函数封装
            .child(self.render_badge())
    }
}

impl UserView {
    fn render_badge(&self) -> Div {
        if self.is_premium {
            div().bg(rgb(0xFBBF24)).child("⭐ VIP")
        } else {
            div().bg(rgb(0xD1D5DB)).child("普通用户")
        }
    }
}
```

### 重要：返回类型必须相同

```rust
// ✅ 正确 - 两个分支都返回 Div
.child(if condition {
    div().child("A")
} else {
    div().child("B")
})

// ❌ 错误 - 类型不匹配
.child(if condition {
    div().child("A")       // Div
} else {
    "B"                    // &str
})

// ✅ 修正 - 统一为 Div
.child(if condition {
    div().child("A")
} else {
    div().child("B")
})
```

### 何时使用
- ✅ 根据状态显示不同内容
- ✅ 显示/隐藏元素
- ✅ 切换样式或布局

---

## 方式 7: 迭代器 + .children() ⭐⭐⭐⭐⭐

### 特点
- **批量添加** - 一次添加多个元素
- **数据驱动** - 从数组/Vec 生成 UI
- **灵活转换** - 支持 map、filter 等操作
- **最常用于列表** - 渲染列表的标准方式

### 核心 API

```rust
.children(iterator)
```

- 接受任何 `Iterator<Item: IntoElement>`
- 自动展开所有元素
- 常与 `.iter()` 和 `.map()` 配合

### 代码示例

```rust
struct TodoList {
    items: Vec<String>,
}

impl Render for TodoList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            
            // 方式 1: 简单列表
            .children(
                self.items.iter().map(|item| {
                    div()
                        .p_2()
                        .bg(rgb(0xF3F4F6))
                        .rounded_md()
                        .child(item.clone())
                })
            )
            
            // 方式 2: 带索引
            .children(
                self.items.iter().enumerate().map(|(i, item)| {
                    div()
                        .flex()
                        .gap_2()
                        .child(format!("{}.", i + 1))
                        .child(item.clone())
                })
            )
            
            // 方式 3: 创建 View 实例
            .children(
                self.items.iter().map(|item| {
                    cx.new(|_| TodoItem {
                        text: item.clone(),
                        done: false,
                    })
                })
            )
    }
}
```

### 高级用法

```rust
// 过滤 + 映射
.children(
    self.items
        .iter()
        .filter(|item| item.is_visible)  // 过滤
        .map(|item| {
            div().child(&item.name)
        })
)

// 排序后渲染
.children(
    self.items
        .iter()
        .sorted_by_key(|item| &item.priority)
        .map(|item| render_item(item))
)

// 分组渲染
.children(
    self.items
        .iter()
        .chunk_by(|item| &item.category)
        .map(|(category, group)| {
            div()
                .child(category)
                .children(group.map(render_item))
        })
)
```

### 何时使用
- ✅ 渲染列表数据
- ✅ 动态数量的元素
- ✅ 从数组生成 UI
- ✅ 表格、网格等结构

---

## 方式 8: 嵌套 View 组合 ⭐⭐⭐⭐

### 特点
- **组件化** - 构建复杂的组件树
- **职责分离** - 每个组件管理自己的状态
- **可维护** - 代码结构清晰
- **可扩展** - 易于添加新功能

### 代码示例

```rust
// 子组件 1
struct Header {
    title: String,
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_2xl()
            .font_weight(FontWeight::BOLD)
            .child(&self.title)
    }
}

// 子组件 2
struct Sidebar {
    items: Vec<String>,
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(200.0))
            .bg(rgb(0xF3F4F6))
            .children(self.items.iter().map(|item| {
                div().p_2().child(item.clone())
            }))
    }
}

// 父组件 - 组合子组件
struct App {
    title: String,
    menu_items: Vec<String>,
}

impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            
            // 嵌套 Header 组件
            .child(cx.new(|_| Header {
                title: self.title.clone(),
            }))
            
            // 嵌套 Sidebar 组件
            .child(cx.new(|_| Sidebar {
                items: self.menu_items.clone(),
            }))
    }
}
```

### 更复杂的嵌套

```rust
struct Dashboard {
    // 可以在结构体中持有子 View 的引用（使用 Model）
    stats: Vec<StatData>,
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .grid()
            .grid_cols_3()
            .gap_4()
            // 多层嵌套
            .children(
                self.stats.iter().map(|stat| {
                    cx.new(|_| StatsCard {
                        title: stat.title.clone(),
                        value: stat.value,
                        // 继续嵌套
                        chart: cx.new(|_| MiniChart {
                            data: stat.history.clone(),
                        }),
                    })
                })
            )
    }
}
```

### 何时使用
- ✅ 复杂的应用结构
- ✅ 组件需要独立状态
- ✅ 团队协作开发
- ✅ 大型项目

---

## 🎯 选择指南

### 流程图

```
需要独立状态？
  ├─ 是 → 使用 cx.new() 创建 View
  └─ 否 ↓

需要复用？
  ├─ 是 → 
  │   ├─ 复杂逻辑 → RenderOnce
  │   └─ 简单片段 → 函数返回元素
  └─ 否 ↓

渲染列表？
  ├─ 是 → 使用 .children() + 迭代器
  └─ 否 ↓

纯文本？
  ├─ 是 → 直接传字符串
  └─ 否 → 内联 div()
```

### 快速决策表

| 需求 | 推荐方式 | 示例 |
|------|---------|------|
| 按钮、卡片等可复用组件 | cx.new() | `cx.new(\|_\| Button { ... })` |
| 渲染待办列表 | .children() | `.children(items.iter().map(...))` |
| 显示用户名 | 字符串 | `.child(&self.username)` |
| 根据登录状态显示 | 条件渲染 | `if is_logged { A } else { B }` |
| 创建标签库 | 函数 | `create_label("文本")` |
| 一次性徽章 | RenderOnce | `Badge { text: "新" }` |
| 快速布局容器 | 内联 div | `div().flex().child(...)` |

---

## 💡 最佳实践

### 1. 优先级原则

```rust
// 第一优先：需要状态 → cx.new()
.child(cx.new(|_| Counter { count: 0 }))

// 第二优先：无状态可复用 → 函数或 RenderOnce
.child(create_button("点击"))

// 第三优先：列表数据 → .children()
.children(items.iter().map(...))

// 最后：简单内容 → 字符串或内联 div
.child("纯文本")
.child(div().child("简单容器"))
```

### 2. 组合使用

```rust
impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            // 1. 字符串
            .child("应用标题")
            
            // 2. cx.new() 创建有状态组件
            .child(cx.new(|_| SearchBar { query: String::new() }))
            
            // 3. 条件渲染
            .child(if self.has_results {
                div().child("找到结果")
            } else {
                div().child("无结果")
            })
            
            // 4. .children() 渲染列表
            .children(
                self.results.iter().map(|item| {
                    // 5. 嵌套使用 cx.new()
                    cx.new(|_| ResultItem { data: item.clone() })
                })
            )
            
            // 6. 函数创建元素
            .child(create_footer())
    }
}
```

### 3. 性能优化

```rust
// ✅ 推荐：使用引用避免克隆
.children(self.items.iter().map(|item| {
    div().child(item.as_str())  // 借用
}))

// ❌ 避免：不必要的克隆
.children(self.items.iter().map(|item| {
    div().child(item.clone())  // 克隆
}))

// ✅ 推荐：条件性创建
.child(if self.show_details {
    Some(cx.new(|_| DetailView { ... }))
} else {
    None
})

// ❌ 避免：总是创建然后隐藏
.child(cx.new(|_| DetailView { ... }))
    .when(self.show_details, |div| div.visible())
```

---

## 🎓 总结

### 记住这些要点

1. **cx.new()** - 有状态组件的标准方式 ⭐⭐⭐⭐⭐
2. **字符串** - 显示文本最简单 ⭐⭐⭐⭐⭐
3. **.children()** - 渲染列表数据 ⭐⭐⭐⭐⭐
4. **条件渲染** - if-else 动态显示 ⭐⭐⭐⭐
5. **函数** - 可复用 UI 片段 ⭐⭐⭐⭐
6. **RenderOnce** - 一次性无状态组件 ⭐⭐⭐⭐
7. **内联 div** - 快速原型和简单布局 ⭐⭐⭐
8. **嵌套 View** - 复杂应用结构 ⭐⭐⭐⭐

### 核心原则

> **"实现 IntoElement 的都可以作为子元素"**

这包括：
- `View<T>` (通过 cx.new() 创建)
- `Div` (通过 div() 创建)
- `&str` 和 `String`
- 实现了 `RenderOnce` 的类型
- 函数返回的 `impl IntoElement`

---

## 🚀 下一步

运行示例代码查看所有方式：

```bash
cargo run --bin view_creation_methods
```

继续学习：
- 第二章：元素系统详解
- 第三章：状态管理和 cx.notify()
- 第五章：事件处理

**祝学习愉快！** 🎉