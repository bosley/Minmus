use std::fmt;

/// Error type for minmus
/// 
/// **ItemDoesNotExist**   : The requested item did not exist in the data store
/// **UnableToObtainLock** : Unable to obtain lock for requested operation 
/// 
#[derive(Debug)]    // This allows fmt::Debug to dump the enum using {:?}
pub enum Error {
    ItemDoesNotExist,
    UnableToObtainLock
}

/// Implementations of the fmt::Display trait, so std::Fmt 
/// understands how to print the error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ItemDoesNotExist => {

                write!(f, "Error: Item does not exist")
            }
            Error::UnableToObtainLock => {

                write!(f, "Error: Unable to obtain lock")
            }
        }
    }
}