Currently supported languages:
Python, JavaScript (node), C, C++, Java, Rust

How to use:
run `cargo build --release` to compile into an executable, then 
use `~/path/to/run file_name` to run the desired file in the current directory. 

On Linux I recommend moving run to `~/usr/bin`, so it can be used as a global command.

Ex: `run main.py`, `run hello_world.cpp`, `run Calculator.java -e foo`

Compiled C, C++ and Rust executables will default to "output" but can be altered with the `-o` flag, i.e. `run -o hello hello_world.c` will
resolve to `gcc -o hello hello_world.c`.

