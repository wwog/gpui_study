// 第六章：GPUI 高级主题
// ======================
// 本章学习 GPUI 的高级特性：
// 1. Action 系统 - 命令模式与键盘快捷键
// 2. 异步操作 - cx.spawn() 和 Task
// 3. 焦点管理 - FocusHandle 详解
// 4. Global 状态 - 全局数据管理

use gpui::prelude::FluentBuilder;
use gpui::*;
use std::time::Duration;

// ============================================================================
// 第一部分：Action 系统
// ============================================================================

// 使用 actions! 宏定义简单的 Action
// 格式: actions!(namespace, [ActionName1, ActionName2, ...])
actions!(
    app,
    [
        Increment,      // 增加计数
        Decrement,      // 减少计数
        Reset,          // 重置
        ToggleTheme,    // 切换主题
    ]
);

// 注意：带参数的 Action 需要完整实现 Action trait
// 这需要 serde 和 schemars 依赖
// 在实际项目中，使用 #[derive(Action)] 宏更方便：
// #[derive(Clone, PartialEq, serde::Deserialize, schemars::JsonSchema, Action)]
// struct SetValue { value: i32 }

// ============================================================================
// 第二部分：Action 演示组件
// ============================================================================

struct ActionDemo {
    count: i32,
    action_log: Vec<String>,
    focus_handle: FocusHandle,
}

impl ActionDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            count: 0,
            action_log: vec!["等待 Action...".to_string()],
            focus_handle: cx.focus_handle(),
        }
    }

    fn log_action(&mut self, action: &str) {
        self.action_log.push(action.to_string());
        if self.action_log.len() > 5 {
            self.action_log.remove(0);
        }
    }

    // Action 处理方法 - 签名: &mut self, action: &ActionType, window: &mut Window, cx: &mut Context<Self>
    fn handle_increment(&mut self, _: &Increment, _window: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        self.log_action(&format!("Increment → {}", self.count));
        cx.notify();
    }

    fn handle_decrement(&mut self, _: &Decrement, _window: &mut Window, cx: &mut Context<Self>) {
        self.count -= 1;
        self.log_action(&format!("Decrement → {}", self.count));
        cx.notify();
    }

    fn handle_reset(&mut self, _: &Reset, _window: &mut Window, cx: &mut Context<Self>) {
        self.count = 0;
        self.log_action("Reset → 0");
        cx.notify();
    }

}

