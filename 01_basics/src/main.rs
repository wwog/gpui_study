// 第一章：GPUI 基础概念
// 文件：main.rs - 演示如何组合多个视图

use gpui::*;

// 定义欢迎视图
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
            .border_1()
            .border_color(rgb(0x0EA5E9))
            .child(format!("欢迎, {}! 👋", self.name))
    }
}

// 定义根视图
struct HelloView {
    // 💡 方式1：在结构体中持有子视图的数据
    user_names: Vec<String>,
}

impl Render for HelloView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0xF8FAFC))
            .p_8()
            // 主标题
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1E293B))
                    .child("Hello, GPUI! 🎉"),
            )
            // 分隔线
            .child(div().w(px(200.0)).h(px(2.0)).bg(rgb(0xCBD5E1)).my_4())
            // ✨ 方式1：直接渲染 WelcomeView 的内容（内联）
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child("【方式1：内联渲染】")
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(rgb(0xDCFCE7))
                            .rounded_lg()
                            .child("欢迎, 小明! 👋"),
                    )
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(rgb(0xDCFCE7))
                            .rounded_lg()
                            .child("欢迎, 小红! 👋"),
                    ),
            )
            .child(div().w(px(200.0)).h(px(1.0)).bg(rgb(0xE2E8F0)).my_2())
            // ✨ 方式2：使用 cx.new() 创建子视图实例
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child("【方式2：使用 View (推荐)】")
                    .child(cx.new(|_| WelcomeView {
                        name: "张三".to_string(),
                    }))
                    .child(cx.new(|_| WelcomeView {
                        name: "李四".to_string(),
                    })),
            )
            .child(div().w(px(200.0)).h(px(1.0)).bg(rgb(0xE2E8F0)).my_2())
            // ✨ 方式3：使用循环动态生成多个子视图
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child("【方式3：循环生成】")
                    .children(
                        self.user_names
                            .iter()
                            .map(|name| cx.new(|_| WelcomeView { name: name.clone() })),
                    ),
            )
            // 底部说明
            .child(
                div()
                    .mt_8()
                    .text_sm()
                    .text_color(rgb(0x64748B))
                    .child("💡 提示：以上演示了三种组合视图的方法"),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point {
                        x: px(100.0),
                        y: px(100.0),
                    },
                    size: Size {
                        width: px(600.0),
                        height: px(700.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("第一章：组合视图示例".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| {
                cx.new(|_cx| HelloView {
                    user_names: vec!["王五".to_string(), "赵六".to_string(), "孙七".to_string()],
                })
            },
        )
        .unwrap();
    });
}

/* 🎓 知识点总结：

方式1：内联渲染
--------------
直接在父视图中创建 div 并设置样式，不使用独立的 View 组件。
优点：简单直接，适合一次性的 UI
缺点：代码不可复用，难以维护

方式2：使用 cx.new() 创建 View 实例（推荐）
----------------------------------------
使用 cx.new(|_| WelcomeView { ... }) 创建独立的视图组件。
优点：
  - 代码复用性强
  - 每个组件有自己的状态和生命周期
  - 便于维护和测试
  - 组件可以独立更新（通过 cx.notify()）
缺点：稍微复杂一点

方式3：使用 .children() 批量添加
--------------------------------
使用迭代器和 .children() 方法动态生成多个子元素。
优点：
  - 适合列表数据
  - 代码简洁
  - 容易处理动态数量的元素
缺点：需要理解迭代器

关键 API：
----------
- .child(element)       : 添加单个子元素
- .children(iterator)   : 添加多个子元素（接收迭代器）
- cx.new(|_| View)      : 创建新的视图实例
- .map()                : 将数据转换为元素

最佳实践：
----------
1. 可复用的 UI 组件 → 使用独立的 View 结构体
2. 简单的一次性 UI → 直接使用 div()
3. 列表数据 → 使用 .children() + .map()
4. 需要独立状态的组件 → 必须使用 View + cx.new()

运行命令：
----------
cargo run -p gpui_basics

*/
