fn main() {
    let bar = spectrbar::Bar::new();

    bar.add_widget("test", spectrbar::DataRetriever::Extern(Command::new("sh")
            .arg("-c")
							    .arg("echo hello")),false);


    println!("{}",bar)
}
