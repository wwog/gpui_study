// 第二章：元素系统
// 文件：main.rs

use gpui::*;

struct ElementsDemo;

impl Render for ElementsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xF0F0F0))
            .child("第二章：元素系统 - 即将上线")
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| ElementsDemo)
        })
        .unwrap();
    });
}
