use std::process::Command;
use std::str::from_utf8;

use my_error::MyError;

pub fn coords() -> Result<(f32, f32), MyError> {
    let output = Command::new("xdotool").arg("getmouselocation").output()?;
    if !output.status.success() {
        return Err(MyError::new("xdotool getmouselocation finished with failure".to_string()));
    }

    let output = from_utf8(&output.stdout)?;
    let mut top_left_strs = output.split(" ");

    let x_str = top_left_strs.nth(0);
    let y_str = top_left_strs.nth(0);
    if x_str.is_none() || y_str.is_none() {
        return Err(MyError::new("xdotool getmouselocation returned invalid data: ".to_string() + output));
    }
    let x_str = x_str.unwrap();
    let y_str = y_str.unwrap();

    let x_str = &x_str[2..];
    let y_str = &y_str[2..];

    let x = x_str.parse::<f32>();
    let y = y_str.parse::<f32>();

    match (x, y) {
        (Ok(x_val), Ok(y_val)) => {
            Ok((x_val, y_val))
        }
        (_, _) => {
            Err(MyError::new(format!("Couldn't parse coords returned by xdotool getmouselocation: {} {}", x_str, y_str)))
        }
    }
}

pub fn down() -> Result<(), MyError> {
    let status = Command::new("xdotool").arg("mousedown").arg("1").status()?;
    if !status.success() {
        Err(MyError::new("xdotool mousedown finished with failure".to_string()))
    } else {
        Ok(())
    }
}

pub fn up() -> Result<(), MyError> {
    let status = Command::new("xdotool").arg("mouseup").arg("1").status()?;
    if !status.success() {
        Err(MyError::new("xdotool mouseup finished with failure".to_string()))
    } else {
        Ok(())
    }
}

pub fn move_to(x: f32, y: f32) -> Result<(), MyError> {
    let status = Command::new("xdotool")
                     .arg("mousemove")
                     .arg(x.to_string())
                     .arg(y.to_string())
                     .status()?;
    if !status.success() {
        Err(MyError::new("xdotool mousemove finished with failure".to_string()))
    } else {
        Ok(())
    }
}

