# clip_slice

clip_slice is a utility inspired by Python's slice, which allows you to specify an index backward from the end by a negative value.

## Usage

`Clip::by(<slice>, <start>..<end>) -> <slice>`  
`Clip::mut_by(<mut_slice>, <start>..<end>) -> <mut_slice>`  

Here, `<start>` and `<end>` are indices, and if they are negative, the position is interpreted as going backward from the back end of the slice.

## Examples

```rust
use clip_slice::{Clip, ClipAsSlice, ClipSlice};

fn main() {
    // generating slices with negative indices.
    let a = [0, 1, 2, 3, 4, 5];
    assert_eq!(Clip::by(&a[..], ..-2), &[0, 1, 2, 3]);
    assert_eq!(Clip::by(&a[..], -4..-1), &[2, 3, 4]);

    // use in combination with rev and step_by
    let a = [0, 1, 2, 3, 4, 5];
    assert_eq!(
        (&a[..]).iter().rev().map(|&n| n).collect::<Vec<isize>>(),
        vec![5, 4, 3, 2, 1, 0]
    ); // normal slice & rev
    macro_rules! ref_iter_to_vec {
        ( $e:expr ; $t:ty ) => {
            $e.map(|&n| n).collect::<Vec<$t>>()
        };
    }
    assert_eq!(
        ref_iter_to_vec!((&a[..]).iter().rev(); isize),
        vec![5, 4, 3, 2, 1, 0]
    ); // normal slice & rev, shortened with a macro

    assert_eq!(
        ref_iter_to_vec!(Clip::by(&a[..], ..-2).iter().rev(); isize),
        vec![3, 2, 1, 0]
    ); // clipped slice & rev
    assert_eq!(
        ref_iter_to_vec!(Clip::by(&a[..], ..-2).iter().rev().step_by(2); isize),
        vec![3, 1]
    ); // clipped slice & rev/step_by

    // generating mutable slices
    let mut a = [0, 1, 2, 3, 4, 5];
    let s = Clip::mut_by(&mut a[..], 1..-2);
    s[0] = 10;
    assert_eq!(a, [0, 10, 2, 3, 4, 5]);

    // accessing items with negative indices
    let a = [0, 1, 2, 3, 4, 5];
    assert_eq!(Clip::by(&a[..], -1..)[0], 5);
    assert_eq!(Clip::by(&a[..], -2..)[0], 4);
    macro_rules! at {
        ( $slice:expr , $index:expr ) => {
            Clip::by(&$slice[..], $index..)[0]
        };
    }
    assert_eq!(at!(a, -1), 5); // shortened with a macro
    assert_eq!(at!(a, -2), 4);

    // assign items with negative index
    // let mut a = [0, 1, 2, 3, 4, 5];
    // a[5] = 50;
    // assert_eq!(a, [0, 1, 2, 3, 4, 50]);

    macro_rules! mut_ref_at {
        ( $slice:expr , $index:expr ) => {
            &mut Clip::mut_by(&mut $slice[..], $index..)[0]
        };
    }
    let mut a = [0, 1, 2, 3, 4, 5];
    *mut_ref_at!(a, -1) = 50;
    assert_eq!(a, [0, 1, 2, 3, 4, 50]);

    let mut v = vec![0, 1, 2, 3, 4, 5];
    *mut_ref_at!(v, -2) = 40;
    assert_eq!(v, vec![0, 1, 2, 3, 40, 5]);

    // generating slices from vectors
    let v = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(Clip::by_as_slice(&v, ..-2), &[0, 1, 2, 3]);

    let mut v = vec![0, 1, 2, 3, 4, 5];
    let s = Clip::by_as_mut_slice(&mut v, 1..-2);
    s[0] = 10;
    assert_eq!(v, vec![0, 10, 2, 3, 4, 5]);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Similar projects

* [slyce](https://github.com/mkmik/slyce/)
