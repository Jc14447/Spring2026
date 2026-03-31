//Write a closure named update inside a function track_changes.
//The closure should increment and print a counter each time it
//is called.

fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Current count: {}", tracker)
        // Your implementation here
    };

    update();
    update();
}

fn main() {
    track_changes();
}