use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum GError {
    IconTypeTransfom,
}

impl Error for GError {
    
}

impl Display for GError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GError::IconTypeTransfom => f.write_str(
                "Cannot transform icon type to target type. You may use the non-exist icon type.",
            ),
        }
    }
}
