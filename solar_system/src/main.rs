mod planet;
use crate::planet::*;

fn main() {
    let system = SolarSystem::new();
    for planet in system.planets{
        println!("{:#?}", planet);
    }

}
