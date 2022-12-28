# terminal-paint
rust library that simplifies printing colorful text to the console with ANSI escape.
It is based on ANSI Escape Code. Should work on Linux, macOS and Windows (from Windows 10 1511+) 

## Example
```
use terminal_paint as tp;

let my_str: String = tp::paint("hello world!", tp::YELLOW);
let my_str2: String = tp::paint("world hello", tp::ON_RED);
println!("{}, {}", my_str, my_str2);

// this will call println! and change the color of your text
tp::color_println("this is blue text", tp::BLUE);
```
