import json
import numbers
from pathlib import Path
from tempfile import NamedTemporaryFile
from typing import Any

import pytest

import json_sum


def _py_sum_json(value: Any, search_key: str) -> float:
    """Recursively sum numeric values for `search_key`."""
    if isinstance(value, dict):
        return sum(
            [
                value
                if key == search_key and isinstance(value, numbers.Number)
                else _py_sum_json(value, search_key)
                for key, value in value.items()
            ]
        )
    elif isinstance(value, list):
        return sum([_py_sum_json(value, search_key) for value in value])
    else:
        return 0.0


def py_sum_json(path: str, key: str) -> float:
    """Open and parse a JSON file and sum numeric values for `key`."""
    with open(path, "r") as f:
        return _py_sum_json(json.load(f), key)


@pytest.mark.parametrize("func", (py_sum_json, json_sum.sum))
def test_bechmark(benchmark, func):
    """Benchmark the native json_sum against the same algorithm in Python."""
    assert (
        benchmark(
            func, Path(__file__).with_name("data.json").as_posix(), "interesting_value"
        )
        == 1273721.0
    )


def test_non_existent_file():
    """Ensure a non-existent file raises an OSError."""
    with pytest.raises(OSError, match="No such file or directory"):
        json_sum.sum("unknown.json", "key")


def test_invalid_json():
    """Ensure an invalid file raises an ValueError."""
    with NamedTemporaryFile() as file:
        file.write(b'{"broken": {')
        file.flush()
        with pytest.raises(ValueError, match="Syntax at character 11"):
            json_sum.sum(file.name, "key")
