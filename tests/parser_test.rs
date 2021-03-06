use lut_parser::{CubeLut, LutKind, LutError};
use num_traits::{Float};
#[test]
fn it_errors_size_line() {
    let res = CubeLut::<f64>::from_str(include_str!("data/size-line-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::SizeLine);
}
#[test]
fn it_errors_title() {
    let res = CubeLut::<f64>::from_str(include_str!("data/title-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::Title);
}
#[test]
fn it_errors_data_line() {
    let res = CubeLut::<f64>::from_str(include_str!("data/data-line-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataLine(5));
}

#[test]
fn it_errors_data_red() {
    let res = CubeLut::<f64>::from_str(include_str!("data/data-red-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataRed(5));
}

#[test]
fn it_errors_data_green() {
    let res = CubeLut::<f64>::from_str(include_str!("data/data-green-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataGreen(5));
}

#[test]
fn it_errors_data_blue() {
    let res = CubeLut::<f64>::from_str(include_str!("data/data-blue-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataBlue(5));
}

#[test]
fn it_errors_domain_min() {
    let res = CubeLut::<f64>::from_str(include_str!("data/domain-min-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DomainMin(2));
}

#[test]
fn it_errors_domain_max() {
    let res = CubeLut::<f64>::from_str(include_str!("data/domain-max-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DomainMax(3));
}

#[test]
fn it_errors_size_value() {
    let res = CubeLut::<f64>::from_str(include_str!("data/size-value-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::Size(1));
}

#[test]
fn it_succeeds() {
    let res = CubeLut::<f64>::from_str(include_str!("data/success.cube")).unwrap();
    check_res(res);
    let res = CubeLut::<f32>::from_str(include_str!("data/success.cube")).unwrap();
    check_res(res);
}

#[test]
fn it_flattens() {
    let res = CubeLut::<f64>::from_str(include_str!("data/success.cube")).unwrap();
    let values = get_values::<f64>();

    for (i,n) in res.flatten_data().iter().enumerate() {
        assert_eq!(*n, values[i]);
    }
}
fn check_res<T: Float + std::fmt::Debug>(res: CubeLut<T>) {
    assert_eq!(res.title, "\"Just for testing\"");
    assert_eq!(res.kind, LutKind::Three); 
    assert_eq!(res.domain_min[1], T::from(2.0).unwrap()); 
    assert_eq!(res.domain_max[2], T::from(3.0).unwrap()); 
    assert_eq!(res.size, 32); 

    let values = get_values();

    for (x,row) in res.data.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            assert_eq!(*col, values[x * 3 + y]);
        }
    }
}

fn get_values<T: Float + std::fmt::Debug>() -> Vec<T> {
    vec![0.000000,0.000000,0.013947,0.002749,0.420000,0.011855,0.048985,0.000000, 0.009458].iter().map(|n| T::from(*n).unwrap()).collect::<Vec<T>>()
}
