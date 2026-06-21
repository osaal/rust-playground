pub mod interpreter {
    /// The Brainfuck interpreter
    ///
    /// This interpreter tries to strictly follow the [Brainfuck specification](https://brainfuck.org/brainfuck.html). For instance:
    ///
    /// -   Under- and overflowing the program counter crashes the interpreter, an "unpredictable" result.
    /// -   Under- and overflowing an element in the output array crashes the interpreter.
    /// -   The two optional characters `#` and `!` are ignored.
    /// -   The `,` command presumes that the input is not streamed, and thus is simply ignored.
    pub struct Interpreter {
        /// The input string to parse
        data: String,
        /// Input index
        input_idx: usize,
        /// Return index stack for looping
        ret_stack: Vec<usize>,
        /// Output index
        output_idx: usize,
        /// The data array, 30k 8-bit values in an array
        output: [u8; 30_000],
        /// The loop depth stack
        stack: Vec<usize>,
    }
    impl Default for Interpreter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Interpreter {
        pub fn new() -> Self {
            Self {
                data: String::new(),
                input_idx: 0,
                ret_stack: Vec::new(),
                output_idx: 0,
                output: [0u8; 30_000],
                stack: Vec::new(),
            }
        }
        pub fn input(mut self, i: String) -> Self {
            self.data = i;
            self
        }
        pub fn run(mut self) -> Result<(), String> {
            // Convert `data` to a vector of `char`s (non-nightly)
            let input = self
                .data
                .into_bytes()
                .into_iter()
                // TYPE COERCION: .into_bytes() always produces `u8`s that can validly represent characters.
                .map(|el| el as char)
                .collect::<Vec<char>>();

            // Iterate through the input
            while let Some(val) = input.get(self.input_idx) {
                match val {
                    '+' => self.output[self.output_idx] += 1,
                    '-' => self.output[self.output_idx] -= 1,
                    '<' => self.output_idx -= 1,
                    '>' => self.output_idx += 1,
                    '[' => {
                        self.ret_stack.push(self.input_idx - 1); // Store return address
                        if self.output[self.output_idx] != 0 {
                            self.stack.push(self.output_idx);
                        }
                    }
                    ']' => {
                        if self.output[self.output_idx] != 0 {
                            self.output_idx = match self.stack.pop() {
                                Some(val) => val,
                                None => return Err("Unmatched command `]`".into()),
                            };
                            self.input_idx = match self.ret_stack.pop() {
                                Some(val) => val,
                                None => return Err("what".into()),
                            };
                        }
                        // Cleanup before exiting loop
                        // (infallible operations, no need to check return values)
                        let _ = self.stack.pop();
                        let _ = self.ret_stack.pop();
                    }
                    '.' => println!("{}", self.output[self.output_idx]),
                    ',' => continue,
                    _ => {
                        // Ignore all other characters
                        continue;
                    }
                }
                self.input_idx += 1;
            }
            //println!("Output array: {:?}", self.output);
            // Return the converted String
            Ok(())
        }
    }
}
