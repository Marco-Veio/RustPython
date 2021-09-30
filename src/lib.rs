#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};
use serde_json::Value as JSONValue;

fn read(_py: Python, val: &str) -> PyResult<String> {
  let data: JSONValue = serde_json::from_str(val).unwrap();
  let test = data["testing"].to_string();
  return Ok(test);
}

py_module_initializer!(librust, initlibrust, PyInit_librust, |py, m| {
  m.add(py, "__doc__", "This module is implemented in Rust")?;
  m.add(py, "read", py_fn!(py, read(val: &str)))?;
  Ok(())
});
