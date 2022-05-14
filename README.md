# Sheetesia

## Convert Synthesia piano roll videos into MIDI

Easily turn piano tutorials into music sheet

![video to sheet](https://web.archive.org/web/20201215033456if_/https://camo.githubusercontent.com/135cd5f263c8d93eb37fc47b787673a687d8bb5112d422b77bd5f4877d2c370d/68747470733a2f2f66696c656675636b746f72792e67612f66696c65732f32383637323335656235353531386138386261393166393532333239663535332f50726f796563746f732f5368656574657369612f796565706572732e6a7067)

## How to use

To compile this project you need [cargo](https://doc.rust-lang.org/cargo/index.html) and [opencv](https://opencv.org/).

- [Compile OpenCV generating a pkg-config file](https://github.com/opencv/opencv/issues/13154#issuecomment-456652297)

- Clone this repository

- Run ```cargo build``` or ```cargo build --release```

- Run ```cargo run``` providing the video you want to convert as the first argument

- Wait for it to finish

- Done! The resulting midi will be at ```out.mid```
