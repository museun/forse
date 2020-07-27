use anyhow::Context as _;
use std::{cell::RefCell, collections::HashMap};

type Func = Box<dyn Fn(&Forth) + 'static>;

#[derive(Default)]
struct Word {
    name: String,
    body: String,
    func: Option<Func>,
}

impl Word {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    fn func(mut self, func: impl Fn(&Forth) + 'static) -> Self {
        self.func = Some(Box::new(move |f| func(f)));
        self
    }
}

#[derive(Default)]
struct Forth {
    // RefCells because of a recursive borrow and passing self to a boxed closure..
    stack: RefCell<Vec<usize>>,
    words: RefCell<HashMap<String, Word>>,
}

impl Forth {
    fn exec_word(&self, word: &str) -> anyhow::Result<()> {
        if let Ok(cell) = word.parse() {
            self.push(cell);
            return Ok(());
        }

        let words = self.words.borrow();
        let w = words
            .get(word)
            .with_context(|| format!("unknown word: '{}'", word))?;

        if let Some(func) = &w.func {
            func(self);
        } else if !w.body.is_empty() {
            self.exec(&w.body)?;
        } else {
            anyhow::bail!("word has nothing to do: '{}'", word.escape_debug())
        }

        Ok(())
    }

    fn exec(&self, code: &str) -> anyhow::Result<()> {
        use std::mem::take;

        let (mut name, mut body) = <(String, String)>::default();
        let mut creating = false;

        for word in code.split_terminator(' ') {
            match word {
                ":" => creating = true,
                ";" => {
                    creating = false;
                    self.add_word(Word::new(take(&mut name)).body(take(&mut body)));
                }
                _ if !creating => self.exec_word(word)?,
                _ if name.is_empty() => name = word.to_string(),
                _ => {
                    body.push_str(word);
                    body.push(' ');
                }
            }
        }

        Ok(())
    }

    fn add_word(&self, word: Word) {
        self.words.borrow_mut().insert(word.name.clone(), word);
    }

    fn push(&self, cell: usize) {
        self.stack.borrow_mut().push(cell)
    }

    fn pop(&self) -> usize {
        self.stack.borrow_mut().pop().unwrap()
    }

    fn top(&self) -> usize {
        self.stack.borrow().last().cloned().unwrap()
    }
}

fn main() -> anyhow::Result<()> {
    let forth = Forth::default();

    forth.add_word(Word::new(".").func(|f| println!("{}", f.top())));

    forth.add_word(Word::new("+").func(|f| {
        let res = f.pop() + f.pop();
        f.push(res)
    }));

    forth.add_word(Word::new("add").body("+"));

    forth.exec("1 1 + .")?;
    forth.exec("1 1 add .")?;

    forth.exec(": add1 1 + ;")?;
    forth.exec("2 add1 .")?;

    forth.exec(": foo ;")?;
    forth.exec("foo").unwrap_err();

    Ok(())
}
