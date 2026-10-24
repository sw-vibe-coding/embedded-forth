use super::Machine;

impl Machine {
    /// Consume terminal output and discard browser diagnostics between inputs.
    /// Native sessions need not retain an ever-growing UI transcript.
    pub fn take_output(&mut self) -> String {
        self.flush_output_line();
        let text = self.output.join("\n");
        self.output.clear();
        self.history.clear();
        self.trace.clear();
        text
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}
