use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// An index that wraps around zero and a maximum value.
#[derive(Copy, Clone)]
pub struct WrappingIndex(usize);

impl WrappingIndex {
    #[inline(always)]
    #[must_use]
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    #[inline(always)]
    #[must_use]
    pub fn get(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut usize {
        &mut self.0
    }

    /// Increment the index by 1, wrapping around if necessary.
    ///
    /// Returns the new index.
    #[inline]
    pub fn increment(&mut self, max: usize) {
        self.0 = if max > 0 { (self.0 + 1) % max } else { max }
    }

    /// Decrement the index by 1, wrapping around if necessary.
    #[inline]
    pub fn decrement(&mut self, max: usize) {
        self.0 = if self.0 == 0 {
            max.saturating_sub(1)
        } else {
            self.0 - 1
        }
    }

    /// Ensure the index is within the given `max` value.
    ///
    /// If the index is out of bounds, it will be set to one less than the given `max`.
    #[inline]
    pub fn update_bounds(&mut self, max: usize) {
        if self.0 >= max {
            self.0 = max.saturating_sub(1);
        }
    }
}

impl PartialEq<usize> for WrappingIndex {
    fn eq(&self, other: &usize) -> bool {
        self.get() == *other
    }
}

impl<T> Index<WrappingIndex> for Vec<T> {
    type Output = T;

    fn index(&self, index: WrappingIndex) -> &Self::Output {
        &self[index.get()]
    }
}

impl<T> IndexMut<WrappingIndex> for Vec<T> {
    fn index_mut(&mut self, index: WrappingIndex) -> &mut Self::Output {
        &mut self[index.get()]
    }
}

impl Into<usize> for WrappingIndex {
    fn into(self) -> usize {
        self.0
    }
}

impl Default for WrappingIndex {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Items that can be selected by a wrapping index.
pub struct WrappedSelection<T, Val>
where
    T: NumItems + Index<usize, Output = Val> + IndexMut<usize>,
{
    index: WrappingIndex,
    items: T,
}

impl<T, Val> WrappedSelection<T, Val>
where
    T: NumItems + Index<usize, Output = Val> + IndexMut<usize>,
{
    #[inline]
    pub fn new(items: T) -> Self {
        Self::with_index(WrappingIndex::new(0), items)
    }

    #[inline]
    pub fn with_index(index: WrappingIndex, items: T) -> Self {
        Self { index, items }
    }

    /// Returns the current index position.
    #[inline]
    pub fn index(&self) -> usize {
        self.index.get()
    }

    /// Returns true if the current selection index is in bounds.
    #[inline]
    pub fn is_valid_index(&self) -> bool {
        self.num_items() > 0 && self.index.get() < self.num_items()
    }

    /// Returns a reference to the currently selected value.
    #[inline]
    pub fn selected(&self) -> Option<&Val> {
        if !self.is_valid_index() {
            return None;
        }

        Some(&self.items[self.index.get()])
    }

    /// Returns a mutable reference to the currently selected value.
    #[inline]
    pub fn selected_mut(&mut self) -> Option<&mut Val> {
        if !self.is_valid_index() {
            return None;
        }

        Some(&mut self.items[self.index.get()])
    }

    /// Increment the selected item index by 1, wrapping around if necessary.
    #[inline]
    pub fn inc_selected(&mut self) {
        self.index.increment(self.items.num_items())
    }

    /// Decrement the selected item index by 1, wrapping around if necessary.
    #[inline]
    pub fn dec_selected(&mut self) {
        self.index.decrement(self.items.num_items())
    }

    /// Set the selection index to `selected`.
    #[inline]
    pub fn set_selected(&mut self, selected: usize) {
        if selected >= self.items.num_items() {
            return;
        }

        *self.index.get_mut() = selected;
    }

    /// Update the selection index to ensure it stays in bounds.
    ///
    /// This should be called whenever an item is removed.
    #[inline]
    pub fn update_bounds(&mut self) {
        self.index.update_bounds(self.items.num_items())
    }

    /// Returns a mutable reference to the contained items.
    ///
    /// Make sure to call [`Self::update_bounds()`] if any items are going to be removed.
    #[inline]
    pub fn items_mut(&mut self) -> &mut T {
        &mut self.items
    }
}

impl<T, Val> Deref for WrappedSelection<T, Val>
where
    T: NumItems + Index<usize, Output = Val> + IndexMut<usize>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T, Val> DerefMut for WrappedSelection<T, Val>
where
    T: NumItems + Index<usize, Output = Val> + IndexMut<usize>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

pub trait NumItems {
    fn num_items(&self) -> usize;
}

impl<T> NumItems for Vec<T> {
    #[inline]
    fn num_items(&self) -> usize {
        self.len()
    }
}

impl<'a, T> NumItems for &'a Vec<T> {
    #[inline]
    fn num_items(&self) -> usize {
        self.len()
    }
}

impl<'a, T> NumItems for &'a [T] {
    #[inline]
    fn num_items(&self) -> usize {
        self.len()
    }
}

/// Enum variants to display on a list.
pub trait EnumListItems: Sized {
    fn items<'a>() -> &'a [Self];

    #[must_use]
    fn num_items() -> usize {
        Self::items().len()
    }
}

/// An enum with selectable variants.
pub struct SelectableEnum<T>
where
    T: EnumListItems + Copy,
{
    index: WrappingIndex,
    _phantom: PhantomData<T>,
}

impl<T> SelectableEnum<T>
where
    T: EnumListItems + Copy,
{
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            index: WrappingIndex::new(0),
            _phantom: PhantomData,
        }
    }

    /// Returns the current selected index.
    #[inline]
    #[must_use]
    pub fn index(&self) -> usize {
        self.index.get()
    }

    /// Returns the currently selected enum variant.
    #[inline]
    #[must_use]
    pub fn selected(&self) -> T {
        let index = self.index.get();
        T::items()[index]
    }

    /// Increment the selected enum variant index by 1, wrapping around as necessary.
    #[inline]
    pub fn increment(&mut self) {
        self.index.increment(T::num_items())
    }

    /// Decrement the selected enum variant index by 1, wrapping around as necessary.
    #[inline]
    pub fn decrement(&mut self) {
        self.index.decrement(T::num_items())
    }
}

impl<T> Default for SelectableEnum<T>
where
    T: EnumListItems + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
