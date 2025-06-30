pub trait Derow {
    type Target;

    fn derow(&self) -> &Self::Target;
}
