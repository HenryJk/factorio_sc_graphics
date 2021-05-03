pub trait LuaSyntax {
    fn _prettyPrint(&self, indent: u32, buffer: &mut String);

    fn prettyPrint(&self) -> String {
        let mut result = String::new();
        self._prettyPrint(0, &mut result);
        result
    }
}

pub struct Block {
    pub stats: Vec<Stat>,
    pub last_stat: Option<LastStat>,
}

pub enum Stat {
    Assignment{var_list: Vec<String>, exp_list: Vec<Exp>},
    FuncCall{function: Exp, par_list: Vec<Exp>},
    While{condition: Exp, body: Block},
    Repeat{condition: Exp, body: Block},
    //If{condition: Exp, then_body: Block, elseif: Vec<Stat::ElseIf>, else_body: Block},
}

pub enum LastStat {
    Return{exp_list: Vec<Exp>},
    Break,
}

pub enum Exp {
    Nil,
    Bool(bool),
    Number(f32),
    String(String),
    Var(String),
    Function{par_list: Vec<String>, body: Block},
    Array{member_list: Vec<Exp>},
    Table{field_list: Vec<(String, Exp)>},
    Binop{left: Box<Exp>, op: String, right: Box<Exp>},
    Unop{op: String, right: Box<Exp>},
}

impl LuaSyntax for Block {
    fn _prettyPrint(&self, indent: u32, buffer: &mut String) {
        for stat in &self.stats {
            for _ in 0..indent {*buffer += "  ";}
            stat._prettyPrint(indent, buffer);
            *buffer += "\n";
        }
        if let Some(stat) = &self.last_stat {
            for _ in 0..indent {*buffer += "  ";}
            stat._prettyPrint(indent, buffer);
            *buffer += "\n";
        }
    }
}

impl LuaSyntax for Stat {
    fn _prettyPrint(&self, indent: u32, buffer: &mut String) {
        match &self {
            Stat::Assignment{var_list, exp_list} => {
                for i in 0..var_list.len() {
                    *buffer += &*var_list[i];
                    if i != var_list.len() - 1 {*buffer += ", "}
                }
                for i in 0..exp_list.len() {
                    exp_list[i]._prettyPrint(indent, buffer);
                    if i != exp_list.len() - 1 {*buffer += ", "}
                }
            },
            Stat::FuncCall{function, par_list} => {
                function._prettyPrint(indent, buffer);
                *buffer += "(";
                for i in 0..par_list.len() {
                    par_list[i]._prettyPrint(indent, buffer);
                    if i != par_list.len() - 1 {*buffer += ", "}
                }
                *buffer += ")";
            },
            Stat::While{condition, body} => {
                *buffer += "while ";
                condition._prettyPrint(indent, buffer);
                *buffer += " do\n";
                body._prettyPrint(indent + 1, buffer);
                for _ in 0..indent {*buffer += "  ";}
                *buffer += "end\n";
            },
            _ => {}
        }
    }
}

impl LuaSyntax for LastStat {
    fn _prettyPrint(&self, indent: u32, buffer: &mut String) {
        match &self {
            LastStat::Return{exp_list} => {
                *buffer += "return ";
                for i in 0..exp_list.len() {
                    exp_list[i]._prettyPrint(indent, buffer);
                    if i != exp_list.len() - 1 {*buffer += ", "}
                }
            },
            LastStat::Break => *buffer += "break",
        }
    }
}

impl LuaSyntax for Exp {
    fn _prettyPrint(&self, indent: u32, buffer: &mut String) {
        match &self {
            Exp::Nil => *buffer += "nil",
            Exp::Bool(val) => *buffer += if *val {"true"} else {"false"},
            Exp::Number(num) => *buffer += &*num.to_string(),
            Exp::String(str) => *buffer += &format!("\"{}\"", str),
            Exp::Var(var_name) => *buffer += var_name,
            Exp::Function{par_list, body} => {
                *buffer += "function(";
                for i in 0..par_list.len() {
                    *buffer += &par_list[i];
                    if i != par_list.len() - 1 {*buffer += ", ";}
                }
                *buffer += ")\n";
                body._prettyPrint(indent + 1, buffer);
                for _ in 0..indent {*buffer += "  ";}
                *buffer += "end";
            },
            Exp::Array{member_list} => {
                *buffer += "{\n";
                for i in 0..member_list.len() {
                    for _ in 0..(indent + 1) {*buffer += "  ";}
                    member_list[i]._prettyPrint(indent + 1, buffer);
                    if i != member_list.len() - 1 {*buffer += ","}
                    *buffer += "\n";
                }
                for _ in 0..indent {*buffer += "  ";}
                *buffer += "}";
            },
            Exp::Table{field_list} => {
                *buffer += "{\n";
                for i in 0..field_list.len() {
                    for _ in 0..(indent + 1) {*buffer += "  ";}
                    *buffer += &field_list[i].0;
                    *buffer += " = ";
                    field_list[i].1._prettyPrint(indent + 1, buffer);
                    if i != field_list.len() - 1 {*buffer += ","}
                    *buffer += "\n";
                }
                for _ in 0..indent {*buffer += "  ";}
                *buffer += "}";
            },
            Exp::Binop{left, op, right} => {
                left._prettyPrint(indent, buffer);
                *buffer += &format!(" {} ", op);
                right._prettyPrint(indent, buffer);
            },
            Exp::Unop{op, right} => {
                *buffer += &op;
                right._prettyPrint(indent, buffer);
            }
        }
    }
}