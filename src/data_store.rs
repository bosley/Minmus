use crate::errors::Error;

use std::{
    collections::HashMap,
    sync::Mutex,
};

/// DataStore object that allows the key and value to be 
/// set by the user
/// 
/// # Example Usage
///
/// ```
/// use minmus::DataStore;
/// 
/// // Create a data store where they key is of type : &str
/// // and the value is of type: i32
/// let new_ds = DataStore::<&str, i32>::new();
/// ```
/// 
pub struct DataStore <K, V> {       // K, V are the 'generic' types being used 

    data: Mutex<HashMap<K, V>>      // K is the Key item type, and V is the Value item type
}

// Implementation for data store. Here we have to include the generics used by
// the DataStore
impl <K, V> DataStore <K, V> 
where V: Clone + Copy,                          // This is used to ensure that type 'V' has the Clone and Copy traits, as we will need them
      K: Clone + std::cmp::Eq + std::hash::Hash // Similarly, these traits are required by the 'K' trait 
{

    /// Create a new data store
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new())
        }
    }

    /// Get an item from the data store
    /// 
    /// If the return matches to **Ok** the item existed and was returned
    /// Otherwise an error was returned and either we weren't able to obtain
    /// a lock, or the item didn't exist
    /// 
    pub fn get_item(&mut self, key: &K) -> Result<V, Error> 
    {
        let lock = match self.data.lock() {
            Ok(v) => v,
            Err(_) => return Err(Error::UnableToObtainLock)
        };

        match lock.get(key) {
            Some(value) => {
                Ok(*value)
            },
            None => {
                Err(Error::ItemDoesNotExist)
            }
        }
    }

    /// Set the value of an item
    /// 
    /// If type **None** is returned then the operation was a success,
    /// in the event **Some** is returned, we weren't able to acquire
    /// a lock for the data
    ///  
    pub fn set_item(&mut self, key: &K, value: &V) -> Option<Error> {

        let mut lock = match self.data.lock() {
            Ok(v) => v,
            Err(_) => return Some(Error::UnableToObtainLock)
        };

        lock.insert(key.clone(), value.clone());

        None
    }
}

/*
    Below we have a very basic test to ensure that the base functionality of the DataStore is working
    as we intend it to. This test can be run with **cargo test**

    As an interesting aside - The example within the documentation above will actually be executed during a test
    which will ensure that out example is accurate!
*/

#[cfg(test)]    // Indicate that the module is a test
mod tests {

    // Since we are in a different module (tests) we need to 'use' the items that are in the containing file.
    // To do this we use 'super::*' meaning that we are using everything that we are allowed to above.
    use super::*;

    #[test]     // Indicate that the function below is its own test
    fn simple_test() {

        // Create a data store
        let mut new_ds = DataStore::<&str, i32>::new();

        // Create a couple of items we want to put into the DataStore
        let item_0 : i32 = 24;
        let item_1 : i32 = 99;

        // Set the item given a certain key
        match new_ds.set_item(&"item_0", &item_0) {
            Some(err) => { panic!("Failed to set item : {}", err); }
            None => { }
        }

        // Set the other item given a different key
        match new_ds.set_item(&"item_1", &item_1) {
            Some(err) => { panic!("Failed to set item : {}", err); }
            None => { }
        }

        // Attempt to get an item
        match new_ds.get_item(&"item_0") {
            Ok(value_retrieved) => {

                // Ensure that the item has the value we expected
                assert_eq!(item_0, value_retrieved);
            }

            Err(err) => {

                // Panic and quit if an error was returned, and leverage fmt::Display trait
                // that the Error enumeration implemented to display what went wrong
                panic!("Got an error attempting to get an item: {}", err);
            }
        }

        // Attempt to get an item
        match new_ds.get_item(&"item_1") {
            Ok(value_retrieved) => {

                // Ensure that the item has the value we expected
                assert_eq!(item_1, value_retrieved);
            }

            Err(err) => {

                // Panic and quit if an error was returned, and leverage fmt::Display trait
                // that the Error enumeration implemented to display what went wrong
                panic!("Got an error attempting to get an item: {}", err);
            }
        }
    }
}