struct SensorReading {
    value: u16,
    timestamp_ms: u32,
}

//------------------------------------------------------------------------------
// 1. Function lifetime annotations

// Explicit: return a reference that lives as long as the input
// fn add_one<'a>(reading: &'a mut SensorReading) -> &'a mut SensorReading {
//     reading.value += 1;
//     reading
// }

// Elision: compiler can infer lifetimes in common patterns
fn add_one(reading: &mut SensorReading) -> &mut SensorReading {
    reading.value += 1;
    reading
}

fn demo_adding() {
    let mut reading = SensorReading{ value: 1, timestamp_ms: 101 };

    let another_ref = add_one(&mut reading);

    println!("{}, {}", another_ref.value, another_ref.timestamp_ms);
}

//------------------------------------------------------------------------------
// 2. Required function lifetime annotations

// Error: missing lifetime specifier
// fn highest_val(r1: &SensorReading, r2: &SensorReading) -> &SensorReading {
//     if r1.value > r2.value {
//         r1
//     } else {
//         r2
//     }
// }

// Fix: provide lifetime annotations
fn highest_val<'a>(r1: &'a SensorReading, r2: &'a SensorReading) -> &'a SensorReading {
    if r1.value > r2.value {
        r1
    } else {
        r2
    }
}

fn demo_highest() {
    let reading_1 = SensorReading{ value: 5, timestamp_ms: 102 };
    let reading_2 = SensorReading{ value: 10, timestamp_ms: 102 };

    let highest = highest_val(&reading_1, &reading_2);

    println!("Highest: {}, {}", highest.value, highest.timestamp_ms);
}

//------------------------------------------------------------------------------
// 3. Struct lifetime annotations

// Error: missing lifetime specifier
// struct SampleHolder {
//     sample: &SensorReading,
// }

// Fix: add a lifetime annotation
struct SampleHolder<'a> {
    sample: &'a SensorReading,
}

fn demo_struct_ref() {
    let reading = SensorReading{ value: 11, timestamp_ms: 103 };

    let holder = SampleHolder{ sample: &reading };

    println!("{}, {}", holder.sample.value, holder.sample.timestamp_ms);
}

//------------------------------------------------------------------------------
// 4. Method annotations

struct SensorBuffer<'a> {
    name: &'a str,
    readings: &'a [SensorReading],
}

// Error: implicit elided lifetime not allowed here
// impl SensorBuffer {
//     fn new(name: & str, readings: &[SensorReading]) -> Self {
//         SensorBuffer { name, readings }
//     }

//     fn get_latest(&self) -> &SensorReading {
//         &self.readings[self.readings.len() - 1]
//     }
// }

// Fix: annotate
impl<'a> SensorBuffer<'a> {
    fn new(name: &'a str, readings: &'a [SensorReading]) -> Self {
        SensorBuffer { name, readings }
    }

    fn get_latest(&self) -> &'a SensorReading {
        &self.readings[self.readings.len() - 1]
    }
}

fn demo_methods() {
    let readings = [
        SensorReading{ value: 23, timestamp_ms: 104},
        SensorReading{ value: 25, timestamp_ms: 204},
        SensorReading{ value: 21, timestamp_ms: 304},
    ];

    let buffer = SensorBuffer::new("My Readings", &readings);
    let latest = buffer.get_latest();

    println!("{} latest: {}, {}", buffer.name, latest.value, latest.timestamp_ms);
}

//------------------------------------------------------------------------------
// 5. Static lifetime

// Works, but not clear: compiler infers that 'a is 'static
// struct SensorConfig<'a> {
//     name: &'a str,
//     units: &'a str,
// }

// Fix: add static lifetime annotation when we know the data lives forever
struct SensorConfig {
    name: &'static str,
    units: &'static str,
}

// const: inline at every use, static: one instance in memory
static TEMP_SENSOR_CONFIG: SensorConfig = SensorConfig {
    name: "Temperature Sensor",
    units: "C"
};

fn demo_static() {
    println!("{}, units: {}", TEMP_SENSOR_CONFIG.name, TEMP_SENSOR_CONFIG.units);
}

//------------------------------------------------------------------------------
// Main

fn main() {
    demo_adding();
    demo_highest();
    demo_struct_ref();
    demo_methods();
    demo_static();
}