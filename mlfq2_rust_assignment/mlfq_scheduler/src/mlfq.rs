// Add Debug trait to the Process struct for better formatting during printing
#[derive(Debug)]
pub struct Process {
    pub id: u32,                        // Unique identifier for the process
    pub priority: usize,                // Represents the current queue index for the process
    pub remaining_time: u32,            // Time left for the process to complete execution
    pub total_executed_time: u32,       // Total time the process has been executed
}

// Define the MLFQ scheduler structure
pub struct MLFQ {
    pub queues: Vec<Vec<Process>>,      // Vector of queues for each priority level
    pub num_levels: usize,               // Total number of priority levels
    time_quanta: Vec<u32>,               // Time slices for each priority level
    current_time: u32,                   // Tracks the current time in the scheduler
}

impl MLFQ {
    // Create a new MLFQ scheduler with specified levels and time quanta
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        // Initialize queues with an empty Vec for each priority level
        let queues = (0..num_levels).map(|_| Vec::new()).collect();

        MLFQ {
            queues,                             // Use the initialized queues
            num_levels,
            time_quanta,
            current_time: 0,                   // Start the current time at zero
        }
    }

    // Add a new process to the appropriate priority queue
    pub fn add_process(&mut self, process: Process) {
        let priority = process.priority;
        // Ensure the process is placed in a valid queue
        if priority < self.num_levels {
            self.queues[priority].push(process);
        } else {
            // If priority is too high, place it in the lowest priority queue
            self.queues[self.num_levels - 1].push(process);
        }
    }

    // Execute the next process in the specified queue
    pub fn execute_process(&mut self, queue_index: usize) {
        // Attempt to retrieve the next process in the queue
        if let Some(mut process) = self.queues[queue_index].pop() {
            let time_quantum = self.time_quanta[queue_index]; // Get the time quantum for this queue
            // Determine the amount of time to execute
            let executed_time = if process.remaining_time > time_quantum {
                time_quantum // Execute for the time quantum if remaining time is greater
            } else {
                process.remaining_time // Execute for the remaining time if it's less
            };

            // Update the process's remaining and total executed time
            process.remaining_time -= executed_time;
            process.total_executed_time += executed_time;
            self.current_time += executed_time; // Update the current time

            // Log the execution details
            println!("Executed Process ID: {}, Time Executed: {}, Time Remaining: {}", 
                     process.id, executed_time, process.remaining_time);

            // If the process is not finished, promote it to a lower priority queue
            if process.remaining_time > 0 {
                if queue_index + 1 < self.num_levels {
                    process.priority += 1; // Increase the priority (decrease the queue index)
                    self.queues[queue_index + 1].push(process); // Move to the next queue
                }
            }
            // Completed processes are not re-added to any queue
        }
    }

    // Boost the priority of processes in lower queues
    pub fn priority_boost(&mut self) {
        // Loop through all queues except the highest priority
        for queue_index in 1..self.num_levels {
            // Move each process in the current queue back to the highest priority queue
            while let Some(mut process) = self.queues[queue_index].pop() {
                process.priority = 0; // Reset priority to the highest
                self.queues[0].push(process); // Add process to the highest priority queue
            }
        }
    }

    // Update the time and trigger a priority boost if necessary
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time; // Increment the current time
        let boost_interval = 100; // Define the time interval for priority boosting
        // Check if it's time to boost priorities
        if self.current_time % boost_interval == 0 {
            self.priority_boost(); // Call the priority boost function
        }
    }
}

// Automated test cases for the MLFQ scheduling system
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        // Create sample processes for testing
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };
        
        // Add processes to the MLFQ
        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);
        
        // Verify the correct distribution of processes across queues
        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        // Add a process to the highest priority queue
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });
        
        // Execute the process
        mlfq.execute_process(0);
        
        // Check the state of the queues after execution
        assert_eq!(mlfq.queues[0].len(), 0); // The process should have been removed from the queue
        assert_eq!(mlfq.queues[1].len(), 1); // The process should now be in the next queue
        assert_eq!(mlfq.queues[1][0].remaining_time, 3); // Check remaining time
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2); // Check total executed time
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        // Add processes to lower priority queues
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });
        
        // Update time to trigger a priority boost
        mlfq.update_time(100);
        
        // Verify that processes have been boosted to the highest priority queue
        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        // Add a process to the lower priority queue
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        // Update time without reaching the boost interval
        mlfq.update_time(50);
        
        // Check that no boost has occurred
        assert_eq!(mlfq.queues[1].len(), 1); // Process should still be in queue 1
        assert_eq!(mlfq.queues[0].len(), 0); // Queue 0 should remain empty
    }
}
