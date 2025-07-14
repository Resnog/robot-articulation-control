pub mod encoder {
    struct Encoder {
        sample_rate: u32,
        resolution: u32,
        startin_pose: u32,
        noise: u32,
        count: u32,
        counter: u32, // TODO: Change for an actual peripheral
    }

    impl Encoder {
        pub fn reset_counter(self) {
            todo!();
        }
        pub fn get_count(self) -> u32 {
            self.count
        }
    }
}
