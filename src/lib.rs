//! Immutable String References
//!
//! A string library for using strings in a fashion more like that used by platforms
//! such as Java or C#, where all strings are immutable and passed around with references.
//!
//! This library uses reference counting for runtime strings
//! and static references for compile-time string literals
//!
//! # Examples
//!
//! ```
//! use strref::{Str, IntoStr, StrRef};
//!
//! use std::collections::{HashMap};
//!
//! // You can store the same string multiple times in a struct
//! // (You can't do this using lifetimes)
//! struct MyStruct {
//!   my_vec: Vec<Str>,            // <-- use "Str" for storage, only reference is stored
//!   my_map: HashMap<Str, usize>, // <-- Can be used as a map key also
//! }
//!
//! impl MyStruct {
//!
//!   // This is an example function that shows taking ownership of the string passed in
//!   // it automatically handles passing in &'static str or Rc<String> or another Str
//!   pub fn add<S: IntoStr>(&mut self, value: S) {
//!     //          ^^^^^^^ allows taking ownership inside the function
//!     let owned = value.into_str();     // <-- take ownership like this
//!     let cloned = owned.clone();       // <-- this is always cheap
//!     self.my_map.insert(cloned, self.my_vec.len());
//!     self.my_vec.push(owned);
//!   }
//!
//!   // This is an example function that shows borrowing
//!   pub fn get<S: StrRef>(&self, value: S) -> Option<&usize> {
//!     //          ^^^^^^ alows borrowing an &str inside the function
//!     let s: &str = value.borrow_str();
//!     //                  ^^^^^^^^^^ ...borrow an &str like this
//!     self.my_map.get(s)
//!   }
//!
//!   // An example of how to return the value as borrowed
//!   pub fn get_str(&self, index: usize) -> Option<&Str> {
//!     //                       return a reference ^^^^
//!     self.my_vec.get(index)
//!   }
//! }
//!
//! let mut my_struct = MyStruct { my_vec: Vec::new(), my_map: HashMap::new() };
//!
//! my_struct.add("literal");           // <-- automatically handles literals without duplication
//!
//! let runtime_val = format!("built at {}", "runtime");
//! my_struct.add(runtime_val);         // <-- also handles taking ownership and wrapping with Rc
//!
//! // comparisons with &str just work
//! assert_eq!("built at runtime", my_struct.get_str(1).unwrap());
//!
//! ```


use std::rc::Rc;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::fmt::Display;

#[derive(Debug)]
pub enum Str {
    Rc(Rc<String>),
    Static(&'static str),
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            &Str::Rc(ref rc) => rc.fmt(f),
            &Str::Static(s) => s.fmt(f),
        }
    }
}

impl Str {
    // This allows you to duplicate the original string
    // into a brand new owned String
    // It duplicates the memory and so it's a separate function you must opt into
    // You should usually find all instances of this function and attempt to find ways of removing it
    pub fn duplicate(&self) -> String {
        let s: &str = self.borrow_str();
        String::from(s)
    }

    fn borrow_str(&self) -> &str {
        match self {
            &Str::Rc(ref s) => StrRef::borrow_str(s),
            &Str::Static(s) => StrRef::borrow_str(s),
        }
    }
}

pub trait StrRef {
    fn borrow_str(&self) -> &str;
}

impl StrRef for Str{
    fn borrow_str(&self) -> &str {
        self.borrow_str()
    }
}

impl StrRef for Rc<String> {
    fn borrow_str(&self) -> &str {
        let s1: &String = self.borrow();
        let s2: &str = s1.borrow();
        s2
    }
}

impl StrRef for str {
    fn borrow_str(&self) -> &str {
        self
    }
}

impl StrRef for &'static str {
    fn borrow_str(&self) -> &str {
        self
    }
}

impl StrRef for String {
    fn borrow_str(&self) -> &str {
        self.borrow()
    }
}

impl<'f> StrRef for &'f String {
    fn borrow_str(&self) -> &str {
        (*self).borrow()
    }
}

pub trait ToStr : StrRef {
    fn to_str(&self) -> Str;
}

pub trait IntoStr : StrRef {
    fn into_str(self) -> Str;
}

impl Clone for Str {
    fn clone(&self) -> Str {
        match self {
            &Str::Rc(ref s) => Str::Rc(s.clone()),
            &Str::Static(s) => Str::Static(s),
        }
    }
}

impl ToStr for Str {
    fn to_str(&self) -> Str {
        self.clone()
    }
}

impl ToStr for Rc<String> {
    fn to_str(&self) -> Str {
        Str::Rc(self.clone())
    }
}

impl ToStr for &'static str {
    fn to_str(&self) -> Str {
        Str::Static(*self)
    }
}

impl IntoStr for Str {
    fn into_str(self) -> Str {
        self
    }
}

impl IntoStr for String {
    fn into_str(self) -> Str {
        Str::Rc(Rc::new(self))
    }
}

impl<'f> IntoStr for &'f String {
    fn into_str(self) -> Str {
        Str::Rc(Rc::new(self.clone()))
    }
}

impl IntoStr for Rc<String> {
    fn into_str(self) -> Str {
        Str::Rc(self)
    }
}

impl IntoStr for &'static str {
    fn into_str(self) -> Str {
        Str::Static(self)
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        self.borrow_str()
    }
}

impl PartialEq<Str> for str {
    fn eq(&self, other: &Str) -> bool {
        let s2: &str = other.borrow_str();
        self.eq(s2)
    }
}

impl PartialEq<Str> for &'static str {
    fn eq(&self, other: &Str) -> bool {
        let s2: &str = other.borrow_str();
        (*self).eq(s2)
    }
}

impl PartialEq<str> for Str {
    fn eq(&self, other: &str) -> bool {
        let s1: &str = self.borrow_str();
        s1.eq(other)
    }
}

impl PartialEq<Str> for Str {
    fn eq(&self, other: &Str) -> bool {
        let s2: &str = other.borrow_str();
        self.eq(s2)
    }
}

impl Hash for Str {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let s: &str = self.borrow_str();
        s.hash(h)
    }
}

impl Eq for Str {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
