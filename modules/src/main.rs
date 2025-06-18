use std::collections::HashMap;
// Using Nested Paths to Clean Up Large use Lists
// use std::cmp::Ordering;
// use std::io;

// The above imports can be written as:
use std::{cmp::Ordering, io};

// To bring all public items in scope using the glob operator
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
