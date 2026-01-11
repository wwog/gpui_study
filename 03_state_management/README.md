# 第三章：GPUI 状态管理

本章深入学习 GPUI 中的状态传递机制，包括父子组件通信、事件系统和观察者模式。

## 核心概念

### Entity<T> - 实体句柄

在 GPUI 中，所有的 View 和 Model 都由 `App` 统一管理。`Entity<T>` 是对实体的**引用句柄**，不是实体本身：

```rust
// Entity<T> 类似于 Rc<T>，但不直接提供对状态的访问
let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });

// 必须通过 read 或 update 来访问状态
let count = counter.read(cx).count;  // 只读访问
counter.update(cx, |counter, cx| {   // 可变访问
    counter.count += 1;
    cx.notify();
});
```

### Context<T> - 实体上下文

`Context<T>` 是绑定到特定实体的上下文，提供：

- `cx.notify()` - 通知需要重新渲染
- `cx.emit(event)` - 发出事件
- `cx.entity()` - 获取当前实体的句柄
- `cx.subscribe()` / `cx.observe()` - 订阅/观察其他实体

## 状态传递方式

### 1. 组件自身状态

```rust
struct MyView {
    count: i32,    // 状态字段
}

impl MyView {
    fn increment(&mut self, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();  // 重要：必须调用以触发重新渲染
    }
}
```

### 2. 父组件 → 子组件

**方式一：构造函数参数（推荐）**

```rust
// 父组件创建子组件时传递初始值
let child = cx.new(|_cx| ChildView::new(
    "名称",           // 传递配置
    initial_value,    // 传递初始状态
));
```

**方式二：Entity::update（动态修改）**

```rust
// 父组件直接修改子组件状态
self.child.update(cx, |child, cx| {
    child.value = new_value;
    cx.notify();
});
```

### 3. 子组件 → 父组件

**方式一：事件系统（推荐）**

```rust
// 1. 定义事件
struct ValueChangedEvent {
    new_value: i32,
}

// 2. 子组件实现 EventEmitter
impl EventEmitter<ValueChangedEvent> for ChildView {}

// 3. 子组件发出事件
impl ChildView {
    fn change_value(&mut self, cx: &mut Context<Self>) {
        self.value = 42;
        cx.emit(ValueChangedEvent { new_value: 42 });
        cx.notify();
    }
}

// 4. 父组件订阅事件
let subscription = cx.subscribe(&child, |parent, _emitter, event, cx| {
    parent.handle_child_change(event.new_value);
    cx.notify();
});
// 重要：保存 subscription，否则订阅会失效
```

**方式二：观察者模式**

```rust
// 监听子组件的 notify() 调用
let subscription = cx.observe(&child, |parent, child, cx| {
    // 读取子组件当前状态
    let value = child.read(cx).value;
    parent.sync_from_child(value);
});
```

### 4. 兄弟组件通信

通过共同的父组件中转：

```rust
struct ParentView {
    sibling_a: Entity<SiblingA>,
    sibling_b: Entity<SiblingB>,
}

impl ParentView {
    fn sync_siblings(&mut self, cx: &mut Context<Self>) {
        // 读取 A 的状态
        let value = self.sibling_a.read(cx).value;
        
        // 更新到 B
        self.sibling_b.update(cx, |b, cx| {
            b.value = value;
            cx.notify();
        });
    }
}
```

## Subscription 生命周期

⚠️ **重要**：`subscribe` 和 `observe` 返回的 `Subscription` 必须保存，丢弃后订阅自动失效：

```rust
struct MyView {
    // 保存订阅以保持其活跃
    _subscriptions: Vec<Subscription>,
}

impl MyView {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut subscriptions = Vec::new();
        
        // 订阅会一直有效，直到 subscription 被 drop
        let sub = cx.subscribe(&other_entity, |this, _, event, cx| {
            // 处理事件
        });
        subscriptions.push(sub);
        
        Self {
            _subscriptions: subscriptions,
        }
    }
}
```

也可以调用 `.detach()` 使订阅在实体存活期间一直有效：

```rust
cx.subscribe(&entity, |this, _, event, cx| {
    // 处理事件
}).detach();  // 订阅将持续到实体被释放
```

## observe vs subscribe

| 特性 | `observe` | `subscribe` |
|------|-----------|-------------|
| 触发条件 | 目标调用 `notify()` | 目标调用 `emit(Event)` |
| 接收数据 | 无（需主动 read） | 事件对象 |
| 用途 | 通用状态监听 | 特定事件处理 |
| 要求 | 无 | 目标实现 `EventEmitter<E>` |

## 完整示例说明

运行本章示例：

```bash
cargo run -p gpui_state_management
```

示例包含：
1. **ChildCounter** - 子计数器组件，通过事件通知父组件
2. **TotalDisplay** - 观察者组件，显示汇总信息
3. **StateManagementApp** - 父组件，协调所有子组件

交互操作：
- 点击 +/- 按钮修改子计数器
- 观察父组件接收到的事件日志
- 点击"重置所有"测试父组件修改子组件
- 点击"同步 B <- A"测试兄弟组件通信

## API 参考

### Entity<T> 方法

```rust
entity.read(cx)                     // 只读访问实体状态
entity.update(cx, |state, cx| ...)  // 可变访问实体状态
entity.entity_id()                  // 获取实体 ID
entity.downgrade()                  // 转为弱引用 WeakEntity<T>
```

### Context<T> 方法

```rust
cx.new(|cx| State::new())           // 创建新实体
cx.entity()                         // 获取当前实体的 Entity<T>
cx.weak_entity()                    // 获取弱引用
cx.notify()                         // 通知需要重新渲染
cx.emit(event)                      // 发出事件（需实现 EventEmitter）
cx.subscribe(&entity, callback)     // 订阅事件
cx.observe(&entity, callback)       // 观察 notify
cx.listener(|this, event, window, cx| ...) // 创建事件监听器闭包
```

## 下一章预告

第四章将学习 GPUI 的样式系统，包括 Flexbox 布局、颜色、边框、阴影等。

