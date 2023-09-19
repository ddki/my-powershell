extern crate embed_resource;

fn main() {
  if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
    embed_resource::compile("build/manifest.rc", embed_resource::NONE);
  }
}