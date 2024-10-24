use makepad_widgets::*;

pub mod breadcrumb;
pub mod button;
pub mod checkbox;
pub mod collapse;
pub mod divider;
pub mod drop_down;
pub mod file_upload;
pub mod icon;
pub mod image;
pub mod input;
pub mod label;
pub mod link;
pub mod loading;
pub mod menu;
pub mod notification;
pub mod popup;
pub mod progress;
pub mod radio;
pub mod router;
pub mod select;
pub mod shader;
pub mod svg;
pub mod tab;
pub mod tabbar;
pub mod table;
pub mod tag;
pub mod toggle;
pub mod tool_btn;
pub mod view;
pub mod window;
pub mod colors;

live_design! {
    // imports -----------------------------------------------------
    import crate::components::label::GLabelBase;
    import crate::components::button::GButtonBase;
    import crate::components::colors::GColorBase;
    import crate::components::view::GViewBase;
    import crate::components::link::GLinkBase;
    import crate::components::icon::GIconBase;
    import crate::components::radio::GRadioBase;
    import crate::components::radio::group::GRadioGroupBase;
    import crate::components::checkbox::GCheckBoxBase;
    import crate::components::checkbox::group::GCheckBoxGroupBase;
    import crate::components::image::GImageBase;
    import crate::components::input::GInputBase;
    import crate::components::svg::GSvgBase;
    import crate::components::divider::GDividerBase;
    import crate::components::shader::GShaderBase;
    import crate::components::popup::GPopupBase;
    import crate::components::popup::GPopupContainerBase;
    import crate::components::drop_down::GDropDownBase;
    import crate::components::toggle::GToggleBase;
    import crate::components::progress::GProgressBase;
    import crate::components::loading::GLoadingBase;
    import crate::components::tag::GTagBase;
    import crate::components::breadcrumb::GBreadCrumbBase;
    import crate::components::breadcrumb::item::GBreadCrumbItemBase;
    import crate::components::tab::header::GTabHeaderBase;
    import crate::components::tab::button::GTabButtonBase;
    import crate::components::tab::body::GTabBodyBase;
    import crate::components::tab::pane::GTabPaneBase;
    import crate::components::tab::GTabBase;
    import crate::components::file_upload::GUploadBase;
    import crate::components::collapse::GCollapseBase;
    import crate::components::table::cell::GTableCellBase;
    import crate::components::table::row::GTableRowBase;
    import crate::components::table::body::GTableBodyBase;
    import crate::components::table::header::GTableHeaderBase;
    import crate::components::table::GTableBase;
    import crate::components::table::virt::GVTableBodyBase;
    import crate::components::tool_btn::GToolButtonBase;
    import crate::components::window::GWindowBase;
    import crate::components::select::GSelectBase;
    import crate::components::select::item::GSelectItemBase;
    import crate::components::select::options::GSelectOptionsBase;
    import crate::components::tabbar::GTabbarBase;
    import crate::components::tabbar::item::GTabbarItemBase;
    import crate::components::router::GRouterBase;
    import crate::components::router::page::GPageBase;
    import crate::components::menu::menu_item::GMenuItemBase;
    import crate::components::menu::sub_menu::GSubMenuBase;
    import crate::components::menu::GMenuBase;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    // globals -----------------------------------------------------
    // -------- colors ---------------------------------------------
    // each theme color has [25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
    // the default color is 500
    COLOR_WHITE = #FFFFFF;
    COLOR_BLACK = #000000;
    // -------- dark-opacity ---------------------------------------
    DARK_OPACITY_25 = #66666640;
    DARK_OPACITY_50 = #66666680;
    DARK_OPACITY_75 = #666666BF;
    // -------- color-info -----------------------------------------
    COLOR_INFO_25 = #FCFCFD;
    COLOR_INFO_50 = #F9FAFB;
    COLOR_INFO_100 = #F2F4F7;
    COLOR_INFO_200 = #EAECF0;
    COLOR_INFO_300 = #D0D5DD;
    COLOR_INFO_400 = #95A2D3;
    COLOR_INFO_500 = #667085;
    COLOR_INFO_600 = #475467;
    COLOR_INFO_700 = #344054;
    COLOR_INFO_800 = #1D2939;
    COLOR_INFO_900 = #101828;
    // -------- color-dark -----------------------------------------
    COLOR_DARK_25 = #6e7176;
    COLOR_DARK_50 = #5b5f64;
    COLOR_DARK_100 = #42464d;
    COLOR_DARK_200 = #3b4047;
    COLOR_DARK_300 = #2f333b;
    COLOR_DARK_400 = #282d35;
    COLOR_DARK_500 = #22272F;
    COLOR_DARK_600 = #1f242b;
    COLOR_DARK_700 = #1d2127;
    COLOR_DARK_800 = #1a1e24;
    COLOR_DARK_900 = #0f1115;
    // -------- color-primary --------------------------------------
    COLOR_PRIMARY_25 = #F5FEFF;
    COLOR_PRIMARY_50 = #ECFDFF;
    COLOR_PRIMARY_100 = #CFF9FE;
    COLOR_PRIMARY_200 = #A5F0FC;
    COLOR_PRIMARY_300 = #67E3F9;
    COLOR_PRIMARY_400 = #22CCEE;
    COLOR_PRIMARY_500 = #06AED4;
    COLOR_PRIMARY_600 = #088AB2;
    COLOR_PRIMARY_700 = #0E6F90;
    COLOR_PRIMARY_800 = #155B75;
    COLOR_PRIMARY_900 = #164C63;
    // -------- color-error ------------------------------------
    COLOR_ERROR_25 = #FFFBFA;
    COLOR_ERROR_50 = #FEF3F2;
    COLOR_ERROR_100 = #FEE4E2;
    COLOR_ERROR_200 = #FECDCA;
    COLOR_ERROR_300 = #FDA29B;
    COLOR_ERROR_400 = #F97066;
    COLOR_ERROR_500 = #F04438;
    COLOR_ERROR_600 = #D92D2D;
    COLOR_ERROR_700 = #B42318;
    COLOR_ERROR_800 = #912018;
    COLOR_ERROR_900 = #7A271A;
    // -------- color-warning ------------------------------------
    COLOR_WARNING_25 = #FFFCF5;
    COLOR_WARNING_50 = #FFFAEB;
    COLOR_WARNING_100 = #FEF0C7;
    COLOR_WARNING_200 = #FEDF89;
    COLOR_WARNING_300 = #FEC84B;
    COLOR_WARNING_400 = #FDB022;
    COLOR_WARNING_500 = #F79009;
    COLOR_WARNING_600 = #DC6803;
    COLOR_WARNING_700 = #B54708;
    COLOR_WARNING_800 = #93370D;
    COLOR_WARNING_900 = #7A2E0E;
    // -------- color-success ------------------------------------
    COLOR_SUCCESS_25 = #F6FEF9;
    COLOR_SUCCESS_50 = #ECFDF3;
    COLOR_SUCCESS_100 = #D1FADF;
    COLOR_SUCCESS_200 = #A6F4C5;
    COLOR_SUCCESS_300 = #6CE9A6;
    COLOR_SUCCESS_400 = #32D583;
    COLOR_SUCCESS_500 = #12B76A;
    COLOR_SUCCESS_600 = #039855;
    COLOR_SUCCESS_700 = #027A48;
    COLOR_SUCCESS_800 = #05603A;
    COLOR_SUCCESS_900 = #054F31;
    // -------- font-family ------------------------------------
    FONT_FAMILY = dep("crate://self/resources/font/OPPOSans-Regular.ttf");
    FONT_SIZE = 10.0;
    FONT_SIZE_SMALL = 9.0;
    // padding -----------------------------------------------------

    GLOBAL_PADDING = {top: 10.0, left: 16.0, bottom: 10.0, right: 16.0};
    GLOBAL_PADDING_SMALL = {top: 4.6, left: 9.0, bottom: 4.6, right: 9.0};
    // align -------------------------------------------------------
    ALIGN_CENTER_WALK = {x: 0.5, y: 0.5};
    ALIGN_LEFT_WALK = {x: 0.0, y: 0.5};
    GLOBAL_DURATION = 0.25;
    // components --------------------------------------------------

    // ## GLabel
    // A label component use white color
    GLabel = <GLabelBase>{
        width: Fit,
        height: Fit,
        color: (COLOR_WHITE),
        font_family: (FONT_FAMILY),
        // top_drop: 1.0,
        line_spacing: 1.5,
        font_size: (FONT_SIZE),
    }
    GLink = <GLinkBase>{
        height: Fit,
        width: Fit,
        padding: 0,
        font_size: (FONT_SIZE),
        font_family: (FONT_FAMILY),
        align: <ALIGN_CENTER_WALK>{},
    }
    // ## GButton
    // A button component which only has a text
    // if you wanna add some other components like icon, you can create a new component use ViewBase
    // ViewBase can help you create a wonderful button quickly and easily
    GButton = <GButtonBase>{
        height: Fit,
        width: Fit,
        theme: Primary,
        padding: <GLOBAL_PADDING>{}
        align: <ALIGN_CENTER_WALK>{},
        slot: <GLabel>{
            text: "GButton"
        }
    }
    // ## GView
    // A view component that you can use to wrap other components
    // view has default styles for border, background color, ...
    GView = <GViewBase>{
        width: 300.0,
        height: 200.0,
        clip_x: true,
        clip_y: true,
    }
    // ## GHLayout
    // A horizontal layout component use ViewBase
    // layout don't have border, background color, border-radius, ... (but you can add if you want)
    GHLayout = <GViewBase>{
        height: Fill,
        width: Fill,
        flow: Right,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        background_visible: false,
        spacing: 0,
        margin: 0,
    }
    // ## GVLayout
    // A vertical layout component use ViewBase
    GVLayout = <GViewBase>{
        height: Fill,
        width: Fill,
        flow: Down,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        background_visible: false,
        spacing: 0,
        margin: 0,
    }
    GColor = <GColorBase>{
        height: Fit,
        width: 440.0,
        flow: Down,
        item: <GView>{
            height: 40.0,
            width: 40.0,
            border_radius: 0.0,
        },
        header: <GVLayout>{
            height: 72.0,
            spacing: 8.0,
            theme_name = <GLabel>{
                font_size: 12.0,
            }
            theme_main = <GLabel>{
                font_size: 10.0,
            }
            padding: 16.0,
        }
        colors: <GHLayout>{
            height: 40.0,
            border_radius: 0.0,
        }
    }
    // ## GScrollBar
    // A scroll bar component use ScrollBarBase, it is a single scroll bar
    GScrollBar = <ScrollBarBase> {
        bar_size: 10.0,
        bar_side_margin: 3.0
        min_handle_size: 20.0
        draw_bar: {
            instance pressed: 0.0
            instance hover: 0.0

            instance color: (DARK_OPACITY_50)
            instance color_hover: (DARK_OPACITY_25)
            instance color_pressed: (DARK_OPACITY_75)

            uniform bar_width: 6.0
            uniform border_radius: 1.5

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                if self.is_vertical > 0.5 {
                    sdf.box(
                        1.,
                        self.rect_size.y * self.norm_scroll,
                        self.bar_width,
                        self.rect_size.y * self.norm_handle,
                        self.border_radius
                    );
                }
                else {
                    sdf.box(
                        self.rect_size.x * self.norm_scroll,
                        1.,
                        self.rect_size.x * self.norm_handle,
                        self.bar_width,
                        self.border_radius
                    );
                }
                return sdf.fill(mix(
                    self.color,
                    mix(
                        self.color_hover,
                        self.color_pressed,
                        self.pressed
                    ),
                    self.hover
                ));
            }
        }
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bar: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    cursor: Default,
                    from: {
                        all: Forward {duration: 0.1}
                        pressed: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bar: {
                            pressed: 0.0,
                            hover: [{time: 0.0, value: 1.0}],
                        }
                    }
                }

                pressed = {
                    cursor: Default,
                    from: {all: Snap}
                    apply: {
                        draw_bar: {
                            pressed: 1.0,
                            hover: 1.0,
                        }
                    }
                }
            }
        }
    }
    // ## GScrollBars
    // A scroll bars component use ScrollBarsBase, it has two scroll bars (x, y)
    // It often use in a View
    GScrollBars = <ScrollBarsBase> {
        show_scroll_x: true,
        show_scroll_y: true,
        scroll_bar_x: <GScrollBar> {}
        scroll_bar_y: <GScrollBar> {}
    }
    GRadio = <GRadioBase>{}
    GRadioGroup = <GRadioGroupBase>{}
    GCheckBox = <GCheckBoxBase>{}
    GCheckBoxGroup = <GCheckBoxGroupBase>{}
    GToggle = <GToggleBase>{}
    GSvg = <GSvgBase>{
        width: 24.0,
        height: 24.0,
    }
    GIcon = <GIconBase>{
        width: 24.0,
        height: 24.0,
    }
    GToolButton = <GToolButtonBase>{
        align: <ALIGN_CENTER_WALK>{},
    }
    GImage = <GImageBase>{
        width: 32.0,
        height: 32.0,
    }
    GInput = <GInputBase>{
        font_family: (FONT_FAMILY),
        font_size: (FONT_SIZE),
    }
    GShader = <GShaderBase>{}
    GDivider = <GDividerBase>{}
    GPopupContainer = <GPopupContainerBase>{
        height: Fill,
        width: Fill,
    }
    GPopup = <GPopupBase>{
        mode: Popup,
        height: Fill,
        width: Fill,
        container: <GPopupContainer>{}
    }
    GToolTip = <GPopup>{
        mode: ToolTip,
        draw_popup: {
            instance inset: vec4(0.0, 0.0, 0.0, 0.0);

            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let spacing = 6.0;
                let w = self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0);
                let h = self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0);
                let center = vec2((self.pos.x + w) * 0.5, (self.pos.y + h)  * 0.5);
                let start_p = vec2(self.inset.x + self.border_width, self.inset.y + self.border_width);
                // let quarter_w = w * 0.25;
                let quarter_w = self.angle_offset;
                // let quarter_h = h * 0.25;
                let quarter_h = self.angle_offset;
                match self.position{
                    Position::Left => {
                        let end_w = self.inset.x + self.border_width + w - spacing;
                        sdf.box(
                            start_p.x,
                            start_p.y,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );

                        sdf.move_to(end_w - 0.4, center.y - spacing * 0.7);
                        sdf.line_to(end_w + spacing - 0.4, center.y);
                        sdf.line_to(end_w - 0.4, center.y + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::LeftTop => {
                        let end_w = self.inset.x + self.border_width + w - spacing;
                        sdf.box(
                            start_p.x,
                            start_p.y,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(end_w - 0.4, start_p.y + quarter_h - spacing * 0.7);
                        sdf.line_to(end_w + spacing - 0.4, start_p.y + quarter_h);
                        sdf.line_to(end_w - 0.4, start_p.y + quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::LeftBottom => {
                        let end_w = self.inset.x + self.border_width + w - spacing;
                        sdf.box(
                            start_p.x,
                            start_p.y,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(end_w - 0.4, start_p.y + h - quarter_h - spacing * 0.7);
                        sdf.line_to(end_w + spacing - 0.4, start_p.y + h - quarter_h);
                        sdf.line_to(end_w - 0.4, start_p.y + h - quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::Right => {
                        let start_w = self.inset.x + self.border_width + spacing;
                        sdf.box(
                            self.inset.x + self.border_width + spacing,
                            self.inset.y + self.border_width,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );

                        sdf.move_to(start_w - spacing, center.y);
                        sdf.line_to(start_w + 0.4, center.y - spacing * 0.7);
                        sdf.line_to(start_w + 0.4, center.y + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::RightTop => {
                        let start_w = self.inset.x + self.border_width + spacing;
                        sdf.box(
                            self.inset.x + self.border_width + spacing,
                            self.inset.y + self.border_width,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_w - spacing, start_p.y + quarter_h);
                        sdf.line_to(start_w + 0.4, start_p.y + quarter_h - spacing * 0.7);
                        sdf.line_to(start_w + 0.4, start_p.y + quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::RightBottom => {
                        let start_w = self.inset.x + self.border_width + spacing;
                        sdf.box(
                            self.inset.x + self.border_width + spacing,
                            self.inset.y + self.border_width,
                            w - spacing,
                            h,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_w - spacing, start_p.y + h - quarter_h);
                        sdf.line_to(start_w + 0.4, start_p.y + h - quarter_h - spacing * 0.7);
                        sdf.line_to(start_w + 0.4, start_p.y + h - quarter_h + spacing * 0.7);
                        sdf.close_path();
                    }
                    Position::Top => {
                        let end_h =  self.inset.y + self.border_width + h - spacing;
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(center.x, end_h + spacing - 0.4);
                        sdf.line_to(center.x - spacing * 0.7, end_h - 0.4);
                        sdf.line_to(center.x + spacing * 0.7, end_h - 0.4);
                        sdf.close_path();

                    }
                    Position::TopLeft => {
                        let end_h =  self.inset.y + self.border_width + h - spacing;
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + quarter_w, end_h + spacing - 0.4);
                        sdf.line_to(start_p.x + quarter_w - spacing * 0.7, end_h - 0.4);
                        sdf.line_to(start_p.x + quarter_w + spacing * 0.7, end_h - 0.4);
                        sdf.close_path();
                    }
                    Position::TopRight => {
                        let end_h =  self.inset.y + self.border_width + h - spacing;
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + w - quarter_w, end_h + spacing - 0.4);
                        sdf.line_to(start_p.x + w - quarter_w - spacing * 0.7, end_h - 0.4);
                        sdf.line_to(start_p.x + w - quarter_w + spacing * 0.7, end_h - 0.4);
                        sdf.close_path();
                    }
                    Position::Bottom => {
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width + spacing,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );

                        sdf.move_to(center.x - spacing * 0.7, spacing + 0.4);
                        sdf.line_to(center.x, self.pos.y);
                        sdf.line_to(center.x + spacing * 0.7, spacing + 0.4);
                        sdf.line_to(center.x - spacing * 0.7, spacing + 0.4);
                    }
                    Position::BottomLeft => {
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width + spacing,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + quarter_w - spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + quarter_w, self.pos.y);
                        sdf.line_to(start_p.x + quarter_w + spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + quarter_w - spacing * 0.7, spacing + 0.4);
                    }
                    Position::BottomRight => {
                        sdf.box(
                            self.inset.x + self.border_width,
                            self.inset.y + self.border_width + spacing,
                            w,
                            h - spacing,
                            max(1.0, self.border_radius)
                        );
                        sdf.move_to(start_p.x + w - quarter_w - spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + w - quarter_w, self.pos.y);
                        sdf.line_to(start_p.x + w - quarter_w + spacing * 0.7, spacing + 0.4);
                        sdf.line_to(start_p.x + w - quarter_w - spacing * 0.7, spacing + 0.4);
                    }
                }
                if self.background_visible == 1.0 {
                    sdf.fill(self.get_background_color());
                }
                sdf.stroke(self.get_border_color(), self.border_width);
                return sdf.result;
            }
        }
        container: <GPopupContainer>{
            background_visible: false,
        }
    }
    GDialog = <GPopup>{
        align: <ALIGN_CENTER_WALK>{}
        mode: Dialog,
        height: All,
        width: All,
        theme: Dark,
        draw_popup: {
            // this is a mask
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(self.pos.x, self.pos.y, self.rect_size.x, self.rect_size.y);
                let color = self.get_background_color();
                sdf.fill(vec4(color.r, color.g, color.b, self.opacity));
                return sdf.result;
            }
        }
        container: <GPopupContainer>{
            clip_x: false,
            clip_y: false,
            background_visible: false,
        }
    }
    GDrawer = <GPopup>{
        // align: <ALIGN_CENTER_WALK>{},
        mode: Drawer,
        height: All,
        width: All,
        theme: Dark,
        draw_popup: {
            // this is a mask
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(self.pos.x, self.pos.y, self.rect_size.x, self.rect_size.y);
                let color = self.get_background_color();
                sdf.fill(vec4(color.r, color.g, color.b, self.opacity));

                return sdf.result;
            }
        }
        container: <GPopupContainer>{
            theme: Dark,
            clip_x: false,
            clip_y: false,
            background_visible: true,
            border_radius: 0.0
        }
    }
    GDropDown = <GDropDownBase>{
        height: Fit,
        width: Fit,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        background_visible: false,
        spacing: 0,
        margin: 0,
        popup: <GPopup> {
            height: 160.0,
            width: 200.0,
        },
    }
    GProgress = <GProgressBase>{
        height: 16.0,
        width: 300.0,
        border_radius: 4.0,
    }
    GLoading = <GLoadingBase>{
        height: 48.0,
        width: 48.0,
    }
    GState404 = <GImage>{
        height: Fill,
        src: dep("crate://self/resources/icons/404.png"),
        fit: Vertical
    }
    GState502 = <GImage>{
        height: Fill,
        src: dep("crate://self/resources/icons/502.png"),
        fit: Vertical
    }
    GStateNoData = <GImage>{
        height: Fill,
        src: dep("crate://self/resources/icons/no_data.png"),
        fit: Vertical
    }
    GStateNoMsg = <GImage>{
        height: Fill,
        src: dep("crate://self/resources/icons/no_msg.png"),
        fit: Vertical
    }
    GStateNetWorkErr = <GImage>{
        height: Fill,
        src: dep("crate://self/resources/icons/network_err.png"),
        fit: Vertical
    }
    GStateSearch = <GImage>{
        height: Fill,
        src: dep("crate://self/resources/icons/searching.png"),
        fit: Vertical
    }
    GTag = <GTagBase>{
        spacing: 4.6,
        theme: Primary,
        text: "",
        padding: <GLOBAL_PADDING_SMALL>{},
        font_size: (FONT_SIZE_SMALL),
        align: <ALIGN_CENTER_WALK>{},

    }
    GBreadCrumbItem = <GBreadCrumbItemBase>{
        height: 24.0,
        width: Fit,
        font_size: (FONT_SIZE),
        align: {
            x: 0.0,
            y: 0.5
        },
    }
    GBreadCrumb = <GBreadCrumbBase>{
        height: 24.0,
        width: Fill,
        padding: 6.0,
        spacing: 8.0,
        align: <ALIGN_LEFT_WALK>{},
        icon = <GSvg>{
            src: dep("crate://self/resources/icons/home.svg"),
            cursor: Hand,
            animation_key: true,
            height: 16.0,
            width: 16.0,
            event_key: true,
        },
        item: <GBreadCrumbItem>{
            theme: Dark,
        }
    }
    GTabButton = <GTabButtonBase>{
        height: Fit,
        width: Fit,
        text: " ",
        border_width: 0.0,
        padding: <GLOBAL_PADDING_SMALL>{}
        // font_size: (FONT_SIZE),
        align: <ALIGN_CENTER_WALK>{},
        plain: true,
        closeable: true,
        margin: 0.0,
    }
    GTabHeader = <GTabHeaderBase>{
        height: Fit,
        width: Fill,
        align: <ALIGN_CENTER_WALK>{},
        scroll_bars: <GScrollBars>{
            show_scroll_x: true
            show_scroll_y: false
            scroll_bar_x: {
                draw_bar: {bar_width: 3.0}
                bar_size: 4
                use_vertical_finger_scroll: true
            }
        },
        item: <GTabButton>{}
    }
    GTabBody = <GTabBodyBase>{
        height: Fill,
        width: Fill,
    }
    GTabPane = <GTabPaneBase>{
        height: Fill,
        width: Fill,
        margin: 0.0,
        padding: 6.0,
        spacing: 0.0,
    }
    GTab = <GTabBase>{
        height: 300.0,
        width: Fill,
        header: <GTabHeader>{
            margin: {top: 0.0, left: 0.0, bottom: 0.0, right: 0.0},
        },
        margin: 0.0,
        padding: 0.0,
        spacing: 0.0,
        flow: Down,
        body: <GTabPane>{
            // clip_x: true,
            // clip_y: true,
        },
    }
    GSplitter = <Splitter>{
        draw_splitter: {
            uniform border_radius: 1.0
            uniform splitter_pad: 1.0
            uniform splitter_grabber: 60.0

            instance pressed: 0.0
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.clear(#DDDDDD10);

                if self.is_vertical > 0.5 {
                    sdf.box(
                        self.splitter_pad,
                        self.rect_size.y * 0.5 - self.splitter_grabber * 0.5,
                        self.rect_size.x - 2.0 * self.splitter_pad,
                        self.splitter_grabber,
                        self.border_radius
                    );
                }
                else {
                    sdf.box(
                        self.rect_size.x * 0.5 - self.splitter_grabber * 0.5,
                        self.splitter_pad,
                        self.splitter_grabber,
                        self.rect_size.y - 2.0 * self.splitter_pad,
                        self.border_radius
                    );
                }
                return sdf.fill_keep(mix(
                    vec4(0.0),
                    mix(
                        vec4(0.6, 1.0, 1.0, 2.0),
                        vec4(1.0, 1.0, 1.0, 4.0),
                        self.pressed
                    ),
                    self.hover
                ));
            }
        }
    }
    GUpload = <GUploadBase>{
        height: 100.0,
        width: Fill,
        align: <ALIGN_CENTER_WALK>{},
        icon: <GSvg>{
            src: dep("crate://self/resources/icons/cloud_upload.svg"),
            height: 32.0,
            width: 32.0,
        }
    }
    GCollapse = <GCollapseBase>{
        height: Fit,
        width: Fill,
        flow: Down,
        opened: false,
        header: <GView>{
            height: 24.0,
            padding: {left: 6.0, right: 6.0, top: 3.0, bottom: 3.0},
            flow: Right,
            align: <ALIGN_LEFT_WALK>{},
            spacing: 6.0,
            margin: 0.0,
            border_radius: 0.0,
        },
        body: <GView>{
            height: 80.0,
            width: Fill,
            padding: {left: 6.0, right: 6.0, top: 3.0, bottom: 3.0},
            margin: 0.0,
            border_radius: 0.0,

        }
    }
    GTCell = <GTableCellBase>{
        align: <ALIGN_CENTER_WALK>{}
    }
    GTRow = <GTableRowBase>{
        width: Fill,
        height: 36.0,
        align: <ALIGN_LEFT_WALK>{}
    }
    GTBody = <GTableBodyBase>{
        height: Fit,
        width: Fill,
        flow: Down,
        align: {
            x: 0.0, y: 0.0
        }
    }
    GTHeader = <GTableHeaderBase>{}
    GTable = <GTableBase>{
        header: <GTHeader>{}
        body: <GTBody>{}
    }
    GVTBody = <GVTableBodyBase>{
        height: Fit,
        width: Fill,
        flow: Down,
        align: {
            x: 0.0, y: 0.0
        }
    }
    GWindow = <GWindowBase>{
        nav_control: <NavControl> {}
        window_bar = <GHLayout>{
            height: 32.0,
            width: Fill,
            background_color: #1F1E25,
            background_visible: true,
            align: {
                x: 0.0, y: 0.5
            }
            spacing: 0.0,
            mac_btns_wrap = <GHLayout>{
                visible: false
                height: 32.0,
                width: Fit,
                spacing: 6.0,
                align: {x: 0.0, y: 0.5},
                padding: {left: 6.0},
                close = <GToolButton> {icon_type: Close, os_type: Mac}
                max = <GToolButton> {icon_type: Max, os_type: Mac}
                min = <GToolButton> {icon_type: Min, os_type: Mac}
            }
            window_title = <GHLayout>{
                height: Fill,
                width: Fill,
                align: {x: 0.5, y: 0.5},
                spacing: 6.0,
                icon = <GImage>{
                    visible: false,
                    src: dep("crate://self/resources/icons/logo.png"),
                    height: 16.0,
                    width: 16.0,
                },
                title = <GLabel>{
                    height: Fit,
                    text: "",
                    font_size: 9.0,
                },
            }
            sub = <GHLayout>{
                visible: false,
                height: Fill,
                width: Fit,
            }
            win_btns_wrap = <GHLayout>{
                visible: false
                height: 32.0,
                width: Fit,
                spacing: 0.0,
                min = <GToolButton> {icon_type: Min, os_type: Windows}
                max = <GToolButton> {icon_type: Max, os_type: Windows}
                close = <GToolButton> {icon_type: Close, os_type: Windows}
            }
            linux_btns_wrap = <GHLayout>{
                visible: false
                height: 32.0,
                width: Fit,
                spacing: 6.0,
                align: {x: 1.0, y: 0.5},
                padding: {right: 6.0},
                min = <GToolButton> {icon_type: Min, os_type: Linux}
                max = <GToolButton> {icon_type: Max, os_type: Linux}
                close = <GToolButton> {icon_type: Close, os_type: Linux}
            }
        }
    }
    // GSelectItem = <GSelectItemBase>{
    //     height: 36.0,
    //     width: Fill,
    //     flow: Right,
    //     spacing: 6.0,
    //     align: {
    //         x: 0.0,
    //         y: 0.5
    //     },
    //     left: <GHLayout>{
    //         align: {x: 0.5, y: 0.5},
    //         height: Fill,
    //         width: Fit,
    //         left_slot = <GView>{
    //             height: 10.0,
    //             width: 10.0,
    //             border_radius: 2.5,
    //             border_width: 0.0,
    //         }
    //     },
    //     center: <GHLayout>{
    //         align: {x: 0.0, y: 0.5},
    //         height: Fill,
    //         width: Fit,
    //         center_slot = <GLabel>{
    //             color: (COLOR_INFO_900),
    //             font_size: (FONT_SIZE),
    //             text: "Select Item"
    //         }
    //     }
    //     right: <GHLayout>{
    //         align: {x: 0.0, y: 0.5},
    //         height: Fill,
    //         width: Fit,
    //         right_slot = <GLabel>{
    //             color: (COLOR_INFO_900),
    //             font_size: (FONT_SIZE_SMALL)
    //             text: "sub info"
    //         }
    //     }
    // }
    GSelectItem = <GSelectItemBase>{}
    GSelectOptions = <GSelectOptionsBase>{
        height: 144.0,
        width: 180.0,
        padding: {left: 8.0, right: 8.0, top: 6.0, bottom: 6.0},
        flow: Down,
        scroll_bars: <GScrollBars>{
            show_scroll_x: false
            show_scroll_y: true
            // scroll_bar_y: {
            //     draw_bar: {bar_width: 4.0}
            //     bar_size: 6
            //     use_vertical_finger_scroll: true
            // }
        },
    }
    GSelect = <GSelectBase>{
        font_family: (FONT_FAMILY),
        flow: Down,
        select_item: <GSelectItem>{},
        select_options: <GSelectOptions>{}
    }
    GTabbarItem = <GTabbarItemBase>{
        icon_slot: <GSvg>{
            height: 18.0,
            width: 18.0,
            color: #161616,
            src: dep("crate://self/resources/icons/home.svg"),
            stroke_hover_color: #FF7043,
            stroke_focus_color: #FF7043,
            animation_key: true,
        }
        text_slot: <GLabel>{
            font_size: 8.0,
            color: #161616,
            animation_key: true,
            text: "Home",
            stroke_hover_color: #FF7043,
            stroke_focus_color: #FF7043,
        }
    }
    GTabbar = <GTabbarBase>{
        height: 42.0,
        width: Fill,
        align: {
            x: 0.5,
            y: 0.5,
        },
        spacing: 0.0,
        padding: {
            left: 8.0,
            right: 8.0
        },
        border_radius: 0.0,

    }
    GBarPage = <GView>{
        visible:false,
        height: Fill,
        width: Fill,
        background_visible: false,
        border_radius: 0.0,
    }
    GPage = <GPageBase>{
        background_visible: false,
        border_radius: 0.0,
        flow: Down,
        height: Fill,
        width: Fill,
        header = <GHLayout>{
            height: 24.0,
            padding: {
                left: 4.0,
                right: 4.0,
            },
            spacing: 8.0,
            align: {
                x: 0.5,
                y: 0.5
            },
            back_wrap = <GHLayout>{
                height: Fill,
                width: 24.0,
                align: {
                    x: 0.5,
                    y: 0.5,
                },
                back = <GIcon>{
                    cursor: Hand,
                    theme: Dark,
                    height: 18.0,
                    width: 18.0,
                    stroke_width: 1.2,
                    icon_type: Left,
                }
            }
            title_wrap = <GHLayout>{
                align: {
                    x: 0.5,
                    y: 0.5
                },
                title = <GLabel>{
                    text: ""
                }
            }
            tool_wrap = <GHLayout>{
                height: Fill,
                width: 24.0,
                align: {
                    x: 0.5,
                    y: 0.5,
                },
            }
        }
        body = <GView>{
            border_radius: 0.0,
            clip_x: true,
            clip_y: true,
            theme: Dark,
            height: Fill,
            width: Fill
            padding: 4.0,
        }
    }
    GNavPage = <GPage>{}
    GRouter = <GRouterBase>{
        background_visible: false,
        height: Fill,
        width: Fill,
        bar_pages = <GView>{
            height: Fill,
            width: Fill,
            border_radius: 0.0,
            background_visible: false,
            flow: Down,
        }
        nav_pages = <GView>{
            height: Fill,
            width: Fill,
            border_radius: 0.0,
            background_visible: false,
            flow: Down,
        }
    }
    GMenuItem = <GMenuItemBase>{
        height: 36.0,
        width: Fill,
        padding: {
            left: 12.0,
            right: 12.0,
            top: 4.0,
            bottom: 4.0
        },
        spacing: 12.0,
        align: {
            x: 0.0,
            y: 0.5
        },
        icon_slot: <GSvg>{
            theme: Dark,
            color: (COLOR_WHITE),
            src: dep("crate://self/resources/icons/home.svg"),
            height: 16.0,
            width: 16.0,
        }
        text_slot: <GLabel>{
            text: "Home",
            font_size: 10.0,
        }
        right: <GHLayout>{
            visible: false
        }
    }
    GSubMenu = <GSubMenuBase>{
        title: <GView>{
            padding: {
                left: 8.0,
                right: 8.0,
                top: 4.0,
                bottom: 4.0
            },
            height: 32.0,
            width: Fill,
            align: {
                x: 0.0,
                y: 0.5
            },
        },
        items: <GVLayout>{
            height: Fill,
            width: Fill,
            align: {
                x: 0.0,
                y: 0.0
            },
        }
    }
    GMenu = <GMenuBase>{
        height: Fill,
        width: 240.0,
        flow: Down,
        border_radius: 0.0,
        header: <GVLayout>{
            visible: false,
        }
        body: <GVLayout>{
            height: Fill,
            width: Fill,
        }
        footer: <GVLayout>{
            visible: false
        }
    }
}
