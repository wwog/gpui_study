
Zed 是如何实现 tooltip 功能的：

## Zed Tooltip 实现机制总结

Zed 的 tooltip 实现主要分为三个层次：

### 1. GPUI 框架层（底层机制）

在 `zed/crates/gpui/src/elements/div.rs` 中：

```zed/crates/gpui/src/elements/div.rs#L46-51
const DRAG_THRESHOLD: f64 = 2.;
const TOOLTIP_SHOW_DELAY: Duration = Duration::from_millis(500);
const HOVERABLE_TOOLTIP_HIDE_DELAY: Duration = Duration::from_millis(500);

/// The styling information for a given group.
```

**核心数据结构：**

```zed/crates/gpui/src/elements/div.rs#L1273-1276
pub(crate) struct TooltipBuilder {
    build: Rc<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>,
    hoverable: bool,
}
```

```zed/crates/gpui/src/elements/div.rs#L2761-2774
pub(crate) enum ActiveTooltip {
    /// Currently delaying before showing the tooltip.
    WaitingForShow { _task: Task<()> },
    /// Tooltip is visible, element was hovered or for hoverable tooltips, the tooltip was hovered.
    Visible {
        tooltip: AnyTooltip,
        is_hoverable: bool,
    },
    /// Tooltip is visible and hoverable, but the mouse is no longer hovering. Currently delaying
    /// before hiding it.
    WaitingForHide {
        tooltip: AnyTooltip,
        _task: Task<()>,
    },
```

**工作流程：**

1. **注册 tooltip**：元素通过 `.tooltip()` 方法注册一个 tooltip builder：

```zed/crates/gpui/src/elements/div.rs#L568-582
    /// Use the given callback to construct a new tooltip view when the mouse hovers over this element.
    /// The imperative API equivalent to [`StatefulInteractiveElement::tooltip`].
    pub fn tooltip(&mut self, build_tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static)
    where
        Self: Sized,
    {
        debug_assert!(
            self.tooltip_builder.is_none(),
            "calling tooltip more than once on the same element is not supported"
        );
        self.tooltip_builder = Some(TooltipBuilder {
            build: Rc::new(build_tooltip),
            hoverable: false,
        });
    }
```

2. **监听鼠标事件**：通过 `register_tooltip_mouse_handlers` 注册鼠标移动事件监听：

```zed/crates/gpui/src/elements/div.rs#L2818-2842
pub(crate) fn register_tooltip_mouse_handlers(
    active_tooltip: &Rc<RefCell<Option<ActiveTooltip>>>,
    tooltip_id: Option<TooltipId>,
    build_tooltip: Rc<dyn Fn(&mut Window, &mut App) -> Option<(AnyView, bool)>>,
    check_is_hovered: Rc<dyn Fn(&Window) -> bool>,
    check_is_hovered_during_prepaint: Rc<dyn Fn(&Window) -> bool>,
    window: &mut Window,
) {
    window.on_mouse_event({
        let active_tooltip = active_tooltip.clone();
        let build_tooltip = build_tooltip.clone();
        let check_is_hovered = check_is_hovered.clone();
        move |_: &MouseMoveEvent, phase, window, cx| {
            handle_tooltip_mouse_move(
                &active_tooltip,
                &build_tooltip,
                &check_is_hovered,
                &check_is_hovered_during_prepaint,
                phase,
                window,
                cx,
            )
        }
    });
```

3. **延迟显示**：hover 500ms 后才显示 tooltip：

```zed/crates/gpui/src/elements/div.rs#L2920-2960
            let delayed_show_task = window.spawn(cx, {
                let active_tooltip = active_tooltip.clone();
                let build_tooltip = build_tooltip.clone();
                let check_is_hovered_during_prepaint = check_is_hovered_during_prepaint.clone();
                async move |cx| {
                    cx.background_executor().timer(TOOLTIP_SHOW_DELAY).await;
                    cx.update(|window, cx| {
                        let new_tooltip =
                            build_tooltip(window, cx).map(|(view, tooltip_is_hoverable)| {
                                let active_tooltip = active_tooltip.clone();
                                ActiveTooltip::Visible {
                                    tooltip: AnyTooltip {
                                        view,
                                        mouse_position: window.mouse_position(),
                                        ...
```

