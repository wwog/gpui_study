// 第三章：GPUI 状态管理
// =====================
// 本章学习 GPUI 中的状态传递机制：
// 1. 组件自身状态管理
// 2. 父组件向子组件传递状态
// 3. 子组件向父组件传递状态（回调 + 事件）
// 4. 兄弟组件之间的通信
// 5. 使用 observe 观察其他实体的变化

use gpui::*;

// ============================================================================
// 第一部分：事件定义
// ============================================================================

/// 子组件发出的事件 - 用于子传父通信
#[derive(Clone, Debug)]
struct CounterChangedEvent {
    new_value: i32,
    delta: i32,
}

/// 为 ChildCounter 实现 EventEmitter，使其能够发出 CounterChangedEvent
impl EventEmitter<CounterChangedEvent> for ChildCounter {}

// ============================================================================
// 第二部分：子组件定义
// ============================================================================

/// 子计数器组件 - 展示子组件如何：
/// 1. 接收父组件传递的初始值
/// 2. 管理自身状态
/// 3. 通过事件通知父组件状态变化
struct ChildCounter {
    /// 计数器名称（从父组件传入）
    name: String,
    /// 当前计数值（自身状态）
    count: i32,
    /// 背景颜色（从父组件传入的样式配置）
    color: Rgba,
}

impl ChildCounter {
    /// 创建新的子计数器
    /// 参数从父组件传入，这是父传子的主要方式
    fn new(name: impl Into<String>, initial_count: i32, color: Rgba) -> Self {
        Self {
            name: name.into(),
            count: initial_count,
            color,
        }
    }

    /// 增加计数
    fn increment(&mut self, cx: &mut Context<Self>) {
        self.count += 1;
        // 发出事件通知父组件
        cx.emit(CounterChangedEvent {
            new_value: self.count,
            delta: 1,
        });
        // 通知视图需要重新渲染
        cx.notify();
    }

    /// 减少计数
    fn decrement(&mut self, cx: &mut Context<Self>) {
        self.count -= 1;
        cx.emit(CounterChangedEvent {
            new_value: self.count,
            delta: -1,
        });
        cx.notify();
    }
}

impl Render for ChildCounter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .rounded_lg()
            .bg(self.color)
            .child(
                // 标题
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1F2937))
                    .child(format!("{}", self.name)),
            )
            .child(
                // 计数显示和按钮
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        // 减少按钮
                        div()
                            .id("dec")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(32.0))
                            .h(px(32.0))
                            .bg(rgb(0xEF4444))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0xDC2626)))
                            .child("-")
                            .on_click(cx.listener(|view, _event, _window, cx| {
                                view.decrement(cx);
                            })),
                    )
                    .child(
                        // 计数显示
                        div()
                            .min_w(px(60.0))
                            .text_center()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x374151))
                            .child(format!("{}", self.count)),
                    )
                    .child(
                        // 增加按钮
                        div()
                            .id("inc")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(32.0))
                            .h(px(32.0))
                            .bg(rgb(0x10B981))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x059669)))
                            .child("+")
                            .on_click(cx.listener(|view, _event, _window, cx| {
                                view.increment(cx);
                            })),
                    ),
            )
    }
}

// ============================================================================
// 第三部分：观察者组件 - 展示 observe 机制
// ============================================================================

/// 总计显示组件 - 通过 observe 监听其他实体变化
struct TotalDisplay {
    /// 总计值
    total: i32,
    /// 最后一次变化
    last_change: String,
}

impl TotalDisplay {
    fn new() -> Self {
        Self {
            total: 0,
            last_change: "等待变化...".to_string(),
        }
    }
}

impl Render for TotalDisplay {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xFEF3C7))
            .border_2()
            .border_color(rgb(0xF59E0B))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x92400E))
                    .child("📊 观察者面板"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .text_color(rgb(0x78350F))
                            .child(format!("总计: {}", self.total)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xA16207))
                            .child(format!("最后变化: {}", self.last_change)),
                    ),
            )
    }
}

// ============================================================================
// 第四部分：父组件/根视图
// ============================================================================

/// 主应用视图 - 父组件
/// 展示如何：
/// 1. 持有子组件的 Entity 句柄
/// 2. 订阅子组件的事件
/// 3. 观察子组件的状态变化
/// 4. 协调多个子组件之间的通信
struct StateManagementApp {
    /// 子计数器 A 的句柄（Entity<T> 是对实体的引用）
    counter_a: Entity<ChildCounter>,
    /// 子计数器 B 的句柄
    counter_b: Entity<ChildCounter>,
    /// 总计显示组件
    total_display: Entity<TotalDisplay>,

    /// 父组件自身的状态：记录总变化次数
    total_changes: i32,
    /// 记录来自各计数器的事件历史
    event_log: Vec<String>,

    /// 保存订阅，防止被丢弃
    _subscriptions: Vec<Subscription>,
}

