// 第五章：GPUI 事件处理
// ======================
// 本章学习 GPUI 的事件系统：
// 1. 点击事件 (on_click)
// 2. 鼠标事件 (mouse down/up/move)
// 3. 键盘事件 (key down/up)
// 4. 滚轮事件 (scroll wheel)
// 5. 悬停事件 (hover)
// 6. cx.listener() 的使用
// 7. 焦点与键盘输入

use gpui::prelude::FluentBuilder;
use gpui::*;

// ============================================================================
// 第一部分：简化版鼠标追踪器 - 展示鼠标事件
// ============================================================================

/// 鼠标追踪演示
struct MouseTracker {
    /// 鼠标位置（相对于窗口）
    mouse_position: Option<Point<Pixels>>,
    /// 是否正在按下
    is_pressing: bool,
    /// 点击次数
    click_count: i32,
    /// 最后一次事件
    last_event: String,
}

impl MouseTracker {
    fn new() -> Self {
        Self {
            mouse_position: None,
            is_pressing: false,
            click_count: 0,
            last_event: "等待鼠标事件...".to_string(),
        }
    }
}

impl Render for MouseTracker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mouse_pos = self.mouse_position;
        let is_pressing = self.is_pressing;

        div()
            .id("mouse-tracker")
            .w_full()
            .h(px(200.0))
            .bg(if is_pressing { rgb(0x1E3A8A) } else { rgb(0x1F2937) })
            .rounded_lg()
            .cursor_crosshair()
            .relative()
            .overflow_hidden()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            // 鼠标按下
            .on_mouse_down(MouseButton::Left, cx.listener(|view, event: &MouseDownEvent, _window, cx| {
                view.is_pressing = true;
                view.mouse_position = Some(event.position);
                view.last_event = format!("按下 @ ({:.0}, {:.0})", event.position.x, event.position.y);
                cx.notify();
            }))
            // 鼠标释放
            .on_mouse_up(MouseButton::Left, cx.listener(|view, event: &MouseUpEvent, _window, cx| {
                view.is_pressing = false;
                view.last_event = format!("释放 @ ({:.0}, {:.0})", event.position.x, event.position.y);
                cx.notify();
            }))
            // 鼠标移动
            .on_mouse_move(cx.listener(|view, event: &MouseMoveEvent, _window, cx| {
                view.mouse_position = Some(event.position);
                cx.notify();
            }))
            // 点击事件
            .on_click(cx.listener(|view, event: &ClickEvent, _window, cx| {
                view.click_count += 1;
                view.last_event = format!("点击 #{} @ ({:.0}, {:.0})", 
                    view.click_count, event.position().x, event.position().y);
                cx.notify();
            }))
            // 显示内容
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(format!("点击次数: {}", self.click_count)),
            )
            .child(
                div()
                    .text_color(rgb(0x9CA3AF))
                    .child(match mouse_pos {
                        Some(pos) => format!("鼠标位置: ({:.0}, {:.0})", pos.x, pos.y),
                        None => "鼠标未进入".to_string(),
                    }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(if is_pressing { rgb(0x60A5FA) } else { rgb(0x6B7280) })
                    .child(self.last_event.clone()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x4B5563))
                    .mt_4()
                    .child("💡 在此区域点击、拖动试试"),
            )
    }
}

// ============================================================================
// 第二部分：按钮演示 - 展示各种点击事件
// ============================================================================

/// 可交互按钮状态
struct ButtonDemo {
    /// 点击次数
    click_count: i32,
    /// 最后一次事件信息
    last_event: String,
    /// 修饰键状态
    modifiers_info: String,
}

impl ButtonDemo {
    fn new() -> Self {
        Self {
            click_count: 0,
            last_event: "等待交互...".to_string(),
            modifiers_info: String::new(),
        }
    }
}

