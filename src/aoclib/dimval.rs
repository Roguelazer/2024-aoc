pub trait DimVal:
    num_traits::Signed
    + num_traits::ToPrimitive
    + num_traits::identities::Zero
    + num_traits::identities::ConstOne
    + std::cmp::PartialOrd
    + std::cmp::PartialEq
    + Clone
    + Copy
    + std::fmt::Display
    + std::fmt::Debug
{
}

impl<
        S: num_traits::Signed
            + num_traits::ToPrimitive
            + num_traits::identities::Zero
            + num_traits::identities::ConstOne
            + std::cmp::PartialOrd
            + std::cmp::PartialEq
            + Clone
            + Copy
            + std::fmt::Display
            + std::fmt::Debug,
    > DimVal for S
{
}
