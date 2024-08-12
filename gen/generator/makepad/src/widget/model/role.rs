use std::collections::HashMap;
#[allow(unused_imports)]
use std::default;

use gen_parser::{For, PropsKey, Value};
use gen_utils::common::{IFSignal, Ulid};

#[derive(Clone, Debug, Default)]
pub enum Role {
    If {
        id: Ulid,
        props: HashMap<PropsKey, Value>,
        signal: IFSignal,
    },
    For {
        id: Ulid,
        credential: For,
        loop_type: String,
        props: HashMap<PropsKey, Value>,
    },
    #[default]
    Normal,
}

impl Role {
    pub fn new_if(props: HashMap<PropsKey, Value>, signal: IFSignal) -> Self {
        Role::If {
            id: Ulid::new(),
            props,
            signal,
        }
    }
    pub fn new_option_if(
        props: HashMap<PropsKey, Value>,
        signal: IFSignal,
        ulid: Option<Ulid>,
    ) -> Self {
        Role::If {
            id: ulid.unwrap_or(Ulid::new()),
            props,
            signal,
        }
    }
    pub fn new_for(credential: For, loop_type: String, props: HashMap<PropsKey, Value>) -> Self {
        Role::For {
            id: Ulid::new(),
            props,
            credential,
            loop_type,
        }
    }
    // is for or if
    pub fn is_special(&self) -> bool {
        !matches!(self, Role::Normal)
    }
    pub fn get_if_uild(&self) -> Option<Ulid> {
        match self {
            Role::If { id, .. } => Some(id.clone()),
            _ => None,
        }
    }
    /// match role use ulid
    /// if role is normal, return None
    pub fn match_role(&self, another: &Ulid) -> Option<bool> {
        match self {
            Role::If { id, .. } | Role::For { id, .. } => Some(id == another),
            Role::Normal => None,
        }
    }
    pub fn prefix_if(&self) -> Option<String>{
        match self {
            Role::If { signal, .. } => Some(signal.to_string()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum RoleType {
    If(IFSignal),
    For,
    Normal,
}

impl RoleType {
    pub fn ignore_if(&self) -> bool {
        matches!(
            self,
            RoleType::If(IFSignal::ElseIf) | RoleType::If(IFSignal::Else)
        )
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
