use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GIconLibExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        // <GLabel>{
        //     text: "GIconLib",
        // }
        <GHLayout>{
            height: Fit,
            spacing: 6.0,
            <GToolButton>{
                button_type: Default,
            }
            <GToolButton>{
                button_type: Min,
            }
            <GToolButton>{
                button_type: Max,
            }
            <GToolButton>{
                button_type: FullScreen,
            }
            <GToolButton>{
                button_type: Left,
            }
            <GToolButton>{
                button_type: Right,
            }
            <GToolButton>{
                button_type: More,
            }
            <GToolButton>{
                button_type: Close,
            }
            <GToolButton>{
                button_type: Up,
            }
            <GToolButton>{
                button_type: Down,
            }
            <GToolButton>{
                button_type: Switch,
            }
            <GToolButton>{
                button_type: Exit,
            }
            <GToolButton>{
                button_type: Expand,
            }
            <GToolButton>{
                button_type: ExpandTop,
            }
            <GToolButton>{
                button_type: ExpandLeft,
            }
            <GToolButton>{
                button_type: ExpandRight,
            }
            <GToolButton>{
                button_type: ExpandBottom,
            }
            <GToolButton>{
                button_type: Add,
            }
            <GToolButton>{
                button_type: Delete,
            }
            
        }
        <GHLayout>{
            spacing: 6.0,
            height: Fit,
            <GToolButton>{
                button_type: Correct,
            }
            <GToolButton>{
                button_type: DeleteKey,
            }
            <GToolButton>{
                button_type: Fresh,
            }
            <GToolButton>{
                button_type: Play,
            }
            <GToolButton>{
                button_type: Stop,
            }
            <GToolButton>{
                button_type: Setting,
            }
            <GToolButton>{
                button_type: Bind,
            }
            <GToolButton>{
                button_type: Menu,
            }
            <GToolButton>{
                button_type: Emoji,
            }
            <GToolButton>{
                button_type: Phone,
            }
            <GToolButton>{
                button_type: FullScreenExpand,
            }
            <GToolButton>{
                button_type: Upload,
            }
            <GToolButton>{
                button_type: Download,
            }
            <GToolButton>{
                button_type: Setting2,
            }
            <GToolButton>{
                button_type: Setting3,
            }
            <GToolButton>{
                button_type: Home,
            }
            <GToolButton>{
                button_type: GoOn,
            }
            <GToolButton>{
                button_type: Hot,
            }
            <GToolButton>{
                button_type: Heart,
            }
        }
        <GHLayout>{
            spacing: 6.0,
            height: Fit,
            <GToolButton>{
                button_type: HeartBroken,
            }
            <GToolButton>{
                button_type: Dislike,
            }
            <GToolButton>{
                button_type: Rss,
            }
            <GToolButton>{
                button_type: Share,
            }
            <GToolButton>{
                button_type: ZoomIn,
            }
            <GToolButton>{
                button_type: ZoomOut,
            }
            <GToolButton>{
                button_type: Eye,
            }
            <GToolButton>{
                button_type: EyeClose,
            }
            <GToolButton>{
                button_type: Search,
            }
            <GToolButton>{
                button_type: Connect,
            }
            <GToolButton>{
                button_type: Disconnect,
            }
            <GToolButton>{
                button_type: Debug,
            }
            <GToolButton>{
                button_type: Code,
            }
            <GToolButton>{
                button_type: Test,
            }
            <GToolButton>{
                button_type: Open,
            }
            <GToolButton>{
                button_type: OpenLeft,
            }
            <GToolButton>{
                button_type: OpenRight,
            }
            <GToolButton>{
                button_type: OpenTop,
            }
            <GToolButton>{
                button_type: OpenBottom,
            }
        }
        
    }
}