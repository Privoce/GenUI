//! # GenUI Style Enum Manuel
//! - 
use std::default;
const ARK_ENUM_MANUAL: [&str; 30] = [
    "CheckBoxShape",
    "Color",
    "ColoringStrategy",
    "ImageFit",
    "BorderStyle",
    "LineJoinStyle",
    "TouchType",
    "MouseButton",
    "MouseAction",
    "AnimationStatus",
    "Curve",
    "FillMode",
    "PlayMode",
    "KeyType",
    "KeySource",
    "Edge",
    "Week",
    "Direction",
    "BarState",
    "EdgeEffect",
    "Alignment",
    "TransitionType",
    "RelateType",
    "Visibility",
    "LineCapStyle",
    "Axis",
    "HorizontalAlign",
    "FlexAlign",
    "ItemAlign",
    "FlexDirection"
];

pub struct EnumManuel;

impl EnumManuel {
    // /// from arkui enum name to makepad enum name
    // pub fn from_ark_to_makepad(ark_enum: &str) -> &str{
    //     // check is in ark enum manuel
    //     if ARK_ENUM_MANUAL.contains(&ark_enum){
    //         // do match
    //         match ark_enum {
    //             "CheckBoxShape" => "CheckBoxShape",
    //             "Color" => "Color",
    //             "ColoringStrategy" => "ColoringStrategy",
    //             "ImageFit" => "ImageFit",
    //             "BorderStyle" => "BorderStyle",
    //             "LineJoinStyle" => "LineJoinStyle",
    //             "TouchType" => "TouchType",
    //             "MouseButton" => "MouseButton",
    //             "MouseAction" => "MouseAction",
    //             "AnimationStatus" => "AnimationStatus",
    //             "Curve" => "Curve",
    //             "FillMode" => "FillMode",
    //             "PlayMode" => "PlayMode",
    //             "KeyType" => "KeyType",
    //             "KeySource" => "KeySource",
    //             "Edge" => "Edge",
    //             "Week" => "Week",
    //             "Direction" => "Direction",
    //             "BarState" => "BarState",
    //             "EdgeEffect" => "EdgeEffect",
    //             "Alignment" => "Alignment",
    //             "TransitionType" => "TransitionType",
    //             "RelateType" => "RelateType",
    //             "Visibility" => "Visibility",
    //             "LineCapStyle" => "LineCapStyle",
    //             "Axis" => "Axis",
    //             "HorizontalAlign" => "HorizontalAlign",
    //             "FlexAlign" => "FlexAlign",
    //             "ItemAlign" => "ItemAlign",
    //             "FlexDirection" => "FlexDirection",
    //             _ => "Unknown"
    //         }
    //     }
    // }
}


pub enum BoxShape {
   /// Circle Box Shape: border_radius: 0.5   Circle,
   /// Round Box Shape: border_radius: 0.25   /// if width is equal to height, it is a circle
   Round,
   /// Normal Box Shape: Square
   
   Normal
}

pub enum Color {
    /// #fff
    White,
   /// #000
    Black,
   /// #00f
    Blue,
    /// #a52a2a
    Brown,
    /// #808080
    Gray,
   /// #008000
    Green,
   /// #ffa500
    Grey,
   /// #ff00ff
    Orange,
    /// #ffc0cb
    Pink,
    /// #ff0000
    Red,
   /// #ffff00
    Yellow,
    /// #0000ff
    Transparent
}

// pub enum ColoringStrategy {
//     
//////////      
//     
////////////      
//     INVERT = 'invert',
//     
//////////      
//     
////////////      
//     AVERAGE = 'average',
//     
//////////      
//     
////////////      
//     PRIMARY = 'primary'
// }

pub enum ImageFit {
    /// 被替换的内容将被缩放，以在填充元素的内容框时保持其宽高比。整个对象在填充盒子的同时保留其长宽比
    /// 
    /// the image is scaled to maintain its aspect ratio while fitting within the element's content box
    /// 
    /// Makepad ImageFit::Smallest
    Contain,
    /// 被替换的内容在保持其宽高比的同时填充元素的整个内容框。如果对象的宽高比与内容框不相匹配，该对象将被剪裁以适应内容框
    /// 
    /// The replaced content fills the entire content box of the element while maintaining its aspect ratio. 
    /// If the aspect ratio of an object does not match the content box, the object will be cropped to fit the content box
    /// 
    /// Makepad ImageFit::Biggest
    Cover,
    /// 被替换的内容正好填充元素的内容框。整个对象将完全填充此框。如果对象的宽高比与内容框不相匹配，那么该对象将被拉伸以适应内容框
    /// 
    /// the image is stretched to fill the entire content box of the element. The object will completely fill this box.
    /// 
    /// Makepad ImageFit::Stretch
    Fill,
    /// 内容的尺寸与 normal 或 contain 中的一个相同，取决于它们两个之间谁得到的对象尺寸会更小一些。
    /// 
    /// the imahe size is the same as normal or contain, depending on which one gets a smaller object size
    /// 
    /// Makepad ImageFit::Smallest
    ScaleDown,
    /// 保持原有尺寸,这是默认的选项
    /// 
    /// the image is displayed at its original size, this is the default value
 
