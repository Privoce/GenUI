# GEN Components (v0.1.0)

- version: `v0.1.0`
- update date: 2024-09-26
- author: [Will-YiFei Sheng](syf20020816@outlook.com)
- makepad_widget version: `v0.6.0`
- makepad branch: `rik`

## Basic Builtin Component

- [x] Font 🆗
- [x] Label 🆗
- [x] Link 🆗
- [x] Button 🆗
- [x] Card 🆗
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

## GEN Components (Super, v0.2.0 ~ v0.5.0)

- [ ] Metric (用于数据统计)
- [ ] Route (用于路由跳转) 🔼
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

## Experimental function (v0.1.0 ~ v0.5.0)

- [x] Radio Switch Container (多个radio切换) (See RadioGroup)
- [x] Link to Web src (点击link跳转网页) (See Link)
- [ ] Text UnderLine (通过文字下划线实现文字下划线, 划除, 上划线效果)
- [ ] Powerful Input (更强大的输入框, 支持多种输入模式, 清除, 数据检验等功能)
- [ ] Expandable Card (可拖动拉伸)
- [ ] Audio Api
- [ ] Video Api
- [ ] Theme Config (通过toml配置的方式设置主题色等默认样式)
- [ ] Right mouse button event (鼠标右击事件展开组件)

## Macro

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

## Example Project
- [ ] AWS Personal Cloud Drive GUI (亚马逊个人云盘GUI界面, 仅实现上传，查看，验证功能)
- [ ] GenUI Builtin Component Lib (GenUI内置组件库学习页面)
- [ ] Fake Phone (仿手机系统)

## Plan

- 09-24 finish v0.1.0
- 09-28 AWS Personal Cloud Drive GUI
- 10-18 GenUI Framework 集成
- 10-31 finish v0.2.0
- 11-30 finish v0.3.0