## Advent Of Code 2023 In Rust

### Day 1

#### Part 1

* Since we are only looking for digits, I decided to loop fromm front and back;
breaking at the sight of the first nummber.
* I attempted to put both last and first iterations in 1 loop but that seemed
more complicated, so scaped this approach.

#### Part 2

* Now that we have words and numbers fromm both front and back, my initial
thoughts were as follows.
    * Use starts_with() and ends_with() over the string for words.
    * If not found at current iterator, slice away the substring and redo the
    search.
    * But iterating throug &str resulted in a char tuple. E.g. (0, 'o'), (1, 'n').
* While looking at &str docs, I came across [match_indices](https://doc.rust-lang.org/std/primitive.str.html#method.match_indices), which was a perfect fit.
* After some prototyping in rust playground ([link](https://play.rust-lang.org/?version=beta&mode=debug&edition=2015&gist=89ea333906aa2b4bcea573883a3dc869)), was able to successfully solve the puzzle.
