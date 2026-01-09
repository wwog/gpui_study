# 🚀 GPUI 学习项目启动指南

欢迎！这份指南将帮助你快速开始 GPUI 的学习之旅。

## 📋 前置要求

在开始之前，请确保你的系统已安装：

### 必需工具
- **Rust** (1.70+): [安装 Rust](https://rustup.rs/)
  ```bash
  # 检查 Rust 版本
  rustc --version
  cargo --version
  ```

- **Git**: 用于版本控制（可选但推荐）

### 系统要求
- **操作系统**: Windows 10/11, macOS 10.15+, 或 Linux
- **GPU**: 支持 OpenGL 3.3+ 或 Metal (macOS) 或 DirectX 12 (Windows)
- **内存**: 至少 4GB RAM

### 平台特定要求

#### Windows
- Visual Studio Build Tools 或 Visual Studio 2019+
- Windows SDK

#### macOS
- Xcode Command Line Tools
  ```bash
  xcode-select --install
  ```

#### Linux
需要以下开发库：
```bash
# Ubuntu/Debian
sudo apt-get install libfontconfig1-dev libfreetype6-dev libxcb-xfixes0-dev libxkbcommon-dev

# Fedora
sudo dnf install fontconfig-devel freetype-devel libxcb-devel libxkbcommon-devel
```

## 🛠️ 项目设置

### 1. 克隆/进入项目目录

```bash
cd gpui_study
```

### 2. 构建项目

首次构建可能需要几分钟，因为需要编译 GPUI 及其依赖：

```bash
# 构建所有 workspace 成员
cargo build
```

### 3. 验证安装

运行 Hello World 示例来验证一切正常：

```bash
cargo run --example hello_world
```

如果看到一个窗口显示 "Hello World! 👋"，恭喜你，安装成功！🎉

## 🎓 学习路径

### 第一步：运行示例程序

先熟悉几个示例程序：

```bash
# Hello World - 最简单的应用
cargo run --example hello_world

# Counter - 带状态的交互应用
cargo run --example counter

# Todo List - 综合示例（后续会添加）
cargo run --example todo_list
```

### 第二步：按章节学习

项目分为 6 个章节，建议按顺序学习：

#### 📘 第一章：基础概念 (01_basics)
```bash
cd 01_basics
cargo run
```
**学习内容：**
- Application、Window、Context
- Render trait 基础
- 第一个 GPUI 应用

**阅读：** `01_basics/README.md`

---

#### 📗 第二章：元素系统 (02_elements)
```bash
cd ../02_elements
cargo run
```
**学习内容：**
- Element trait
- div() 和元素组合
- UI 结构构建

**阅读：** `02_elements/README.md`

---

#### 📙 第三章：状态管理 (03_state_management)
```bash
cd ../03_state_management
cargo run
```
**学习内容：**
- 响应式状态
- cx.notify() 机制
- Model 概念

**阅读：** `03_state_management/README.md`

---

#### 📕 第四章：样式系统 (04_styling)
```bash
cd ../04_styling
cargo run
```
**学习内容：**
- Styled trait
- Flexbox 布局
- 颜色和主题

**阅读：** `04_styling/README.md`

---

#### 📓 第五章：事件处理 (05_events)
```bash
cd ../05_events
cargo run
```
**学习内容：**
- 鼠标和键盘事件
- cx.listener()
- 事件传播

**阅读：** `05_events/README.md`

---

#### 📔 第六章：高级主题 (06_advanced)
```bash
cd ../06_advanced
cargo run
```
**学习内容：**
- Action 系统
- 焦点管理
- 异步操作

**阅读：** `06_advanced/README.md`

---

## 💡 学习建议

### ✅ DO（推荐做法）

1. **按顺序学习** - 每章都建立在前一章的基础上
2. **动手实践** - 亲自编写代码，不要只是阅读
3. **完成测验** - 每章末尾的测验帮助巩固知识
4. **修改代码** - 尝试改变参数，观察效果
5. **做笔记** - 记录你的理解和问题
6. **查阅文档** - 遇到问题时查看官方文档

### ❌ DON'T（避免）

1. **不要跳章** - 基础概念很重要
2. **不要只看不练** - 编程是实践技能
3. **不要死记硬背** - 理解原理更重要
4. **不要害怕出错** - 错误是最好的老师

## 🔧 常用命令

```bash
# 运行特定章节
cargo run -p gpui_basics          # 第一章
cargo run -p gpui_elements         # 第二章
cargo run -p gpui_state_management # 第三章
# ... 以此类推

# 运行示例
cargo run --example <example_name>

# 构建所有项目（不运行）
cargo build --workspace

# 检查代码（不编译）
cargo check --workspace

# 清理构建产物
cargo clean

# 查看依赖树
cargo tree

# 更新依赖
cargo update
```

## 📚 资源链接

### 官方资源
- 🌐 [GPUI 官网](https://www.gpui.rs/)
- 📖 [GPUI Book](https://matinaniss.github.io/gpui-book/)
- 📦 [GPUI Crates.io](https://crates.io/crates/gpui)
- 🔧 [Zed 编辑器源码](https://github.com/zed-industries/zed)

### 社区资源
- 💬 [Zed Discord](https://discord.gg/zed) - 获取帮助和交流
- 🐦 [Zed Twitter](https://twitter.com/zed_dot_dev)
- 📺 YouTube 搜索 "GPUI tutorial"

### Rust 学习资源
- 📕 [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- 📘 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 🦀 [Rustlings](https://github.com/rust-lang/rustlings)

## ❓ 常见问题

### Q1: 编译很慢怎么办？

**A:** GPUI 是一个大型库，首次编译需要时间。建议：
- 使用 `cargo check` 快速检查代码
- 启用增量编译（默认开启）
- 考虑使用 [sccache](https://github.com/mozilla/sccache) 或 [mold](https://github.com/rui314/mold)

### Q2: 窗口无法打开？

**A:** 可能的原因：
- 检查 GPU 驱动是否更新
- 确认系统支持所需的图形 API
- 查看终端输出的错误信息

### Q3: 代码无法编译？

**A:** 
- 确保 Rust 版本 >= 1.70
- 运行 `cargo clean` 清理后重试
- 检查是否有 typo
- 查看编译器错误提示

### Q4: 如何调试 GPUI 应用？

**A:**
- 使用 `dbg!()` 宏打印调试信息
- 使用 `println!()` 输出到控制台
- 使用 VS Code 或其他 IDE 的调试功能
- 查看 GPUI 的日志输出

### Q5: 性能问题？

**A:**
- 确保使用 release 模式：`cargo run --release`
- 避免在 `render()` 中执行耗时操作
- 合理使用 `cx.notify()` 避免过度渲染

### Q6: 找不到某个类型或函数？

**A:**
- 检查导入：`use gpui::*;`
- 查看 GPUI 文档：[docs.rs/gpui](https://docs.rs/gpui)
- 搜索 Zed 源码看如何使用

## 🎯 学习进度追踪

在 README.md 中有一个完整的检查清单，建议你：

1. 打开 `README.md`
2. 在每个复选框前打勾 ✅
3. 记录学习日期和笔记
4. 完成所有测验题目

## 🤝 获取帮助

遇到问题？有以下途径：

1. **查看 README** - 每章都有详细说明
2. **阅读代码注释** - 代码中有大量注释
3. **搜索错误信息** - Google/Stack Overflow
4. **加入 Discord** - Zed 社区很活跃
5. **查看示例** - examples 目录有完整示例

## 🎊 准备开始

现在你已经准备好开始学习了！

### 下一步：

1. ✅ 确认环境已设置好
2. ✅ 运行 `cargo run --example hello_world` 测试
3. ✅ 打开 `01_basics/README.md` 开始第一章
4. ✅ 运行 `cargo run -p gpui_basics`
5. ✅ 动手编写你的第一个 GPUI 应用

```bash
# 让我们开始吧！
cd 01_basics
cat README.md  # 阅读第一章说明
cargo run      # 运行第一章代码
```

---

**祝学习愉快！记住：实践是最好的老师。** 🚀

有问题随时查看文档或寻求帮助。你能行的！💪