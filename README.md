# betterm
is a better terminal crate  
how? the idea behind it is INCLUDE EVERYTHING ✨  
why? similar crates leave out helpful, common [ANSI codes](https://en.wikipedia.org/wiki/ANSI_escape_code) 😬  

## it is simple
most ANSI codes are just `const &'static str`,  
and dynamic ones are structs that implement `std::fmt::Display`  
(that also blanketly implements `std::string::ToString`)  

## it makes sense
you will never have to guess what something does, naming scheme IS documentation!  
once you read the name, you know absolutely everything about what it does  

## modules
### basic
escape, bell, backspace, line feed, carriage return, horizontal/vertical tab  

### clear
in screen, in line, scrollback buffer  

### color
ANSI colors, 256 lookup table, RGB  
for text, background and underline  

### cursor
save/restore position, hide/show, relative movement, set/get absolute position  

### mode
focus reports on/off, bracketed paste on/off, raw mode, mouse support  

### printer
page break, aux on/off  

### screen
enter/leave alternate screen, get size in cells/pixels  

### scroll
up, down, set region  

### style
bold, faint, italic, (double) under/over line, strikethrough, slow/fast blink, conceal, alt fonts, gothic font, proportional spacing, emoji frame/circle, ideograms, super/sub script  

## notes
there is no input event handling,  
currently the goal of this crate is to provide  
everything you would ever want to print  

## help me out!
is something you need missing or not working?  
just submit a pull request or complain in issues/discussions!  

