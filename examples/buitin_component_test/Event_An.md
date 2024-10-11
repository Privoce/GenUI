## 组件事件

### No-Hover模式

将 Hover 事件拆解为 HoverIn 和 HoverOut 来减少不必要的重复传递，确实可以有效降低性能开销，尤其是当鼠标移动频繁时。通过这种方式，你可以更精确地管理组件间的事件冒泡，并减少系统负担。

### 冒泡传递的统一事件 UnifiedEvent

只传递HoverIn和HoverOut

### 事件是否触发

event_key: bool

### 动画是否触发

animation_key: bool

### 动画类型

- Hover 触摸
- Focus 聚焦（属于持续状态）
  - 选中 selected
  - 开启 opened
  - 展开 expanded
  - 长按 pressed
