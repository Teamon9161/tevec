fn main() {
    {
        cxx_build::bridge("src/lib.rs")
            .include("src/include/special")
            .flag_if_supported("-std=c++11")
            .compile("tea_ffi_cpp")
    }
}
