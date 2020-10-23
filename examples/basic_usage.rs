extern crate minmus;
use minmus::DataStore;

fn main() {

    println!("Creating the data store");

    // Create a data store
    let mut new_ds = DataStore::<&str, i32>::new();

    // Create a couple of items we want to put into the DataStore
    let item_0 : i32 = 24;
    let item_1 : i32 = 99;

    println!("Inserting '{}' into the data store under key 'item_0'", {item_0});

    // Set the item given a certain key
    match new_ds.set_item(&"item_0", &item_0) {
        Some(err) => { panic!("Failed to set item : {}", err); }
        None => { 
            println!("Item was stored")
        }
    }

    println!("Inserting '{}' into the data store under key 'item_1'", {item_1});

    // Set the other item given a different key
    match new_ds.set_item(&"item_1", &item_1) {
        Some(err) => { panic!("Failed to set item : {}", err); }
        None => { 
            println!("Item was stored")
        }
    }

    println!("Attempting to get 'item_0'");

    // Attempt to get an item
    match new_ds.get_item(&"item_0") {
        Ok(value_retrieved) => {

            println!("Got item : {}", value_retrieved);

            // Ensure that the item has the value we expected
            assert_eq!(item_0, value_retrieved);
        }

        Err(err) => {

            // Panic and quit if an error was returned, and leverage fmt::Display trait
            // that the Error enumeration implemented to display what went wrong
            panic!("Got an error attempting to get an item: {}", err);
        }
    }

    println!("Attempting to get 'item_1'");

    // Attempt to get an item
    match new_ds.get_item(&"item_1") {
        Ok(value_retrieved) => {

            println!("Got item : {}", value_retrieved);

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