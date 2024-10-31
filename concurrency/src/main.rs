use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..9 {
            println!("The code inside the spawn {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..6 {
        println!("Hello from for loop {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("++++++++++++++++++++++ Outside of for loop ++++++++++++++++++++++");
    handle.join().unwrap();

    let vector = vec![1, 2, 3];
    let thread_handle = thread::spawn(move || {
        println!("The vector print {:?}", vector);
    });

    thread_handle.join().unwrap();
}

// --  The output from this program might be a little different every time in above code.
// --  Hear both loops are running parlelly.
// --  The threads will probably take turns, but that isn’t guaranteed: it depends on how your
//     operating system schedules the threads.
// -- join method will makesure spawn thread is finished before main exits.
