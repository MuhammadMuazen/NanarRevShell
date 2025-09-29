pub fn process_kill(process_id: u32) -> String {
    
    let task_list: tasklist::Tasklist = match tasklist::Tasklist::new() {
        
        Ok(list) => list,
        Err(e) => return format!("[-] Error: Failed to get task list: {}", e),
    };

    for task in task_list {
        
        if task.get_pid() == process_id {
            
            return match task.kill() {
                
                Ok(_) => format!("[+] Process {} killed successfully!", process_id),
                Err(e) => format!("[-] Error: Failed to kill process {}: {}", process_id, e),
            };
        }
    }

    format!("[-] Error: No process found with ID: {}", process_id)
}