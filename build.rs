extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::Codegen::new()
    .out_dir("src/protobuf")
    .inputs(&["protos/account.proto", "protos/payload.proto"])
    .include("protos")
    .run()
    .expect("protoc");
}
