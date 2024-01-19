// Needed to get encase to work with Rust Nightly toolchain
#![feature(trivial_bounds)]
fn main() {
    pollster::block_on(voxel_animator::graphics::run());
}
