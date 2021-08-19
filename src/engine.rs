#[derive(Debug)]
pub struct TerminalEngine {
    output_buffer: Vec<String>
}

impl TerminalEngine {
    pub fn new() -> Self {
        let instance = Self {
            output_buffer: Vec::new()
        };
        return instance;
    }
    pub fn nobuffer_print(&mut self, out: String) {
        println!("{}", out);
    }
    pub fn buff_addln(&mut self, input: String) {
        self.output_buffer.push(format!("\n{}", input));
    }
    pub fn buff_println(&mut self) {
        let mut combined: String = String::from("");
        for x in &self.output_buffer {
            combined = format!("{0}{1}", combined, x)
        }
        println!("{}", combined);
    }
    pub fn recv_buff(&self) -> &Vec<String> {
        &self.output_buffer
    }
}