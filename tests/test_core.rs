//! Licensed under the Apache License, Version 2.0
//! http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
//! http://opensource.org/licenses/MIT, at your
//! option. This file may not be copied, modified, or distributed
//! except according to those terms.
#![no_std]

#[macro_use] extern crate itertools as it;

use it::Itertools;
use it::interleave;
use it::multizip;
use it::multizip_fallback;
use it::free::put_back;

#[test]
fn product2() {
    let s = "αβ";

    let mut prod = iproduct!(s.chars(), 0..2);
    assert!(prod.next() == Some(('α', 0)));
    assert!(prod.next() == Some(('α', 1)));
    assert!(prod.next() == Some(('β', 0)));
    assert!(prod.next() == Some(('β', 1)));
    assert!(prod.next() == None);
}

#[test]
fn product_temporary() {
    for (_x, _y, _z) in iproduct!(
        [0, 1, 2].iter().cloned(),
        [0, 1, 2].iter().cloned(),
        [0, 1, 2].iter().cloned())
    {
        // ok
    }
}


#[test]
fn izip_macro() {
    let mut zip = izip!(2..3);
    assert!(zip.next() == Some(2));
    assert!(zip.next().is_none());

    let mut zip = izip!(0..3, 0..2, 0..2i8);
    for i in 0..2 {
        assert!((i as usize, i, i as i8) == zip.next().unwrap());
    }
    assert!(zip.next().is_none());

    let xs: [isize; 0] = [];
    let mut zip = izip!(0..3, 0..2, 0..2i8, &xs);
    assert!(zip.next().is_none());
}

#[test]
fn izip3() {
    let mut zip = multizip((0..3, 0..2, 0..2i8));
    for i in 0..2 {
        assert!((i as usize, i, i as i8) == zip.next().unwrap());
    }
    assert!(zip.next().is_none());

    let xs: [isize; 0] = [];
    let mut zip = multizip((0..3, 0..2, 0..2i8, xs.iter()));
    assert!(zip.next().is_none());

    for (_, _, _, _, _) in multizip((0..3, 0..2, xs.iter(), &xs, xs.to_vec())) {
        /* test compiles */
    }
}

#[test]
fn zip_fallback() {
    let mut zip = multizip_fallback(((0..3, 11), (0..2, 12), (0..2i8, 13)));
    for i in 0..2 {
        assert!((i as usize, i, i as i8) == zip.next().unwrap());
    }
    assert!((2, 12, 13) == zip.next().unwrap());

    let xs: [isize; 0] = [];
    let default_int = 14;
    let mut zip = multizip_fallback(((0..3, 11), (0..2, 12), (0..2i8, 13), (xs.iter(), &default_int)));
    assert!((0, 0, 0, &14) == zip.next().unwrap());

    let st: [&str; 1] = ["te"];
    let default_str = "ing";
    let mut zip = multizip_fallback(((0..1, 11), (0..1, 12), (0..1, 13), (st.iter(), &default_str)));
    assert!((0, 0, 0, &"te") == zip.next().unwrap());
    assert!(zip.next().is_none());
}

#[test]
fn write_to() {
    let xs = [7, 9, 8];
    let mut ys = [0; 5];
    let cnt = ys.iter_mut().set_from(xs.iter().map(|x| *x));
    assert!(cnt == xs.len());
    assert!(ys == [7, 9, 8, 0, 0]);

    let cnt = ys.iter_mut().set_from(0..10);
    assert!(cnt == ys.len());
    assert!(ys == [0, 1, 2, 3, 4]);
}

#[test]
fn test_interleave() {
    let xs: [u8; 0]  = [];
    let ys = [7u8, 9, 8, 10];
    let zs = [2u8, 77];
    let it = interleave(xs.iter(), ys.iter());
    it::assert_equal(it, ys.iter());

    let rs = [7u8, 2, 9, 77, 8, 10];
    let it = interleave(ys.iter(), zs.iter());
    it::assert_equal(it, rs.iter());
}

#[test]
fn foreach() {
    let xs = [1i32, 2, 3];
    let mut sum = 0;
    xs.iter().foreach(|elt| sum += *elt);
    assert!(sum == 6);
}

#[test]
fn dropping() {
    let xs = [1, 2, 3];
    let mut it = xs.iter().dropping(2);
    assert_eq!(it.next(), Some(&3));
    assert!(it.next().is_none());
    let mut it = xs.iter().dropping(5);
    assert!(it.next().is_none());
}

#[test]
fn batching() {
    let xs = [0, 1, 2, 1, 3];
    let ys = [(0, 1), (2, 1)];

    // An iterator that gathers elements up in pairs
    let pit = xs.iter().cloned().batching(|it| {
               match it.next() {
                   None => None,
                   Some(x) => match it.next() {
                       None => None,
                       Some(y) => Some((x, y)),
                   }
               }
           });
    it::assert_equal(pit, ys.iter().cloned());
}

#[test]
fn test_put_back() {
    let xs = [0, 1, 1, 1, 2, 1, 3, 3];
    let mut pb = put_back(xs.iter().cloned());
    pb.next();
    pb.put_back(1);
    pb.put_back(0);
    it::assert_equal(pb, xs.iter().cloned());
}

#[test]
fn step() {
    it::assert_equal((0..10).step(1), (0..10));
    it::assert_equal((0..10).step(2), (0..10).filter(|x: &i32| *x % 2 == 0));
    it::assert_equal((0..10).step(10), 0..1);
}

#[test]
fn merge() {
    it::assert_equal((0..10).step(2).merge((1..10).step(2)), (0..10));
}


#[test]
fn repeatn() {
    let s = "α";
    let mut it = it::repeat_n(s, 3);
    assert_eq!(it.len(), 3);
    assert_eq!(it.next(), Some(s));
    assert_eq!(it.next(), Some(s));
    assert_eq!(it.next(), Some(s));
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}

#[test]
fn count_clones() {
    // Check that RepeatN only clones N - 1 times.

    use core::cell::Cell;
    #[derive(PartialEq, Debug)]
    struct Foo {
        n: Cell<usize>
    }

    impl Clone for Foo
    {
        fn clone(&self) -> Self
        {
            let n = self.n.get();
            self.n.set(n + 1);
            Foo { n: Cell::new(n + 1) }
        }
    }


    for n in 0..10 {
        let f = Foo{n: Cell::new(0)};
        let it = it::repeat_n(f, n);
        // drain it
        let last = it.last();
        if n == 0 {
            assert_eq!(last, None);
        } else {
            assert_eq!(last, Some(Foo{n: Cell::new(n - 1)}));
        }
    }
}

#[test]
fn part() {
    let mut data = [7, 1, 1, 9, 1, 1, 3];
    let i = it::partition(&mut data, |elt| *elt >= 3);
    assert_eq!(i, 3);
    assert_eq!(data, [7, 3, 9, 1, 1, 1, 1]);

    let i = it::partition(&mut data, |elt| *elt == 1);
    assert_eq!(i, 4);
    assert_eq!(data, [1, 1, 1, 1, 9, 3, 7]);

    let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let i = it::partition(&mut data, |elt| *elt % 3 == 0);
    assert_eq!(i, 3);
    assert_eq!(data, [9, 6, 3, 4, 5, 2, 7, 8, 1]);
}

#[test]
fn flatten_clone() {
    let data = &[
        &[1,2,3],
        &[4,5,6]
    ];
    let flattened1 = data.into_iter().cloned().flatten();
    let flattened2 = flattened1.clone();

    it::assert_equal(flattened1, &[1,2,3,4,5,6]);
    it::assert_equal(flattened2, &[1,2,3,4,5,6]);
}


