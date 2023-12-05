use std::time::Duration;

pub trait Runner {
    fn part_one(&self) -> u32;
    fn part_two(&self) -> u32;

    fn benchmark(&self) -> (u32, u32, Duration) {
        let start = std::time::Instant::now();
        let p1 = self.part_one();
        let p2 = self.part_two();
        return (p1, p2, start.elapsed())
    }
}