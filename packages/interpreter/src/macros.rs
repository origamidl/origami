#[macro_export]
macro_rules! bail {
  ($err:expr $(,)?) => {
    return Err($err)
  }
}
