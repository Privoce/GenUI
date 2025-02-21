use gen_analyzer::Callbacks;
use gen_utils::error::Error;

// use crate::model::Widget;

pub fn no_callback(callbacks: Option<&Callbacks>, widget: &str) -> Result<(), Error> {
    if callbacks.is_some() {
        return Err(format!("{} widget can't have callbacks", widget).into());
    }
    Ok(())
}

// pub fn handle_children(
//     w_children: &mut Option<Vec<Widget>>,
//     children: Option<Vec<TemplateModel>>,
//     is_static: bool,
// ) -> Result<(), Error> {
//     if let Some(children) = children {
//         let mut children_widgets = Vec::new();
//         for child in children {
//             todo!()
//             // let w = Widget::try_from((child, is_static))?;
//             // children_widgets.push(w);
//         }
//         if !children_widgets.is_empty() {
//             // res.children.replace(children_widgets);
//             w_children.replace(children_widgets);
//         }
//     }
//     Ok(())
// }
