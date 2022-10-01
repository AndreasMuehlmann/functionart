# functionart
A program that generates pictures from functions.

![alt text](https://github.com/AndreasMuehlmann/functionart/blob/main/results/another_universe.png)

Author: Andreas Mühlmann
GitHub repository: "https://github.com/AndreasMuehlmann/functionart"

# Quickstart

Clone the git repository with
"git clone https://github.com/AndreasMuehlmann/functionart.git".

if git is not installed download the
repository from the website via the zip-archive (click on the green button).

All the pictures are in the results Directory

## Generate own pictures

There is no way to use this program without recompiling everytime

To be able to do that you need rust to be installed, since this
project is written in rust.

To do that visit this website: "https://www.rust-lang.org/learn/get-started"

If that is done run "cargo run", to compile and run the program.
The result is going to be "results/functionart\_result"

The functions can be changed in the "src/functions.rs" file.
There also the intervals for the functions are changeable.

The size of the picture can be changed in "src/main.rs"

You can also try other features of the raster module to 
for example blend or transform your pictures.

