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

### Day 2

#### Part 1

* Wrote a parsing function to take the input and convert them to structs which
are much more manageable.
* I got the question wrong initially, I assumed that if any turn within a game
was deemed 'impossible' then I ignored the game ID.
* Unfortunately the unit test/example in the question was unable to catch the 
error.

#### Part 2

* The language in the question was again a tad bit confusing but was able to
figure out whats needed based on the described example.

### Day 3

#### Part 1 & 2

* In this question, we could either anchor on the symbols first and check for
number neighbors or we could do it the other way around.
* I chose to parse the input as a 2D Vector so we can treat it like a
coordinate system.
