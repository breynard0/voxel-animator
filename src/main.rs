fn main() {
    pollster::block_on(voxel_animator::graphics::run());
}
