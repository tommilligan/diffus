pub mod diffable_impls;
pub mod edit;
pub mod same;
mod lcs;

pub trait Diffable<'a> {
    type D: Sized + 'a;
    type Target: Diffable<'a> + ?Sized + 'a;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self::Target>;
}

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}
