// TODO!(移除)
// use syn::{ItemEnum, ItemStruct, ItemUse, Stmt, StmtMacro};

// use super::lifetime::LifeTime;

// /// # GenUI 脚本模型
// /// 具体处理见rssyin以及generator层
// #[derive(Debug, Clone, Default)]
// pub struct GenScriptModel {
//     /// 使用的包,依赖
//     pub uses: Option<ItemUse>,
//     /// 组件的导入
//     /// 例如：
//     /// ```rust
//     /// // 表示导入my_button mod下所有的组件
//     /// import!{
//     ///     crate::views::my_button::*;
//     /// }
//     /// ```
//     pub imports: Option<StmtMacro>,
//     /// 组件的属性
//     /// 这表示组件允许外部传入给内部的属性，需要使用GenUI的prop属性宏进行标注
//     /// 例如：
//     /// ```rust
//     /// #[prop]
//     /// pub struct Props{
//     ///     text: String,
//     ///     height: f64,
//     /// }
//     /// ```
//     // prop_ptr: Box<dyn Prop>,
//     pub prop_ptr: Option<ItemStruct>,
//     /// 组件事件
//     /// 事件也可以被认为是组件状态
//     /// 由编写者决定，所以并不一定存在，但若存在则必须要使用GenUI的event属性宏进行标注
//     /// 例如，一个按钮组件，它有一个点击事件，那么这个按钮被点击时，这个事件就会被触发，也就是这个按钮进入了点击状态
//     /// GenuI中事件实际上是由外部影响的
//     /// 例如，在组件中有一个按钮，当这个按钮被点击时，这个按钮会激发组件的点击事件并把这个事件传递给外部（连带参数）
//     /// 外部可以根据这个事件来做一些事情
//     /// 对于定义组件时就需要相应的使用Rust编写
//     /// ```rust
//     /// #[event]
//     /// pub enum Events{
//     ///     #[name("click")]
//     ///     Clicked(//内部给到外部的参数),
//     /// }
//     /// ```
//     // event_ptr: Box<dyn Event>,
//     pub event_ptr: Option<ItemEnum>,
//     /// 组件的生命周期
//     /// 在GenUI中声明周期使用属性宏来进行标注
//     /// 例如: `#[on_startup]` `#[on_shutdown]`等
//     pub lifetimes: Option<LifeTime>,
//     // /// 表示当前组件的内部子组件的属性绑定
//     // pub sub_prop_binds: Option<Vec<PropFn>>,
//     // /// 表示当前组件的内部子组件的事件绑定
//     // pub sub_event_binds: Option<Vec<PropFn>>,
//     // /// 当前组件的实例
//     // /// 该实例在组件的draw_walk中被构建, 表示当前实例调用了default()方法来构建
//     // /// 例如：`let instance = MyButton::default();`
//     // /// 常见声明如下:
//     // /// ```rust
//     // /// <script>
//     // /// use makepad_widget::*;
//     // ///
//     // /// #[prop]
//     // /// pub struct MyButtonWidget{
//     // ///     text: String,
//     // ///     walk: Walk, //(你不该设置这个属性⛔, 该属性是自动添加的)
//     // /// }
//     // ///
//     // /// let mut current_instance = MyButtonWidget::default(); // 这里的
//     // /// current_instance.text = "Hello".to_string(); // 这不是
//     // /// </script>
//     // /// ```
//     // pub current_instance: Option<CurrentInstance>,
//     // /// 对当前组件实例处理的代码
//     // /// 例如上面的代码中的`current_instance.text = "Hello".to_string();`
//     // /// 这里应该都是Expr，但使用Stmt，因为Stmt能表示完整语句
//     // pub instance_opt: Option<Vec<Stmt>>,
//     // /// 实例的初始化代码
//     // /// ```
//     // /// impl Default for MyButtonWidget{
//     // ///    fn default() -> Self{
//     // ///       MyButtonWidget{
//     // ///          text: "".to_string(),
//     // ///       }
//     // ///    }
//     // /// }
//     // /// ```
//     // pub instance_default_impl: Option<(Vec<PropFnOnly>, ItemImpl)>,
//     /// 其他的代码，例如一些过程代码
//     /// 但是这些过程代码也需要进行处理，因为这些代码中可能会使用到当前组件的实例的属性
//     pub other: Option<Vec<Stmt>>,
// }
