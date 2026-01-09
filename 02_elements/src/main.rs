// 第二章：元素系统 - 综合导航
// 这是第二章的主入口，展示所有步骤的概览

use gpui::*;

// ============================================================================
// 主导航界面
// ============================================================================

struct ChapterTwoOverview;

impl Render for ChapterTwoOverview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0xF9FAFB)).child(
            div()
                .max_w(px(1200.0))
                .mx_auto()
                .p_8()
                .flex()
                .flex_col()
                .gap_8()
                // 标题区域
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_3xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x1F2937))
                                .child("第二章：元素系统"),
                        )
                        .child(
                            div()
                                .text_lg()
                                .text_color(rgb(0x6B7280))
                                .child("学习 GPUI 的元素系统，构建复杂的 UI 结构"),
                        ),
                )
                // 学习路径说明
                .child(
                    div()
                        .p_6()
                        .bg(rgb(0xDCFCE7))
                        .border_1()
                        .border_color(rgb(0x10B981))
                        .rounded_xl()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(
                            div()
                                .text_xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x065F46))
                                .child("📚 学习路径"),
                        )
                        .child(
                            div()
                                .text_color(rgb(0x064E3B))
                                .child("本章采用步进式教学，每个步骤都建立在前一步的基础上。"),
                        )
                        .child(
                            div()
                                .text_color(rgb(0x064E3B))
                                .child("建议按顺序学习，每个步骤都包含详细的示例和练习。"),
                        ),
                )
                // 步骤卡片网格
                .child(
                    div()
                        .grid()
                        .grid_cols(2)
                        .gap_6()
                        // 步骤 1
                        .child(create_step_card(
                            "1️⃣".to_string(),
                            "Element trait 基础".to_string(),
                            "理解元素的本质和 IntoElement trait".to_string(),
                            vec![
                                "Element trait".to_string(),
                                "IntoElement".to_string(),
                                "类型转换".to_string(),
                            ],
                            0x3B82F6,
                            "📖 理论基础",
                        ))
                        // 步骤 2
                        .child(create_step_card(
                            "2️⃣".to_string(),
                            "div() 创建容器".to_string(),
                            "学习最基础的容器元素和链式调用".to_string(),
                            vec![
                                "div()".to_string(),
                                "链式调用".to_string(),
                                "样式方法".to_string(),
                            ],
                            0x10B981,
                            "cargo run --bin step2_div_basics",
                        ))
                        // 步骤 3
                        .child(create_step_card(
                            "3️⃣".to_string(),
                            "child() 添加子元素".to_string(),
                            "学习如何使用 child() 添加单个子元素".to_string(),
                            vec![
                                "child()".to_string(),
                                "嵌套".to_string(),
                                "多次调用".to_string(),
                            ],
                            0xF59E0B,
                            "cargo run --bin step3_child",
                        ))
                        // 步骤 4
                        .child(create_step_card(
                            "4️⃣".to_string(),
                            "children() 批量添加".to_string(),
                            "使用迭代器批量添加多个子元素".to_string(),
                            vec![
                                "children()".to_string(),
                                "迭代器".to_string(),
                                "map/filter".to_string(),
                            ],
                            0xEC4899,
                            "cargo run --bin step4_children",
                        ))
                        // 步骤 5
                        .child(create_step_card(
                            "5️⃣".to_string(),
                            "嵌套与组合".to_string(),
                            "构建复杂的嵌套 UI 结构和布局".to_string(),
                            vec![
                                "多层嵌套".to_string(),
                                "复杂布局".to_string(),
                                "组合模式".to_string(),
                            ],
                            0x8B5CF6,
                            "cargo run --bin step5_nesting",
                        ))
                        // 步骤 6
                        .child(create_step_card(
                            "6️⃣".to_string(),
                            "实战练习".to_string(),
                            "综合运用所学知识完成项目".to_string(),
                            vec![
                                "博客列表".to_string(),
                                "仪表盘".to_string(),
                                "个人资料".to_string(),
                            ],
                            0xEF4444,
                            "📝 动手实践",
                        )),
                )
                // 核心概念总结
                .child(
                    div()
                        .p_6()
                        .bg(rgb(0xFFFFFF))
                        .rounded_xl()
                        .shadow_lg()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(
                            div()
                                .text_2xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x1F2937))
                                .child("🎯 核心概念"),
                        )
                        .child(
                            div()
                                .grid()
                                .grid_cols(3)
                                .gap_4()
                                .child(create_concept_card(
                                    "Element".to_string(),
                                    "所有 UI 元素的抽象".to_string(),
                                    "🧱".to_string(),
                                ))
                                .child(create_concept_card(
                                    "IntoElement".to_string(),
                                    "可转换为 Element 的类型".to_string(),
                                    "🔄".to_string(),
                                ))
                                .child(create_concept_card(
                                    "ParentElement".to_string(),
                                    "可包含子元素的容器".to_string(),
                                    "📦".to_string(),
                                ))
                                .child(create_concept_card(
                                    "div()".to_string(),
                                    "最常用的容器元素".to_string(),
                                    "⬜".to_string(),
                                ))
                                .child(create_concept_card(
                                    "child()".to_string(),
                                    "添加单个子元素".to_string(),
                                    "➕".to_string(),
                                ))
                                .child(create_concept_card(
                                    "children()".to_string(),
                                    "批量添加子元素".to_string(),
                                    "✨".to_string(),
                                )),
                        ),
                )
                // 下一步
                .child(
                    div()
                        .p_6()
                        .bg(rgb(0xDDD6FE))
                        .border_1()
                        .border_color(rgb(0x8B5CF6))
                        .rounded_xl()
                        .flex()
                        .items_center()
                        .gap_4()
                        .child(
                            div()
                                .w(px(60.0))
                                .h(px(60.0))
                                .bg(rgb(0x8B5CF6))
                                .rounded_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_3xl()
                                .child("🚀"),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xl()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x5B21B6))
                                        .child("准备好了吗？"),
                                )
                                .child(div().text_color(rgb(0x6B21A8)).child(
                                    "完成本章后，继续学习第三章：状态管理，让你的 UI 可以交互！",
                                )),
                        ),
                ),
        )
    }
}