impl Render for ActionDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_focus = self.focus_handle.is_focused(window);

        div()
            .id("action-demo")
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .border_2()
            .when(has_focus, |s| s.border_color(rgb(0x3B82F6)).bg(rgb(0xEFF6FF)))
            .when(!has_focus, |s| s.border_color(rgb(0xE5E7EB)).bg(rgb(0xFFFFFF)))
            .track_focus(&self.focus_handle)
            // 注册 Action 处理器
            .on_action(cx.listener(Self::handle_increment))
            .on_action(cx.listener(Self::handle_decrement))
            .on_action(cx.listener(Self::handle_reset))
            // 绑定键盘快捷键
            .key_context("ActionDemo")
            .on_key_down(cx.listener(|view, event: &KeyDownEvent, window, cx| {
                let key = &event.keystroke.key;
                // 手动分发 Action
                match key.as_str() {
                    "up" | "k" => window.dispatch_action(Increment.boxed_clone(), cx),
                    "down" | "j" => window.dispatch_action(Decrement.boxed_clone(), cx),
                    "r" => window.dispatch_action(Reset.boxed_clone(), cx),
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                        // 数字键直接设置计数值
                        if let Ok(num) = key.parse::<i32>() {
                            view.count = num;
                            view.log_action(&format!("数字键 → {}", num));
                            cx.notify();
                        }
                    }
                    _ => {}
                }
            }))
            .on_click(cx.listener(|view, _, window, cx| {
                view.focus_handle.focus(window);
                cx.notify();
            }))
            // 提示
            .child(
                div()
                    .text_sm()
                    .text_color(if has_focus { rgb(0x1D4ED8) } else { rgb(0x6B7280) })
                    .font_weight(if has_focus { FontWeight::BOLD } else { FontWeight::NORMAL })
                    .child(if has_focus {
                        "🎮 Action 区域已聚焦 - 按 ↑/↓/R/0/1/2"
                    } else {
                        "👆 点击此区域以启用快捷键"
                    }),
            )
            // 计数显示
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(80.0))
                    .bg(rgb(0x1F2937))
                    .rounded_lg()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x10B981))
                    .child(format!("{}", self.count)),
            )
            // 按钮组
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_center()
                    .child(
                        div()
                            .id("btn-dec")
                            .px_4()
                            .py_2()
                            .bg(rgb(0xEF4444))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0xDC2626)))
                            .child("- 减少 (↓)")
                            .on_click(cx.listener(|_view, _, window, cx| {
                                window.dispatch_action(Decrement.boxed_clone(), cx);
                            })),
                    )
                    .child(
                        div()
                            .id("btn-reset")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x6B7280))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x4B5563)))
                            .child("重置 (R)")
                            .on_click(cx.listener(|_view, _, window, cx| {
                                window.dispatch_action(Reset.boxed_clone(), cx);
                            })),
                    )
                    .child(
                        div()
                            .id("btn-inc")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x10B981))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x059669)))
                            .child("+ 增加 (↑)")
                            .on_click(cx.listener(|_view, _, window, cx| {
                                window.dispatch_action(Increment.boxed_clone(), cx);
                            })),
                    ),
            )
            // Action 日志
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_2()
                    .bg(rgb(0xF3F4F6))
                    .rounded_md()
                    .text_xs()
                    .child(
                        div()
                            .text_color(rgb(0x374151))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Action 日志:"),
                    )
                    .children(self.action_log.iter().map(|log| {
                        div().text_color(rgb(0x6B7280)).child(format!("• {}", log))
                    })),
            )
    }
}

// ============================================================================
// 第三部分：异步操作演示
// ============================================================================

struct AsyncDemo {
    /// 加载状态
    loading: bool,
    /// 加载结果
    result: Option<String>,
    /// 进度 (0-100)
    progress: i32,
    /// 任务句柄（保存以防止被 drop）
    _task: Option<Task<()>>,
}

impl AsyncDemo {
    fn new() -> Self {
        Self {
            loading: false,
            result: None,
            progress: 0,
            _task: None,
        }
    }

    /// 模拟异步加载
    fn start_loading(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        self.progress = 0;
        self.result = None;
        cx.notify();

        // cx.spawn 用于在视图上下文中启动异步任务
        let task = cx.spawn(async |weak_view: WeakEntity<Self>, cx: &mut AsyncApp| {
            // 模拟分步加载
            for i in 1..=10 {
                // 模拟网络延迟
                cx.background_executor().timer(Duration::from_millis(200)).await;
                
                // 更新进度
                let _ = weak_view.update(cx, |view, cx| {
                    view.progress = i * 10;
                    cx.notify();
                });
            }

            // 加载完成
            let _ = weak_view.update(cx, |view, cx| {
                view.loading = false;
                view.result = Some(format!("加载完成！时间: {:?}", std::time::SystemTime::now()));
                cx.notify();
            });
        });

        // 保存任务句柄，防止被 drop
        self._task = Some(task);
    }

    /// 取消加载
    fn cancel_loading(&mut self, cx: &mut Context<Self>) {
        self._task = None; // drop task 会取消它
        self.loading = false;
        self.progress = 0;
        self.result = Some("已取消".to_string());
        cx.notify();
    }
}

