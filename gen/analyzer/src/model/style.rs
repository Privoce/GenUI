use std::collections::HashMap;

use crate::{value::Value, PropKey};
use gen_utils::{
    common::string::FixedString, err_from_to, error::{ConvertError, Error}
};

/// ### Style Value
pub type StyleVal = HashMap<PropKey, Value>;
/// ## Style
/// in gen-ui no difference between style and props,
/// so we use the same struct to represent them
/// `<id|class, HashMap<prop, value>>`
pub type Style = HashMap<String, StyleVal>;

pub struct StyleVisitor;

impl StyleVisitor {
    /// 根据层级找出所有的样式
    /// 例如:
    /// ```
    /// <view class="a">
    ///     <view class="b"></view>
    ///     <view id="d">
    ///         <view id="c"></view>
    ///     </view>
    /// </view>
    ///
    /// .a{
    ///   height: 100px;
    ///   .b{
    ///     width: 100px;
    ///     #c {
    ///      font-size: 100px;
    ///    }
    ///   }
    ///   #c{
    ///     height: 200px;
    ///   }
    /// }
    ///
    /// #c {
    ///  width: 200px;
    /// }
    /// ```
    /// 当访问id = c的时候，我们需要找出所有的样式，也就是`.a-#c`和`#c`的样式
    /// 这个方法就是用来找出所有的样式的
    /// 参数中styles存储了展开后的所有样式，例如:
    /// ```
    /// k: .a        v: {height: 100px}
    /// k: .a-.b     v: {width: 100px}
    /// k: .a-.b-#c  v: {font-size: 100px}
    /// k: .a-#c     v: {height: 200px}
    /// k: #c        v: {width: 200px}
    ///
    /// 当id = c时, chain = [IdClass{id: None, class: Some("a")}, IdClass{id: Some("d"), class: None}]
    /// 在template结构中，c的父级是d，但是在style中是`.a#c`，d没有样式所以不写，所以我们需要找出所有包含c的样式
    /// 然后根据chain进行正确的过滤, 因为从存储的样式中单纯靠c会找到`#c`,`.a-#c`,`.a-.b-#c`，但是我们需要的是`.a-#c`和`#c`
    /// ```
    pub fn visit(
        styles: &HashMap<String, HashMap<PropKey, Value>>,
        id: Option<&String>,
        class: Option<&Value>,
        chain: &Vec<IdClass>,
    ) -> Result<Vec<HashMap<PropKey, Value>>, Error> {
        let mut res = vec![];
        // [id] ------------------------------------------------------------------------------------
        if let Some(id) = id {
            //对所有包含id的styles进行过滤
            for (k, c_styles) in styles.iter() {
                let id = format!("#{}", id);
                if matcher(k, &id, chain)? {
                    res.push(c_styles.clone());
                }
            }
        }

        // [class] ------------------------------------------------------------------------------------
        if let Some(class) = class {
            let class = class_value_to_string(class)?;
            for c_class in class {
                //对所有包含class的styles进行过滤
                for (k, c_styles) in styles.iter() {
                    if matcher(k, &c_class, chain)? {
                        res.push(c_styles.clone());
                    }
                }
            }
        }

        Ok(res)
    }
}

/// 匹配名称是否相同
/// 当end = c时, chain = [IdClass{id: None, class: Some("a")}, IdClass{id: Some("d"), class: None}]
/// k: .a-.b
/// k: .a-.b-#c
/// k: .a-#c
/// k: #c
/// 匹配方式:
/// 通过`-`进行分割，然后从后往前匹配，如果匹配成功则返回true，整个vec全匹配成功则返回true
fn matcher(k: &str, end: &str, chain: &Vec<IdClass>) -> Result<bool, Error> {
    // 分割
    let k = k.split_fixed("-");
    // dbg!(&k, end, chain);
    // 准备一个flag作为匹配标志
    let mut flag = 0;
    let mut start = 0;
    let len = k.len();
    // 从后往前匹配
    for k in k.iter().rev() {
        if k == &end {
            flag += 1;
        } else {
            // 这说明第一次匹配就失败了，直接break
            if flag == 0 {
                break;
            }
            let is_id = k.starts_with("#");
            let mut c_flag = false;
            // 说明当前k不是end，现在需要匹配chain
            // 用k到chain中查找是否存在，不存在直接break, 存在则将索引记录到start中，然后flag+1，start作为下次匹配的起始位置
            for (i, id_class) in chain.iter().rev().enumerate().skip(start) {
                if is_id {
                    if let Some(c_id) = id_class.fmt_id() {
                        if &c_id == k {
                            c_flag = true;
                            start = i;
                        }
                    }
                    continue;
                } else {
                    if let Some(c_class) = id_class.fmt_class() {
                        let c_class = c_class?;
                        if c_class.contains(&k.to_string()) {
                            c_flag = true;
                            start = i;
                        }
                    }
                    continue;
                }
            }

            if c_flag {
                flag += 1;
            } else {
                break;
            }
        }
    }

    Ok(flag == len)
}

/// 存储历史父级的id或class的节点
/// 例如:
/// ```
/// .a {
///     .b{.c{}} => .a .b
///     .d{} => .a
/// }
/// ```
/// 也就是当访问c的时候，我们需要知道它的父级是a.b
/// 这个结构体就是用来存储这个信息的，所以我们需要一个Vec<IdClass>来存储这个信息
/// 通过简单join就能得到最终的id或class
#[derive(Debug, Clone)]
pub struct IdClass {
    pub id: Option<String>,
    pub class: Option<Value>,
}

impl IdClass {
    /// 根据index截断链
    pub fn split_chain(chain: &Vec<IdClass>, index: usize) -> Vec<IdClass> {
        chain[..index].to_vec()
    }
    pub fn fmt_id(&self) -> Option<String> {
        self.id.as_ref().map(|id| format!("#{}", id))
    }
    pub fn fmt_class(&self) -> Option<Result<Vec<String>, Error>> {
        self.class
            .as_ref()
            .map(|class| class_value_to_string(class))
    }
}

/// 将class的值转换为字符串
pub fn class_value_to_string(class: &Value) -> Result<Vec<String>, Error> {
    match class {
        Value::Vec(vec_val) => {
            let mut leaf_vec = vec![];
            for v in vec_val {
                if let Ok(e_val) = v.as_enum() {
                    leaf_vec.push(
                        e_val
                            .leaf()
                            .map(|leaf| format!(".{}", leaf.to_string()))
                            .unwrap(),
                    );
                } else {
                    return Err(err_from_to!("Value" => "String"));
                }
            }
            Ok(leaf_vec)
        }
        Value::Enum(e_val) => Ok(vec![e_val
            .leaf()
            .map(|leaf| format!(".{}", leaf.to_string()))
            .unwrap()]),
        _ => Err(err_from_to!("Value" => "String")),
    }
}

