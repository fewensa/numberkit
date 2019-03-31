use std::ffi::OsStr;

pub fn is_number<S>(text: S) -> bool where S: AsRef<OsStr> {
  let chars = self::to_chars(text);
  let mut has_dot = false;
  let mut has_negative = false;
  for (ix, &ch) in chars.iter().enumerate() {
    if ch == '-' && ix == 0 {
      has_negative = true;
      continue;
    }
    if ch == '.' {
      if has_dot { return false; }
      has_dot = true;
      continue;
    }
    if ch == 'f' || ch == 'u' || ch == 'i' {
      // fixme: check like 0.2f32   3u32 and more
    }
    if self::char_is_digit(ch) { continue; }
    return false;
  }
  // fixme: check like 0.2f32   3u32 and more
  true
}

pub fn is_udigit<S>(text: S) -> bool where S: AsRef<OsStr> {
  self::is_digit(text, false)
}

pub fn is_idigit<S>(text: S) -> bool where S: AsRef<OsStr> {
  self::is_digit(text, true)
}

fn is_digit<S>(text: S, negative: bool) -> bool where S: AsRef<OsStr> {
  let chars = self::to_chars(text);
  for (ix, &ch) in chars.iter().enumerate() {
    if negative && ch == '-' && ix == 0 { continue; }
    if self::char_is_digit(ch) { continue; }
    return false;
  }
  true
}

fn to_chars<S>(text: S) -> Vec<char> where S: AsRef<OsStr> {
  text.as_ref().to_str().unwrap().chars().collect()
}

fn char_is_digit(ch: char) -> bool {
  ch == '0' ||
    ch == '1' ||
    ch == '2' ||
    ch == '3' ||
    ch == '4' ||
    ch == '5' ||
    ch == '6' ||
    ch == '7' ||
    ch == '8' ||
    ch == '9'
}
