use std::cmp::Ordering;

#[tauri::command]
fn guessing_game(input_string: &str, random_number: i32) -> String {
    let guessed_number: i32 = match input_string.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            return "Please Provide a valid number!".to_string();
        }
    };
    let message = match guessed_number.cmp(&random_number) {
        Ordering::Less => format!("Your guess is lower than the secret number"),
        Ordering::Greater => format!("Your guess is greater than the secret number"),
        Ordering::Equal => format!("Your guess is correct"),
    };

    return message;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![guessing_game])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
