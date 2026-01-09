// Counter 示例 - 展示状态管理和事件处理

use gpui::*;

struct CounterView {
    count: i32,
}

impl Render for CounterView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0xF5F5F5))
            .child(
                // 减少按钮
                div()
                    .id("decrement")
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(48.0))
                    .h(px(48.0))
                    .bg(rgb(0xEF4444))
                    .text_color(rgb(0xFFFFFF))
                    .text_xl()
                    .rounded_lg()
                    .cursor_pointer()
                    .hover(|style| style.bg(rgb(0xDC2626)))
                    .active(|style| style.bg(rgb(0xB91C1C)))
                    .child("-")
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.count -= 1;
                        cx.notify();
                    })),
            )
            .child(
                // 计数显示
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .min_w(px(100.0))
                    .h(px(48.0))
                    .px_4()
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xE5E7EB))
                    .rounded_lg()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1F2937))
                    .child(format!("{}", self.count)),
            )
            .child(
                // 增加按钮
                div()
                    .id("increment")
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(48.0))
                    .h(px(48.0))
                    .bg(rgb(0x10B981))
                    .text_color(rgb(0xFFFFFF))
                    .text_xl()
                    .rounded_lg()
                    .cursor_pointer()
                    .hover(|style| style.bg(rgb(0x059669)))
                    .active(|style| style.bg(rgb(0x047857)))
                    .child("+")
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.count += 1;
                        cx.notify();
                    })),
            )
            .child(
                // 重置按钮
                div()
                    .id("reset")
                    .flex()
                    .items_center()
                    .justify_center()
                    .px_4()
                    .h(px(36.0))
                    .ml_4()
                    .bg(rgb(0x6B7280))
                    .text_color(rgb(0xFFFFFF))
                    .rounded_md()
                    .cursor_pointer()
                    .hover(|style| style.bg(rgb(0x4B5563)))
                    .active(|style| style.bg(rgb(0x374151)))
                    .child("Reset")
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.count = 0;
                        cx.notify();
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point {
                        x: px(200.0),
                        y: px(200.0),
                    },
                    size: Size {
                        width: px(500.0),
                        height: px(300.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Counter App".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| CounterView { count: 0 }),
        )
        .unwrap();
    });
}
