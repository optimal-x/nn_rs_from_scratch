use super::vec_r2::Vec2;
use crate::ndarr::arr1::Arr1;
use crate::ndarr::{X, Y};
use crate::boxed_slice;

#[test]
pub(self) fn test_add() {
    let v1 = Vec2::new(1, 2);
    let v2 = Vec2::new(3, 4);
    assert_eq!(v1 + v2, Vec2::new(4, 6));
}

#[test]
pub(self) fn test_sub() {
    let v1 = Vec2::new(1, 2);
    let v2 = Vec2::new(3, 4);
    assert_eq!(v1 - v2, Vec2::new(-2, -2));
}

#[test]
pub(self) fn test_mul() {
    let v1 = Vec2::new(1, 2);
    let v2 = Vec2::new(3, 4);
    assert_eq!(v1 * v2, 11);
}

#[test]
pub(self) fn test_arr1_deref() {
    let arr1 = Arr1::new(boxed_slice![0.0; 10]);
    assert_eq!(&0.0, &(*arr1)[&[0]]);
    // test deref coercion as well
    assert_eq!(&0.0, &arr1[[0]]);

    let arr1 = Arr1::new(boxed_slice![3.0; 100]);
    assert_eq!(3.0, arr1[[50]]);
}

#[test]
pub(self) fn test_arr1_distance() {
    // immitating vec2 for legibility
    let arr_0 = Arr1::new(boxed_slice![-1.0, 0.0]);
    let arr_1 = Arr1::new(boxed_slice![0.0, -0.0]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!(1.0, dist);

    let arr_0 = Arr1::new(boxed_slice![3.0, 4.0]);
    let arr_1 = Arr1::new(boxed_slice![3.0, 0.0]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!(4.0, dist);

    let arr_0 = Arr1::new(boxed_slice![3.0, 4.0]);
    let arr_1 = Arr1::new(boxed_slice![4.0, 3.0]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!(2.0_f64.sqrt(), dist);

    let arr_0 = Arr1::new(boxed_slice![1.0; 100]);
    let arr_1 = Arr1::new(boxed_slice![5.0; 100]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!(40.0, dist);
}

#[test]
pub(self) fn test_vec2_index_using_x_y_structs() {
    let v2 = Vec2::new(1.0f64, 2.0f64);
    let x = v2[X];
    let y = v2[Y];
    assert_eq!((x, y), (1.0, 2.0));
}
