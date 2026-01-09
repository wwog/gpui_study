// Todo List 示例 - 综合应用（占位）
// 本示例将在学习完前几章后实现

use gpui::*;

struct TodoApp;

impl Render for TodoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xF9FAFB))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_8()
                    .bg(rgb(0xFFFFFF))
                    .rounded_lg()
                    .shadow_lg()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1F2937))
                            .child("📝 Todo List")
                    )
                    .child(
                        div()
                            .text_color(rgb(0x6B7280))
                            .child("这个综合示例正在开发中...")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x9CA3AF))
                            .child("请先完成前几章的学习！")
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point {
                        x: px(300.0),
                        y: px(200.0),
                    },
                    size: Size {
                        width: px(500.0),
                        height: px(600.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Todo List - 即将上线".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| TodoApp),
        )
        .unwrap();
    });
}
