use gen_utils::common::Source;
use syn::Block;

/// # Script File
/// 以.gen结尾的脚本文件，这些文件一般用于描述一些特殊的逻辑（例如网络层插件， 错误处理插件等）
/// 当然它也可以是一个普通的rust文件，但最好直接使用.rs文件如果不是特殊情况，因为.gen文件会强制需要<script>标签进行包裹来解析
pub struct Script {
    pub source: Source,
    pub file: Block,
}
