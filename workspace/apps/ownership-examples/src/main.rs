struct SensorReading {
    value: u16,
    timestamp_ms: u32,
}

//------------------------------------------------------------------------------
// 1. Each value in Rust has an owner.

fn demo_ownership() {
    let reading = SensorReading {value: 1, timestamp_ms: 100};

    println!("{}", reading.value);
    println!("{}", reading.timestamp_ms);
}

//------------------------------------------------------------------------------
// 2. There can only be one owner at a time.

fn demo_one_owner() {
    let reading = SensorReading {value: 2, timestamp_ms: 100};

    // Transfer (move) ownership
    let new_owner = reading;

    // Error: borrow of moved value: `reading`
    // println!("{}", reading.value);
    // println!("{}", reading.timestamp_ms);

    // This works
    println!("{}", new_owner.value);
    println!("{}", new_owner.timestamp_ms);
}

fn demo_copy() {
    let my_array = [1, 1, 2, 3, 5, 8];

    // Primitives and arrays implement the Copy trait
    let my_copy = my_array;

    // Both of these work
    println!("{:?}", my_array);
    println!("{:?}", my_copy);
}

//------------------------------------------------------------------------------
// 3. When the owner goes out of scope, the value will be dropped.

// fn print_reading(reading: SensorReading) {
//     // Ownership of reading is "consumed" by this function
//     println!("{}", reading.value);
//     println!("{}", reading.timestamp_ms);
//     // reading goes out of scope, so the value is dropped here
// }

fn print_reading(reading: SensorReading) -> SensorReading {
    // Ownership of reading is "consumed" by this function
    println!("{}", reading.value);
    println!("{}", reading.timestamp_ms);
    
    // Fix: return reading (shorthand: `reading` without a semicolon)
    return reading;
}

fn demo_scope_drop_value() {
    // New scope
    {
        let mut reading = SensorReading {value: 3, timestamp_ms: 100};

        // Error: borrow of moved value: `reading`
        // print_reading(reading);

        // Fix: return reading ownership
        reading = print_reading(reading);

        println!("{}", reading.value);
        println!("{}", reading.timestamp_ms);
    }

    // Error: cannot find value `reading` in this scope
    // println!("{}", reading.value);
    // println!("{}", reading.timestamp_ms);
}

//------------------------------------------------------------------------------
// 4. You can have either one mutable reference or any number of immutable references.



//------------------------------------------------------------------------------
// Main

fn main() {
    demo_ownership();
    demo_one_owner();
    demo_copy();
    demo_scope_drop_value();
}
