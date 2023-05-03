fn main() {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/routes/gen")
        .compile(&["proto/hello.proto", "proto/world.proto"], &["/proto"])
        .unwrap();
}
