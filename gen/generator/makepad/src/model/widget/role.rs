use gen_converter::Parent;
use gen_parser::{For, PropsKey, Value};
use gen_utils::common::{IFSignal, Ulid};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub enum Role {
    If {
        id: Ulid,
        props: HashMap<PropsKey, Value>,
        signal: IFSignal,
    },
    For {
        parent: ForParent,
        creditial: For,
        /// 原始组件在父组件中的位置
        origin_pos: usize,
        /// 涉及到的变量
        props: HashMap<String, String>,
        id: String,
        name: String,
        children: Vec<Role>,
    },
    #[default]
    Normal,
}

impl Role {
    pub fn is_single_for(&self) -> bool {
        if let Role::For { children, .. } = self {
            children.is_empty()
        } else {
            false
        }
    }
    pub fn is_nested_for(&self) -> bool {
        if let Role::For { parent, .. } = self {
             parent.is_for()
        } else {
            false
        }
    }
    // is for or if
    pub fn is_virtual(&self) -> bool {
        !matches!(self, Role::Normal)
    }
    pub fn is_for(&self) -> bool {
        matches!(self, Role::For { .. })
    }
    pub fn push_child(&mut self, role: Role) -> () {
        match self {
            Role::For { children, .. } => {
                if role.is_for() {
                    children.push(role);
                }
            }
            _ => (),
        }
    }
    pub fn for_field(&self) -> Option<String> {
        match self {
            Role::For { creditial, .. } => Some(creditial.fmt_iter_ident()),
            _ => None,
        }
    }
    pub fn get_if_uild(&self) -> Option<Ulid> {
        match self {
            Role::If { id, .. } => Some(id.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ForParent {
    pub id: String,
    pub name: String,
    /// is parent is root
    pub is_root: bool,
    pub creditial: Option<For>,
}

impl ForParent {
    pub fn set_credential(&mut self, role: Role) -> () {
        if role.is_for() {
            if let Role::For { creditial, .. } = role {
                self.creditial = Some(creditial);
            }
        }
    }
    pub fn is_for(&self) -> bool {
        self.creditial.is_some()
    }
}

impl From<&Parent> for ForParent {
    fn from(value: &Parent) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.to_string(),
            is_root: value.root,
            creditial: None,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum RoleType {
    If(IFSignal),
    For,
    Normal,
}

#[allow(dead_code)]
impl RoleType {
    /// ## convert to prefix camel
    pub fn to_prefix_camel<'a>(&self, for_name: &'a str) -> Option<&'a str> {
        match self {
            RoleType::If(_) => Some("IfWidget"),
            RoleType::For => Some(for_name),
            _ => None,
        }
    }
    pub fn to_prefix_snake(&self) -> Option<&str> {
        match self {
            RoleType::If(_) => Some("if_widget"),
            RoleType::For => Some("for_widget"),
            _ => None,
        }
    }
    pub fn is_virtual(&self) -> bool {
        !matches!(self, RoleType::Normal)
    }
    pub fn ignore_if(&self) -> bool {
        matches!(
            self,
            RoleType::If(IFSignal::ElseIf) | RoleType::If(IFSignal::Else)
        )
    }
}

impl From<&str> for RoleType {
    fn from(value: &str) -> Self {
        match value {
            "if" => Self::If(IFSignal::If),
            "else_if" => Self::If(IFSignal::ElseIf),
            "else" => Self::If(IFSignal::Else),
            "for" => Self::For,
            _ => Self::Normal,
        }
    }
}

impl From<&Role> for RoleType {
    fn from(role: &Role) -> Self {
        match role {
            Role::If { signal, .. } => RoleType::If(*signal),
            Role::For { .. } => RoleType::For,
            Role::Normal => RoleType::Normal,
        }
    }
}