impl Render for ButtonDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            // 主按钮
            .child(
                div()
                    .id("main-button")
                    .flex()
                    .items_center()
                    .justify_center()
                    .px_6()
                    .py_3()
                    .bg(rgb(0x3B82F6))
                    .text_color(rgb(0xFFFFFF))
                    .rounded_lg()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(0x2563EB)))
                    .active(|s| s.bg(rgb(0x1D4ED8)))
                    .child(format!("点击我！({}次)", self.click_count))
                    .on_click(cx.listener(|view, event: &ClickEvent, _window, cx| {
                        view.click_count += 1;
                        view.last_event = format!("点击位置: ({:.0}, {:.0})", 
                            event.position().x, event.position().y);
                        view.modifiers_info = format!(
                            "修饰键: Ctrl={}, Shift={}, Alt={}",
                            event.modifiers().control,
                            event.modifiers().shift,
                            event.modifiers().alt,
                        );
                        cx.notify();
                    })),
            )
            // 鼠标按钮演示
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .id("left-btn")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x10B981))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x059669)))
                            .child("左键按下")
                            .on_mouse_down(MouseButton::Left, cx.listener(|view, _event, _window, cx| {
                                view.last_event = "左键按下".to_string();
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .id("right-btn")
                            .px_4()
                            .py_2()
                            .bg(rgb(0xF59E0B))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0xD97706)))
                            .child("右键按下")
                            .on_mouse_down(MouseButton::Right, cx.listener(|view, _event, _window, cx| {
                                view.last_event = "右键按下".to_string();
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .id("middle-btn")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x8B5CF6))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x7C3AED)))
                            .child("中键按下")
                            .on_mouse_down(MouseButton::Middle, cx.listener(|view, _event, _window, cx| {
                                view.last_event = "中键按下".to_string();
                                cx.notify();
                            })),
                    ),
            )
            // 事件信息显示
            .child(
                div()
                    .p_3()
                    .bg(rgb(0xF3F4F6))
                    .rounded_md()
                    .text_sm()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(format!("最后事件: {}", self.last_event))
                            .when(!self.modifiers_info.is_empty(), |el| {
                                el.child(self.modifiers_info.clone())
                            }),
                    ),
            )
    }
}

// ============================================================================
// 第三部分：键盘事件演示
// ============================================================================

/// 键盘事件演示组件
struct KeyboardDemo {
    /// 按键历史
    key_history: Vec<String>,
    /// 当前按住的键
    held_keys: Vec<String>,
    /// 焦点句柄 - 必须存储以保持稳定
    focus_handle: FocusHandle,
}

impl KeyboardDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            key_history: Vec::new(),
            held_keys: Vec::new(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn add_key(&mut self, key: String) {
        self.key_history.push(key);
        if self.key_history.len() > 10 {
            self.key_history.remove(0);
        }
    }
}

impl Render for KeyboardDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 检查是否有焦点
        let has_focus = self.focus_handle.is_focused(window);

        div()
            .id("keyboard-area")
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .rounded_lg()
            .border_2()
            .cursor_pointer()
            .when(has_focus, |s| s.border_color(rgb(0x3B82F6)).bg(rgb(0xEFF6FF)))
            .when(!has_focus, |s| s.border_color(rgb(0xE5E7EB)).bg(rgb(0xFFFFFF)))
            // 键盘事件需要元素可聚焦 - 使用存储的 focus_handle
            .track_focus(&self.focus_handle)
            // 点击时获取焦点
            .on_click(cx.listener(|view, _event, window, cx| {
                view.focus_handle.focus(window);
                cx.notify();
            }))
            // 键盘按下事件
            .on_key_down(cx.listener(|view, event: &KeyDownEvent, _window, cx| {
                let key_str = format!("{}", event.keystroke);
                if !event.is_held {
                    view.add_key(key_str.clone());
                    if !view.held_keys.contains(&key_str) {
                        view.held_keys.push(key_str);
                    }
                }
                cx.notify();
            }))
            // 键盘释放事件
            .on_key_up(cx.listener(|view, event: &KeyUpEvent, _window, cx| {
                let key_str = format!("{}", event.keystroke);
                view.held_keys.retain(|k| k != &key_str);
                cx.notify();
            }))
            .child(
                div()
                    .text_sm()
                    .text_color(if has_focus { rgb(0x1D4ED8) } else { rgb(0x6B7280) })
                    .font_weight(if has_focus { FontWeight::BOLD } else { FontWeight::NORMAL })
                    .child(if has_focus {
                        "🎹 键盘区域已聚焦 - 按任意键！"
                    } else {
                        "👆 点击此区域以获取键盘焦点"
                    }),
            )
            // 当前按住的键
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .min_h(px(40.0))
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x374151))
                            .child("按住的键:"),
                    )
                    .when(self.held_keys.is_empty() && has_focus, |el| {
                        el.child(
                            div()
                                .text_sm()
                                .text_color(rgb(0x9CA3AF))
                                .italic()
                                .child("(按下键盘)")
                        )
                    })
                    .children(self.held_keys.iter().map(|key| {
                        div()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x3B82F6))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .child(key.clone())
                    })),
            )
            // 按键历史
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x374151))
                            .child(format!("按键历史 ({}):", self.key_history.len())),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_1()
                            .min_h(px(30.0))
                            .when(self.key_history.is_empty(), |el| {
                                el.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x9CA3AF))
                                        .italic()
                                        .child("(还没有按键记录)")
                                )
                            })
                            .children(self.key_history.iter().map(|key| {
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(rgb(0xE5E7EB))
                                    .text_color(rgb(0x374151))
                                    .rounded_md()
                                    .text_xs()
                                    .child(key.clone())
                            })),
                    ),
            )
    }
}

