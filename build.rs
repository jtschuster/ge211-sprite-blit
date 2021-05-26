fn main() {
    println!("cargo:rustc-link-search=[KIND=dependency]./deps/");
    eprintln!("asdfs{:#?}", 
        cc::Build::new()
            .file("deps/add.c")
            .no_default_flags(true)
            .flag("-s EXPORTED_FUNCTIONS='[\"_add\"]'")
            .get_compiler());
    cc::Build::new()
        .file("deps/add.c")
        // .no_default_flags(true)
        // .flag("-s EXPORTED_FUNCTIONS='[\"_add\"]'")
        .compile("add");
}