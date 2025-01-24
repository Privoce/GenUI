#[derive(Debug,Clone,PartialEq)]
pub enum OfflinePosition{
    AboveTemplate,
    AboveScript,
    AboveStyle,
    /// in fact no below template be use in parse
    /// it is used in `up()` or `down()` when need to reorganize
    // BelowTemplate,
    /// same as BelowTemplate
    // BelowScript,
    /// maybe is end of the file
    /// maybe no template no style no rust script, only has comment
    End
}