// ============================================================================
// 第四部分：滚轮事件演示
// ============================================================================

/// 滚轮事件演示
struct ScrollWheelDemo {
    /// 累计滚动量
    scroll_delta: Point<f32>,
    /// 滚动事件计数
    scroll_count: i32,
}

impl ScrollWheelDemo {
    fn new() -> Self {
        Self {
            scroll_delta: Point { x: 0.0, y: 0.0 },
            scroll_count: 0,
        }
    }

    fn reset(&mut self) {
        self.scroll_delta = Point { x: 0.0, y: 0.0 };
        self.scroll_count = 0;
    }
}

impl Render for ScrollWheelDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let offset_x = self.scroll_delta.x.clamp(-100.0, 100.0);
        let offset_y = self.scroll_delta.y.clamp(-100.0, 100.0);

        div()
            .id("scroll-area")
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .bg(rgb(0xFEF3C7))
            .rounded_lg()
            // 滚轮事件
            .on_scroll_wheel(cx.listener(|view, event: &ScrollWheelEvent, _window, cx| {
                view.scroll_count += 1;
                let delta = event.delta.pixel_delta(px(20.0));
                view.scroll_delta.x += f32::from(delta.x);
                view.scroll_delta.y += f32::from(delta.y);
                cx.notify();
            }))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x92400E))
                    .child("🖱️ 在此区域滚动鼠标滚轮"),
            )
            // 可视化滚动方向
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w_full()
                    .h(px(120.0))
                    .bg(rgb(0xFFFFFF))
                    .rounded_md()
                    .relative()
                    .overflow_hidden()
                    .child(
                        // 指示器
                        div()
                            .absolute()
                            .left(px(60.0) + px(offset_x))
                            .top(px(50.0) - px(offset_y))
                            .w(px(20.0))
                            .h(px(20.0))
                            .bg(rgb(0xF59E0B))
                            .rounded_full()
                    )
                    .child(
                        // 中心十字
                        div()
                            .absolute()
                            .left(px(68.0))
                            .top(px(10.0))
                            .w(px(4.0))
                            .h(px(100.0))
                            .bg(rgb(0xE5E7EB))
                    )
                    .child(
                        div()
                            .absolute()
                            .left(px(20.0))
                            .top(px(58.0))
                            .w(px(100.0))
                            .h(px(4.0))
                            .bg(rgb(0xE5E7EB))
                    ),
            )
            // 滚动信息
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_sm()
                    .text_color(rgb(0x78350F))
                    .child(format!("X: {:.1}, Y: {:.1}", self.scroll_delta.x, self.scroll_delta.y))
                    .child(format!("滚动次数: {}", self.scroll_count)),
            )
            // 重置按钮
            .child(
                div()
                    .id("reset-scroll")
                    .px_3()
                    .py_1()
                    .bg(rgb(0xF59E0B))
                    .text_color(rgb(0xFFFFFF))
                    .rounded_md()
                    .cursor_pointer()
                    .text_sm()
                    .text_center()
                    .hover(|s| s.bg(rgb(0xD97706)))
                    .child("重置")
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.reset();
                        cx.notify();
                    })),
            )
    }
}

