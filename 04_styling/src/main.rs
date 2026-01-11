// 第四章：GPUI 样式系统
// ======================
// 本章学习 GPUI 中类似 Tailwind CSS 的样式系统：
// 1. Flexbox 布局
// 2. 尺寸与间距
// 3. 颜色与背景
// 4. 边框与圆角
// 5. 阴影与透明度
// 6. 文字样式
// 7. 伪状态（hover、active）
// 8. 条件样式（when）

use gpui::prelude::FluentBuilder;
use gpui::*;

// ============================================================================
// 第一部分：按钮组件库 - 展示各种样式组合
// ============================================================================

/// 按钮变体枚举
#[derive(Clone, Copy, PartialEq)]
enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

/// 按钮尺寸枚举
#[derive(Clone, Copy, PartialEq)]
enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// 可复用的按钮组件
struct StyledButton {
    label: String,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
}

impl StyledButton {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
        }
    }

    fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 渲染按钮 - 注意这不是 View，只是一个辅助函数返回 Element
    fn render(self, id: impl Into<ElementId>) -> Stateful<Div> {
        // 基础样式
        let mut button = div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .rounded_md()
            .font_weight(FontWeight::MEDIUM);

        // 根据尺寸设置 padding 和字体大小
        button = match self.size {
            ButtonSize::Small => button.px_2().py_1().text_xs(),
            ButtonSize::Medium => button.px_4().py_2().text_sm(),
            ButtonSize::Large => button.px_6().py_3().text_base(),
        };

        // 根据变体设置颜色
        button = match self.variant {
            ButtonVariant::Primary => button
                .bg(rgb(0x3B82F6))
                .text_color(rgb(0xFFFFFF))
                .hover(|s| s.bg(rgb(0x2563EB)))
                .active(|s| s.bg(rgb(0x1D4ED8))),

            ButtonVariant::Secondary => button
                .bg(rgb(0x6B7280))
                .text_color(rgb(0xFFFFFF))
                .hover(|s| s.bg(rgb(0x4B5563)))
                .active(|s| s.bg(rgb(0x374151))),

            ButtonVariant::Outline => button
                .bg(rgb(0xFFFFFF))
                .text_color(rgb(0x3B82F6))
                .border_1()
                .border_color(rgb(0x3B82F6))
                .hover(|s| s.bg(rgb(0xEFF6FF)))
                .active(|s| s.bg(rgb(0xDBEAFE))),

            ButtonVariant::Ghost => button
                .bg(rgba(0x00000000))
                .text_color(rgb(0x374151))
                .hover(|s| s.bg(rgb(0xF3F4F6)))
                .active(|s| s.bg(rgb(0xE5E7EB))),

            ButtonVariant::Danger => button
                .bg(rgb(0xEF4444))
                .text_color(rgb(0xFFFFFF))
                .hover(|s| s.bg(rgb(0xDC2626)))
                .active(|s| s.bg(rgb(0xB91C1C))),
        };

        // 禁用状态
        if self.disabled {
            button = button
                .opacity(0.5)
                .cursor_default();
        }

        button.child(self.label)
    }
}

// ============================================================================
// 第二部分：卡片组件 - 展示阴影和边框
// ============================================================================

/// 卡片样式变体
#[derive(Clone, Copy)]
enum CardVariant {
    Default,
    Elevated,
    Bordered,
}

struct Card;

impl Card {
    fn render(
        variant: CardVariant,
        children: impl IntoElement,
    ) -> Div {
        let mut card = div()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xFFFFFF));

        card = match variant {
            CardVariant::Default => card.shadow_sm(),
            CardVariant::Elevated => card.shadow_lg(),
            CardVariant::Bordered => card.border_1().border_color(rgb(0xE5E7EB)),
        };

        card.child(children)
    }
}

// ============================================================================
// 第三部分：主应用 - 展示完整的样式系统
// ============================================================================

