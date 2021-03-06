pub trait Vehicle {
    fn get_price(&self) -> u64;
}

pub trait Car: Vehicle {
    fn model(&self) -> String;
}

pub struct TeslaRoadcast {
    model: String,
    release_date: u16,
}

impl TeslaRoadcast {
    pub fn new(model: &str, release_date: u16) -> Self {
        Self { model:model.to_string(), release_date}
    }
}

impl Car for TeslaRoadcast {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}

// 由于 Car 继承了 Vehicel
// 叵不 impl Vehicel
// impl Car for TeslaRoadcast 报错
// the trait `Vehicle` is not implemented for `TeslaRoadcast`

impl Vehicle for TeslaRoadcast {
    fn get_price(&self) -> u64 {
        200_000 
    }
}