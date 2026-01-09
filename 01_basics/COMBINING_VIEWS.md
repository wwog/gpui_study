# 视图组合详解 - 如何追加多个 View

## 🎯 学习目标

掌握在 GPUI 中组合多个视图的三种主要方法，理解何时使用每种方法。

## 📚 核心概念

在 GPUI 中，**组合视图**是构建复杂 UI 的基础。就像搭积木一样，我们可以把小的视图组件组合成大的界面。

---

## 方法一：内联渲染（适合简单场景）

### 什么时候使用？
- 一次性的 UI 元素
- 不需要复用的简单布局
- 快速原型开发

### 示例代码

```rust
impl Render for HelloView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            // 直接创建 div，不使用独立组件
            .child(
                div()
                    .px_4()
                    .py_2()
                    .bg(rgb(0xDCFCE7))
                    .rounded_lg()
                    .child("欢迎, 小明!")
            )
            .child(
                div()
                    .px_4()
                    .py_2()
                    .bg(rgb(0xDCFCE7))
                    .rounded_lg()
                    .child("欢迎, 小红!")
            )
    }
}
```

### 优点 ✅
- 代码简单直接
- 适合快速开发
- 无需额外结构体

### 缺点 ❌
- 代码重复
- 不能复用
- 难以维护
- 无法独立管理状态

---

## 方法二：使用 cx.new() 创建 View 实例（推荐⭐）

### 什么时候使用？
- 需要复用的组件
- 有独立状态的组件
- 复杂的 UI 模块
- 团队协作项目

### 示例代码

```rust
// 1. 定义独立的 View 组件
struct WelcomeView {
    name: String,
}

impl Render for WelcomeView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_4()
            .py_2()
            .bg(rgb(0xE0F2FE))
            .rounded_lg()
            .child(format!("欢迎, {}! 👋", self.name))
    }
}

// 2. 在父 View 中使用
impl Render for HelloView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            // 使用 cx.new() 创建子视图
            .child(cx.new(|_| WelcomeView {
                name: "张三".to_string(),
            }))
            .child(cx.new(|_| WelcomeView {
                name: "李四".to_string(),
            }))
    }
}
```

### 核心 API：cx.new()

```rust
cx.new(|_cx| ViewStruct { field: value })
```

**参数说明：**
- 闭包接收 `&mut Context<ViewStruct>`
- 返回视图实例
- GPUI 自动管理生命周期

### 优点 ✅
- **代码复用** - 同一组件可以多次使用
- **独立状态** - 每个实例有自己的状态
- **独立更新** - 可以调用 `cx.notify()` 单独更新
- **易于维护** - 组件职责清晰
- **可测试** - 组件可以独立测试

### 缺点 ❌
- 需要定义额外的结构体
- 略微增加代码量（但这是值得的！）

---

## 方法三：使用 .children() 批量添加（适合列表）

### 什么时候使用？
- 渲染列表数据
- 动态数量的元素
- 从数组/Vec 生成 UI

### 示例代码

```rust
struct HelloView {
    user_names: Vec<String>,
}

impl Render for HelloView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            // 使用 .children() 批量添加
            .children(
                self.user_names
                    .iter()
                    .map(|name| {
                        cx.new(|_| WelcomeView {
                            name: name.clone()
                        })
                    })
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| HelloView {
                user_names: vec![
                    "王五".to_string(),
                    "赵六".to_string(),
                    "孙七".to_string(),
                ],
            })
        }).unwrap();
    });
}
```

### 核心 API：.children()

```rust
.children(iterator)
```

**参数说明：**
- 接收任何实现了 `Iterator<Item: IntoElement>` 的迭代器
- 自动展开所有元素
- 常与 `.iter()` 和 `.map()` 配合使用

### 优点 ✅
- 处理动态数据非常方便
- 代码简洁
- 适合列表、表格等场景
- 易于过滤和转换数据

### 缺点 ❌
- 需要理解迭代器
- 每次渲染都会重新创建（性能考虑，后续章节会讲优化）

---

## 🔥 实战示例：完整的组合视图

```rust
use gpui::*;

// 用户卡片组件
struct UserCard {
    name: String,
    role: String,
}

impl Render for UserCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p_4()
            .bg(rgb(0xFFFFFF))
            .rounded_lg()
            .shadow_md()
            .border_1()
            .border_color(rgb(0xE5E7EB))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1F2937))
                    .child(&self.name)
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6B7280))
                    .child(&self.role)
            )
    }
}

// 团队视图
struct TeamView {
    team_name: String,
    members: Vec<(String, String)>,  // (name, role)
}

impl Render for TeamView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .p_8()
            .bg(rgb(0xF9FAFB))
            // 标题
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .mb_6()
                    .child(format!("团队：{}", self.team_name))
            )
            // 成员列表（使用 .children() 批量添加）
            .child(
                div()
                    .flex()
                    .gap_4()
                    .children(
                        self.members.iter().map(|(name, role)| {
                            cx.new(|_| UserCard {
                                name: name.clone(),
                                role: role.clone(),
                            })
                        })
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| TeamView {
                team_name: "开发团队".to_string(),
                members: vec![
                    ("张三".to_string(), "前端工程师".to_string()),
                    ("李四".to_string(), "后端工程师".to_string()),
                    ("王五".to_string(), "UI 设计师".to_string()),
                ],
            })
        }).unwrap();
    });
}
```

