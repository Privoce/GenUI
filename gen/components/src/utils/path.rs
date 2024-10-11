#[macro_export]
macro_rules! set_scope_path {
    () => {
        pub fn set_scope_path(&mut self, path: &HeapLiveIdPath) {
            if self.scope_path.is_none(){
                self.scope_path.replace(path.clone());
            }
        }
    };
}