impl Render for AsyncDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            // 状态显示
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w(px(12.0))
                            .h(px(12.0))
                            .rounded_full()
                            .bg(if self.loading { rgb(0xF59E0B) } else { rgb(0x10B981) }),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x374151))
                            .child(if self.loading { "加载中..." } else { "就绪" }),
                    ),
            )
            // 进度条
            .when(self.loading, |el| {
                el.child(
                    div()
                        .w_full()
                        .h(px(8.0))
                        .bg(rgb(0xE5E7EB))
                        .rounded_full()
                        .overflow_hidden()
                        .child(
                            div()
                                .h_full()
                                .w(relative(self.progress as f32 / 100.0))
                                .bg(rgb(0x3B82F6))
                                .rounded_full(),
                        ),
                )
                .child(
                    div()
                        .text_center()
                        .text_sm()
                        .text_color(rgb(0x6B7280))
                        .child(format!("{}%", self.progress)),
                )
            })
            // 结果显示
            .when_some(self.result.clone(), |el, result| {
                el.child(
                    div()
                        .p_3()
                        .bg(rgb(0xDCFCE7))
                        .rounded_md()
                        .text_sm()
                        .text_color(rgb(0x166534))
                        .child(result),
                )
            })
            // 按钮
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .id("btn-start")
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .when(!self.loading, |s| {
                                s.bg(rgb(0x3B82F6))
                                    .text_color(rgb(0xFFFFFF))
                                    .hover(|s| s.bg(rgb(0x2563EB)))
                            })
                            .when(self.loading, |s| {
                                s.bg(rgb(0x9CA3AF))
                                    .text_color(rgb(0xFFFFFF))
                                    .cursor_default()
                            })
                            .child("开始加载")
                            .when(!self.loading, |el| {
                                el.on_click(cx.listener(|view, _, _window, cx| {
                                    view.start_loading(cx);
                                }))
                            }),
                    )
                    .child(
                        div()
                            .id("btn-cancel")
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .when(self.loading, |s| {
                                s.bg(rgb(0xEF4444))
                                    .text_color(rgb(0xFFFFFF))
                                    .hover(|s| s.bg(rgb(0xDC2626)))
                            })
                            .when(!self.loading, |s| {
                                s.bg(rgb(0xE5E7EB))
                                    .text_color(rgb(0x9CA3AF))
                                    .cursor_default()
                            })
                            .child("取消")
                            .when(self.loading, |el| {
                                el.on_click(cx.listener(|view, _, _window, cx| {
                                    view.cancel_loading(cx);
                                }))
                            }),
                    ),
            )
    }
}

// ============================================================================
// 第四部分：焦点管理演示
// ============================================================================

struct FocusDemo {
    /// 多个可聚焦区域的焦点句柄
    focus_handles: Vec<FocusHandle>,
    /// 当前聚焦的索引
    focused_index: Option<usize>,
    /// 焦点变化日志
    focus_log: Vec<String>,
}

