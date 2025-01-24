use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;

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
                    theme: Error,
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
               
                theme: Dark,
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
