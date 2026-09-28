Implement a FIFO character queue named Queue with these public methods:
`new() -> Queue`, `push(&mut self, c: char)`, `pop(&mut self) -> Option<char>`,
`is_empty(&self) -> bool`, `split(self) -> (Vec<char>, Vec<char>)`.
Use two vectors: older stores older elements with the next element at its end;
younger stores newly pushed elements in insertion order. When popping with
older empty, move younger into older and reverse it. split returns these two
vectors in (older, younger) order. Support Unicode characters, repeated empty
pops, and reuse after draining. Do not include tests or a main function.
