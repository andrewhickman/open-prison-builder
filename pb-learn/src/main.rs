use gym::Gym;

mod gym;

fn main() {
    let mut gym = Gym::new();
    gym.step();
}
