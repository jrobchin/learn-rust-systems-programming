fn main() {
    let needle = 0o52;
    let haystack = [1, 2, 3, 4, 3, 42, 345, 1231, 654, 3332];
    
    // for item in haystack.iter() {
    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }

    // Ownership
    for item in haystack {
        println!("{}", item);
    }

    // Read-only
    for item in &haystack {
        println!("{}", item);
    }

    // Read-write unavailable for an immutable variable (declared with let)
    // for item in &mut haystack {
    //     *item = *item + 1;
    // }

    let mut collection = [1, 3, 6, 2, 6, 4, 2354, 7543];
    // Read-write
    for item in &mut collection {
        *item = *item * 2;
    }
    for item in collection {
        println!("{}", item);
    }
}