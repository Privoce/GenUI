use ra_ap_syntax::ast::{Path, Type};

pub trait AnalyzerStr {
    fn is_path_segment(&self, path: &Path) -> bool;
    fn is_trait(&self, trait_: Type) -> bool;
    fn is_self_type(&self, self_type: Type) -> bool;
}

impl AnalyzerStr for str {
    fn is_path_segment(&self, path: &Path) -> bool {
        is_path_segment(path, self)
    }
    fn is_trait(&self, trait_: Type) -> bool {
        is_trait(trait_, self)
    }
    fn is_self_type(&self, self_type: Type) -> bool {
        is_self_type(self_type, self)
    }
}

impl AnalyzerStr for String {
    fn is_path_segment(&self, path: &Path) -> bool {
        is_path_segment(path, self)
    }
    fn is_trait(&self, trait_: Type) -> bool {
        is_trait(trait_, self)
    }
    fn is_self_type(&self, self_type: Type) -> bool {
        is_self_type(self_type, self)
    }
}

pub fn is_path_segment(path: &Path, target: &str) -> bool {
    path.segment()
        .map(|seg| {
            seg.name_ref()
                .map(|name_ref| name_ref.text().as_str() == target)
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

pub fn is_trait(trait_: Type, target: &str) -> bool {
    if let Type::PathType(path_type) = trait_ {
        path_type
            .path()
            .map(|path| target.is_path_segment(&path))
            .unwrap_or_default()
    } else {
        false
    }
}

pub fn is_self_type(self_type: Type, target: &str) -> bool {
    is_trait(self_type, target)
}