// ============================================================================
// 第五部分：悬停状态演示
// ============================================================================

/// 悬停演示
struct HoverDemo {
    /// 当前悬停的项目索引
    hovered_index: Option<usize>,
    /// 悬停计数
    hover_count: i32,
}

impl HoverDemo {
    fn new() -> Self {
        Self { 
            hovered_index: None,
            hover_count: 0,
        }
    }
}

impl Render for HoverDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = [
            ("Red", rgb(0xEF4444)),
            ("Orange", rgb(0xF59E0B)),
            ("Green", rgb(0x10B981)),
            ("Blue", rgb(0x3B82F6)),
            ("Purple", rgb(0x8B5CF6)),
        ];

        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6B7280))
                    .child(match self.hovered_index {
                        Some(i) => format!("悬停在: {} (共{}次悬停)", colors[i].0, self.hover_count),
                        None => format!("将鼠标移到色块上 (共{}次悬停)", self.hover_count),
                    }),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .children(colors.iter().enumerate().map(|(i, (name, color))| {
                        let is_hovered = self.hovered_index == Some(i);
                        let color = *color;
                        
                        div()
                            .id(SharedString::from(format!("hover-{}", i)))
                            .w(px(70.0))
                            .h(px(70.0))
                            .rounded_lg()
                            .bg(color)
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            // hover 伪状态样式
                            .hover(|s| s.opacity(0.8).shadow_lg())
                            // 使用 when 根据状态变化
                            .when(is_hovered, |s| {
                                s.border_4().border_color(rgb(0xFFFFFF)).shadow_xl()
                            })
                            .child(*name)
                            // 鼠标进入 - 通过 mouse_move 检测
                            .on_mouse_move(cx.listener(move |view, _event, _window, cx| {
                                if view.hovered_index != Some(i) {
                                    view.hovered_index = Some(i);
                                    view.hover_count += 1;
                                    cx.notify();
                                }
                            }))
                    })),
            )
            // 清除悬停状态的区域
            .child(
                div()
                    .id("clear-hover")
                    .h(px(30.0))
                    .flex()
                    .items_center()
                    .text_xs()
                    .text_color(rgb(0x9CA3AF))
                    .child("(移到这里清除悬停状态)")
                    .on_mouse_move(cx.listener(|view, _event, _window, cx| {
                        if view.hovered_index.is_some() {
                            view.hovered_index = None;
                            cx.notify();
                        }
                    }))
            )
    }
}

// ============================================================================
// 第六部分：主应用
// ============================================================================

struct EventsApp {
    /// 鼠标追踪器
    mouse_tracker: Entity<MouseTracker>,
    /// 按钮演示
    button_demo: Entity<ButtonDemo>,
    /// 键盘演示
    keyboard_demo: Entity<KeyboardDemo>,
    /// 滚轮演示
    scroll_demo: Entity<ScrollWheelDemo>,
    /// 悬停演示
    hover_demo: Entity<HoverDemo>,
}

impl EventsApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let mouse_tracker = cx.new(|_cx| MouseTracker::new());
        let button_demo = cx.new(|_cx| ButtonDemo::new());
        let keyboard_demo = cx.new(KeyboardDemo::new);
        let scroll_demo = cx.new(|_cx| ScrollWheelDemo::new());
        let hover_demo = cx.new(|_cx| HoverDemo::new());

        Self {
            mouse_tracker,
            button_demo,
            keyboard_demo,
            scroll_demo,
            hover_demo,
        }
    }
}

