use serenity::json::Value;

pub struct Arg<'a> {
    pub name: &'a str,
    pub value: Value,
}

pub struct CommandInfo<'a> {
    pub name: &'a str,
    pub args: Vec<Arg<'a>>,
}

impl CommandInfo<'_> {
    pub fn get_arg(self: &Self, name: &str) -> Option<Value> {
        match self.args.iter().find(|arg| arg.name == name) {
            Some(arg) => Some(arg.value.clone()),
            _ => None,
        }
    }
}
