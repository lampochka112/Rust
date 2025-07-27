use std::fs;
use std::path::Path;
use clap::Parser;
use tokio::fs as tokio_fs;

/// Анализатор текстовых файлов
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Путь к файлу
    #[arg(short, long)]
    file: String,

    /// Асинхронный режим (опционально)
    #[arg(short, long, default_value_t = false)]
    async_mode: bool,
}

/// Результат анализа файла
struct FileStats {
    lines: usize,
    words: usize,
    chars: usize,
}

impl FileStats {
    fn new() -> Self {
        FileStats {
            lines: 0,
            words: 0,
            chars: 0,
        }
    }
}

/// Синхронная обработка файла
fn process_file_sync(path: &Path) -> Result<FileStats, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let mut stats = FileStats::new();

    for line in content.lines() {
        stats.lines += 1;
        stats.words += line.split_whitespace().count();
        stats.chars += line.chars().count();
    }

    Ok(stats)
}

/// Асинхронная обработка файла
async fn process_file_async(path: &Path) -> Result<FileStats, std::io::Error> {
    let content = tokio_fs::read_to_string(path).await?;
    let mut stats = FileStats::new();

    for line in content.lines() {
        stats.lines += 1;
        stats.words += line.split_whitespace().count();
        stats.chars += line.chars().count();
    }

    Ok(stats)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = Path::new(&args.file);

    if !path.exists() {
        eprintln!("Ошибка: файл не найден!");
        std::process::exit(1);
    }

    let stats = if args.async_mode {
        println!("🔄 Асинхронный режим...");
        process_file_async(path).await?
    } else {
        println!("🦀 Синхронный режим...");
        process_file_sync(path)?
    };

    println!("📊 Результат для файла '{}':", args.file);
    println!("➖ Строк: {}", stats.lines);
    println!("➖ Слов: {}", stats.words);
    println!("➖ Символов: {}", stats.chars);

    Ok(())
}