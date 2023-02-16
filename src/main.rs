mod brainfuck;
use brainfuck::interpreter::Interpreter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;
    let mut interpreter = Interpreter::new();
    _ = interpreter.run(data);
    Ok(())
}
