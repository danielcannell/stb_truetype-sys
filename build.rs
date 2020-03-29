fn main() {
    cc::Build::new()
        .file("vendor/stb_truetype.c")
        .compile("stb_truetype");
}
