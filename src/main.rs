fn main() {
    let mut bar = spectrbar::Bar::new();

    bar.add_widget(
        "test",
        spectrbar::DataRetriever::Extern("echo hello"),
        false,
    );

    println!("{}", bar)
}
