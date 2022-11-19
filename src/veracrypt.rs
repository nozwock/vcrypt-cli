const CMD: &str = "veracrypt -t";

pub enum Flag {
    Mount,
    Dismount,
}
pub enum Arg {
    Pim,
    Password,
}

impl Flag {
    fn to_str<'a>(&self) -> &'a str {
        match self {
            Flag::Mount => "--mount",
            Flag::Dismount => "--dismount",
        }
    }
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl Arg {
    fn with(&self, val: &str) -> String {
        match self {
            Arg::Pim => format!("{} {}", "--pim", val),
            Arg::Password => format!("{} {}", "--password", val),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Cli {
    cmd: String,
    args: Option<Vec<String>>,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            cmd: CMD.to_string(),
            ..Default::default()
        }
    }
    pub fn add_flag(self, flag: Flag) -> Self {
        let mut cli = self;
        match cli.args {
            Some(ref mut args) => {
                args.push(flag.to_string());
            }
            None => {
                cli.args = Some(vec![flag.to_string()]);
            }
        };
        cli
    }
    pub fn add_arg(self, arg: Arg, val: &str) -> Self {
        let mut cli = self;
        match cli.args {
            Some(ref mut args) => {
                args.push(arg.with(val));
            }
            None => {
                cli.args = Some(vec![arg.with(val)]);
            }
        };
        cli
    }
    pub fn to_string(&self) -> String {
        match self.args {
            Some(ref args) => {
                format!(
                    "{}{}",
                    self.cmd,
                    args.iter()
                        .fold(String::new(), |acc, x| { format!("{} {}", acc, x) })
                )
            }
            None => self.cmd.to_string(),
        }
    }
}
