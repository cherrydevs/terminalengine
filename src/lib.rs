pub mod engine;

#[cfg(test)]
mod tests {
    use crate::engine::*;
    #[test]
    fn test() {
        let mut instance = TerminalEngine::new();
        instance.buff_addln(String::from("test"));
        instance.buff_addln(String::from("this should print another line, terminalengine is epic"));
        instance.buff_println();
        instance.nobuffer_print(format!("this is the buffer variable raw: {:?}", instance.recv_buff()));
        instance.nobuffer_print(String::from("this is not apart of the buffer, meant for debugging"));
    }
}
