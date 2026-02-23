mod process;
mod sensor;

use crate::process::{LowPassFilter, ProcessorPipeline, ThresholdValidator};
use crate::sensor::{Sensor, SensorFactory};

fn main() {
    // Build sensors via factory (caller never touches concrete types)
    let mut sensors: Vec<Box<dyn Sensor>> = vec![
        SensorFactory::create("ImuSensor"),
        SensorFactory::create("RangeSensor"),
    ];

    // Build pipeline — order matters: filter then validate
    let mut pipeline = ProcessorPipeline::new();
    pipeline.add_processor(Box::new(LowPassFilter::new(0.1)));
    pipeline.add_processor(Box::new(ThresholdValidator::new(-100.0, 100.0)));

    // Run: sensor -> raw data -> pipeline -> processed data
    for sensor in sensors.iter_mut() {
        let raw = sensor.update();
        println!("{} raw value={}", raw.sensor_id, raw.value);
        let processed = pipeline.run(raw);
        println!("  processed: valid={} value={}", processed.valid, processed.value);
    }
}