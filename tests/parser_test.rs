use lut_parser::{parse_cube_lut, CubeLut, LutKind, LutError};
use num_traits::{Float};
#[test]
fn it_errors_size_line() {
    let res = parse_cube_lut::<f64>(include_str!("data/size-line-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::SizeLine);
}
#[test]
fn it_errors_title() {
    let res = parse_cube_lut::<f64>(include_str!("data/title-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::Title);
}
#[test]
fn it_errors_data_line() {
    let res = parse_cube_lut::<f64>(include_str!("data/data-line-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataLine(5));
}

#[test]
fn it_errors_data_red() {
    let res = parse_cube_lut::<f64>(include_str!("data/data-red-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataRed(5));
}

#[test]
fn it_errors_data_green() {
    let res = parse_cube_lut::<f64>(include_str!("data/data-green-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataGreen(5));
}

#[test]
fn it_errors_data_blue() {
    let res = parse_cube_lut::<f64>(include_str!("data/data-blue-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DataBlue(5));
}

#[test]
fn it_errors_domain_min() {
    let res = parse_cube_lut::<f64>(include_str!("data/domain-min-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DomainMin(2));
}

#[test]
fn it_errors_domain_max() {
    let res = parse_cube_lut::<f64>(include_str!("data/domain-max-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::DomainMax(3));
}

#[test]
fn it_errors_size_value() {
    let res = parse_cube_lut::<f64>(include_str!("data/size-value-fail.cube")).unwrap_err();
    assert_eq!(res, LutError::Size(1));
}

#[test]
fn it_succeeds() {
    let res = parse_cube_lut::<f64>(include_str!("data/success.cube")).unwrap();
    check_res(res);
    let res = parse_cube_lut::<f32>(include_str!("data/success.cube")).unwrap();
    check_res(res);
}

fn check_res<T: Float + std::fmt::Debug>(res: CubeLut<T>) {
    assert_eq!(res.title, "\"Just for testing\"");
    assert_eq!(res.kind, LutKind::Three); 
    assert_eq!(res.domain_min[1], T::from(2.0).unwrap()); 
    assert_eq!(res.domain_max[2], T::from(3.0).unwrap()); 
    assert_eq!(res.size, 32); 
    assert_eq!(res.data[1][1], T::from(0.42).unwrap()); 
}
