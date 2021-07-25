use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

pub struct Bar(Vec<Widget>);

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bar: String = self.0.iter().map(|x| x.to_string() + " | ").collect();

        write!(f, "{}", bar)
    }
}

impl Bar {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_widget(&mut self, id: &'static str, retriever: DataRetriever, colorizer: Colorizer, is_num: bool) {
        self.0.push(Widget::new(id, retriever, colorizer, is_num));
    }
}

#[derive(PartialEq, Debug)]
enum Data {
    Number(i32),
    Text(String),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::Text(x) => write!(f, "{}", x),
        }
    }
}

pub enum DataRetriever {
    Extern(&'static str), // data is updated with an external command
    File(&'static str),   // data is updated by reading a file
}

fn str_to_command(st: &'static str) -> Command {
    let mut s_iter = st.split_whitespace();
    let mut c = Command::new(s_iter.next().unwrap());
    for i in s_iter {
        c.arg(i);
    }
    c
}

impl DataRetriever {
    fn retrieve(&self, is_num: bool) -> Data {
        match self {
            Self::Extern(c) => {
                let data = match str_to_command(c).output() {
                    Ok(data) => data.stdout,
                    Err(_) => return Data::Text(String::from("Error running command")),
                };

                if is_num {
                    Data::Number(
                        String::from_utf8_lossy(&data)
                            .chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>()
                            .replace("\n", "")
                            .parse::<i32>()
                            .expect("command returned non number"),
                    )
                } else {
                    Data::Text(String::from_utf8_lossy(&data).replace("\n", ""))
                }
            }
            Self::File(f) => {
                let file = match File::open(f) {
                    Ok(file) => file,
                    Err(_) => return Data::Text(String::from("Error reading file")),
                };
                let mut buffer = BufReader::new(file);

                let mut line = String::new();
                let _ = buffer.read_line(&mut line);
                if is_num {
                    Data::Number(
                        line.chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap(),
                    )
                } else {
                    Data::Text(line.replace("\n", ""))
                }
            }
        }
    }
}

pub enum Colorizer {
    Constant(u8),
    Binary((u8, u8), &'static str), //null color and active color and a null value
    Trinary((u8, u8, u8), i32, i32), // three color values and two threshold values
}

impl Colorizer {
    fn get_color(&self, d: &Data) -> u8 {
        match self {
            Colorizer::Constant(c) => {*c}
            Colorizer::Binary((n, a), nv) => {
                if d.to_string() == nv.to_string() {
                    *n
                }
                else{
                    *a
                }
            }
            Colorizer::Trinary((l,m,h),lt,mt) => {
                match d {
                    Data::Text(_) => {*m}
                    Data::Number(n) if n < lt => {*l}
                    Data::Number(n) if n < mt => {*m}
                    _ => *h
                }
            }
        }
    }
}

struct Widget {
    id: &'static str,         // The identifier of the widget
    retriever: DataRetriever, // the mechanism for updating the data
    colorizer: Colorizer,
    is_num: bool,
}

impl fmt::Display for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let d = self.update();
        write!(f, "{}: +@fg={};{}+@fg={};", self.id, self.colorizer.get_color(&d), d, 0)
    }
}

impl Widget {
    fn new(
        id: &'static str,
        retriever: DataRetriever,
        colorizer: Colorizer,
        is_num: bool,
    ) -> Widget {
        Widget {
            id,
            retriever,
            colorizer,
            is_num,
        }
    }

    fn update(&self) -> Data {
        self.retriever.retrieve(self.is_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trivial_command_widget() {
        let widget = Widget::new("test", DataRetriever::Extern("hello"), Colorizer::Constant(1), false);
        assert_eq!(widget.update(), Data::Text(String::from("Hello, world!")))
    }
    #[test]
    fn trivial_text_file_widget() {
        let widget = Widget::new(
            "test",
            DataRetriever::File("/home/ethan/code/rust/spectrbar/test.txt"),
	    Colorizer::Constant(1),
            false,
        );
        assert_eq!(widget.update(), Data::Text(String::from("hello 32")))
    }
    #[test]
    fn trivial_numeric_file_widget() {
        let widget = Widget::new(
            "test",
            DataRetriever::File("/home/ethan/code/rust/spectrbar/test.txt"),
	    Colorizer::Constant(1),
            true,
        );
        assert_eq!(widget.update(), Data::Number(32))
    }
    #[test]
    fn nontrivial_text_command_widget() {
        let widget = Widget::new("test", DataRetriever::Extern("echo hello world"), Colorizer::Constant(1), false);
        assert_eq!(widget.update(), Data::Text(String::from("hello world")))
    }
    #[test]
    fn nontrivial_numeric_command_widget() {
        let widget = Widget::new("test", DataRetriever::Extern("echo hello world 32"), Colorizer::Constant(1), true);
        assert_eq!(widget.update(), Data::Number(32))
    }
    #[test]
    fn trivial_display_bar() {
        let mut bar = Bar::new();
        bar.add_widget(
            "text",
            DataRetriever::File("/home/ethan/code/rust/spectrbar/test.txt"),
	    Colorizer::Constant(1),
            false,
        );
        bar.add_widget(
            "num",
            DataRetriever::File("/home/ethan/code/rust/spectrbar/test.txt"),
	    Colorizer::Binary((0,1),"31"),
	    true,
        );
        assert_eq!(bar.to_string(), String::from("text: +@fg=1;hello 32+@fg=0; | num: +@fg=1;32+@fg=0; | "))
    }
}
