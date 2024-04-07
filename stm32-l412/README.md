# Rust on the Nucleo-32 STM32L214

## The board

![image](https://github.com/Naphtaline/rust-xperiments/assets/1192256/cdb44ea9-481a-4afe-b84e-17df75a88a1d) | ![image](https://github.com/Naphtaline/rust-xperiments/assets/1192256/3c99ec57-e840-4044-af86-efbed909d7db)

This board is equiped with pins maped (I guess when flashed with arduino stuff) like an arduino nano, so it can be used as an Arduino Nano out of the box, I haven't tried that yet.
The blue pinout is the regular STM32 pinout and the pink is the mapped Arduino Nano one.

## The code
So I didn't invent anyhting, I started to look for resources, I found a lot of outdated tutorials, documentations, forum post and so on. But I managed to find some really nice resources ! First of all, I read a lot of crates documentations, this is helpfull to get an idea of the big picture, how things are supposed to work together. 

Then I stumbled across a youtube channel, which is either up to date AND with crystal clear inforamations on embedded programing with rust :  [The Rusty Bits ](https://www.youtube.com/@therustybits). Following this video, I just came back to a "Getting Started" I read earlier on the [github](https://github.com/David-OConnor/stm32-hal) of the [stm32-hal2 crate](https://crates.io/crates/stm32-hal2).

> Also, not rust related but if you are looking for good, quick and very effective basic MCU "lessons", I recommend [Mitch Davis channel](https://www.youtube.com/@MitchDavis2) on youtube.

This is all basic stuff, but once you've understand this properly you really have one foot in the rust embeded world !
