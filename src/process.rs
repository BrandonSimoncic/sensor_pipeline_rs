use crate::sensor::SensorData;

/// Processor trait: transforms sensor data in a pipeline (order matters).
pub trait Processor {
    fn process(&mut self, data: SensorData) -> SensorData;
}

/// Pipeline that runs data through a sequence of processors.
pub struct ProcessorPipeline {
    processors: Vec<Box<dyn Processor>>,
}

impl ProcessorPipeline {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn Processor>) {
        self.processors.push(processor);
    }

    /// Runs data through all processors in order and returns the result.
    pub fn run(&mut self, data: SensorData) -> SensorData {
        let mut current = data;
        for p in self.processors.iter_mut() {
            current = p.process(current);
        }
        current
    }
}

impl Default for ProcessorPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Low-pass filter: smoothed value = alpha * new + (1 - alpha) * previous.
pub struct LowPassFilter {
    alpha: f64,
    last_value: f64,
}

impl LowPassFilter {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha,
            last_value: 0.0,
        }
    }
}

impl Processor for LowPassFilter {
    fn process(&mut self, data: SensorData) -> SensorData {
        self.last_value = self.alpha * data.value + (1.0 - self.alpha) * self.last_value;
        let mut result = data;
        result.value = self.last_value;
        result
    }
}

/// Marks data as invalid when value is outside [min, max].
pub struct ThresholdValidator {
    min: f64,
    max: f64,
}

impl ThresholdValidator {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
}

impl Processor for ThresholdValidator {
    fn process(&mut self, data: SensorData) -> SensorData {
        let mut result = data;
        result.valid = result.value >= self.min && result.value <= self.max;
        result
    }
}
