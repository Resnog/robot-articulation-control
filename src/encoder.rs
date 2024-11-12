pub mod encoder {
    struct Encoder {
        sampleRate: u32,
        resolution: u32,
        startinPose: u32,
        noise: u32,
        count: u32,
        counter: u32, // TODO: Change for an actual peripheral
    }

    impl Encoder {
        pub fn resetCounter(self) {
            todo!();
        }
        pub fn getCount(self) -> u32 {
            self.count
        }
    }
}
