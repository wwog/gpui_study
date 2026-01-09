# 第二章快速开始指南

## 🚀 5分钟快速上手

### 1. 运行主程序查看概览

```bash
cd 02_elements
cargo run
```

这会打开一个概览界面，展示所有学习步骤。

### 2. 按步骤学习

#### 步骤 2：div() 基础
```bash
cargo run --bin step2_div_basics
```

**学习内容：**
- 创建基础容器
- 链式调用方法
- 设置样式属性

**核心代码：**
```rust
div()
    .w(px(200.0))          // 宽度
    .h(px(100.0))          // 高度
    .bg(rgb(0x3B82F6))     // 背景色
    .child("内容")          // 子元素
```

---

#### 步骤 3：child() 添加子元素
```bash
cargo run --bin step3_child
```

**学习内容：**
- 使用 child() 添加单个子元素
- 嵌套 div 结构
- 构建卡片布局

**核心代码：**
```rust
div()
    .child("第一个子元素")
    .child(div().child("嵌套的 div"))
    .child("第三个子元素")
```

---

#### 步骤 4：children() 批量添加
```bash
cargo run --bin step4_children
```

**学习内容：**
- 使用迭代器批量添加
- map、filter 等操作
- 从数据生成 UI

**核心代码：**
```rust
let items = vec!["A", "B", "C"];

div()
    .children(
        items.iter().map(|item| {
            div().child(*item)
        })
    )
```

---

#### 步骤 5：嵌套与组合
```bash
cargo run --bin step5_nesting
```

**学习内容：**
- 多层嵌套结构
- 复杂布局模式
- 组件组合

**展示内容：**
- 三层布局（导航+侧边栏+内容）
- 卡片网格
- 个人资料页面
- 仪表盘

---

## 📝 核心 API 速查

| API | 说明 | 示例 |
|-----|------|------|
| `div()` | 创建容器 | `div().child("text")` |
| `.child(element)` | 添加单个子元素 | `.child(div())` |
| `.children(iter)` | 批量添加子元素 | `.children(vec.iter().map(...))` |
| `.w(px(n))` | 设置宽度 | `.w(px(200.0))` |
| `.h(px(n))` | 设置高度 | `.h(px(100.0))` |
| `.bg(color)` | 设置背景色 | `.bg(rgb(0xFF0000))` |
| `.p_*()` | 设置内边距 | `.p_4()` |
| `.m_*()` | 设置外边距 | `.m_2()` |
| `.flex()` | 启用 flexbox | `.flex().flex_col()` |
| `.grid()` | 启用 grid | `.grid().grid_cols(3)` |

---

## 🎯 学习路径

```
1. 阅读 README.md (15分钟)
   了解整体概念和学习目标

2. 运行步骤 2 (20分钟)
   学习 div() 的基本用法
   完成练习：创建不同样式的 div

3. 运行步骤 3 (30分钟)
   学习 child() 添加子元素
   完成练习：创建卡片布局

4. 运行步骤 4 (30分钟)
   学习 children() 批量添加
   完成练习：从数据生成列表

5. 运行步骤 5 (40分钟)
   学习复杂嵌套和组合
   完成练习：构建完整页面

总计：约 2-3 小时
```

---

## ✅ 测验题目

### 测验 1：基础知识

**Q1: Element 和 IntoElement 的区别是什么？**

<details>
<summary>查看答案</summary>
Element 是最终的可渲染对象，IntoElement 是可以转换为 Element 的类型。
</details>

**Q2: 以下哪些类型实现了 IntoElement？**
- [ ] `&str`
- [ ] `String`
- [ ] `Div`
- [ ] `View<T>`
- [ ] `i32`

<details>
<summary>查看答案</summary>
前四个都实现了。i32 需要先转换为 String。
</details>

---

### 测验 2：实践操作

**创建一个包含以下内容的 div：**
- 宽度 300px
- 高度 200px
- 蓝色背景
- 包含两个子 div，分别显示 "标题" 和 "内容"

<details>
<summary>查看答案</summary>

