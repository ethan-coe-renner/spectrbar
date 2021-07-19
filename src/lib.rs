use std::fmt;
use std::fs::File;
use std::process::Command;
use std::io::{BufReader, BufRead};

pub struct Bar(Vec<Widget>);

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bar: String = self.0.iter().map(|x| x.to_string() + " | ").collect();

        write!(f, "{}", bar)
    }
}

impl Bar {
    fn new() -> Self {
	Self(Vec::new())
    }

    fn add_widget(&mut self, id: &'static str, retriever: DataRetriever, is_num: bool) {
	self.0.push(Widget::new(id, retriever, is_num));
    }
}

enum Data {
    Number(i32),
    Text(String),
}

impl Data {
    fn new(isNum: bool) -> Self {
        if isNum {
            Self::Number(0)
        } else {
            Self::Text(String::new())
        }
    }
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

impl DataRetriever {
    fn retrieve(&self, is_num: bool) -> Data {
        match self {
            Self::Extern(c) => {
                let data = match Command::new(c).output() {
		    Ok(data) => data.stdout,
		    Err(_) => return Data::new(is_num)
		};

                if is_num {
                    Data::Number(
                        String::from_utf8_lossy(&data)
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
                    Err(_) => return Data::new(is_num),
                };
                let mut buffer = BufReader::new(file);

                let mut line = String::new();
		let _ = buffer.read_line(&mut line);
		if is_num {
		        Data::Number(line.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap())
		}
		else {
		        Data::Text(line)
		}
            }
        }
    }
}

struct Widget {
    id: &'static str,         // The identifier of the widget
    retriever: DataRetriever, // the mechanism for updating the data
    is_num: bool
}

impl fmt::Display for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.update())
    }
}

impl Widget {
    fn new(id: &'static str, retriever: DataRetriever, is_num: bool) -> Widget {
        Widget {
            id,
            retriever,
	    is_num
        }
    }

    fn update(&self) -> Data {
	self.retriever.retrieve(self.is_num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn construct_bar() {
    }
}
