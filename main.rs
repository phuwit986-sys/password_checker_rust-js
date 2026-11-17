use crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal};
use std::io::{self, Write};

const MIN_LEN: usize = 15;

fn read_masked() -> io::Result<String> {
    let mut pw = String::new();  // เก็บตามจริง ไม่ trim ไม่ตัด
    loop {
        let Event::Key(k) = event::read()? else { continue };
        if k.kind == KeyEventKind::Release { continue; }
        match k.code {
            KeyCode::Enter => return Ok(pw),
            KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => return Err(io::ErrorKind::Interrupted.into()),
            KeyCode::Backspace if pw.pop().is_some() => print!("\x08 \x08"),
            KeyCode::Char(c) => { pw.push(c); print!("*"); }
            _ => continue,
        }
        io::stdout().flush()?;
    }
}

fn ask_password() -> io::Result<String> {
    print!("Password: ");
    io::stdout().flush()?;
    terminal::enable_raw_mode()?;
    let pw = read_masked();
    terminal::disable_raw_mode()?;
    println!();
    pw
}

fn main() -> io::Result<()> {
    for i in 1..=3 {
        print!("Username (email): ");
        io::stdout().flush()?;
        let mut user = String::new();
        io::stdin().read_line(&mut user)?;
        let user = user.trim();
        let pw = ask_password()?;
        let len = pw.chars().count();  // นับเป็นตัวอักษร (ไม่ใช่ len() ที่นับ byte)
        let email_ok = user.split_once('@').is_some_and(|(l, d)| !l.is_empty() && d.contains('.') && !user.contains(char::is_whitespace));
        if !email_ok { println!("ไม่ผ่าน ({i}/3): username ต้องเป็นอีเมล\n"); }
        else if len < MIN_LEN { println!("ไม่ผ่าน ({i}/3): รหัสผ่านสั้นเกินไป ({len}/{MIN_LEN} ตัวอักษร)\n"); }
        else { println!("ผ่าน: ความยาว {len} ตัวอักษร\nWelcome {user}"); return Ok(()); }
    }
    println!("ผิดครบ 3 ครั้ง Logout!");
    Ok(())
}
