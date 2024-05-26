fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/style.rs")
        .input_extern_file("src/style_enums.rs")
        .input_extern_file("src/tree.rs")
        .input_extern_file("src/value.rs")
        .input_extern_file("src/error.rs")
        .csharp_dll_name("ctaffy")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Taffy.Native")
         //.csharp_imported_namespaces("MyLib")
        /* .csharp_type_rename(|rust_type_name| match rust_type_name {     // optional, default: `|x| x`
            //"FfiConfiguration" => "Configuration".into(),
            _ => x,
        })*/
        .generate_csharp_file("../../../PrimroseEngine/Engine/Source/Taffy.g.cs")
       
        .unwrap();
}