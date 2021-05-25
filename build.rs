fn main() {
    cc::Build::new()
        .file("deps/add.c")
        .compile("add");
}