// ============================================================================
// 辅助函数：创建步骤卡片
// ============================================================================

fn create_step_card(
    icon: String,
    title: String,
    description: String,
    tags: Vec<String>,
    color: u32,
    command: &str,
) -> Div {
    div()
        .flex()
        .flex_col()
        .bg(rgb(0xFFFFFF))
        .rounded_xl()
        .shadow_lg()
        .overflow_hidden()
        .hover(|style| style.shadow_2xl())
        // 顶部彩色条
        .child(div().h(px(6.0)).bg(rgb(color)))
        // 卡片内容
        .child(
            div()
                .p_6()
                .flex()
                .flex_col()
                .gap_4()
                // 图标和标题
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(div().text_3xl().child(icon))
                        .child(
                            div()
                                .text_xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x1F2937))
                                .child(title),
                        ),
                )
                // 描述
                .child(div().text_sm().text_color(rgb(0x6B7280)).child(description))
                // 标签
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_2()
                        .children(tags.iter().map(|tag| {
                            div()
                                .px_2()
                                .py_1()
                                .bg(rgb(0xF3F4F6))
                                .text_color(rgb(0x4B5563))
                                .rounded_md()
                                .text_xs()
                                .child(tag.clone())
                        })),
                )
                // 运行命令
                .child(
                    div()
                        .mt_2()
                        .px_3()
                        .py_2()
                        .bg(rgb(0xF9FAFB))
                        .border_1()
                        .border_color(rgb(0xE5E7EB))
                        .rounded_md()
                        .text_xs()
                        .font_family("monospace")
                        .text_color(rgb(color))
                        .child(command.to_string()),
                ),
        )
}

// ============================================================================
// 辅助函数：创建概念卡片
// ============================================================================

fn create_concept_card(title: String, description: String, icon: String) -> Div {
    div()
        .p_4()
        .bg(rgb(0xF9FAFB))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xE5E7EB))
        .flex()
        .flex_col()
        .gap_2()
        .hover(|style| style.bg(rgb(0xF3F4F6)))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().text_2xl().child(icon))
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x1F2937))
                        .child(title),
                ),
        )
        .child(div().text_xs().text_color(rgb(0x6B7280)).child(description))
}

// ============================================================================
// 程序入口
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
                        width: px(1200.0),
                        height: px(900.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("第二章：元素系统".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_| ChapterTwoOverview),
        )
        .unwrap();
    });
}

/*
===============================================================================
第二章：元素系统 - 学习指南
===============================================================================

📚 本章内容
-----------
本章深入学习 GPUI 的元素系统，这是构建 UI 的基础。通过 6 个步骤，
你将掌握从简单元素到复杂布局的所有知识。

🎯 学习目标
-----------
1. 理解 Element 和 IntoElement trait
2. 熟练使用 div() 创建容器
3. 掌握 child() 和 children() 的用法
4. 构建复杂的嵌套 UI 结构
5. 学会常见的布局模式

📖 步骤说明
-----------
步骤 1: Element trait 基础 (理论)
  - 理解元素的抽象概念
  - 了解 IntoElement 的作用
  - 阅读 README.md

步骤 2: div() 创建容器 (实践)
  - 学习 div() 的基本用法
  - 掌握链式调用
  - 运行: cargo run --bin step2_div_basics

步骤 3: child() 添加子元素 (实践)
  - 学习添加单个子元素
  - 理解嵌套结构
  - 运行: cargo run --bin step3_child

步骤 4: children() 批量添加 (实践)
  - 学习批量添加元素
  - 掌握迭代器用法
  - 运行: cargo run --bin step4_children

步骤 5: 嵌套与组合 (综合)
  - 构建复杂布局
  - 学习设计模式
  - 运行: cargo run --bin step5_nesting

步骤 6: 实战练习 (挑战)
  - 完成综合项目
  - 巩固所学知识
  - 查看 README.md 的练习部分

🚀 快速开始
-----------
1. 运行本程序查看概览：
   cargo run -p gpui_elements

2. 开始步骤 2 的学习：
   cargo run --bin step2_div_basics

3. 阅读详细文档：
   cat 02_elements/README.md

💡 学习建议
-----------
• 按顺序学习，每个步骤都很重要
• 运行所有示例代码，观察效果
• 完成每个步骤的练习题
• 尝试修改代码，实验不同效果
• 遇到问题查看注释和文档

📊 核心 API 速查
-----------
div()                    - 创建容器元素
.child(element)          - 添加单个子元素
.children(iterator)      - 批量添加子元素
.w(px(n)) / .h(px(n))   - 设置宽高
.bg(color)              - 设置背景色
.flex()                 - 启用 flexbox
.grid()                 - 启用 grid 布局

===============================================================================
准备好了吗？让我们开始第二章的学习之旅！🎉
===============================================================================
*/
