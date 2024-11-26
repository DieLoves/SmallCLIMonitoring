use sysinfo::{System, Networks};
use std::{fs::File, io::Write, thread, time::Duration};
use chrono::Local;

fn main() {
    println!("Кроссплатформенный мониторинг энергопотребления на Rust");

    // Запрос параметров от пользователя
    let duration: u64 = {
        println!("Введите длительность мониторинга в секундах:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap_or(60)
    };

    let interval: u64 = {
        println!("Введите интервал между замерами в секундах:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap_or(5)
    };

    let output_file = {
        println!("Введите имя файла для сохранения данных (по умолчанию: power_log.csv):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let file_name = input.trim();
        if file_name.is_empty() {
            "power_log.csv".to_string()
        } else {
            file_name.to_string()
        }
    };

    // Открытие файла для записи
    let mut file = File::create(&output_file).expect("Не удалось создать файл");
    writeln!(
        file,
        "Time,CPU Usage (%),Memory Usage (%),Network Sent (bytes),Network Received (bytes)"
    )
        .unwrap();

    println!("Начинаем мониторинг на {} секунд...", duration);

    // Инициализация системного мониторинга
    let mut system = System::new_all();
    let mut start_time = std::time::Instant::now();

    while start_time.elapsed().as_secs() < duration {
        // Обновление данных системы
        system.refresh_all();

        // Получение данных
        let cpu_usage = system.global_cpu_usage();
        let memory_total = system.total_memory();
        let memory_used = system.used_memory();
        let memory_usage = (memory_used as f64 / memory_total as f64) * 100.0;

        let networks = Networks::new_with_refreshed_list();
        let total_sent: u64 = networks.iter().map(|(_, data)| data.total_transmitted()).sum();
        let total_received: u64 = networks.iter().map(|(_, data)| data.total_received()).sum();

        // Текущее время
        let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Логирование в консоль
        println!(
            "{} - CPU: {:.2}%, Memory: {:.2}%, Sent: {} bytes, Received: {} bytes",
            current_time, cpu_usage, memory_usage, total_sent, total_received
        );

        // Запись в файл
        writeln!(
            file,
            "{},{:.2},{:.2},{},{}",
            current_time, cpu_usage, memory_usage, total_sent, total_received
        )
            .unwrap();

        // Задержка
        thread::sleep(Duration::from_secs(interval));
    }

    println!("Мониторинг завершен. Данные сохранены в {}", output_file);
}
