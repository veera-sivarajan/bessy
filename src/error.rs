use std::error::Error;
use std::fmt;

macro_rules! scan_error {
    () => {
        Err(BessyError::Scan)
    };
}


#[derive(Debug)]
pub enum BessyError {
    Scan,
}

impl Error for BessyError {}

impl fmt::Display for BessyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BessyError::Scan => write!(f, "Scan error!"),
        }
    }
}
    
    
