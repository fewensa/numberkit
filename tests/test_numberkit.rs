use numberkit;

#[test]
fn test_is_number() {
  assert_eq!(true, numberkit::is_number("0"));
  assert_eq!(false, numberkit::is_number("-1u32"));
  assert_eq!(true, numberkit::is_number("2usize"));
  assert_eq!(true, numberkit::is_number("3.5f32"));
  assert_eq!(false, numberkit::is_number("0.2.1f32"));
}

#[test]
fn test_is_digit() {
  assert_eq!(true, numberkit::is_digit("2", false));
  assert_eq!(false, numberkit::is_digit("-2", false));
  assert_eq!(false, numberkit::is_digit("0.2", false));
  assert_eq!(true, numberkit::is_idigit("-2"));
  assert_eq!(true, numberkit::is_udigit("2"));
}


#[test]
fn test_as() {
  assert_eq!(1 as isize, numberkit::as_isize("1").unwrap());
  assert_eq!(-1 as isize, numberkit::as_isized("a", -1 as isize));
  assert_eq!(1 as usize, numberkit::as_usize("1").unwrap());
  assert_eq!(52 as i128, numberkit::as_i128("52").unwrap());
  assert_eq!(0.5 as f64, numberkit::as_f64("0.5").unwrap());
  assert_eq!(0.5 as f64, numberkit::as_f64d("0.5", 0.5 as f64));
}
