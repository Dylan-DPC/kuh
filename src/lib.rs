use std::ops::Deref;
use std::fmt::Debug;
use std::cmp::Ordering;
pub use crate::kuhnvert::{KuhnvertOwned, KuhnvertBorrowed};
use serde::{Serialize, Deserializer, Deserialize, Serializer};

pub mod kuhnvert;

#[derive(Clone)]
pub enum Kuh<'a, B, C=<B as KuhnvertOwned>::To>
where
    B: KuhnvertOwned<To=C>,
    C: KuhnvertBorrowed<To=B>
{
    Borrowed(&'a C),
    Owned(B)
}

impl<'a, B, C> Deref for Kuh<'a, B, C>
where
    B: KuhnvertOwned<To=C>,
    C: KuhnvertBorrowed<To = B>,
{
    type Target = C::To;
    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Borrowed(b) => b.convert(),
            Self::Owned(ref o) => o,
        }
}
}

impl<'a, B, C> Debug for Kuh<'a, B, C>
    where
        B: KuhnvertOwned<To=C> + Debug,
        C: KuhnvertBorrowed<To = B> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Borrowed(b) => b.fmt(f),
            Self::Owned(b) => b.fmt(f),
        }
    }
}

impl<'a, B, C> PartialEq<Kuh<'a, B, C>> for Kuh<'a, B, C> 
where
    B: KuhnvertOwned<To=C> + PartialEq<B>,
    C: KuhnvertBorrowed<To=B> + PartialEq<C>,
{
    fn eq(&self, other: &Kuh<'a, B, C>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}
        
impl<'a, B, C> Eq for Kuh<'a, B, C> 
where
    B: KuhnvertOwned<To=C> + Eq,
    C: KuhnvertBorrowed<To=B> + Eq,
{
}

impl<'a, B, C> PartialOrd for Kuh<'a,B, C> 
where

    B: KuhnvertOwned<To=C> + PartialOrd,
    C: KuhnvertBorrowed<To=B> + PartialOrd,
{

    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
       PartialOrd::partial_cmp(&**self, &**other)
    }
}
    
impl<'a, B, C> Ord for Kuh<'a, B, C> 
where

    B: KuhnvertOwned<To=C> + Ord,
    C: KuhnvertBorrowed<To=B> + Ord,
{
    fn cmp(&self, other:&Self) -> Ordering {
       Ord::cmp(&**self, &**other)
}
}

impl<'a, B, C> AsRef<B> for Kuh<'a, B, C>
where
    B: KuhnvertOwned<To = C> + Ord,
    C: KuhnvertBorrowed<To=B> + Ord,
{
    fn as_ref(&self) -> &B {
        self
    }
}

impl<'a, B, C> Serialize for Kuh<'a, B, C> 
where
    B: Serialize + KuhnvertOwned<To=C> + Ord + 'a,
    C: Serialize + KuhnvertBorrowed<To=B> + Ord + 'a,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where
            S: Serializer,
    {
        (**self).serialize(serializer)
    }
}



impl<'de, B, C> Deserialize<'de> for Kuh<'de, B, C>
where
    B: Deserialize<'de> + KuhnvertOwned<To=C> + Ord,
    C: Deserialize<'de> + KuhnvertBorrowed<To=B> + Ord,
  {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where
            D: Deserializer<'de>
      {
          Ok(Kuh::Owned(B::deserialize(deserializer)?))

      }
  }