---

## 📊 方法对比表

| 特性 | 方法一：内联 | 方法二：cx.new() | 方法三：.children() |
|------|------------|-----------------|-------------------|
| 代码复用 | ❌ 不可复用 | ✅ 高复用性 | ✅ 适合列表 |
| 独立状态 | ❌ 无 | ✅ 有 | ✅ 有 |
| 代码简洁 | ✅ 简单 | ⭕ 中等 | ✅ 简洁 |
| 维护性 | ❌ 差 | ✅ 优秀 | ✅ 良好 |
| 学习曲线 | ✅ 简单 | ⭕ 中等 | ⭕ 需理解迭代器 |
| 适用场景 | 原型/一次性 | 正式项目 | 列表/动态数据 |
| 推荐度 | ⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |

---

## 🎯 最佳实践建议

### 1. 优先使用 cx.new() 创建组件

```rust
// ✅ 推荐
.child(cx.new(|_| MyComponent { ... }))

// ❌ 避免（除非非常简单）
.child(div().child("hardcoded content"))
```

### 2. 列表数据使用 .children()

```rust
// ✅ 推荐
.children(items.iter().map(|item| cx.new(|_| ItemView { item })))

// ❌ 避免
.child(cx.new(|_| items[0]))
.child(cx.new(|_| items[1]))
.child(cx.new(|_| items[2]))
```

### 3. 合理拆分组件

```rust
// ✅ 推荐：职责单一
struct UserAvatar { ... }
struct UserInfo { ... }
struct UserCard { avatar: View<UserAvatar>, info: View<UserInfo> }

// ❌ 避免：所有逻辑都在一个组件里
struct GiantComponent { /* 1000 行代码 */ }
```

### 4. 组件命名清晰

```rust
// ✅ 推荐
struct TodoListItem { ... }
struct TodoList { ... }

// ❌ 避免
struct View1 { ... }
struct MyView { ... }
```

---

## 🧪 练习题

### 练习 1：创建博客文章列表

创建一个博客应用，包含：
- `BlogPost` 组件（标题、作者、日期）
- `BlogList` 组件（显示多篇文章）
- 使用 `.children()` 渲染文章列表

**提示：**
```rust
struct BlogPost {
    title: String,
    author: String,
    date: String,
}

struct BlogList {
    posts: Vec<BlogPost>,
}
```

### 练习 2：创建导航菜单

创建一个导航菜单，包含：
- `MenuItem` 组件（图标、标题）
- `NavBar` 组件（横向排列多个菜单项）

**提示：** 使用 `.flex_row()` 实现横向布局

### 练习 3：嵌套组合

创建一个仪表盘，包含：
- `StatsCard` 组件（显示统计数据）
- `StatsRow` 组件（一行显示 3 个卡片）
- `Dashboard` 组件（显示多行统计）

**挑战：** 实现三层嵌套的组件结构

---

## 🔍 常见问题

### Q1: child() 和 children() 有什么区别？

**A:** 
- `.child(element)` - 添加**单个**元素
- `.children(iterator)` - 添加**多个**元素（批量）

```rust
// child() - 单个
.child(div().child("Hello"))

// children() - 多个
.children(vec!["A", "B", "C"].iter().map(|s| div().child(*s)))
```

### Q2: cx.new() 中的闭包参数是什么？

**A:** 闭包接收 `&mut Context<ViewType>`，可以用来：
- 创建子视图
- 访问上下文信息
- 设置初始状态

通常用 `_cx` 表示暂时不用。

### Q3: 如何在 children() 中访问索引？

**A:** 使用 `.enumerate()`：

```rust
.children(
    items.iter().enumerate().map(|(index, item)| {
        cx.new(|_| ItemView { index, item: item.clone() })
    })
)
```

### Q4: View 创建后可以修改吗？

**A:** 不能直接修改。但可以：
1. 改变父组件的状态，重新渲染时创建新的子组件
2. 使用 `Model<T>` 共享状态（第三章会讲）

### Q5: 性能会不会有问题？

**A:** GPUI 有优化机制：
- 只有状态变化的组件才重新渲染
- 使用 `cx.notify()` 精确控制更新
- 后续章节会讲性能优化技巧

---

## 📝 本节总结

✅ 学会了三种组合视图的方法  
✅ 理解了 `cx.new()` 创建组件实例  
✅ 掌握了 `.children()` 批量添加元素  
✅ 知道了何时使用哪种方法  

**核心要点：**
- 可复用组件 → 使用 `cx.new()`
- 列表数据 → 使用 `.children()`
- 简单 UI → 直接用 `div()`

---

## ➡️ 下一步

完成练习题后，你可以：
1. 继续学习第二章（元素系统）
2. 查看 `examples/` 目录的更多示例
3. 尝试构建自己的组件库

**运行本章代码：**
```bash
cd 01_basics
cargo run
```

---

**恭喜！你已经掌握了视图组合的核心技能！** 🎉