4. **渲染 tooltip**：在窗口的 `prepaint_tooltip` 方法中计算位置并渲染：

```zed/crates/gpui/src/window.rs#L2251-2317
    fn prepaint_tooltip(&mut self, cx: &mut App) -> Option<AnyElement> {
        // Use indexing instead of iteration to avoid borrowing self for the duration of the loop.
        for tooltip_request_index in (0..self.next_frame.tooltip_requests.len()).rev() {
            let Some(Some(tooltip_request)) = self
                .next_frame
                .tooltip_requests
                .get(tooltip_request_index)
                .cloned()
            else {
                log::error!("Unexpectedly absent TooltipRequest");
                continue;
            };
            let mut element = tooltip_request.tooltip.view.clone().into_any();
            let mouse_position = tooltip_request.tooltip.mouse_position;
            let tooltip_size = element.layout_as_root(AvailableSpace::min_size(), self, cx);

            let mut tooltip_bounds =
                Bounds::new(mouse_position + point(px(1.), px(1.)), tooltip_size);
            let window_bounds = Bounds {
                origin: Point::default(),
                size: self.viewport_size(),
            };

            if tooltip_bounds.right() > window_bounds.right() {
                // ... 处理边界情况，确保 tooltip 不会超出窗口
            }
```

### 2. UI 组件层（Tooltip 视图）

在 `zed/crates/ui/src/components/tooltip.rs` 中定义了 `Tooltip` 组件：

```zed/crates/ui/src/components/tooltip.rs#L12-16
pub struct Tooltip {
    title: Title,
    meta: Option<SharedString>,
    key_binding: Option<KeyBinding>,
}
```

提供了多种便捷方法：

```zed/crates/ui/src/components/tooltip.rs#L39-57
impl Tooltip {
    pub fn simple(title: impl Into<SharedString>, cx: &mut App) -> AnyView {
        cx.new(|_| Self {
            title: Title::Str(title.into()),
            meta: None,
            key_binding: None,
        })
        .into()
    }

    pub fn text(title: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView {
        let title = title.into();
        move |_, cx| {
            cx.new(|_| Self {
                title: title.clone().into(),
                meta: None,
                key_binding: None,
            })
            .into()
        }
    }
```

### 3. 使用方式

在实际组件中使用非常简单，比如 `IconButton`：

```zed/crates/ui/src/components/button/icon_button.rs#L160-164
    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }
```

**实际使用示例：**

```zed/crates/acp_tools/src/acp_tools.rs#L579-581
                IconButton::new("clear_messages", IconName::Trash)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Clear Messages"))
```

### 总结流程图

```/dev/null/tooltip_flow.txt#L1-15
1. 元素调用 .tooltip(Tooltip::text("提示文字"))
   ↓
2. GPUI 在 paint 阶段注册 tooltip_builder 和鼠标事件监听器
   ↓
3. 用户 hover 到元素上 → 触发 MouseMoveEvent
   ↓
4. 启动 500ms 延迟任务 (ActiveTooltip::WaitingForShow)
   ↓
5. 延迟结束，调用 build_tooltip 创建 tooltip 视图
   ↓
6. 设置 ActiveTooltip::Visible，调用 window.refresh()
   ↓
7. 窗口 prepaint 时调用 prepaint_tooltip，计算位置并渲染
   ↓
8. 鼠标离开 → 清除 tooltip 或延迟隐藏（可悬停的 tooltip）
```

这个设计的优点：
1. **延迟显示**：避免快速移动鼠标时频繁弹出 tooltip
2. **智能定位**：自动处理边界情况，确保 tooltip 在窗口内
3. **可悬停 tooltip**：支持用户将鼠标移到 tooltip 上（用于可交互的 tooltip）
4. **组件化**：Tooltip 本身是一个可渲染的视图，支持自定义样式和内容

