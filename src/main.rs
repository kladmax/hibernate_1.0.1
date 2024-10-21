use std::process::Command;

fn main() {
    // Шлях до вашого bat-файлу
    let bat_file = "hibernate.bat";

    // Запуск bat-файлу
    let output = Command::new("cmd")
        .args(&["/C", bat_file])
        .output()
        .expect("failed to execute bat file");

    // Вивести результат виконання
    if output.status.success() {
        println!("Hibernate script executed successfully.");
    } else {
        eprintln!("Failed to execute hibernate script.");
    }
}
