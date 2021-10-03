# better_input
> Efficient high-speed improvement of Python's input systems (using Rust)

[📖 **Release Notes**](./CHANGELOG.md)

## Features

- Flexible input function
  - Prefixes and suffixes
  - Masking
  - Allow specific characters
- Selection (combobox)
  - Select between multiple options
- Spinbox

## Setup

### PyPi
1. Install `better_input` from PyPi:  
   `python -m pip install better_input`
2. Import it into your projects  
   `import better_input`  
   or alternatively,  
   `from better_input import *`

### Manual

- Clone this repository
- Install the Python module `maturin` (preferably in a venv)  
  `python -m pip install maturin`
- Develop the project  
  `maturin develop`
- Import and use the module  
  `import better_input`
- Build the project  
  `maturin build`

Wheels are located at `taret/wheels`

## Examples

### Input

Note that it is safe to override Python's built-in input function, it will not break code.
```Python
from better_input import input
import string

pw = input(prefix="Enter password: ", mask="•")
print(f"Password: {pw}")

age = input(prefix="Age: ", suffix=" years old", allow=string.digits)
print(f"Age: {age}")
```

### Selection

```Python
from better_input import selection

grade = selection(
    ["A*", "A", "B", "C", "D", "E", "F", "U"], 
    prefix="Select grade:", 
    suffix="Student 4/30"
)

print(f"Selected grade: {grade}")
```

### Spinbox
```Python
from better_input import spinbox

minutes = spinbox(
    0,
    3600,
    prefix="Minutes: ",
    suffix=" mins",
    step=30,
)

print(f"Minutes: {minutes}")
```

## Built with
- [Rust](https://www.rust-lang.org/)
- [Python](https://www.python.org/)
- [pyo3](https://pyo3.rs/)
- [crossterm](https://github.com/crossterm-rs/crossterm)

## License

[MIT License](./LICENSE)