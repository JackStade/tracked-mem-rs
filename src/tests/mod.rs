use SafeUninitializedVec;

#[test]
fn test_uninit_vec() {
    let mut vec = SafeUninitializedVec::new(4);
    vec.set_value(2, 4);
    vec.set_value(1, 2);
    vec.set_value(0, 1);
    vec.set_value(2, 3);
    vec.set_value(3, 4);
    assert_eq!(vec.into_vec().unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn test_uninit_vec_drop() {
    let mut vec = SafeUninitializedVec::new(4);
    vec.set_value(3, vec![0, 1, 2]);
    vec.set_value(1, vec![3, 4, 5]);
    // drop the vec. This theoretically aborts if there is an issue with drop
}

#[test]
fn test_uninit_vec_fail() {
    let mut vec = SafeUninitializedVec::new(4);
    vec.set_value(3, 2);
    vec.set_value(1, 5);
    if let Ok(_) = vec.into_vec() {
        panic!("Retured a value that contains unintialized data!");
    }
}

#[test]
fn test_uninit_vec_take() {
    let mut vec = SafeUninitializedVec::from_vec(vec![1, 2, 3, 4, 5, 6], 4);
    let _three = vec.take(2);
    if let Ok(_) = vec.into_vec() {
        panic!("Returned a value that contains unintialized data!");
    }
}

#[test]
fn test_uninit_vec_from() {
    let mut vec = SafeUninitializedVec::from_vec(vec![1, 2], 4);
    vec.set_value(2, 3);
    vec.set_value(3, 4);
    assert_eq!(vec.into_vec().unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn test_uninit_vec_overcapacity() {
    let mut vec = SafeUninitializedVec::from_vec(vec![1, 2, 2, 4, 5, 6], 4);
    vec.set_value(2, 3);
    assert_eq!(vec.into_vec().unwrap(), vec![1, 2, 3, 4, 5, 6]);
    let mut vec = SafeUninitializedVec::from_vec(vec![1, 2, 3, 4, 5, 6], 4);
    let _three = vec.take(2);
    if let Ok(_) = vec.into_vec() {
        panic!("Returned a value that contains unintialized data");
    }
    let mut vec =
        SafeUninitializedVec::from_vec(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], 2);
    let _take = vec.take(1);
    // drop the vec
}
