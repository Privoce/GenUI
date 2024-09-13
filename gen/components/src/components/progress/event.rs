use makepad_widgets::DefaultNone;

#[derive(Clone, Debug, DefaultNone)]
pub enum GProgressEvent {
    BeforeMove(f64),
    Moving(f64),
    AfterMove(f64),
    None,
}
