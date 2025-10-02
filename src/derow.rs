pub trait Derow<'b>{
    type Target: ?Sized;

    fn derow(&self) -> &Self::Target;
    fn from_borrowed(b: &'b Self::Target) -> Self; 
}

macro_rules! de {
    ($typ: ty) => {
        impl<'a, 'b> Derow<'b> for $typ {
            type Target = <Self as core::ops::Deref>::Target;

            fn derow(&self) -> &Self::Target {
                self
            }

            fn from_borrowed(b: &'b Self::Target) -> Self {
                    b.to_owned()
            }
        }
    };
}

macro_rules! row {
    ($typ: ty) => {
        impl<'a, 'b> Derow<'b> for $typ {
            type Target = Self;

            fn derow(&self) -> &Self::Target {
                self
            }

            fn from_borrowed(b: &'b Self::Target) -> Self {
                b.to_owned()
            }
        }
    }
}

row!(u8);
row!(u32);
de!(String);


