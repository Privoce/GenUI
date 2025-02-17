/// Parse Strategy
/// Convert ParseTarget To AST
#[derive(Debug, Clone, Default)]
pub enum Strategy {
    /// an empty file
    None,
    /// only has template tag
    SingleTemplate,
    /// only has rust script
    SingleScript,
    /// only has style tag
    SingleStyle,
    /// no template, rust script, style
    /// only comment (should with signatures)
    SingleComment,
    /// template with rust script
    TemplateScript,
    /// template with style
    TemplateStyle,
    /// template with comment
    TemplateComment,
    /// script with comment
    ScriptComment,
    /// style with comment
    StyleComment,
    TemplateScriptComment,
    TemplateStyleComment,
    /// has all means: TemplateScriptStyle
    #[default]
    All,
    Error(String),
}