impl FocusDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        // 创建多个焦点句柄
        let focus_handles: Vec<FocusHandle> = (0..4)
            .map(|_| cx.focus_handle())
            .collect();

        Self {
            focus_handles,
            focused_index: None,
            focus_log: vec!["等待焦点变化...".to_string()],
        }
    }

    fn log_focus(&mut self, msg: &str) {
        self.focus_log.push(msg.to_string());
        if self.focus_log.len() > 5 {
            self.focus_log.remove(0);
        }
    }

    fn focus_next(&mut self, window: &mut Window) {
        let next = match self.focused_index {
            Some(i) => (i + 1) % self.focus_handles.len(),
            None => 0,
        };
        self.focus_handles[next].focus(window);
    }

    fn focus_prev(&mut self, window: &mut Window) {
        let prev = match self.focused_index {
            Some(i) => {
                if i == 0 {
                    self.focus_handles.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.focus_handles.len() - 1,
        };
        self.focus_handles[prev].focus(window);
    }
}

impl Render for FocusDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 更新焦点状态
        let mut new_focused = None;
        for (i, handle) in self.focus_handles.iter().enumerate() {
            if handle.is_focused(window) {
                new_focused = Some(i);
                break;
            }
        }
        
        if new_focused != self.focused_index {
            if let Some(i) = new_focused {
                self.log_focus(&format!("聚焦到区域 {}", i + 1));
            }
            self.focused_index = new_focused;
        }

        let colors = [
            ("区域 1", rgb(0xFEE2E2), rgb(0xEF4444)),
            ("区域 2", rgb(0xFEF3C7), rgb(0xF59E0B)),
            ("区域 3", rgb(0xDCFCE7), rgb(0x10B981)),
            ("区域 4", rgb(0xDBEAFE), rgb(0x3B82F6)),
        ];

        div()
            .flex()
            .flex_col()
            .gap_4()
            // 说明
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6B7280))
                    .child("点击区域聚焦，按 Tab/Shift+Tab 切换焦点，按方向键导航"),
            )
            // 焦点区域网格
            .child(
                div()
                    .grid()
                    .grid_cols(2)
                    .gap_3()
                    .children(self.focus_handles.iter().enumerate().map(|(i, handle)| {
                        let is_focused = handle.is_focused(window);
                        let (label, bg_color, border_color) = colors[i];
                        let handle_clone = handle.clone();

                        div()
                            .id(SharedString::from(format!("focus-area-{}", i)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .h(px(80.0))
                            .rounded_lg()
                            .cursor_pointer()
                            .border_2()
                            .when(is_focused, |s| {
                                s.bg(bg_color)
                                    .border_color(border_color)
                                    .shadow_lg()
                            })
                            .when(!is_focused, |s| {
                                s.bg(rgb(0xF9FAFB))
                                    .border_color(rgb(0xE5E7EB))
                            })
                            .track_focus(&handle_clone)
                            .on_click(cx.listener(move |view, _, window, cx| {
                                view.focus_handles[i].focus(window);
                                cx.notify();
                            }))
                            // 键盘导航
                            .on_key_down(cx.listener(move |view, event: &KeyDownEvent, window, cx| {
                                match event.keystroke.key.as_str() {
                                    "tab" => {
                                        if event.keystroke.modifiers.shift {
                                            view.focus_prev(window);
                                        } else {
                                            view.focus_next(window);
                                        }
                                    }
                                    "right" | "down" => view.focus_next(window),
                                    "left" | "up" => view.focus_prev(window),
                                    _ => {}
                                }
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(if is_focused { FontWeight::BOLD } else { FontWeight::NORMAL })
                                    .text_color(if is_focused { border_color } else { rgb(0x6B7280) })
                                    .child(label),
                            )
                    })),
            )
            // 焦点日志
            .child(
                div()
                    .p_2()
                    .bg(rgb(0xF3F4F6))
                    .rounded_md()
                    .text_xs()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x374151))
                                    .child(format!("当前焦点: {:?}", self.focused_index.map(|i| i + 1))),
                            )
                            .children(self.focus_log.iter().map(|log| {
                                div().text_color(rgb(0x6B7280)).child(format!("• {}", log))
                            })),
                    ),
            )
    }
}

// ============================================================================
// 第五部分：Global 状态演示
// ============================================================================

/// 全局主题状态
#[derive(Clone)]
struct ThemeState {
    is_dark: bool,
}

impl Global for ThemeState {}

struct GlobalDemo {
    local_count: i32,
}

impl GlobalDemo {
    fn new() -> Self {
        Self { local_count: 0 }
    }
}

impl Render for GlobalDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 读取全局状态
        let is_dark = cx.try_global::<ThemeState>()
            .map(|t| t.is_dark)
            .unwrap_or(false);

        let (bg, text, accent) = if is_dark {
            (rgb(0x1F2937), rgb(0xF9FAFB), rgb(0x60A5FA))
        } else {
            (rgb(0xFFFFFF), rgb(0x1F2937), rgb(0x3B82F6))
        };

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .bg(bg)
            // 主题显示
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .text_color(text)
                            .child(if is_dark { "🌙 深色主题" } else { "☀️ 浅色主题" }),
                    ),
            )
            // 本地状态
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_color(text)
                            .child(format!("本地计数: {}", self.local_count)),
                    )
                    .child(
                        div()
                            .id("local-inc")
                            .px_3()
                            .py_1()
                            .bg(accent)
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .child("+1")
                            .on_click(cx.listener(|view, _, _window, cx| {
                                view.local_count += 1;
                                cx.notify();
                            })),
                    ),
            )
            // 切换主题按钮
            .child(
                div()
                    .id("toggle-theme")
                    .px_4()
                    .py_2()
                    .bg(accent)
                    .text_color(rgb(0xFFFFFF))
                    .rounded_md()
                    .cursor_pointer()
                    .text_center()
                    .hover(|s| s.opacity(0.9))
                    .child("切换主题 (全局状态)")
                    .on_click(cx.listener(|_view, _, _window, cx| {
                        // 更新全局状态
                        let current = cx.try_global::<ThemeState>()
                            .map(|t| t.is_dark)
                            .unwrap_or(false);
                        cx.set_global(ThemeState { is_dark: !current });
                    })),
            )
            // 说明
            .child(
                div()
                    .text_xs()
                    .text_color(if is_dark { rgb(0x9CA3AF) } else { rgb(0x6B7280) })
                    .child("💡 Global 状态在所有组件间共享，切换主题会影响其他使用该状态的组件"),
            )
    }
}

