mod configurator;
use configurator::file_handler::FileHandler;
use std::io; // Импортируем модуль io для работы с результатами

fn main() -> io::Result<()> {
    
    let mut file_handler = FileHandler::open("config.toml")?;
    
    
    let content = file_handler.read()?;
    
    println!("Data: {}", content);

    Ok(())
}