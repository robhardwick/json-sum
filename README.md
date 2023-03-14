# json-sum ![Build Status](https://github.com/robhardwick/json-sum/actions/workflows/CI.yml/badge.svg)

Example native Python module written in Rust that sums all numeric values for a given key from a JSON file.

## Guide

1. Create and activate a virtual environment:

```shell
python -m venv .env
source .env/bin/activate
```

2. Install `maturin`:

```shell
pip install maturin
```

3. Initialise the package (select `pyo3` as the bindings to use):

```shell
maturin init
```

4. Build the package and install it into the virtual environment:

```shell
maturin develop
```

5. Test out the example function:

```shell
python
>>> import json_sum
>>> json_sum.sum_as_string(5, 20)
'25'
```

6. Add some optional python dependencies to `pyproject.toml`:

```toml
[project.optional-dependencies]
test = [
  "black",
  "pytest",
  "pytest-benchmark"
]
```

7. Install the test dependencies:

```shell
pip install '.[test]'
```

8. Add some tests to `tests/test_json_sum.py`

9. Ensure the tests pass:

```shell
pytest
```

10. Add rust dependencies:

```shell
cargo add simd-json
```

11. Add build flags to `.cargo/config.toml`:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

12. Add an error wrapper in `error.rs`
13. Add our functionality to `sum_json.rs`
14. Add a new exported `sum` function
15. Add a new test to `tests/test_json_sum.py`
16. Compare the performance of the two solutions:

```shell
pytest
```
