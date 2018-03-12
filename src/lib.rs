use std::rc::Rc;
use std::sync::Arc;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum Str {
    Rc(Rc<String>),
    Arc(Arc<String>),
    Static(&'static str),
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
            &Str::Arc(ref s) => StrRef::borrow_str(s),
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

impl StrRef for Arc<String> {
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
            &Str::Arc(ref s) => Str::Arc(s.clone()),
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

impl ToStr for Arc<String> {
    fn to_str(&self) -> Str {
        Str::Arc(self.clone())
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

impl IntoStr for Arc<String> {
    fn into_str(self) -> Str {
        Str::Arc(self)
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
