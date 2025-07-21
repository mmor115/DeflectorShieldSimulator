use serde::{Serialize, Serializer};

pub enum Bull<'a, O, B> where &'a O: Into<B> {
    InnerBorrowed(Vec<B>),
    OuterBorrowed(&'a Vec<O>)
}

impl <'a, O, B: From<&'a O>> Bull<'a, O, B> where B: Clone {
    pub fn as_owned_vec(&self) -> Vec<B> {
        match self {
            Bull::InnerBorrowed(ov) => ov.clone(),
            Bull::OuterBorrowed(bv) => bv.iter().map(|o| B::from(o)).collect()
        }
    }
}

impl <'a, O, B: From<&'a O>> Serialize for Bull<'a, O, B> where O: Serialize, B: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Bull::InnerBorrowed(v) => {
                v.serialize(serializer)
            }
            Bull::OuterBorrowed(v) => {
                v.serialize(serializer)
            }
        }
    }
}

impl <'a, O, B: From<&'a O>> Clone for Bull<'a, O, B> where B: Clone {
    fn clone(&self) -> Bull<'a, O, B> {
        match self {
            Bull::InnerBorrowed(ov) => Bull::InnerBorrowed((*ov).clone()),
            Bull::OuterBorrowed(bv) => Bull::OuterBorrowed(*bv)
        }
    }
}