    Normal
}

pub enum BorderStyle {
    /// 点线
    Dotted,
    /// 虚线
    Dashed,
    /// 实线

    Solid
}

// pub enum LineJoinStyle {
    
//     Miter,
   
//     Round,
    
//     Bevel
// }

pub enum TouchType {
   
    Down,
   
    Up,
   
    Move,
    
    Cancel
}

pub enum MouseButton {
   
    Left,
    
    Right,
    
    Middle,
   
    Back,
    
    Forward,
    
    None
}

pub enum MouseAction {
    
    Press,
    
    Release,
   
    Move,
    
    Hover
}

pub enum AnimationStatus {
    
    Initial,
    
    Running,
    
    Paused,
    
    Stopped
}

pub enum Curve {
    
    Linear,
    
    Ease,
    
    EaseIn,
   
    EaseOut,
   
    EaseInOut,
   
    FastOutSlowIn,
    
    LinearOutSlowIn,
   
    FastOutLinearIn,
    
    ExtremeDeceleration,
   
    Sharp,
    
    Rhythm,
    
    Smooth,
    
    Friction
}

pub enum FillMode {
   
    None,
    
    Forwards,
    
    Backwards,
    
    Both
}

pub enum PlayMode {
   
    Normal,
    
    Reverse,
   
    Alternate,
   
    AlternateReverse
}

pub enum KeyType {
    
    Down,
   
    Up
}

pub enum KeySource {
    
    Unknown,

    Keyboard
}

pub enum Edge {
   
    Top,

    Center,

    Bottom,

    Baseline,
   
    Start,
    
    Middle,
    
    End
}

pub enum Week {
  
    Mon,
  
    Tue,
   
    Wed,

    Thur,
   
    Fri,
   
    Sat,
   
    Sun
}

pub enum Direction {                    
    Ltr,                    
    Rtl,                    
    Auto
}    
pub enum BarState {                    
    Off,                    
    Auto,                    
    On
}    
pub enum EdgeEffect {                    
    Spring,                    
    Fade,                    
    None
}    
pub enum Alignment {                    
    TopStart,                    
    Top,                    
    TopEnd,                    
    Start,                    
    Center,                    
    End,                    
    BottomStart,                    
    Bottom,                    
    BottomEnd
}    
pub enum TransitionType {                    
    All,                    
    Insert,                    
    Delete
}   
pub enum RelateType {               
    FILL,               
    FIT
}    
pub enum Visibility {                    
    Visible,                    
    Hidden,                    
    None
}    
pub enum LineCapStyle {                    
    Butt,                    
    Round,                    
    Square
}    
pub enum Axis {                    
    Vertical,                    
    Horizontal
}    
pub enum HorizontalAlign {                    
    Start,                    
    Center,                    
    End
}    
pub enum FlexAlign {                    
    Start,                    
    Center,                    
    End,                    
    SpaceBetween,                    
    SpaceAround,                    
    SpaceEvenly
}    
pub enum ItemAlign {                    
    Auto,                    
    Start,                    
    Center,                    
    End,                    
    Baseline,                    
    Stretch
}    
pub enum FlexDirection {                    
    Row,                    
    Column,                    
    RowReverse,                    
    ColumnReverse
} 
pub enum PixelRoundCalcPolicy {     
    NO_FORCE_ROUND = 0,     
    FORCE_CEIL = 1,     
    FORCE_FLOOR = 2
}    
pub enum FlexWrap {                    
    NoWrap,                    
    Wrap,                    
    WrapReverse
}    
pub enum VerticalAlign {                    
    Top,                    
    Center,                    
    Bottom
}    
pub enum ImageRepeat {                    
    NoRepeat,                    
    X,                    
    Y,                    
    XY
}    
pub enum ImageSize {                    
    Auto,                    
    Cover,                    
    Contain,     
    FILL = 3
}    
pub enum GradientDirection {                    
    Left,                    
    Top,                    
    Right,                    
    Bottom,                    
    LeftTop,                    
    LeftBottom,                    
    RightTop,                    
    RightBottom,                    
    None
}   
pub enum SharedTransitionEffectType {               
    Static,               
    Exchange
}    
pub enum FontStyle {                    
    Normal,                    
    Italic
}    
pub enum FontWeight {                    
    Lighter,                    
    Normal,                    
    Regular,                    
    Medium,                    
    Bold,                    
    Bolder
}    
pub enum TextAlign {                    
    Center,                    
    Start,                    
    End,          
    JUSTIFY
}    
pub enum TextOverflow {                    
    None,                    
    Clip,                    
    Ellipsis,          
    MARQUEE
}    
pub enum TextDecorationType {                    
    None,                    
    Underline,                    
    Overline,                    
    LineThrough
}    
pub enum TextCase {                    
    Normal,                    
    LowerCase,                    
    UpperCase
}  
pub enum TextHeightAdaptivePolicy {          
    MAX_LINES_FIRST,          
    MIN_FONT_SIZE_FIRST,          
    LAYOUT_CONSTRAINT_FIRST
}   
pub enum ResponseType {               
    RightClick,               
    LongPress
}   
pub enum HoverEffect {               
    Auto,               
    Scale,               
    Highlight,               
    None
}   
pub enum Placement {               
    Left,               
    Right,               
    Top,               
    Bottom,               
    TopLeft,               
    TopRight,               
    BottomLeft,               
    BottomRight,               
    LeftTop,               
    LeftBottom,               
    RightTop,               
    RightBottom
}  
pub enum ArrowPointPosition {          
    START,          
    CENTER,          
    END
}   
pub enum CopyOptions {               
    None = 0,               
    InApp = 1,               
    LocalDevice = 2,     
    CROSS_DEVICE = 3
}   
pub enum HitTestMode {               
    Default,               
    Block,               
    Transparent,               
    None
}   
pub enum TitleHeight {               
    MainOnly,               
    MainWithSub
}  
pub enum ModifierKey {          
    CTRL,          
    SHIFT,          
    ALT
}  
pub enum FunctionKey {          
    ESC,          
    F1,          
    F2,          
    F3,          
    F4,          
    F5,          
    F6,          
    F7,          
    F8,          
    F9,          
    F10,          
    F11,          
    F12,     
    TAB,     
    DPAD_UP,     
    DPAD_DOWN,     
    DPAD_LEFT,     
    DPAD_RIGHT
}  
pub enum ImageSpanAlignment {          
    BASELINE,          
    BOTTOM,          
    CENTER,          
    TOP
}  
pub enum ObscuredReasons {          
    PLACEHOLDER = 0
}  
pub enum TextContentStyle {          
    DEFAULT,          
    INLINE
}  
pub enum ClickEffectLevel {          
    LIGHT,          
    MIDDLE,          
    HEAVY
}  
pub enum XComponentType {          
    SURFACE,          
    COMPONENT,          
    TEXTURE,     
    NODE
}  
pub enum NestedScrollMode {          
    SELF_ONLY,          
    SELF_FIRST,          
    PARENT_FIRST,          
    PARALLEL
} 
pub enum ScrollSource {     
    DRAG = 0,     
    FLING,     
    EDGE_EFFECT,     
    OTHER_USER_INPUT,     
    SCROLL_BAR,     
    SCROLL_BAR_FLING,     
    SCROLLER,     
    SCROLLER_ANIMATION
}

