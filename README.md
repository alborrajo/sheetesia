# Sheetesia

## Convert Synthesia piano roll videos into MIDI

Easily turn piano tutorials into music sheet

![video to sheet](https://filefucktory.ga/files/2867235eb55518a88ba91f952329f553/Proyectos/Sheetesia/yeepers.jpg)

## How to use

To compile this project you need [cargo](https://doc.rust-lang.org/cargo/index.html) and [opencv 4.1](https://opencv.org/).

- [Compile OpenCV generating a pkg-config file](https://github.com/opencv/opencv/issues/13154#issuecomment-456652297)

- Clone this repository

- Run ```cargo build``` or ```cargo build --release```

- Run ```cargo run``` providing the video you want to convert as the first argument

- Wait for it to finish

- Done! The resulting midi will be at ```out.mid```
