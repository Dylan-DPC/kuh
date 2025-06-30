pub use crate::derow::Derow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Deref;

pub mod derow;

#[derive(Clone)]
pub enum Kuh<'a, B>
where
    B: Derow,
{
    Borrowed(&'a B::Target),
    Owned(B),
}

impl<'a, B> Deref for Kuh<'a, B>
where
    B: Derow<Target = B>,
{
    type Target = <Self as Derow>::Target;

    fn deref(&self) -> &Self::Target {
        self.derow()
    }
}

impl<'a, B> Debug for Kuh<'a, B>
where
    B: Derow + Debug,
    <B as Derow>::Target: Debug,
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
    B: Derow + PartialEq,
    <B as Derow>::Target: PartialEq,
    Self: Derow<Target = <B as Derow>::Target>,
{
    fn eq(&self, other: &Kuh<'a, B>) -> bool {
        PartialEq::eq(self.derow(), other.derow())
    }
}

impl<'a, B> Eq for Kuh<'a, B>
where
    B: Derow + Eq,
    <B as Derow>::Target: Eq,
    Self: Derow<Target = <B as Derow>::Target>,
{
}

impl<'a, B> PartialOrd for Kuh<'a, B>
where
    Self: Derow<Target = <B as Derow>::Target>,
    B: Derow + PartialOrd,
    <B as Derow>::Target: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.derow(), other.derow())
    }
}

impl<'a, B> Ord for Kuh<'a, B>
where
    B: Derow + Ord,
    <B as Derow>::Target: Ord,
    Self: Derow<Target = <B as Derow>::Target>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self.derow(), other.derow())
    }
}

impl<'a, B> Serialize for Kuh<'a, B>
where
    B: Derow + Serialize,
    Self: Derow<Target = <B as Derow>::Target>,
    <B as Derow>::Target: Serialize,
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
    B: Derow + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Kuh::Owned(B::deserialize(deserializer)?))
    }
}

impl<'a, B> Derow for Kuh<'a, B>
where
    B: Derow,
    <B as Derow>::Target: Derow,
{
    type Target = <B as Derow>::Target;

    fn derow(&self) -> &Self::Target {
        match self {
            Self::Borrowed(b) => b,
            Self::Owned(o) => o.derow(),
        }
    }
}