pub enum RenderFit {
    
    CENTER = 0,
   
    TOP = 1,

    BOTTOM = 2,

    LEFT = 3,

    RIGHT = 4,

    TOP_LEFT = 5,

    TOP_RIGHT = 6,

    BOTTOM_LEFT = 7,

    BOTTOM_RIGHT = 8,

    RESIZE_FILL = 9,

    RESIZE_CONTAIN = 10,

    RESIZE_CONTAIN_TOP_LEFT = 11,

    RESIZE_CONTAIN_BOTTOM_RIGHT = 12,

    RESIZE_COVER = 13,

    RESIZE_COVER_TOP_LEFT = 14,

    RESIZE_COVER_BOTTOM_RIGHT = 15
}

pub enum DialogButtonStyle {

    DEFAULT = 0,

    HIGHLIGHT = 1
}

pub enum WordBreak {

    NORMAL = 0,

    BREAK_ALL = 1,

    BREAK_WORD = 2
}

pub enum LineBreakStrategy {

    GREEDY = 0,

    HIGH_QUALITY = 1,

    BALANCED = 2
}

pub enum EllipsisMode {

    START = 0,

    CENTER = 1,

    END = 2
}


pub enum OptionWidthMode {

    FIT_CONTENT,

    FIT_TRIGGER
}

pub enum FoldStatus {

    FOLD_STATUS_UNKNOWN = 0,

    FOLD_STATUS_EXPANDED = 1,

    FOLD_STATUS_FOLDED = 2,

    FOLD_STATUS_HALF_FOLDED = 3
}

pub enum AppRotation {

    ROTATION_0 = 0,

    ROTATION_90 = 1,

    ROTATION_180 = 2,

    ROTATION_270 = 3
}

pub enum EmbeddedType {

    EMBEDDED_UI_EXTENSION = 0
}

pub enum MarqueeUpdateStrategy {

    DEFAULT = 0,

    PRESERVE_POSITION = 1
}

pub enum TextDecorationStyle {

    SOLID = 0,
   
    DOUBLE = 1,

    DOTTED = 2,

    DASHED = 3,

    WAVY = 4
}
