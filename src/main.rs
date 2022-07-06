use std::{thread, time};
use std::io::Read;

mod world;
use world::World;

mod bugs;
use bugs::Bug;
use bugs::BugType;

fn main()
{

    //let mut a: [bugs::Bugs; 2] = [bugs::Bugs::Empty; 2];
    let mut world = World::new();
    world.update_index(11, Bug::new(BugType::Prey));
    world.update_index(22, Bug::new(BugType::Predator));

    let sleep_time = time::Duration::from_millis(1000);
    for i in 0..10
    {
        world.print();
        world.cycle();

let input: Option<i32> = std::io::stdin()
    .bytes() 
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as i32);

//        thread::sleep(sleep_time);
    }
}
