use crate::{
    Diffable,
};

pub enum Edit<'a, T: Diffable<'a>> {
    Copy(&'a T),
    Change(T::D),
}

impl<'a, T: Diffable<'a>> Edit<'a, T> {
    pub fn is_copy(&self) -> bool {
        if let Self::Copy(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_change(&self) -> bool {
        if let Self::Change(_) = self {
            true
        } else {
            false
        }
    }

    pub fn change(&self) -> Option<&T::D> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
}


pub enum EditField<'a, T: Diffable<'a>> {
    Insert(&'a T),
    Remove,
    Copy(&'a T),
    Change(T::D),
}

impl<'a, T: Diffable<'a>> EditField<'a, T> {
    // FIXME run doctests default
    //
    // Checks if the edit is an insert.
    //
    // # Examples
    //
    // ```
    // assert_eq!(Edit::Insert(&2).is_insert(), true);
    // assert_eq!(Edit::Remove.is_insert(), false);
    // ```
    pub fn is_insert(&self) -> bool {
        if let Self::Insert(_) = self {
            true
        } else {
            false
        }
    }
    // FIXME doc tests
    pub fn is_remove(&self) -> bool {
        if let Self::Remove = self {
            true
        } else {
            false
        }
    }
    pub fn is_copy(&self) -> bool {
        if let Self::Copy(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_change(&self) -> bool {
        if let Self::Change(_) = self {
            true
        } else {
            false
        }
    }
    pub fn insert(&self) -> Option<&'a T> {
        if let Self::Insert(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn change(&self) -> Option<&T::D> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
}

impl<'a, T: Diffable<'a>> Into<EditField<'a, T>> for Edit<'a, T> {
    fn into(self) -> EditField<'a, T> {
        match self {
            Self::Copy(value) => EditField::Copy(value),
            Self::Change(diff) => EditField::Change(diff),
        }
    }
}

pub enum EditSection<T: Eq> {
    Copy(T),
    Add(T),
    Remove(T),
}

impl<T: Eq + std::fmt::Debug> std::fmt::Debug for EditSection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Copy(value) => write!(f, "Copy({:?})", value),
            Self::Add(value) => write!(f, "Add({:?})", value),
            Self::Remove(value) => write!(f, "Remove({:?})", value),
        }
    }
}

impl<T: Eq + PartialEq> PartialEq for EditSection<T> {
    fn eq(&self, other: &Self) -> bool {
        let left = match self { Self::Copy(left) | Self::Add(left) | Self::Remove(left) => left };
        let right = match other { Self::Copy(right) | Self::Add(right) | Self::Remove(right) => right };
        left == right
    }
}

impl<T: Eq> Eq for EditSection<T> {}
