use chrono::{DateTime, Utc};

/// Shared data produced by any sensor.

#[derive(Debug, Clone)]
pub struct SensorData {
    pub sensor_id: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub valid: bool,
}

/// IMU sensor.
struct Imu {
    data: SensorData
}

/// Range sensor.
struct Range {
    data: SensorData
}

pub trait Sensor {

    fn update(&mut self) -> SensorData{
        let update = self.read_raw();
        self.apply_calibration(update)
    }
    fn read_raw(&mut self) -> SensorData; 
    fn apply_calibration(&self, data: SensorData) -> SensorData{
        data
    }   
}


impl Imu {
    fn new() -> Imu {
        Imu {
            data: SensorData {
                sensor_id: String::from("IMU_data"),
                value: 0.0,
                timestamp: Utc::now(),
                valid: true,
            },
        }
    }
}


impl Sensor for Imu {
    fn update(&mut self) -> SensorData{
        let update= self.read_raw();
        self.apply_calibration(update)
    }
    fn read_raw(&mut self) -> SensorData{
        self.data.sensor_id = String::from("IMU_data");
        self.data.timestamp = Utc::now();
        self.data.valid = true;
        self.data.value = 0.0;
        self.data.clone()
    }
    fn apply_calibration(&self, data: SensorData) -> SensorData {
        let mut temp =  data;
        temp.value = temp.value + 1.0;
        temp
    }
}

impl Range {
    fn new() -> Range {
        Range {
            data: SensorData {
                sensor_id: String::from("Range_data"),
                value: 0.0,
                timestamp: Utc::now(),
                valid: true,
            },
        }
    }
}


impl Sensor for Range {
    fn update(&mut self) -> SensorData{
        let update = self.read_raw();
        self.apply_calibration(update)
    }
    fn read_raw(&mut self) -> SensorData{
        self.data.sensor_id = String::from("Range_data");
        self.data.timestamp = Utc::now();
        self.data.valid = true;
        self.data.value = 0.0;
        self.data.clone()
    }
    fn apply_calibration(&self, data: SensorData) -> SensorData {
        let mut temp =  data;
        temp.value = temp.value + 1.0;
        temp
    }
}

pub struct SensorFactory {}

impl SensorFactory {
    pub fn create(t: &str) -> Box<dyn Sensor> {
        match t {
            "ImuSensor" => Box::new(Imu::new()),
            "RangeSensor" => Box::new(Range::new()),
            _ => panic!("unknown sensor type"),
        }
    }
}