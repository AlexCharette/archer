extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src")
        .inputs(&[
            "../protos/account.proto",
            "../protos/merchant.proto",
            "../protos/payload.proto",
        ])
        .include("../protos")
        .run()
        .expect("protoc");
}
