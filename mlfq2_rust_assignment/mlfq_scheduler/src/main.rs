// Importing the mlfq module, which contains the MLFQ and Process structs
mod mlfq;

fn main() {
    // Create a new MLFQ scheduler with 3 levels and time slices of 2, 4, and 8 units respectively
    let mut scheduler = mlfq::MLFQ::new(3, vec![2, 4, 8]);

    // Adding processes to the scheduler with their respective attributes
    scheduler.add_process(mlfq::Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 });
    scheduler.add_process(mlfq::Process { id: 2, priority: 0, remaining_time: 3, total_executed_time: 0 });
    scheduler.add_process(mlfq::Process { id: 3, priority: 1, remaining_time: 5, total_executed_time: 0 });

    // Iterate through each queue level in the scheduler
    for queue_index in 0..scheduler.num_levels {
        // While there are processes in the current queue level
        while !scheduler.queues[queue_index].is_empty() {
            // Execute the next process in the queue
            scheduler.execute_process(queue_index);
        }
    }

    // Update the scheduler's time after processing
    scheduler.update_time(100);

    // Print the state of each queue in the scheduler
    for (index, queue) in scheduler.queues.iter().enumerate() {
        println!("Queue {}: {:?}", index, queue);
    }
}

