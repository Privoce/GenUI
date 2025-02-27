use std::fmt::Display;

pub type SCResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Import(ImportError),
    AttrMacro(AttrMacroError),
    Item(ItemError),
    ProcMacro(ProcMacroError),
    Parse(syn::Error)
}

impl From<syn::Error> for Error {
    fn from(value: syn::Error) -> Self {
        Error::Parse(value)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "GenUI Error: {:?}",
            match self {
                Error::Import(e) => e.to_string(),
                Error::AttrMacro(e) => e.to_string(),
                Error::Item(e) => e.to_string(),
                Error::ProcMacro(e) => e.to_string(),
                Error::Parse(e) => e.to_string(),
            }
        ))
    }
}

impl From<ImportError> for Error {
    fn from(value: ImportError) -> Self {
        Error::Import(value)
    }
}

impl From<AttrMacroError> for Error {
    fn from(value: AttrMacroError) -> Self {
        Error::AttrMacro(value)
    }
}

impl From<ItemError> for Error {
    fn from(value: ItemError) -> Self {
        Error::Item(value)
    }
}

impl From<ProcMacroError> for Error {
    fn from(value: ProcMacroError) -> Self {
        Error::ProcMacro(value)
    }
}

// ProcMacro error ------------------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProcMacroError {
    MultiDefaultProp,
    NamedFieldEvent,
}

impl Display for ProcMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcMacroError::MultiDefaultProp => {
                f.write_str("GenUI `default_prop!` can only be used once!")
            }
            ProcMacroError::NamedFieldEvent => {
                f.write_str("GenUI `#[event]` can only be used on unnamed fields, means you can not use like: `enum $Enum{ $field{$arg: $arg_ty, ..} }`!")
            }
        }
    }
}

// Item error ------------------------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ItemError {
    ItemConvertError(String),
}

impl Display for ItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemError::ItemConvertError(s) => {
                f.write_fmt(format_args!("Item convert error: {}", s))
            }
        }
    }
}

// Attr error ------------------------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttrMacroError {
    MultiPropMacro,
    MultiInstanceMacro,
    LiveRustConflict,
    NotEventMacro,
    LifeCycleConflict(String),
    NoLifeCycleMacro,
}

impl Display for AttrMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrMacroError::MultiPropMacro => f.write_str("GenUI `#[component]` can only be used once!"),

            AttrMacroError::LiveRustConflict => {
                f.write_str("#[live] and #[rust] can not both exist in the same field!")
            }
            AttrMacroError::NotEventMacro => f.write_str("GenUI `#[event]` not matched!"),
            AttrMacroError::LifeCycleConflict(s) => {
                f.write_fmt(format_args!("GenUI `#[{}]` can not be convert to!", s))
            }
            AttrMacroError::NoLifeCycleMacro => {
                f.write_str("GenUI lifetime proc macro is not matched!")
            }
            AttrMacroError::MultiInstanceMacro => {
                f.write_str("GenUI `default_prop!` can only be used once!")
            }
        }
    }
}
// Import error ---------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImportError {
    MultiImportMacro,
    ImportSyncError(String),
}

impl Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::MultiImportMacro => f.write_str("GenUI `import!` can only be used once!"),
            ImportError::ImportSyncError(s) => {
                f.write_fmt(format_args!("import! sync error: {}", s))
            }
        }
    }
}

// ----------------------------------------------------------------------------------------------------