// ============================================================================
// 第六部分：主应用
// ============================================================================

struct AdvancedApp {
    action_demo: Entity<ActionDemo>,
    async_demo: Entity<AsyncDemo>,
    focus_demo: Entity<FocusDemo>,
    global_demo: Entity<GlobalDemo>,
}

impl AdvancedApp {
    fn new(cx: &mut Context<Self>) -> Self {
        // 初始化全局主题状态
        cx.set_global(ThemeState { is_dark: false });

        let action_demo = cx.new(ActionDemo::new);
        let async_demo = cx.new(|_| AsyncDemo::new());
        let focus_demo = cx.new(FocusDemo::new);
        let global_demo = cx.new(|_| GlobalDemo::new());

        Self {
            action_demo,
            async_demo,
            focus_demo,
            global_demo,
        }
    }
}

impl Render for AdvancedApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 读取全局主题
        let is_dark = cx.try_global::<ThemeState>()
            .map(|t| t.is_dark)
            .unwrap_or(false);

        let bg = if is_dark { rgb(0x111827) } else { rgb(0xF1F5F9) };
        let text = if is_dark { rgb(0xF9FAFB) } else { rgb(0x1E293B) };
        let subtext = if is_dark { rgb(0x9CA3AF) } else { rgb(0x64748B) };

        div()
            .id("main-container")
            .size_full()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .bg(bg)
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
                            .text_color(text)
                            .child("第六章：高级主题 🚀"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(subtext)
                            .child("学习 Action 系统、异步操作、焦点管理和全局状态"),
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
                            // Action 演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(if is_dark { rgb(0x1F2937) } else { rgb(0xFFFFFF) })
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(text)
                                            .child("🎮 Action 系统"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(subtext)
                                            .child("使用 actions! 宏定义命令，绑定快捷键"),
                                    )
                                    .child(self.action_demo.clone()),
                            )
                            // 异步演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(if is_dark { rgb(0x1F2937) } else { rgb(0xFFFFFF) })
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(text)
                                            .child("⏳ 异步操作"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(subtext)
                                            .child("使用 cx.spawn() 执行异步任务"),
                                    )
                                    .child(self.async_demo.clone()),
                            ),
                    )
                    // 右列
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_6()
                            // 焦点演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(if is_dark { rgb(0x1F2937) } else { rgb(0xFFFFFF) })
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(text)
                                            .child("🎯 焦点管理"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(subtext)
                                            .child("使用 FocusHandle 管理多个焦点区域"),
                                    )
                                    .child(self.focus_demo.clone()),
                            )
                            // Global 演示
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .bg(if is_dark { rgb(0x1F2937) } else { rgb(0xFFFFFF) })
                                    .rounded_lg()
                                    .shadow_sm()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(text)
                                            .child("🌍 Global 状态"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(subtext)
                                            .child("使用 Global trait 管理全局共享状态"),
                                    )
                                    .child(self.global_demo.clone()),
                            ),
                    ),
            )
            // 知识点总结
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .bg(if is_dark { rgb(0x374151) } else { rgb(0xFEF3C7) })
                    .border_1()
                    .border_color(if is_dark { rgb(0x4B5563) } else { rgb(0xFCD34D) })
                    .child(
                        div()
                            .text_sm()
                            .text_color(if is_dark { rgb(0xFCD34D) } else { rgb(0x92400E) })
                            .child("💡 关键API：actions! | on_action | dispatch_action | cx.spawn() | Task | FocusHandle | track_focus | Global | cx.set_global()"),
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
                        height: px(900.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("第六章：高级主题".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(AdvancedApp::new),
        )
        .unwrap();
    });
}

