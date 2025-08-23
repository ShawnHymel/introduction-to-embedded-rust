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
    
    // Fix: return reading (shorthand for "return reading;")
    reading
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

fn print_borrowed_reading(reading: &SensorReading) {
    // We borrow reading instead of consuming ownership
    println!("{}", reading.value);
    println!("{}", reading.timestamp_ms);
}

fn demo_mutable_references() {
    let mut reading = SensorReading {value: 4, timestamp_ms: 100};

    // References are easier than consuming and returing ownership
    print_borrowed_reading(&reading);

    // Can have any number of immutable references
    let immut_ref_1 = &reading;
    let immut_ref_2 = &reading;
    let immut_ref_3 = &reading;
    println!("{}", (*immut_ref_1).timestamp_ms);    // Explicit dereference
    println!("{}", immut_ref_2.timestamp_ms);       // Automatic dereference
    println!("{}", immut_ref_3.timestamp_ms);
    // immut_refs are no longer used, so they go out of scope

    // Only one mutable reference at a time
    let mut_ref_1 = &mut reading;

    // Error: cannot borrow `reading` as mutable more than once at a time
    // let mut_ref_2 = &mut reading;
    // println!("{}", mut_ref_2.timestamp_ms);

    // Error: cannot borrow `reading` as immutable because it is also borrowed as mutable
    // let immut_ref_4 = &reading;
    // println!("{}", immut_ref_4.timestamp_ms);

    // Change value in struct through the mutable reference
    mut_ref_1.timestamp_ms = 1000;
    println!("{}", reading.timestamp_ms);
    // mut_ref_1 is no longer used, so it goes out of scope
    
    // Now we can borrow again!
    let mut_ref_3 = &mut reading;
    mut_ref_3.timestamp_ms = 2000;
    println!("{}", reading.timestamp_ms);
}

//------------------------------------------------------------------------------
// 5. References must always be valid.

// Error: cannot return reference to local variable `some_reading`
// fn return_reading() -> &SensorReading {
//     let some_reading = SensorReading {value: 5, timestamp_ms: 100};
//     &some_reading
// }

// Fix: return value with full ownership
fn return_reading() -> SensorReading {
    let some_reading = SensorReading {value: 5, timestamp_ms: 100};
    some_reading
}

fn demo_valid_references() {
    let reading = return_reading();
    println!("{}", reading.value);
    println!("{}", reading.timestamp_ms);
}

//------------------------------------------------------------------------------
// 6. If you move out part of a value, you cannot use the whole value anymore

fn demo_partial_move() {
    let my_tuple = (
        SensorReading {value: 6, timestamp_ms: 100},
        SensorReading {value: 6, timestamp_ms: 101},
    );

    // Partially move ownership
    let first_reading = my_tuple.0;

    // Error: borrow of moved value: `my_tuple.0`
    // println!("{}", my_tuple.0.value);
    
    // Error: use of partially moved value: `my_tuple`
    // let new_owner = my_tuple;
    // println!("{}", new_owner.1.value);
    
    // Can print new owner
    println!("{}", first_reading.value);

    // Can move and borrow other parts
    println!("{}", my_tuple.1.value);
}

//------------------------------------------------------------------------------
// 7. Slices are references to the whole value and follow the same borrow rules

fn demo_slices() {
    let my_array = [
        SensorReading {value: 7, timestamp_ms: 100},
        SensorReading {value: 7, timestamp_ms: 101},
        SensorReading {value: 7, timestamp_ms: 102},
    ];

    // Create a slice (section of the array), borrows all of my_array immutably
    let slice_1 = &my_array[0..1];

    // Error: cannot borrow `my_array` as mutable because it is also borrowed as immutable
    // let slice_2 = &mut my_array[1..3];

    // Fix: we can have multiple immutable references
    let slice_2 = &my_array[1..3];

    // Print out some of our slices
    println!("{}", slice_1[0].value);
    println!("{}", slice_2[0].timestamp_ms);
    println!("{}", slice_2[1].timestamp_ms);
}

//------------------------------------------------------------------------------
// Main

fn main() {
    demo_ownership();
    demo_one_owner();
    demo_copy();
    demo_scope_drop_value();
    demo_mutable_references();
    demo_valid_references();
    demo_partial_move();
    demo_slices();
}
