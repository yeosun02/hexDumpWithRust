use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::io::Write;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("{}", err);
    process::exit(1);
  });

  // hexdump function call
  if let Err(e) = hex_dump(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}

fn hex_dump(config: Config) -> Result<(), Box<dyn Error>>{
  let mut contents = fs::read(config.filename)?;
  let mut len = if config.length == "not specified" {
    contents.len()
  } else {
    config.length.parse().unwrap()
  };

  len = std::cmp::min(len, contents.len());

  // Make length even number because hex_dumping expects 
  // length of even numbers but save original len for later
  let new_len = if len % 2 == 1{
    if len == contents.len() {
      contents.push(0);
    } else {
      contents[len] = 0;
    }
    len + 1
  } else {
    len
  };
  
  for row in (0..new_len).step_by(16) {
    print!("{:07x}", row);
    
    let mut i = row;
    while i < new_len && i < row + 16{
      print!(" {:02x}{:02x}", contents[i + 1], contents[i]);
      i += 2;
    }
    std::io::stdout().flush().unwrap();
    println!("");
  }

  // I followed the outcome of builtin hexdump even though
  // it somtimes makes the outcome different from the example given
  println!("{:07x}", len);

  Ok(())
}

struct Config {
  length: String,
  filename: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &str> {
    let mut length = "not specified".to_string();
    let filename;

    // Two possible usages: 
    // 1) program -n Len FILE 
    // 2) program FILE
    // make sure Len is a number
    if args.len() == 4 && args[1].as_str() == "-n" {
      if args[2].parse::<u32>().is_ok(){
        length = args[2].clone();
        filename = args[3].clone();
      } else {
        return Err("hexdump: bad length value");
      }
    } else if args.len() == 2 {
      filename = args[1].clone();
    } else {
      return Err("Usage: hexdump [-n Len] FILE");
    }
    
    Ok(Config { length, filename })
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_config_new() {
    // Should set length = 123 and filename = sample.txt
    let args = vec!["hexdump".to_string(), "-n".to_string(), "123".to_string(), "sample.txt".to_string()];

    let config = Config::new(&args).unwrap_or_else(|err| {
      panic!("PANIC {}", err);
    });

    assert_eq!(config.length, "123".to_string());
    assert_eq!(config.filename, "sample.txt".to_string());
  }

  #[test]
  #[should_panic]
  fn test_config_invalid_num() {
    // Should fail as Len is not a number
    let args = vec!["hexdump".to_string(), "-n".to_string(), "onetwoThree".to_string(), "file.txt".to_string()];

    let _ = Config::new(&args).unwrap_or_else(|err| {
      panic!("PANIC {}", err);
    });
  }

  #[test]
  #[should_panic]
  fn test_config_invalid_option() {
    // Should fail as -k is not valid
    let args = vec!["hexdump".to_string(), "-k".to_string(), "256".to_string(), "file.txt".to_string()];

    let _ = Config::new(&args).unwrap_or_else(|err| {
      panic!("PANIC {}", err);
    });
  }
}