use std::time::Duration;

pub trait Runner {
    fn part_one(&self) -> u32;
    fn part_two(&self) -> u32;

    fn benchmark(&self) -> (u32, u32, Duration, Duration) {
        let start = std::time::Instant::now();
        let v1 = self.part_one();
        let f1 = start.elapsed();
        let start = std::time::Instant::now();
        let v2 = self.part_two();
        let f2 = start.elapsed();
        return (v1, v2, f1, f2)
    }
}