impl StateManagementApp {
    fn new(cx: &mut Context<Self>) -> Self {
        // 1. 创建子组件 - 通过构造函数参数传递初始状态（父传子）
        let counter_a = cx.new(|_cx| {
            ChildCounter::new("计数器 A", 0, rgba(0xBFDBFEFF)) // 蓝色背景
        });

        let counter_b = cx.new(|_cx| {
            ChildCounter::new("计数器 B", 10, rgba(0xBBF7D0FF)) // 绿色背景
        });

        // 2. 创建观察者组件
        let total_display = cx.new(|_cx| TotalDisplay::new());

        let mut subscriptions = Vec::new();

        // 3. 订阅子组件事件（子传父的事件方式）
        // subscribe 用于监听 EventEmitter 发出的事件
        let sub_a = cx.subscribe(&counter_a, {
            let total_display = total_display.clone();
            move |parent, _emitter, event: &CounterChangedEvent, cx| {
                // 更新父组件自身状态
                parent.total_changes += 1;
                parent.event_log.push(format!(
                    "A: {} ({}{})",
                    event.new_value,
                    if event.delta > 0 { "+" } else { "" },
                    event.delta
                ));
                // 保持日志不超过5条
                if parent.event_log.len() > 5 {
                    parent.event_log.remove(0);
                }
                
                // 更新总计显示组件
                total_display.update(cx, |display, cx| {
                    display.total += event.delta;
                    display.last_change = format!("A: {}{}", 
                        if event.delta > 0 { "+" } else { "" }, 
                        event.delta);
                    cx.notify();
                });

                cx.notify(); // 通知父视图需要重新渲染
            }
        });
        subscriptions.push(sub_a);

        let sub_b = cx.subscribe(&counter_b, {
            let total_display = total_display.clone();
            move |parent, _emitter, event: &CounterChangedEvent, cx| {
                parent.total_changes += 1;
                parent.event_log.push(format!(
                    "B: {} ({}{})",
                    event.new_value,
                    if event.delta > 0 { "+" } else { "" },
                    event.delta
                ));
                if parent.event_log.len() > 5 {
                    parent.event_log.remove(0);
                }
                
                total_display.update(cx, |display, cx| {
                    display.total += event.delta;
                    display.last_change = format!("B: {}{}", 
                        if event.delta > 0 { "+" } else { "" }, 
                        event.delta);
                    cx.notify();
                });

                cx.notify();
            }
        });
        subscriptions.push(sub_b);

        // 4. 使用 observe 观察子组件（另一种子传父方式）
        // observe 用于监听 notify() 调用，而不是特定事件
        let obs_a = cx.observe(&counter_a, |_parent, counter, cx| {
            // 当 counter_a 调用 notify() 时触发
            // 可以读取计数器的当前状态
            let count = counter.read(cx).count;
            println!("观察到计数器 A 变化: {}", count);
        });
        subscriptions.push(obs_a);

        Self {
            counter_a,
            counter_b,
            total_display,
            total_changes: 0,
            event_log: Vec::new(),
            _subscriptions: subscriptions,
        }
    }

    /// 重置所有计数器 - 父组件修改子组件状态
    fn reset_all(&mut self, cx: &mut Context<Self>) {
        // 使用 update 方法修改子组件状态
        self.counter_a.update(cx, |counter, cx| {
            counter.count = 0;
            cx.notify();
        });

        self.counter_b.update(cx, |counter, cx| {
            counter.count = 0;
            cx.notify();
        });

        self.total_display.update(cx, |display, cx| {
            display.total = 0;
            display.last_change = "已重置".to_string();
            cx.notify();
        });

        self.total_changes = 0;
        self.event_log.clear();
        cx.notify();
    }

    /// 同步计数器 - 演示兄弟组件通信（通过父组件中转）
    fn sync_counters(&mut self, cx: &mut Context<Self>) {
        // 读取计数器 A 的值
        let count_a = self.counter_a.read(cx).count;
        
        // 将值设置到计数器 B
        self.counter_b.update(cx, |counter, cx| {
            counter.count = count_a;
            cx.notify();
        });

        self.event_log.push(format!("同步: B <- A ({})", count_a));
        if self.event_log.len() > 5 {
            self.event_log.remove(0);
        }
        
        cx.notify();
    }
}

