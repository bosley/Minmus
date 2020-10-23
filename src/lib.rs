// Bring in the module 'errors' (error.rs) so we can use it in the library
mod errors;

// Declare 'pub use' on the enumeration 'Error' in the 'error.rs' file so users
// of the library can access it directly
pub use errors::Error;

// Bring in the module 'data_store' (data_store.rs) so we can use it in the library
mod data_store;

// Declare 'pub use' on the DataStore from the 'data_store.rs' file so users
// of the library can access it directly
pub use data_store::DataStore;