struct StylingApp {
    /// 当前选中的布局模式
    layout_mode: LayoutMode,
    /// 是否显示调试边框
    show_debug: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum LayoutMode {
    Row,
    Column,
    Wrap,
    Grid,
}

impl StylingApp {
    fn new() -> Self {
        Self {
            layout_mode: LayoutMode::Row,
            show_debug: false,
        }
    }

    /// 渲染 Flexbox 布局示例区域
    fn render_flexbox_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xF8FAFC))
            // 标题
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1E293B))
                    .child("📐 Flexbox 布局"),
            )
            // 布局模式切换按钮
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(self.layout_button("Row", LayoutMode::Row, cx))
                    .child(self.layout_button("Column", LayoutMode::Column, cx))
                    .child(self.layout_button("Wrap", LayoutMode::Wrap, cx))
                    .child(self.layout_button("Grid", LayoutMode::Grid, cx)),
            )
            // 布局展示区
            .child(self.render_layout_demo())
    }

    fn layout_button(
        &self,
        label: &'static str,
        mode: LayoutMode,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.layout_mode == mode;

        div()
            .id(SharedString::from(format!("layout-{:?}", mode)))
            .px_3()
            .py_1()
            .rounded_md()
            .cursor_pointer()
            .text_sm()
            // 使用 when 进行条件样式
            .when(is_active, |s| {
                s.bg(rgb(0x3B82F6)).text_color(rgb(0xFFFFFF))
            })
            .when(!is_active, |s| {
                s.bg(rgb(0xE2E8F0))
                    .text_color(rgb(0x475569))
                    .hover(|s| s.bg(rgb(0xCBD5E1)))
            })
            .child(label)
            .on_click(cx.listener(move |view, _event, _window, cx| {
                view.layout_mode = mode;
                cx.notify();
            }))
    }

    fn render_layout_demo(&self) -> impl IntoElement {
        let items = vec![
            ("A", rgb(0xFCA5A5)),
            ("B", rgb(0xFCD34D)),
            ("C", rgb(0x86EFAC)),
            ("D", rgb(0x93C5FD)),
            ("E", rgb(0xC4B5FD)),
        ];

        let mut container = div()
            .min_h(px(150.0))
            .p_4()
            .rounded_lg()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE2E8F0));

        container = match self.layout_mode {
            LayoutMode::Row => container
                .flex()
                .flex_row()
                .gap_4()
                .items_center(),

            LayoutMode::Column => container
                .flex()
                .flex_col()
                .gap_2()
                .items_start(),

            LayoutMode::Wrap => container
                .flex()
                .flex_row()
                .flex_wrap()
                .gap_2(),

            LayoutMode::Grid => container
                .grid()
                .grid_cols(3)
                .gap_2(),
        };

        container.children(items.into_iter().map(|(label, color)| {
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(60.0))
                .h(px(60.0))
                .rounded_md()
                .bg(color)
                .text_color(rgb(0x1F2937))
                .font_weight(FontWeight::BOLD)
                .child(label)
        }))
    }

    /// 渲染按钮展示区
    fn render_buttons_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xF8FAFC))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1E293B))
                    .child("🎨 按钮样式"),
            )
            // 按钮变体
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x64748B))
                            .child("变体 (Variants)"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .flex_wrap()
                            .child(StyledButton::new("Primary").variant(ButtonVariant::Primary).render("btn-primary"))
                            .child(StyledButton::new("Secondary").variant(ButtonVariant::Secondary).render("btn-secondary"))
                            .child(StyledButton::new("Outline").variant(ButtonVariant::Outline).render("btn-outline"))
                            .child(StyledButton::new("Ghost").variant(ButtonVariant::Ghost).render("btn-ghost"))
                            .child(StyledButton::new("Danger").variant(ButtonVariant::Danger).render("btn-danger")),
                    ),
            )
            // 按钮尺寸
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x64748B))
                            .child("尺寸 (Sizes)"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .items_center()
                            .child(StyledButton::new("Small").size(ButtonSize::Small).render("btn-small"))
                            .child(StyledButton::new("Medium").size(ButtonSize::Medium).render("btn-medium"))
                            .child(StyledButton::new("Large").size(ButtonSize::Large).render("btn-large")),
                    ),
            )
            // 禁用状态
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x64748B))
                            .child("状态 (States)"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(StyledButton::new("Normal").render("btn-normal"))
                            .child(StyledButton::new("Disabled").disabled(true).render("btn-disabled")),
                    ),
            )
    }

    /// 渲染文字样式区
    fn render_text_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xF8FAFC))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1E293B))
                    .child("✍️ 文字样式"),
            )
            // 字体大小
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_xs().child("text_xs - 超小号文字"))
                    .child(div().text_sm().child("text_sm - 小号文字"))
                    .child(div().text_base().child("text_base - 基准文字"))
                    .child(div().text_lg().child("text_lg - 大号文字"))
                    .child(div().text_xl().child("text_xl - 超大号文字"))
                    .child(div().text_2xl().child("text_2xl - 特大号文字"))
                    .child(div().text_3xl().child("text_3xl - 巨大号文字")),
            )
            // 字体粗细
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .mt_2()
                    .child(div().font_weight(FontWeight::THIN).child("Thin"))
                    .child(div().font_weight(FontWeight::LIGHT).child("Light"))
                    .child(div().font_weight(FontWeight::NORMAL).child("Normal"))
                    .child(div().font_weight(FontWeight::MEDIUM).child("Medium"))
                    .child(div().font_weight(FontWeight::SEMIBOLD).child("Semibold"))
                    .child(div().font_weight(FontWeight::BOLD).child("Bold"))
                    .child(div().font_weight(FontWeight::EXTRA_BOLD).child("Extra Bold")),
            )
            // 文字装饰
            .child(
                div()
                    .flex()
                    .gap_4()
                    .mt_2()
                    .child(div().italic().child("Italic 斜体"))
                    .child(div().underline().child("Underline 下划线"))
                    .child(div().line_through().child("Strikethrough 删除线")),
            )
            // 文字颜色
            .child(
                div()
                    .flex()
                    .gap_4()
                    .mt_2()
                    .child(div().text_color(rgb(0xEF4444)).child("Red"))
                    .child(div().text_color(rgb(0xF59E0B)).child("Orange"))
                    .child(div().text_color(rgb(0x22C55E)).child("Green"))
                    .child(div().text_color(rgb(0x3B82F6)).child("Blue"))
                    .child(div().text_color(rgb(0x8B5CF6)).child("Purple")),
            )
    }

    /// 渲染卡片展示区
    fn render_cards_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xF8FAFC))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1E293B))
                    .child("🃏 卡片与阴影"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .flex_wrap()
                    // 默认卡片
                    .child(
                        Card::render(
                            CardVariant::Default,
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(div().font_weight(FontWeight::SEMIBOLD).child("Default Card"))
                                .child(div().text_sm().text_color(rgb(0x6B7280)).child("shadow_sm")),
                        ),
                    )
                    // 悬浮卡片
                    .child(
                        Card::render(
                            CardVariant::Elevated,
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(div().font_weight(FontWeight::SEMIBOLD).child("Elevated Card"))
                                .child(div().text_sm().text_color(rgb(0x6B7280)).child("shadow_lg")),
                        ),
                    )
                    // 边框卡片
                    .child(
                        Card::render(
                            CardVariant::Bordered,
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(div().font_weight(FontWeight::SEMIBOLD).child("Bordered Card"))
                                .child(div().text_sm().text_color(rgb(0x6B7280)).child("border_1")),
                        ),
                    ),
            )
    }

    /// 渲染间距与尺寸示例
    fn render_spacing_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xF8FAFC))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1E293B))
                    .child("📏 间距与尺寸"),
            )
            // Padding 示例
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_sm().text_color(rgb(0x64748B)).child("Padding (内边距)"))
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .p_1()
                                    .bg(rgb(0xDBEAFE))
                                    .rounded_md()
                                    .child(div().bg(rgb(0x3B82F6)).text_color(rgb(0xFFFFFF)).child("p_1")),
                            )
                            .child(
                                div()
                                    .p_2()
                                    .bg(rgb(0xDBEAFE))
                                    .rounded_md()
                                    .child(div().bg(rgb(0x3B82F6)).text_color(rgb(0xFFFFFF)).child("p_2")),
                            )
                            .child(
                                div()
                                    .p_4()
                                    .bg(rgb(0xDBEAFE))
                                    .rounded_md()
                                    .child(div().bg(rgb(0x3B82F6)).text_color(rgb(0xFFFFFF)).child("p_4")),
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0xDBEAFE))
                                    .rounded_md()
                                    .child(div().bg(rgb(0x3B82F6)).text_color(rgb(0xFFFFFF)).child("px_4 py_2")),
                            ),
                    ),
            )
            // Gap 示例
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_sm().text_color(rgb(0x64748B)).child("Gap (间隙)"))
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .children((0..5).map(|_| {
                                div().w(px(30.0)).h(px(30.0)).bg(rgb(0x10B981)).rounded_md()
                            }))
                            .child(div().ml_2().text_sm().child("gap_1")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .children((0..5).map(|_| {
                                div().w(px(30.0)).h(px(30.0)).bg(rgb(0x10B981)).rounded_md()
                            }))
                            .child(div().ml_2().text_sm().child("gap_4")),
                    ),
            )
            // 固定尺寸 vs 弹性尺寸
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_sm().text_color(rgb(0x64748B)).child("尺寸"))
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .w(px(100.0))
                                    .h(px(40.0))
                                    .bg(rgb(0x8B5CF6))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(0xFFFFFF))
                                    .text_xs()
                                    .child("w(px(100))"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .h(px(40.0))
                                    .bg(rgb(0xEC4899))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(0xFFFFFF))
                                    .text_xs()
                                    .child("flex_1 (填充剩余)"),
                            ),
                    ),
            )
    }

    /// 渲染边框和圆角示例
    fn render_borders_section(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xF8FAFC))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x1E293B))
                    .child("🔲 边框与圆角"),
            )
            // 边框宽度
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .w(px(80.0))
                            .h(px(50.0))
                            .bg(rgb(0xFFFFFF))
                            .border_1()
                            .border_color(rgb(0x3B82F6))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .child("border_1"),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .h(px(50.0))
                            .bg(rgb(0xFFFFFF))
                            .border_2()
                            .border_color(rgb(0x3B82F6))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .child("border_2"),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .h(px(50.0))
                            .bg(rgb(0xFFFFFF))
                            .border_4()
                            .border_color(rgb(0x3B82F6))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .child("border_4"),
                    ),
            )
            // 圆角
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .w(px(60.0))
                            .h(px(60.0))
                            .bg(rgb(0x10B981))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .text_xs()
                            .child("none"),
                    )
                    .child(
                        div()
                            .w(px(60.0))
                            .h(px(60.0))
                            .bg(rgb(0x10B981))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .text_xs()
                            .child("rounded"),
                    )
                    .child(
                        div()
                            .w(px(60.0))
                            .h(px(60.0))
                            .bg(rgb(0x10B981))
                            .rounded_md()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .text_xs()
                            .child("md"),
                    )
                    .child(
                        div()
                            .w(px(60.0))
                            .h(px(60.0))
                            .bg(rgb(0x10B981))
                            .rounded_lg()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .text_xs()
                            .child("lg"),
                    )
                    .child(
                        div()
                            .w(px(60.0))
                            .h(px(60.0))
                            .bg(rgb(0x10B981))
                            .rounded_xl()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .text_xs()
                            .child("xl"),
                    )
                    .child(
                        div()
                            .w(px(60.0))
                            .h(px(60.0))
                            .bg(rgb(0x10B981))
                            .rounded_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .text_xs()
                            .child("full"),
                    ),
            )
    }

    /// 渲染调试工具栏
    fn render_debug_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_4()
            .p_2()
            .bg(rgb(0x1F2937))
            .rounded_lg()
            .child(
                div()
                    .id("toggle-debug")
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .when(self.show_debug, |s| s.bg(rgb(0x10B981)))
                    .when(!self.show_debug, |s| s.bg(rgb(0x374151)))
                    .text_color(rgb(0xFFFFFF))
                    .text_sm()
                    .child(if self.show_debug { "🔍 Debug ON" } else { "🔍 Debug OFF" })
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.show_debug = !view.show_debug;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x9CA3AF))
                    .child("提示：开启后可看到元素边框（仅 debug 构建）"),
            )
    }
}

