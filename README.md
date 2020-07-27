# forse. like forth but worse

this is just a simple pedagogical implementation of a forth-like language.

an example:
```rust
let forth = Forth::default();

// add a print word, prints the top of the stack
forth.add_word(Word::new(".").func(|f| println!("{}", f.top())));

// add an 'add' word
forth.add_word(Word::new("+").func(|f| {
    let res = f.pop() + f.pop();
    f.push(res)
}));

// alias 'add' to '+'
forth.add_word(Word::new("add").body("+"));

// add two numbers and restore result on the stack, and then prints it
forth.exec("1 1 + .")?; // => 2

// do the same thing, but with the alias
forth.exec("1 1 add .")?; // => 2

// define a function which adds 1 to the top of the stack
forth.exec(": add1 1 + ;")?; 

// push 2 to the stack and call add2 and then print it
forth.exec("2 add1 .")?; // => 3
```

license: 0BSD
