#![feature(associated_type_defaults)]
pub use crate::derow::Derow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Deref;

pub mod derow;

pub enum Kuh<'a, B>
where
    B: Derow<'a> + Clone,
{
    Borrowed(&'a B::Target),
    Owned(B),
}

impl<'a, B> Clone for Kuh<'a, B>
where
    B: Derow<'a> + Clone
{
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(b) => Kuh::Owned(B::from_borrowed(b)),
            Self::Owned(b) => Kuh::Owned(b.clone()),
        }
    }
}

impl<'a, B> Deref for Kuh<'a, B>
where
    B: Derow<'a> + Clone ,
{
    type Target = <Self as Derow<'a>>::Target;

    fn deref(&self) -> &Self::Target {
        self.derow()
    }
}

impl<'a, B> Debug for Kuh<'a, B>
where
    B: Derow<'a> + Clone + Debug,
    <B as Derow<'a>>::Target: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Borrowed(b) => b.fmt(f),
            Self::Owned(b) => b.fmt(f),
        }
    }
}

impl<'a, B> PartialEq<Kuh<'a, B>> for Kuh<'a, B>
where
    B: Derow<'a> + Clone + PartialEq,
    <B as Derow<'a>>::Target: PartialEq,
{
    fn eq(&self, other: &Kuh<'a, B>) -> bool {
        PartialEq::eq(self.derow(), other.derow())
    }
}

impl<'a, B> Eq for Kuh<'a, B>
where
    B: Derow<'a> + Clone + Eq,
    <B as Derow<'a>>::Target: Eq,
    Self: Derow<'a, Target = <B as Derow<'a>>::Target>,
{
}

impl<'a, B> PartialOrd for Kuh<'a, B>
where
    Self: Derow<'a, Target = <B as Derow<'a>>::Target>,
    B: Derow<'a> + Clone + PartialOrd,
    <B as Derow<'a>>::Target: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.derow(), other.derow())
    }
}

impl<'a, B> Ord for Kuh<'a, B>
where
    B: Derow<'a> + Clone + Ord,
    <B as Derow<'a>>::Target: Ord,
    Self: Derow<'a, Target = <B as Derow<'a>>::Target>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self.derow(), other.derow())
    }
}

impl<'a, B> Default for Kuh<'a, B> 
where
    B: Derow<'a> + Clone + Ord + Default,
    <B as Derow<'a>>::Target: Ord,
    Self: Derow<'a, Target = <B as Derow<'a>>::Target>,
{
    fn default() -> Self {
        Self::Owned(B::default())
    }

}

impl<'a, B> AsRef<B> for Kuh<'a, B>
where
    B: Derow<'a, Target = B> + Clone + Ord,
    Self: Derow<'a, Target = <B as Derow<'a>>::Target>,
{
    fn as_ref(&self) -> &B {
        self.derow()
    }
}

impl<'a, B> Serialize for Kuh<'a, B>
where
    B: Derow<'a> + Clone + Serialize,
    Self: Derow<'a, Target = <B as Derow<'a>>::Target>,
    <B as Derow<'a>>::Target: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.derow().serialize(serializer)
    }
}

impl<'de, B> Deserialize<'de> for Kuh<'de, B>
where
    B: Derow<'de> + Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Kuh::Owned(B::deserialize(deserializer)?))
    }
}

impl<'a, B> Derow<'a> for Kuh<'a, B>
where
    B: Derow<'a> + Clone,
{
    type Target = <B as Derow<'a>>::Target;

    fn derow(&self) -> &Self::Target {
        match self {
            Self::Borrowed(b) => b,
            Self::Owned(o) => o.derow(),
        }
    }

    fn from_borrowed(b: &'a Self::Target) -> Self {
        Kuh::Borrowed(b)
    }


}