impl Render for StylingApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut content = div()
            .id("main-content")  // 添加 id 以启用滚动
            .size_full()
            .flex()
            .flex_col()
            .overflow_y_scroll()  // 启用垂直滚动
            .bg(rgb(0xE2E8F0))
            .p_6()
            .gap_6();

        // 标题
        content = content.child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(rgb(0x1E293B))
                        .child("第四章：样式系统 🎨"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x64748B))
                        .child("学习 GPUI 类似 Tailwind CSS 的样式 API"),
                ),
        );

        // 调试工具栏
        content = content.child(self.render_debug_toolbar(cx));

        // 主内容区域 - 两列布局
        content = content.child(
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
                        .child(self.render_flexbox_section(cx))
                        .child(self.render_buttons_section())
                        .child(self.render_spacing_section()),
                )
                // 右列
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap_6()
                        .child(self.render_text_section())
                        .child(self.render_cards_section())
                        .child(self.render_borders_section()),
                ),
        );

        // 知识点总结
        content = content.child(
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
                        .child("💡 本章涵盖：Flexbox布局 | 尺寸间距 | 颜色背景 | 边框圆角 | 阴影透明度 | 文字样式 | hover/active伪状态 | when条件样式"),
                ),
        );

        // 调试模式
        #[cfg(debug_assertions)]
        if self.show_debug {
            content = content.debug_below();
        }

        content
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
                        width: px(1100.0),
                        height: px(900.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("第四章：样式系统".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| StylingApp::new()),
        )
        .unwrap();
    });
}

