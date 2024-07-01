fn main() {
    #[cfg(feature = "ffi")]
    {
        cxx_build::bridge("src/ffi.rs")
            .include("src/include/special")
            .flag_if_supported("-std=c++11")
            .compile("t_ffi")
    }
}
