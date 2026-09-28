Implement a generic FIFO Queue<T>, without requiring T: Copy or T: Clone.
Methods: new() -> Self, push(&mut self, value: T), pop(&mut self) -> Option<T>,
is_empty(&self) -> bool, split(self) -> (Vec<T>, Vec<T>).
Use older and younger vectors. Push onto younger. When popping with older empty,
move younger into older and reverse it. Pop from the end of older. split returns
(older, younger). Do not include tests or a main function.
