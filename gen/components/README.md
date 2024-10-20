# GEN Components (v0.1.0)

- version: `v0.1.0`
- update date: 2024-10-24
- author: [Will-YiFei Sheng](syf20020816@outlook.com)
- makepad_widget version: `v0.6.0`
- makepad branch: `rik`

## Basic Builtin Component

- [x] Font 🆗
- [x] Label 🆗
- [x] Link 🆗
- [x] Button 🆗
- [x] View 🆗
  - [x] VLayout 🆗
  - [x] HLayout 🆗
- [x] Radio 🆗
- [x] CheckBox 🆗
- [x] Icon 🆗
- [x] Svg 🆗
- [x] Image 🆗
- [x] Input 🆗
- [x] Popup 🆗
- [x] Progress 🆗
- [x] Loading 🆗
- [x] Tag 🆗
- [x] DropDown 🆗
- [x] Toggle 🆗
- [x] Collapse 🆗
- [x] ToolTip 🆗
- [x] Tab 🆗
- [x] Table 🆗
- [x] Dialog 🆗
- [x] Splitter 🆗
- [x] FileUpload 🆗
- [x] Divider 🆗
- [x] State 🆗
- [x] BreadCrumb 🆗
- [x] ScrollBar(s) 🆗
- [x] icon_lib 🆗
---
- [x] Virtual Table
- [x] Window
- [x] ToolButton
- [x] RadioGroup
- [ ] Virtual RadioGroup
- [ ] CheckBoxGroup
- [ ] Virtual CheckBoxGroup
- [x] Select
- [x] Drawer
- [x] Router
  - [x] Page
- [x] Tabbar
- [x] Menu


## GEN Components (Super, v0.2.0 ~ v0.5.0)

- [ ] Metric (用于数据统计)
- [x] Route (用于路由跳转) 🔼
- [x] ToolButton (用于处理界面控制事件，如：放大，缩小，全屏，关闭等) 🔼
- [ ] Notification (用于编程式弹出通知) (测试成功, 思路cx.global绑定到窗口或Root上，可能需要后续自己实现一个Root) 🔼
- [x] Drawer (用于侧面展开) 
- [ ] ColorPicker (颜色选择器)
- [ ] DatePicker(日期选择器)
- [ ] TimePicker(时间选择器)
- [ ] NumberInput(数字输入框)
- [ ] Badge(勋章，按钮和图标上的数字或状态标记)
- [ ] Carousel(在有限空间内，循环播放同一类型的图片、文字等内容)
- [ ] TimeLine(时间轴)
- [ ] Calender(日历)
- [ ] Container(用于快速放置界面)
- [ ] Audio
- [ ] Video
- [ ] Virtual Select(提供抽象与扩展能力，自定义模型)
- [ ] Animation Optimize
- [ ] Table ReBuild
- [ ] Code
- [ ] State Card for Doc(Note, Important, Info)
- [ ] Button ReBuild (icon + text + slot)
- [ ] Component disable (组件禁用状态)
  
## Experimental function (v0.1.0 ~ v0.5.0)

- [x] Radio Switch Container (多个radio切换) (See RadioGroup)
- [x] Link to Web src (点击link跳转网页) (See Link)
- [ ] Text UnderLine (通过文字下划线实现文字下划线, 划除, 上划线效果)
- [ ] Powerful Input (更强大的输入框, 支持多种输入模式, 清除, 数据检验等功能)
- [ ] Expandable View (可拖动拉伸)
- [ ] Audio Api
- [ ] Video Api
- [ ] Theme Config (通过toml配置的方式设置主题色等默认样式)
- [ ] Right mouse button event (鼠标右击事件展开组件)
- [ ] CallBack control (回调控制， 例如当开发者调用hover激活组件hover时，也可以调用active_hover方法控制是否需要激活回调)

## after v0.5.0

- [ ] Component Abstract
- [ ] Virtual Component

## Macro

带来统一的构建组件的体验，更少的重复代码

- [x] widget_area!
- [x] set_text_and_visible_fn!
- [x] widget_origin_fn!
- [x] ref_event_option!
- [x] ref_event_bool!
- [x] set_event!
- [x] set_event_bool!
- [x] event_option!
- [x] event_bool!
- [x] animatie_fn!
- [x] active_event!
- [x] ref_area!
- [x] ref_redraw!
- [x] ref_render!
- [x] ref_redraw_mut!
- [x] default_hit_finger_down!
- [x] default_hit_finger_up!
- [x] default_hit_hover_in!
- [x] default_hit_hover_out!
- [x] default_handle_animation!

## Example Project
- [x] AWS Personal Cloud Drive GUI (亚马逊个人云盘GUI界面, 仅实现上传，查看，验证功能)
- [ ] GenUI Builtin Component Lib (GenUI内置组件库学习页面)
- [ ] Fake Phone (仿手机系统)

## Plan

- [x] 09-28 AWS Personal Cloud Drive GUI
- [ ] 10-24 finish v0.1.0
- [ ] 11-18 GenUI Framework 集成v0.1.0
- [ ] 11-31 finish v0.2.0