/* ==========================================================================
   🎓 GPUI 样式系统知识点总结
   ==========================================================================

   一、样式 API 概述
   -----------------
   GPUI 使用类似 Tailwind CSS 的方法链式调用 API：

   div()
       .flex()           // 显示模式
       .flex_col()       // Flex 方向
       .gap_4()          // 间隙
       .p_4()            // 内边距
       .bg(rgb(0xFFFFFF))// 背景色
       .rounded_lg()     // 圆角
       .shadow_md()      // 阴影

   二、Flexbox 布局
   ----------------
   显示模式：
   - .flex()              启用 flex 布局
   - .block()             块级布局
   - .grid()              网格布局
   - .hidden()            隐藏元素

   方向：
   - .flex_row()          水平方向（默认）
   - .flex_col()          垂直方向
   - .flex_row_reverse()  水平反向
   - .flex_col_reverse()  垂直反向

   对齐（主轴 justify）：
   - .justify_start()     开始对齐
   - .justify_center()    居中对齐
   - .justify_end()       结束对齐
   - .justify_between()   两端对齐
   - .justify_around()    均匀分布

   对齐（交叉轴 items）：
   - .items_start()       开始对齐
   - .items_center()      居中对齐
   - .items_end()         结束对齐
   - .items_baseline()    基线对齐

   弹性：
   - .flex_1()            flex: 1 1 0%
   - .flex_auto()         flex: 1 1 auto
   - .flex_none()         flex: 0 0 auto
   - .flex_grow()         允许增长
   - .flex_shrink()       允许收缩
   - .flex_wrap()         允许换行

   三、尺寸
   --------
   固定尺寸：
   - .w(px(100.0))        宽度 100 像素
   - .h(px(50.0))         高度 50 像素
   - .size(px(100.0))     宽高都是 100
   - .min_w(px(50.0))     最小宽度
   - .max_w(px(200.0))    最大宽度
   - .min_h(px(50.0))     最小高度
   - .max_h(px(200.0))    最大高度

   相对尺寸：
   - .w_full()            宽度 100%
   - .h_full()            高度 100%
   - .size_full()         宽高都是 100%

   四、间距
   --------
   内边距 (Padding)：
   - .p_1(), .p_2(), .p_4(), .p_8() 等   四周内边距
   - .px_4()              水平内边距
   - .py_2()              垂直内边距
   - .pt_2(), .pb_2()     顶部/底部
   - .pl_2(), .pr_2()     左侧/右侧

   外边距 (Margin)：
   - .m_1(), .m_2(), .m_4() 等           四周外边距
   - .mx_4()              水平外边距
   - .my_2()              垂直外边距
   - .mt_2(), .mb_2()     顶部/底部
   - .ml_2(), .mr_2()     左侧/右侧

   间隙 (Gap)：
   - .gap_1(), .gap_2(), .gap_4() 等     子元素间隙

   五、颜色
   --------
   背景色：
   - .bg(rgb(0xFFFFFF))       白色背景
   - .bg(rgba(0x00000080))    半透明黑色

   文字颜色：
   - .text_color(rgb(0x000000))  黑色文字

   边框颜色：
   - .border_color(rgb(0xCCCCCC))

   颜色函数：
   - rgb(0xRRGGBB)            RGB 颜色
   - rgba(0xRRGGBBAA)         RGBA 颜色
   - hsla(h, s, l, a)         HSLA 颜色

   六、边框与圆角
   --------------
   边框宽度：
   - .border_1()          1px 边框
   - .border_2()          2px 边框
   - .border_4()          4px 边框

   圆角：
   - .rounded(px(4.0))    小圆角（需要参数）
   - .rounded_md()        中等圆角
   - .rounded_lg()        大圆角
   - .rounded_xl()        超大圆角
   - .rounded_full()      完全圆形

   七、阴影
   --------
   - .shadow_sm()         小阴影
   - .shadow_md()         中等阴影
   - .shadow_lg()         大阴影
   - .shadow_xl()         超大阴影

   八、文字样式
   ------------
   字体大小：
   - .text_xs()           extra small
   - .text_sm()           small
   - .text_base()         base (默认)
   - .text_lg()           large
   - .text_xl()           extra large
   - .text_2xl(), .text_3xl()  更大

   字体粗细：
   - .font_weight(FontWeight::NORMAL)
   - .font_weight(FontWeight::MEDIUM)
   - .font_weight(FontWeight::SEMIBOLD)
   - .font_weight(FontWeight::BOLD)

   文字装饰：
   - .italic()            斜体
   - .underline()         下划线
   - .line_through()      删除线
   - .truncate()          超出截断...

   九、伪状态
   ----------
   - .hover(|s| s.bg(...))     鼠标悬停
   - .active(|s| s.bg(...))    鼠标按下
   - .focus(|s| s.bg(...))     获得焦点

   十、条件样式
   ------------
   - .when(condition, |s| s.bg(...))    条件为真时应用
   - .when_some(option, |s, v| ...)     Option 有值时应用

   十一、透明度
   ------------
   - .opacity(0.5)        50% 透明度

   十二、调试
   ----------
   - .debug()             显示元素边框（仅 debug 构建）
   - .debug_below()       显示所有子元素边框

   运行命令：
   ---------
   cargo run -p gpui_styling

========================================================================== */
