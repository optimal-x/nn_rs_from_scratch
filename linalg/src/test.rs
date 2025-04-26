use super::vec_r2::Vec2;
use crate::ndarr::arr1::Arr1;
use crate::ndarr::container;

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
    assert_eq!(v1 - v2, Vec2::new((-2), (-2)));
}

#[test]
pub(self) fn test_mul() {
    let v1 = Vec2::new(1, 2);
    let v2 = Vec2::new(3, 4);
    assert_eq!(v1 * v2, 11);
}

#[test]
pub(self) fn test_arr1_deref() {
    let arr1 = Arr1::from(vec![0.0; 10]);
    assert_eq!(&0.0, &(*arr1)[0]);
    // test deref coercion as well
    assert_eq!(&0.0, &arr1[0]);
}

#[test]
pub(self) fn test_arr1_get() {
    let arr1 = Arr1::from(vec![(0.0); 10]);
    assert_eq!(Some(&0.0), arr1.get(&[0]));
    assert_eq!(Some(&0.0), arr1.get(&[2]));
    assert_eq!(Some(&0.0), arr1.get(&[0]));
}

#[test]
pub(self) fn test_arr1_distance() {
    // immitating vec2 for legibility
    let arr_0 = Arr1::from(vec![-1.0, 0.0]);
    let arr_1 = Arr1::from(vec![0.0, -0.0]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!((1.0_f32), dist);

    let arr_0 = Arr1::from(vec![3.0, 4.0]);
    let arr_1 = Arr1::from(vec![3.0, 0.0]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!((4.0_f32), dist);

    let arr_0 = Arr1::from(vec![3.0, 4.0]);
    let arr_1 = Arr1::from(vec![4.0, 3.0]);
    let dist = arr_0.distance(&arr_1);
    assert_eq!((2.0_f32).sqrt(), dist);
}
