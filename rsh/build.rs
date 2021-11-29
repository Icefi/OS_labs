 extern crate cc;

 fn main () {
    cc::Build::new()
        .file("src/rsh_c_module.c")
        .compile("lib_rsh_c_module");
 }
