fn main() {
    cc::Build::new()
        .file("src/problems/coco/suits/bbob/legacy_code.c")
        .compile("bbob_legacy");
}
