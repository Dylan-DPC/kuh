pub trait KuhnvertOwned : Clone {
    type To;

    fn convert(&self) -> Self::To;
}

pub trait KuhnvertBorrowed : Clone {
    type To;

    fn convert(&self) -> &Self::To;
}
