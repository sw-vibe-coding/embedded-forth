use super::{Machine, PrimitiveId, Value};

impl Machine {
    pub(super) fn prim_version(&mut self) {
        self.flush_output_line();
        self.output.push(crate::build_version());
    }

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

type Primitive = (&'static str, PrimitiveId, fn(&mut Machine));

pub(super) const PRIMITIVES: &[Primitive] = &[
    ("DUP", PrimitiveId::Dup, Machine::prim_dup),
    ("SWAP", PrimitiveId::Swap, Machine::prim_swap),
    ("DROP", PrimitiveId::Drop, Machine::prim_drop),
    ("OVER", PrimitiveId::Over, Machine::prim_over),
    ("ROT", PrimitiveId::Rot, Machine::prim_rot),
    ("+", PrimitiveId::Add, Machine::prim_add),
    ("-", PrimitiveId::Sub, Machine::prim_sub),
    ("*", PrimitiveId::Mul, Machine::prim_mul),
    ("/", PrimitiveId::Div, Machine::prim_div),
    ("MOD", PrimitiveId::Mod, Machine::prim_mod),
    ("/MOD", PrimitiveId::SlashMod, Machine::prim_slash_mod),
    (
        "*/MOD",
        PrimitiveId::StarSlashMod,
        Machine::prim_star_slash_mod,
    ),
    ("=", PrimitiveId::Eq, Machine::prim_eq),
    ("<", PrimitiveId::Lt, Machine::prim_lt),
    (">", PrimitiveId::Gt, Machine::prim_gt),
    ("AND", PrimitiveId::And, Machine::prim_and),
    ("OR", PrimitiveId::Or, Machine::prim_or),
    ("XOR", PrimitiveId::Xor, Machine::prim_xor),
    ("INVERT", PrimitiveId::Invert, Machine::prim_invert),
    ("LSHIFT", PrimitiveId::LShift, Machine::prim_lshift),
    ("RSHIFT", PrimitiveId::RShift, Machine::prim_rshift),
    (
        "COMPILE,",
        PrimitiveId::CompileComma,
        Machine::prim_compile_comma,
    ),
    ("IF", PrimitiveId::If, Machine::prim_if),
    ("ELSE", PrimitiveId::Else, Machine::prim_else),
    ("THEN", PrimitiveId::Then, Machine::prim_then),
    ("BEGIN", PrimitiveId::Begin, Machine::prim_begin),
    ("UNTIL", PrimitiveId::Until, Machine::prim_until),
    ("WHILE", PrimitiveId::While, Machine::prim_while),
    ("REPEAT", PrimitiveId::Repeat, Machine::prim_repeat),
    ("DO", PrimitiveId::Do, Machine::prim_do),
    ("LOOP", PrimitiveId::LoopPrim, Machine::prim_loop),
    ("+LOOP", PrimitiveId::PlusLoop, Machine::prim_plus_loop),
    ("LEAVE", PrimitiveId::LeavePrim, Machine::prim_leave),
    ("[", PrimitiveId::LBracket, Machine::prim_l_bracket),
    ("]", PrimitiveId::RBracket, Machine::prim_r_bracket),
    ("LITERAL", PrimitiveId::Literal, Machine::prim_literal),
    ("DOES>", PrimitiveId::DoesArrow, Machine::prim_does_arrow),
    ("POSTPONE", PrimitiveId::Postpone, Machine::prim_postpone),
    (".", PrimitiveId::Dot, Machine::prim_dot),
    (".S", PrimitiveId::DotS, Machine::prim_dot_s),
    ("CLEAR", PrimitiveId::Clear, Machine::prim_clear),
    (">R", PrimitiveId::ToR, Machine::prim_to_r),
    ("R>", PrimitiveId::FromR, Machine::prim_r_from),
    ("R@", PrimitiveId::RFetch, Machine::prim_r_fetch),
    ("@", PrimitiveId::Fetch, Machine::prim_fetch),
    ("!", PrimitiveId::Store, Machine::prim_store),
    ("+!", PrimitiveId::PlusStore, Machine::prim_plus_store),
    ("WORDS", PrimitiveId::Words, Machine::prim_words),
    ("CR", PrimitiveId::Cr, Machine::prim_cr),
    ("EMIT", PrimitiveId::Emit, Machine::prim_emit),
    ("SPACE", PrimitiveId::Space, Machine::prim_space),
    ("I", PrimitiveId::I, Machine::prim_i),
    ("J", PrimitiveId::J, Machine::prim_j),
    ("ALLOT", PrimitiveId::Allot, Machine::prim_allot),
    ("EXECUTE", PrimitiveId::Execute, Machine::prim_execute),
    ("HERE", PrimitiveId::Here, Machine::prim_here),
    (",", PrimitiveId::Comma, Machine::prim_comma),
    ("LATEST", PrimitiveId::Latest, Machine::prim_latest),
    ("IMMEDIATE", PrimitiveId::Immediate, Machine::prim_immediate),
    ("CREATE", PrimitiveId::Create, Machine::prim_create),
    ("TYPE", PrimitiveId::Type, Machine::prim_type),
    ("ABORT", PrimitiveId::Abort, Machine::do_abort),
    ("EXIT", PrimitiveId::Exit, |machine| machine.exiting = true),
    ("?DO", PrimitiveId::QDo, Machine::prim_q_do),
    ("PICK", PrimitiveId::Pick, Machine::prim_pick),
    ("ROLL", PrimitiveId::Roll, Machine::prim_roll),
    ("DEPTH", PrimitiveId::Depth, |machine| {
        machine.stack.push(Value::Int(machine.stack.len() as i32))
    }),
    ("RDEPTH", PrimitiveId::RDepth, |machine| {
        machine
            .stack
            .push(Value::Int(machine.return_stack.len() as i32))
    }),
    ("VERSION", PrimitiveId::Version, Machine::prim_version),
];
