# resultish

A `Resultish` represents success (`Ok`), error (`Err`), or `Both`.
It can be converted into a `Result`:
- `lenient`ly, where `Both` is mapped to `Ok`, and the
  error value is discarded.
- `strict`ly, where `Both` is mapped to `Err`, and the
  success value is discarded.
