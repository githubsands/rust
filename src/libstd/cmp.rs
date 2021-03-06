// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

The `Ord` and `Eq` comparison traits

This module contains the definition of both `Ord` and `Eq` which define
the common interfaces for doing comparison. Both are language items
that the compiler uses to implement the comparison operators. Rust code
may implement `Ord` to overload the `<`, `<=`, `>`, and `>=` operators,
and `Eq` to overload the `==` and `!=` operators.

*/

#![allow(missing_doc)]

/**
* Trait for values that can be compared for equality and inequality.
*
* This trait allows partial equality, where types can be unordered instead of strictly equal or
* unequal. For example, with the built-in floating-point types `a == b` and `a != b` will both
* evaluate to false if either `a` or `b` is NaN (cf. IEEE 754-2008 section 5.11).
*
* Eq only requires the `eq` method to be implemented; `ne` is its negation by default.
*
* Eventually, this will be implemented by default for types that implement `TotalEq`.
*/
#[lang="eq"]
pub trait Eq {
    fn eq(&self, other: &Self) -> bool;

    #[inline]
    fn ne(&self, other: &Self) -> bool { !self.eq(other) }
}

/// Trait for equality comparisons where `a == b` and `a != b` are strict inverses.
pub trait TotalEq: Eq {
    // FIXME #13101: this method is used solely by #[deriving] to
    // assert that every component of a type implements #[deriving]
    // itself, the current deriving infrastructure means doing this
    // assertion without using a method on this trait is nearly
    // impossible.
    //
    // This should never be implemented by hand.
    #[doc(hidden)]
    #[inline(always)]
    fn assert_receiver_is_total_eq(&self) {}
}

macro_rules! totaleq_impl(
    ($t:ty) => {
        impl TotalEq for $t {}
    }
)

totaleq_impl!(bool)

totaleq_impl!(u8)
totaleq_impl!(u16)
totaleq_impl!(u32)
totaleq_impl!(u64)

totaleq_impl!(i8)
totaleq_impl!(i16)
totaleq_impl!(i32)
totaleq_impl!(i64)

totaleq_impl!(int)
totaleq_impl!(uint)

totaleq_impl!(char)

#[deriving(Clone, Eq, Show)]
pub enum Ordering { Less = -1, Equal = 0, Greater = 1 }

/// Trait for types that form a total order
pub trait TotalOrd: TotalEq + Ord {
    fn cmp(&self, other: &Self) -> Ordering;
}

impl TotalEq for Ordering {}
impl TotalOrd for Ordering {
    #[inline]
    fn cmp(&self, other: &Ordering) -> Ordering {
        (*self as int).cmp(&(*other as int))
    }
}

impl Ord for Ordering {
    #[inline]
    fn lt(&self, other: &Ordering) -> bool { (*self as int) < (*other as int) }
}

macro_rules! totalord_impl(
    ($t:ty) => {
        impl TotalOrd for $t {
            #[inline]
            fn cmp(&self, other: &$t) -> Ordering {
                if *self < *other { Less }
                else if *self > *other { Greater }
                else { Equal }
            }
        }
    }
)

totalord_impl!(u8)
totalord_impl!(u16)
totalord_impl!(u32)
totalord_impl!(u64)

totalord_impl!(i8)
totalord_impl!(i16)
totalord_impl!(i32)
totalord_impl!(i64)

totalord_impl!(int)
totalord_impl!(uint)

totalord_impl!(char)

/**
Return `o1` if it is not `Equal`, otherwise `o2`. Simulates the
lexical ordering on a type `(int, int)`.
*/
#[inline]
pub fn lexical_ordering(o1: Ordering, o2: Ordering) -> Ordering {
    match o1 {
        Equal => o2,
        _ => o1
    }
}

/**
* Trait for values that can be compared for a sort-order.
*
* Ord only requires implementation of the `lt` method,
* with the others generated from default implementations.
*
* However it remains possible to implement the others separately,
* for compatibility with floating-point NaN semantics
* (cf. IEEE 754-2008 section 5.11).
*/
#[lang="ord"]
pub trait Ord: Eq {
    fn lt(&self, other: &Self) -> bool;
    #[inline]
    fn le(&self, other: &Self) -> bool { !other.lt(self) }
    #[inline]
    fn gt(&self, other: &Self) -> bool {  other.lt(self) }
    #[inline]
    fn ge(&self, other: &Self) -> bool { !self.lt(other) }
}

/// The equivalence relation. Two values may be equivalent even if they are
/// of different types. The most common use case for this relation is
/// container types; e.g. it is often desirable to be able to use `&str`
/// values to look up entries in a container with `~str` keys.
pub trait Equiv<T> {
    fn equiv(&self, other: &T) -> bool;
}

#[inline]
pub fn min<T: TotalOrd>(v1: T, v2: T) -> T {
    if v1 < v2 { v1 } else { v2 }
}

#[inline]
pub fn max<T: TotalOrd>(v1: T, v2: T) -> T {
    if v1 > v2 { v1 } else { v2 }
}

#[cfg(test)]
mod test {
    use super::lexical_ordering;

    #[test]
    fn test_int_totalord() {
        assert_eq!(5.cmp(&10), Less);
        assert_eq!(10.cmp(&5), Greater);
        assert_eq!(5.cmp(&5), Equal);
        assert_eq!((-5).cmp(&12), Less);
        assert_eq!(12.cmp(-5), Greater);
    }

    #[test]
    fn test_ordering_order() {
        assert!(Less < Equal);
        assert_eq!(Greater.cmp(&Less), Greater);
    }

    #[test]
    fn test_lexical_ordering() {
        fn t(o1: Ordering, o2: Ordering, e: Ordering) {
            assert_eq!(lexical_ordering(o1, o2), e);
        }

        let xs = [Less, Equal, Greater];
        for &o in xs.iter() {
            t(Less, o, Less);
            t(Equal, o, o);
            t(Greater, o, Greater);
         }
    }
}
