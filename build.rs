fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/c_exports.rs")
        .csharp_dll_name("mini_syringe")
        .csharp_class_accessibility("public")
        .csharp_namespace("mini_syringe.Net.Sys")
        .generate_csharp_file("bindings/csharp/NativeMethods.g.cs")
        .unwrap();
}
