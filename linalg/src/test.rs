use super::vec_r2::Vec2;
use crate::{
    ndarr::{arr1::Arr1, ArrD},
    number::{NumberFuncs, F32, I32},
};

#[test]
pub(self) fn test_add() {
    let v1 = Vec2::new(I32(1), I32(2));
    let v2 = Vec2::new(I32(3), I32(4));
    assert_eq!(v1 + v2, Vec2::new(I32(4), I32(6)));
}

#[test]
pub(self) fn test_sub() {
    let v1 = Vec2::new(I32(1), I32(2));
    let v2 = Vec2::new(I32(3), I32(4));
    assert_eq!(v1 - v2, Vec2::new(I32(-2), I32(-2)));
}

#[test]
pub(self) fn test_mul() {
    let v1 = Vec2::new(I32(1), I32(2));
    let v2 = Vec2::new(I32(3), I32(4));
    assert_eq!(v1 * v2, I32(11));
}

#[test]
pub(self) fn test_arr1_deref() {
    let arr1 = Arr1::from(vec![F32(0.0); 10]);
    assert_eq!(&F32(0.0), &(*arr1)[0]);
    // test deref coercion as well
    assert_eq!(&F32(0.0), &arr1[0]);
}

#[test]
pub(self) fn test_arr1_get() {
    let arr1 = Arr1::from(vec![F32(0.0); 10]);
    assert_eq!(Some(&F32(0.0)), arr1.get(&[0]));
    assert_eq!(Some(&F32(0.0)), arr1.get(&[2]));
    assert_eq!(Some(&F32(0.0)), arr1.get(&[0]));
}

#[test]
pub(self) fn test_arr1_distance() {
    // immitating vec2 for legibility
    let arr_0 = Arr1::from(vec![F32(-1.0), F32(0.0)]);
    let arr_1 = Arr1::from(vec![F32(0.0), F32(-0.0)]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!(F32(1.0), dist);

    let arr_0 = Arr1::from(vec![F32(3.0), F32(4.0)]);
    let arr_1 = Arr1::from(vec![F32(3.0), F32(0.0)]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!(F32(4.0), dist);

    let arr_0 = Arr1::from(vec![F32(3.0), F32(4.0)]);
    let arr_1 = Arr1::from(vec![F32(4.0), F32(3.0)]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!(F32(2.0).sqrt(), dist);
}
