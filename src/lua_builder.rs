pub struct LuaAssignment {
    pub var_name: String,
    pub exp: LuaExp,
}

pub enum LuaExp {
    Nil,
    False,
    True,
    Number(f32),
    String(String),
    Function{parameters: Vec<String>, body: Vec<LuaExp>},
    // Assignment(String, LuaExp),
    Return(Box<LuaExp>),
    Array(Vec<LuaExp>),
    Table(Vec<(String, LuaExp)>),
}

impl LuaExp {
    pub fn build(&self, indent: u32) -> String {
        match self {
            LuaExp::Nil => String::from("nil"),
            LuaExp::False => String::from("false"),
            LuaExp::True => String::from("true"),
            LuaExp::Number(num) => format!("{}", num),
            LuaExp::String(str) => format!("\"{}\"", str),
            LuaExp::Function{parameters, body} => {
                let mut result = String::from("function(");
                for param in parameters {
                    result += param;
                    result += ",";
                }
                result += ")";
                for exp in body {
                    result += "\n";
                    for _ in 0..(indent + 1) { result += "  "; }
                    result += &*exp.build(indent + 1);
                }
                result += "\n";
                for _ in 0..indent {result += "  "; }
                result += "end";
                result
            },
            LuaExp::Return(exp) => format!("return {}", exp.build(indent)),
            LuaExp::Array(expressions) => {
                let mut result = String::from("\n");
                for _ in 0..indent {result += "  ";}
                result += "{";
                for exp in expressions {
                    result += "\n";
                    for _ in 0..(indent + 1) {result += "  ";}
                    result += &*exp.build(indent + 1);
                    result += ",";
                }
                result += "\n";
                for _ in 0..indent {result += "  ";}
                result += "}";
                result
            },
            LuaExp::Table(kv) => {
                let mut result = String::from("\n");
                for _ in 0..indent {result += "  "; }
                result += "{";
                for (key, exp) in kv {
                    result += "\n";
                    for _ in 0..(indent + 1) { result += "  ";}
                    result += key;
                    result += " = ";
                    result += &*exp.build(indent + 1);
                    result += ",";
                }
                result += "\n";
                for _ in 0..indent {result += "  ";}
                result += "}";
                result
            }
        }
    }
}

