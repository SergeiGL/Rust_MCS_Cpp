# Rust_MCS_Cpp

This repository demonstrates the MCS optimizer ([Rust_MCS](https://github.com/SergeiGL/Rust_MCS)) usage from the `C++` programming language.

## Getting Started

### Prerequisites

* **Rust:** Make sure you have Rust installed. If not, you can download it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
* **C++ Compiler and CMake:** You will need a C++ compiler (like g++ or MSVC) and CMake to build the C++ project.

### Building and Running the Example

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/SergeiGL/Rust_MCS_Cpp.git
    cd Rust_MCS_Cpp
    ```

2.  **Build the Rust code in `release` mode:**

    ```bash
    cargo build --release
    ```

3.  **Move Rust build artifacts:**

    Navigate to `\target\release\` and copy the generated files (`Rust_MCS_Cpp.*`)  into the `Cpp_sample_code/lib` folder.


4.  **Build the C++ project:**

    Navigate to the `Cpp_sample_code` directory and use CMake to build the C++ executable.

    ```bash
    cd Cpp_sample_code
    mkdir build
    cd build
    cmake ..
    cmake --build .
    ```

5.  **Launch the executable:**

    Find the executable `your_program` in the `build` directory. Double click or:
    
    ```bash
    # On Linux/macOS
    ./your_program

    # On Windows
    .\your_program.exe
    ```