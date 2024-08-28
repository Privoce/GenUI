use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    GBreadCrumbExample = <ScrollYView>{
        height: 180.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GBreadCrumb"
        }
        <GVLayout>{
            spacing: 10.0,
            height: 140.0,
            width: Fill,
            <GHLayout>{
                height: 30.0,
                spacing: 10.0,
                <GBreadCrumbItem>{
                    text: "Spliter",
                }
                <GBreadCrumbItem>{
                    text: "Line",
                    split_type: Line
                }
                <GBreadCrumbItem>{
                    text: "Arrow",
                    split_type: Arrow
                }
            }
            <GBreadCrumb>{
                labels: ["home", "components", "button"],
            }
            <GBreadCrumb>{
                labels: ["home", "components", "gen", "ui", "crumb"],
                crumb_item: {
                    split_type: Arrow,
                }
            }
            // <GBreadCrumb>{
            //     items: vec!["home", "components", "button"],
            // }
        }
    }
}
