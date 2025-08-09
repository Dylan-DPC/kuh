pub trait Derow {
    type Target: ?Sized;

    fn derow(&self) -> &Self::Target;
}

macro_rules! de {
    ($typ: ty) => {
        impl<'a> Derow for $typ {
            type Target = <Self as core::ops::Deref>::Target;

            fn derow(&self) -> &Self::Target {
                self
            }
        }
    };
}

macro_rules! row {
    ($typ: ty) => {
        impl<'a> Derow for $typ {
            type Target = Self;

            fn derow(&self) -> &Self::Target {
                self
            }
        }
    };
}

row!(u8);
row!(u32);
de!(String);
