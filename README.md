# Immutable String References

A rust library for passing around references to immutable strings like that used in platforms such as Java or C#.

It stores these references roughly as follows:

```rust
pub enum Str {
    Small(TinyStr),
    Rc(Arc<String>),
    Static(&'static str),
}
```

Calling ```.clone()``` is always as cheap as possible, incurring at most an atomic reference increment/decrement using Arc<String> and using a stack-allocated string for strings smaller than the size of an Arc<String>

## Purpose

This library is to bridge the gap between that which would be infeasible using rust lifetimes and would be inefficient using ```String::clone()```.


## Example Usage

```rust
extern crate strref;
use strref::{Str, IntoStr, StrRef};

use std::collections::{HashMap};

// You can store the same string multiple times in a struct
// (You can't do this using lifetimes)
struct MyStruct {
  my_vec: Vec<Str>,            // <-- use "Str" for storage, only reference is stored
  my_map: HashMap<Str, usize>, // <-- Can be used as a map key also
}

impl MyStruct {

  // This is an example function that shows taking ownership of the string passed in
  // it automatically handles passing in &'static str or Arc<String> or another Str
  pub fn add<S: IntoStr>(&mut self, value: S) {
    //          ^^^^^^^ allows taking ownership inside the function
    let owned = value.into_str();     // <-- take ownership like this
    let cloned = owned.clone();       // <-- this is always cheap
    self.my_map.insert(cloned, self.my_vec.len());
    self.my_vec.push(owned);
  }

  // This is an example function that shows borrowing
  pub fn get<S: StrRef>(&self, value: S) -> Option<&usize> {
    //          ^^^^^^ alows borrowing an &str inside the function
    let s: &str = value.borrow_str();
    //                  ^^^^^^^^^^ ...borrow an &str like this
    self.my_map.get(s)
  }

  // An example of how to return the value as borrowed
  pub fn get_str(&self, index: usize) -> Option<&Str> {
    //                       return a reference ^^^^
    self.my_vec.get(index)
  }
}

let mut my_struct = MyStruct { my_vec: Vec::new(), my_map: HashMap::new() };

my_struct.add("literal");           // <-- automatically handles literals without duplication

let runtime_val = format!("built at {}", "runtime");
my_struct.add(runtime_val);         // <-- also handles taking ownership and wrapping with Rc

// comparisons with &str just work
assert_eq!("built at runtime", my_struct.get_str(1).unwrap());
```