impl Render for EventsApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main-container")
            .size_full()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .bg(rgb(0xF1F5F9))
            .p_6()
            .gap_6()
            // 标题
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(rgb(0x1E293B))
                            .child("第五章：事件处理 🎯"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x64748B))
                            .child("学习鼠标、键盘、滚轮等事件的处理方式"),
                    ),
            )
            // 主内容 - 两列布局
            .child(
                div()
                    .flex()
                    .gap_6()
                    // 左列
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_6()
                            // 鼠标追踪
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x1E293B))
                                            .child("🎨 鼠标追踪"),
                                    )
                                    .child(self.mouse_tracker.clone()),
                            )
                            // 按钮演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x1E293B))
                                            .child("🖱️ 点击事件"),
                                    )
                                    .child(self.button_demo.clone()),
                            ),
                    )
                    // 右列
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_6()
                            // 键盘演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x1E293B))
                                            .child("⌨️ 键盘事件（点击获取焦点）"),
                                    )
                                    .child(self.keyboard_demo.clone()),
                            )
                            // 滚轮演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x1E293B))
                                            .child("🖱️ 滚轮事件"),
                                    )
                                    .child(self.scroll_demo.clone()),
                            )
                            // 悬停演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(rgb(0xFFFFFF))
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x1E293B))
                                            .child("👆 悬停状态"),
                                    )
                                    .child(self.hover_demo.clone()),
                            ),
                    ),
            )
            // 知识点总结
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .bg(rgb(0xFEF3C7))
                    .border_1()
                    .border_color(rgb(0xFCD34D))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x92400E))
                            .child("💡 关键API：on_click | on_mouse_down/up/move | on_key_down/up | on_scroll_wheel | hover/active | cx.listener() | track_focus()"),
                    ),
            )
    }
}

// ============================================================================
// 主函数
// ============================================================================

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point {
                        x: px(50.0),
                        y: px(50.0),
                    },
                    size: Size {
                        width: px(1000.0),
                        height: px(850.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("第五章：事件处理".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(EventsApp::new),
        )
        .unwrap();
    });
}

/* ==========================================================================
   🎓 GPUI 事件处理知识点总结
   ==========================================================================

   一、事件类型
   -----------
   鼠标事件：
   - ClickEvent        点击（按下+释放）
   - MouseDownEvent    鼠标按下
   - MouseUpEvent      鼠标释放
   - MouseMoveEvent    鼠标移动
   - ScrollWheelEvent  滚轮滚动

   键盘事件：
   - KeyDownEvent      键盘按下
   - KeyUpEvent        键盘释放

   二、事件监听方法
   ----------------
   .on_click(cx.listener(|view, event, window, cx| { ... }))
   .on_mouse_down(MouseButton::Left, cx.listener(...))
   .on_mouse_up(MouseButton::Left, cx.listener(...))
   .on_mouse_move(cx.listener(...))
   .on_scroll_wheel(cx.listener(...))
   .on_key_down(cx.listener(...))
   .on_key_up(cx.listener(...))

   三、cx.listener() 的作用
   -----------------------
   将事件回调中的 &mut App 转换为 &mut Context<Self>，
   使得可以访问视图状态并调用 cx.notify()

   四、焦点管理（键盘事件必需）
   ---------------------------
   1. 在组件中存储 FocusHandle
      focus_handle: FocusHandle

   2. 在创建时初始化
      focus_handle: cx.focus_handle()

   3. 在渲染时关联
      .track_focus(&self.focus_handle)

   4. 点击时获取焦点
      .on_click(cx.listener(|view, _, window, cx| {
          view.focus_handle.focus(window);
          cx.notify();
      }))

   5. 检查焦点状态
      let has_focus = self.focus_handle.is_focused(window);

   五、重要注意事项
   ---------------
   1. on_click 需要元素有 id
   2. 键盘事件需要 track_focus + focus_handle
   3. 滚轮事件需要元素有 id
   4. 事件处理后调用 cx.notify()
   5. 鼠标位置是相对于窗口的

   运行命令：
   ---------
   cargo run -p gpui_events

========================================================================== */
