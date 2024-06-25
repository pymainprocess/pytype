# pytype

What is pytype.

Pytype is an pyo3 extension module for rust, with many Types for Python Extension Module workings.

For Example

```rust
use pytype::{PyInt, PyFloat, PyList, PyDict};
use pyo3::prelude::*;

/// Not to the Differences
/// Maybe you make Following Function
#[pyfunction]
fn add(a: PyFloat, b: PyFloat) -> PyResult<PyFloat> {
    Ok(a + b)
}
```

The Thing is, generally is the Type an normal Rust type, but with Better Working.

So is PyInt different on 32 bit arches and 64 bit arches.

32 bit
------

```rust
#[cfg(target_point_width = "32")]
pub type PyInt = i32;
```

64 bit
------

```rust
#[cfg(target_point_width = "64")]
pub type PyInt = i64;
```

Why you want do that?

Generally you want a Best Perfomance Python Module with best work on any arches.

'i64' can be very slower on 32 bit arches, but 'i32' is very fast on there.
64 bit arches can be work better with 'i64' and will work with that.

The Same is on PyFloat.

32 bit
------

```rust
#[cfg(target_point_width = "32")]
pub type PyFloat = f32;
```

64 bit
------

```rust
#[cfg(target_point_width = "64")]
pub type PyFloat = f64;
```