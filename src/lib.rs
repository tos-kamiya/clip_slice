#![doc = include_str!("../README.md")]

use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

fn clip(pos: isize, len: usize) -> usize {
    if pos < 0 {
        let neg_pos = (-pos) as usize;
        if neg_pos >= len {
            0
        } else {
            len - neg_pos
        }
    } else if pos as usize > len {
        len
    } else {
        pos as usize
    }
}

/// A struct that serves as a prefix for the functions `by`, `mut_by`, `by_as_slice`, and `by_as_mut_slice`.
pub struct Clip;

/// A trait that defines `by` and `mut_by`.
pub trait ClipSlice<T, R> {
    /// The arguments are a slice and a range possibly negative indices. The return value is a slice. 
    /// If an index is a negative value, the position is interpreted as going backwards from the back end of the slice.
    fn by<'t, S>(sl: S, range: R) -> &'t [T]
    where
        S: Into<&'t [T]>;

    /// Almost the same as function `by`, but takes an immutable slice as argument or return value.
    fn mut_by<'t, S>(sl: S, range: R) -> &'t mut [T]
    where
        S: Into<&'t mut [T]>;
}

impl<T> ClipSlice<T, Range<isize>> for Clip {
    fn by<'t, S>(sl: S, range: Range<isize>) -> &'t [T]
    where
        S: Into<&'t [T]>,
    {
        let slice = sl.into();
        let len = slice.len();
        let start = clip(range.start, len);
        let end = clip(range.end, len);
        &slice[start..end]
    }
    fn mut_by<'t, S>(sl: S, range: Range<isize>) -> &'t mut [T]
    where
        S: Into<&'t mut [T]>,
    {
        let slice = sl.into();
        let len = slice.len();
        let start = clip(range.start, len);
        let end = clip(range.end, len);
        &mut slice[start..end]
    }
}

impl<T> ClipSlice<T, RangeFrom<isize>> for Clip {
    fn by<'t, S>(sl: S, range: RangeFrom<isize>) -> &'t [T]
    where
        S: Into<&'t [T]>,
    {
        let slice = sl.into();
        let len = slice.len();
        let start = clip(range.start, len);
        &slice[start..]
    }
    fn mut_by<'t, S>(sl: S, range: RangeFrom<isize>) -> &'t mut [T]
    where
        S: Into<&'t mut [T]>,
    {
        let slice = sl.into();
        let len = slice.len();
        let start = clip(range.start, len);
        &mut slice[start..]
    }
}

impl<T> ClipSlice<T, RangeTo<isize>> for Clip {
    fn by<'t, S>(sl: S, range: RangeTo<isize>) -> &'t [T]
    where
        S: Into<&'t [T]>,
    {
        let slice = sl.into();
        let len = slice.len();
        let end = clip(range.end, len);
        &slice[..end]
    }
    fn mut_by<'t, S>(sl: S, range: RangeTo<isize>) -> &'t mut [T]
    where
        S: Into<&'t mut [T]>,
    {
        let slice = sl.into();
        let len = slice.len();
        let end = clip(range.end, len);
        &mut slice[..end]
    }
}

impl<T> ClipSlice<T, RangeFull> for Clip {
    fn by<'t, S>(sl: S, _range: RangeFull) -> &'t [T]
    where
        S: Into<&'t [T]>,
    {
        let slice = sl.into();
        &slice[..]
    }
    fn mut_by<'t, S>(sl: S, _range: RangeFull) -> &'t mut [T]
    where
        S: Into<&'t mut [T]>,
    {
        let slice = sl.into();
        &mut slice[..]
    }
}

/// A trait that defines `by_as_slice` and `by_as_mut_slice`.
pub trait ClipAsSlice<T, R> {
    /// A helper function. Generate a slice and apply Clip::by to it.
    fn by_as_slice<'t>(vec: &'t Vec<T>, range: R) -> &'t [T];
    
    /// A helper function. Generate a mutable slice and apply Clip::mut_by to it.
    fn by_as_mut_slice<'t>(vec: &'t mut Vec<T>, range: R) -> &'t mut [T];
}

impl<T> ClipAsSlice<T, Range<isize>> for Clip {
    fn by_as_slice<'t>(vec: &'t Vec<T>, range: Range<isize>) -> &'t [T] {
        let slice = vec.as_slice();
        Clip::by(slice, range)
    }
    fn by_as_mut_slice<'t>(vec: &'t mut Vec<T>, range: Range<isize>) -> &'t mut [T] {
        let slice = vec.as_mut_slice();
        Clip::mut_by(slice, range)
    }
}

