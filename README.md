# sensor_pipeline_rs

A small Rust example: **sensors** produce data, a **processing pipeline** runs that data through a sequence of **processors** (e.g. filter, validate). Uses traits, `Box<dyn Trait>`, and a factory so callers never depend on concrete sensor/processor types.

## Layout

- **`sensor.rs`** — `Sensor` trait, `SensorData`, concrete sensors (IMU, Range), `SensorFactory`
- **`process.rs`** — `Processor` trait, `ProcessorPipeline`, `LowPassFilter`, `ThresholdValidator`
- **`main.rs`** — builds sensors via factory, builds pipeline, runs sensor → pipeline and prints raw vs processed

## Build & run

From the workspace root:

```bash
cargo build -p sensor_pipeline_rs
cargo run -p sensor_pipeline_rs
```

From this directory:

```bash
cargo build
cargo run
```

## Example output

```
IMU_data raw value=1
  processed: valid=true value=0.1
Range_data raw value=1
  processed: valid=true value=0.19
```

## Adding a processor

Implement the `Processor` trait and push a `Box<dyn Processor>` into the pipeline with `add_processor`:

```rust
impl Processor for MyFilter {
    fn process(&mut self, data: SensorData) -> SensorData {
        // ...
    }
}
pipeline.add_processor(Box::new(MyFilter::new(...)));
```

## Dependencies

- `chrono` — timestamps in `SensorData`
