use std::io;

pub fn read_user_input(user_comment: Option<&str>) -> io::Result<String> {
    let mut buffer = String::new();

    match user_comment {
        None => println!(),
        Some(text) => println!("{}", text),
    }

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}