```rust
div()
    .w(px(300.0))
    .h(px(200.0))
    .bg(rgb(0x3B82F6))
    .flex()
    .flex_col()
    .child(div().child("标题"))
    .child(div().child("内容"))
```
</details>

---

### 测验 3：迭代器应用

**从数组 `["红", "绿", "蓝"]` 生成三个彩色方块**

<details>
<summary>查看答案</summary>

```rust
let colors = vec![0xFF0000, 0x00FF00, 0x0000FF];
let labels = vec!["红", "绿", "蓝"];

div()
    .flex()
    .gap_2()
    .children(
        colors.iter().zip(labels.iter()).map(|(color, label)| {
            div()
                .w(px(50.0))
                .h(px(50.0))
                .bg(rgb(*color))
                .child(*label)
        })
    )
```
</details>

---

## 🔧 常见问题

### Q: 为什么我的代码无法编译？

**A:** 检查以下几点：
1. 是否导入了 `use gpui::*;`
2. 尺寸是否使用 `px()` 包裹：`.w(px(100.0))`
3. 颜色是否使用 `rgb()` 包裹：`.bg(rgb(0xFF0000))`
4. child() 的参数是否实现了 IntoElement

---

### Q: child() 和 children() 有什么区别？

**A:**
- `child()` - 添加**单个**子元素，可以多次调用
- `children()` - 接受**迭代器**，批量添加多个子元素

```rust
// 方式 1: 多次 child()
div()
    .child("A")
    .child("B")
    .child("C")

// 方式 2: 一次 children()
div()
    .children(vec!["A", "B", "C"].iter().map(|s| div().child(*s)))
```

---

### Q: 如何实现垂直/水平布局？

**A:** 使用 flex 布局：

```rust
// 垂直（默认）
div()
    .flex()
    .flex_col()
    .child("上")
    .child("下")

// 水平
div()
    .flex()
    .flex_row()  // 或者只用 .flex()，默认就是 row
    .child("左")
    .child("右")
```

---

### Q: 如何让元素居中？

**A:**

```rust
// 水平垂直居中
div()
    .size_full()
    .flex()
    .items_center()    // 垂直居中
    .justify_center()  // 水平居中
    .child("居中的内容")
```

---

## 💡 实用技巧

### 技巧 1：使用 .when() 条件样式

```rust
div()
    .bg(rgb(0xFFFFFF))
    .when(is_active, |div| div.bg(rgb(0x3B82F6)))
```

### 技巧 2：提取重复代码为函数

```rust
fn create_button(label: &str) -> impl IntoElement {
    div()
        .px_4()
        .py_2()
        .bg(rgb(0x3B82F6))
        .rounded_md()
        .child(label)
}

// 使用
div()
    .child(create_button("确定"))
    .child(create_button("取消"))
```

### 技巧 3：使用 .hover() 添加悬停效果

```rust
div()
    .bg(rgb(0x3B82F6))
    .hover(|style| style.bg(rgb(0x2563EB)))
    .child("鼠标悬停试试")
```

---

## 📚 推荐学习顺序

```
第一天：步骤 2 + 步骤 3
- 掌握 div() 和 child() 基础
- 完成基础练习

第二天：步骤 4
- 学习 children() 和迭代器
- 实现列表渲染

第三天：步骤 5
- 学习复杂布局
- 完成综合项目

第四天：实战
- 创建个人项目
- 巩固所学知识
```

---

## 🎉 完成检查清单

学完本章后，你应该能够：

- [ ] 使用 div() 创建基础容器
- [ ] 理解链式调用的原理
- [ ] 使用 child() 添加单个子元素
- [ ] 使用 children() 批量添加元素
- [ ] 构建 3 层以上的嵌套结构
- [ ] 从数组生成 UI 元素
- [ ] 创建卡片、列表等常见布局
- [ ] 完成所有练习题

---

## ➡️ 下一步

完成本章后，继续学习：

**第三章：状态管理**
- 学习如何让 UI 可以交互
- 掌握 cx.notify() 的使用
- 实现计数器、表单等交互组件

```bash
cd ../03_state_management
cat README.md
```

---

**祝学习愉快！有问题随时查看 README.md 或代码注释。** 🚀