impl<T> ClipAsSlice<T, RangeFrom<isize>> for Clip {
    fn by_as_slice<'t>(vec: &'t Vec<T>, range: RangeFrom<isize>) -> &'t [T] {
        let slice = vec.as_slice();
        Clip::by(slice, range)
    }
    fn by_as_mut_slice<'t>(vec: &'t mut Vec<T>, range: RangeFrom<isize>) -> &'t mut [T] {
        let slice = vec.as_mut_slice();
        Clip::mut_by(slice, range)
    }
}

impl<T> ClipAsSlice<T, RangeTo<isize>> for Clip {
    fn by_as_slice<'t>(vec: &'t Vec<T>, range: RangeTo<isize>) -> &'t [T] {
        let slice = vec.as_slice();
        Clip::by(slice, range)
    }
    fn by_as_mut_slice<'t>(vec: &'t mut Vec<T>, range: RangeTo<isize>) -> &'t mut [T] {
        let slice = vec.as_mut_slice();
        Clip::mut_by(slice, range)
    }
}

impl<T> ClipAsSlice<T, RangeFull> for Clip {
    fn by_as_slice<'t>(vec: &'t Vec<T>, _range: RangeFull) -> &'t [T] {
        let slice = vec.as_slice();
        &slice[..]
    }
    fn by_as_mut_slice<'t>(vec: &'t mut Vec<T>, _range: RangeFull) -> &'t mut [T] {
        let slice = vec.as_mut_slice();
        &mut slice[..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_range_simple() {
        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], 0..2);
        assert_eq!(s, &[0, 1]);

        let mut a = [0, 1, 2, 3];

        let s = Clip::mut_by(&mut a[..], 0..2);
        s[1] = 10;
        assert_eq!(a, [0, 10, 2, 3]);
    }

    #[test]
    fn clip_range_negative() {
        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], -2..4);
        assert_eq!(s, &[2, 3]);

        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], 1..-1);
        assert_eq!(s, &[1, 2]);
    }

    #[test]
    fn clip_range_from() {
        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], -2..);
        assert_eq!(s, &[2, 3]);

        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], 1..);
        assert_eq!(s, &[1, 2, 3]);
    }

    #[test]
    fn clip_range_to() {
        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], ..4);
        assert_eq!(&s, &[0, 1, 2, 3]);

        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], ..-1);
        assert_eq!(s, &[0, 1, 2]);
    }

    #[test]
    fn clip_range_full() {
        let a = [0, 1, 2, 3];

        let s = Clip::by(&a[..], ..);
        assert_eq!(s, &[0, 1, 2, 3]);
    }

    #[test]
    fn clip_as_slice_simple() {
        let v = vec![0, 1, 2, 3];

        let s = Clip::by_as_slice(&v, 0..2);
        assert_eq!(Vec::from(s), vec![0, 1]);

        let mut v = vec![0, 1, 2, 3];

        let s = Clip::by_as_mut_slice(&mut v, 0..2);
        s[1] = 10;
        assert_eq!(Vec::from(v), vec![0, 10, 2, 3]);
    }

    #[test]
    fn clip_as_slice_from() {
        let v = vec![0, 1, 2, 3];

        let s = Clip::by_as_slice(&v, -2..);
        assert_eq!(Vec::from(s), vec![2, 3]);

        let v = vec![0, 1, 2, 3];

        let s = Clip::by_as_slice(&v, 1..);
        assert_eq!(Vec::from(s), vec![1, 2, 3]);
    }

    #[test]
    fn clip_as_slice_to() {
        let v = vec![0, 1, 2, 3];

        let s = Clip::by_as_slice(&v, ..4);
        assert_eq!(Vec::from(s), vec![0, 1, 2, 3]);

        let v = vec![0, 1, 2, 3];

        let s = Clip::by_as_slice(&v, ..-1);
        assert_eq!(Vec::from(s), vec![0, 1, 2]);
    }

    #[test]
    fn clip_as_slice_full() {
        let v = vec![0, 1, 2, 3];

        let s = Clip::by_as_slice(&v, ..);
        assert_eq!(Vec::from(s), vec![0, 1, 2, 3]);
    }
}