/* ==========================================================================
   🎓 GPUI 高级主题知识点总结
   ==========================================================================

   一、Action 系统
   ---------------
   Action 是 GPUI 的命令模式实现，用于键盘驱动的 UI。

   1. 定义简单 Action：
   ```rust
   actions!(namespace, [ActionName1, ActionName2]);
   ```

   2. 带参数的 Action：
   ```rust
   #[derive(Clone, PartialEq, Debug)]
   struct MyAction { value: i32 }

   impl Action for MyAction {
       fn boxed_clone(&self) -> Box<dyn Action> { Box::new(self.clone()) }
       fn partial_eq(&self, other: &dyn Action) -> bool { ... }
       fn name(&self) -> &'static str { "MyAction" }
       fn name_for_type() -> &'static str { "MyAction" }
       fn build(value: serde_json::Value) -> anyhow::Result<Box<dyn Action>> { ... }
   }
   ```

   3. 注册 Action 处理器：
   ```rust
   div()
       .on_action(cx.listener(Self::handle_my_action))
   ```

   4. 分发 Action：
   ```rust
   window.dispatch_action(MyAction { value: 42 }.boxed_clone(), cx);
   ```

   二、异步操作
   -----------
   使用 cx.spawn() 在视图上下文中执行异步任务。

   ```rust
   let task = cx.spawn(|weak_view, mut cx| async move {
       // 异步操作
       cx.background_executor().timer(Duration::from_secs(1)).await;

       // 更新视图
       let _ = weak_view.update(&mut cx, |view, cx| {
           view.data = "完成";
           cx.notify();
       });
   });

   // 保存 task 防止被 drop（drop = 取消）
   self._task = Some(task);
   ```

   关键点：
   - cx.spawn() 返回 Task，需要保存或 detach
   - 闭包接收 WeakEntity 和 AsyncApp
   - 使用 weak_view.update() 更新视图状态
   - drop Task 会取消异步操作

   三、焦点管理
   -----------
   使用 FocusHandle 管理元素焦点。

   1. 创建焦点句柄：
   ```rust
   struct MyView {
       focus_handle: FocusHandle,
   }

   fn new(cx: &mut Context<Self>) -> Self {
       Self {
           focus_handle: cx.focus_handle(),
       }
   }
   ```

   2. 关联焦点：
   ```rust
   div()
       .id("focusable")
       .track_focus(&self.focus_handle)
   ```

   3. 检查焦点状态：
   ```rust
   let is_focused = self.focus_handle.is_focused(window);
   ```

   4. 编程式设置焦点：
   ```rust
   self.focus_handle.focus(window);
   ```

   四、Global 状态
   --------------
   使用 Global trait 管理跨组件共享的全局状态。

   1. 定义全局状态：
   ```rust
   #[derive(Clone)]
   struct MyGlobal { value: i32 }
   impl Global for MyGlobal {}
   ```

   2. 设置全局状态：
   ```rust
   cx.set_global(MyGlobal { value: 42 });
   ```

   3. 读取全局状态：
   ```rust
   // 安全读取（可能不存在）
   let value = cx.try_global::<MyGlobal>().map(|g| g.value);

   // 假设存在（会 panic 如果不存在）
   let value = cx.global::<MyGlobal>().value;
   ```

   4. 观察全局状态变化：
   ```rust
   cx.observe_global::<MyGlobal>(|view, cx| {
       // 全局状态变化时调用
       cx.notify();
   });
   ```

   五、键盘快捷键绑定
   -----------------
   结合 Action 和键盘事件实现快捷键。

   ```rust
   div()
       .track_focus(&self.focus_handle)
       .on_action(cx.listener(Self::handle_action))
       .on_key_down(cx.listener(|view, event, window, cx| {
           match event.keystroke.key.as_str() {
               "ctrl-s" => window.dispatch_action(Save.boxed_clone(), cx),
               _ => {}
           }
       }))
   ```

   运行命令：
   ---------
   cargo run -p gpui_advanced

========================================================================== */
