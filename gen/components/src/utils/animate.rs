#[macro_export]
macro_rules! play_animation {
    () => {
        fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) {
            if self.animation_key {
                self.animator_play(cx, state);
            }
        }
    };
}