impl Render for StateManagementApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
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
                            .child("第三章：状态管理 🔄"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x64748B))
                            .child("学习父子组件通信、事件系统和观察者模式"),
                    ),
            )
            // 主内容区
            .child(
                div()
                    .flex()
                    .gap_6()
                    // 左侧：子组件区域
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x475569))
                                    .child("子组件（点击按钮修改状态）"),
                            )
                            // 渲染子组件 - 直接将 Entity<T> 作为 child
                            .child(self.counter_a.clone())
                            .child(self.counter_b.clone()),
                    )
                    // 右侧：父组件状态显示
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x475569))
                                    .child("父组件状态（通过事件接收）"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .p_4()
                                    .rounded_lg()
                                    .bg(rgb(0xE0E7FF))
                                    .border_2()
                                    .border_color(rgb(0x6366F1))
                                    .child(
                                        div()
                                            .text_color(rgb(0x3730A3))
                                            .child(format!("总变化次数: {}", self.total_changes)),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x4F46E5))
                                            .child("事件日志:"),
                                    )
                                    .children(self.event_log.iter().map(|log| {
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x6366F1))
                                            .pl_2()
                                            .child(format!("• {}", log))
                                    })),
                            ),
                    ),
            )
            // 观察者面板
            .child(self.total_display.clone())
            // 操作按钮
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .id("reset-all")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_4()
                            .py_2()
                            .bg(rgb(0xEF4444))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_lg()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0xDC2626)))
                            .child("🔄 重置所有")
                            .on_click(cx.listener(|view, _event, _window, cx| {
                                view.reset_all(cx);
                            })),
                    )
                    .child(
                        div()
                            .id("sync")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x8B5CF6))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_lg()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x7C3AED)))
                            .child("🔗 同步 B <- A")
                            .on_click(cx.listener(|view, _event, _window, cx| {
                                view.sync_counters(cx);
                            })),
                    ),
            )
            // 知识点说明
            .child(
                div()
                    .mt_4()
                    .p_4()
                    .rounded_lg()
                    .bg(rgb(0xFFFBEB))
                    .border_1()
                    .border_color(rgb(0xFCD34D))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x92400E))
                            .child("💡 本示例演示：1) 父传子：构造函数参数 2) 子传父：EventEmitter + subscribe 3) 观察变化：observe 4) 兄弟通信：通过父组件中转"),
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
                        x: px(100.0),
                        y: px(100.0),
                    },
                    size: Size {
                        width: px(800.0),
                        height: px(700.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("第三章：状态管理".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(StateManagementApp::new),
        )
        .unwrap();
    });
}

/* ==========================================================================
   🎓 GPUI 状态管理知识点总结
   ==========================================================================

   一、核心概念
   -----------
   - Entity<T>: 对实体的强引用句柄，可以克隆和传递
   - WeakEntity<T>: 弱引用，不阻止实体被释放
   - Context<T>: 实体上下文，提供状态管理 API
   - App: 应用上下文，拥有所有实体的所有权

   二、组件自身状态
   ---------------
   1. 在 struct 中定义字段存储状态
   2. 通过方法修改状态
   3. 调用 cx.notify() 通知 GPUI 需要重新渲染

   struct MyView {
       count: i32,
   }

   impl MyView {
       fn increment(&mut self, cx: &mut Context<Self>) {
           self.count += 1;
           cx.notify(); // 重要！通知视图更新
       }
   }

   三、父组件向子组件传递状态
   -------------------------
   1. 通过构造函数参数传递初始值
   2. 使用 cx.new() 创建子组件时传入

   let child = cx.new(|_cx| ChildView::new(
       "名称",      // 传递字符串
       42,          // 传递数值
       config,      // 传递配置对象
   ));

   四、子组件向父组件传递状态
   -------------------------
   方式一：事件系统（推荐）
   1. 定义事件结构体
   2. 为子组件实现 EventEmitter<Event>
   3. 子组件调用 cx.emit(event)
   4. 父组件使用 cx.subscribe() 订阅事件

   // 定义事件
   struct MyEvent { value: i32 }

   // 实现 EventEmitter
   impl EventEmitter<MyEvent> for ChildView {}

   // 子组件发出事件
   cx.emit(MyEvent { value: 42 });

   // 父组件订阅
   cx.subscribe(&child, |parent, _emitter, event, cx| {
       parent.handle_event(event);
       cx.notify();
   });

   方式二：观察者模式
   使用 cx.observe() 监听实体的 notify() 调用

   cx.observe(&child, |parent, child, cx| {
       let value = child.read(cx).some_field;
       // 处理变化
   });

   五、兄弟组件通信
   ---------------
   通过共同的父组件中转：
   1. 父组件持有所有子组件的 Entity 句柄
   2. 父组件订阅子组件 A 的事件
   3. 在事件处理中更新子组件 B

   六、父组件修改子组件
   -------------------
   使用 Entity::update() 或 Entity::read()

   // 读取子组件状态
   let value = self.child.read(cx).count;

   // 修改子组件状态
   self.child.update(cx, |child, cx| {
       child.count = 100;
       cx.notify();
   });

   七、重要注意事项
   ---------------
   1. Subscription 必须保存，丢弃后订阅失效
   2. 修改状态后必须调用 cx.notify()
   3. Entity<T> 是引用计数的，可以安全克隆
   4. 避免循环订阅导致无限循环

   运行命令：
   ---------
   cargo run -p gpui_state_management

========================================================================== */
