#[derive(Debug)]
pub struct Planet{
    name: String,
    size: i32,
    dist: i32,
}

impl Planet{
    pub fn new(name: String, size: i32, dist: i32) -> Planet {
        Planet{
            name: String::from(name),
            size: size,
            dist: dist,
        }
    }
}

#[derive(Debug)]
pub struct SolarSystem{
    pub planets : [Planet; 8],
    pub star : Planet,
}

impl SolarSystem{
    pub fn new() -> SolarSystem {
        SolarSystem{ 
            planets: [
                Planet::new("Mercury".to_string(), 2_440, 46),
                Planet::new("Venus".to_string(), 6_051, 107),
                Planet::new("Earth".to_string(), 6_378, 147),
                Planet::new("Mars".to_string(), 3_396, 207),
                Planet::new("Jupiter".to_string(), 69_911, 741),
                Planet::new("Saturne".to_string(), 58_232, 1_350),
                Planet::new("Uranus".to_string(), 25_362, 2_735),
                Planet::new("Neptune".to_string(), 24_622, 4_459)
            ],
            star: Planet::new("Sun".to_string(),2,2),
        }
    }
}

// Mercury
// Venus
// Earth
// Mars
// Jupiter
// Saturn
// Uranus
// Neptune