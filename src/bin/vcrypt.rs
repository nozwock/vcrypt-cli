use vcrypt::veracrypt;
use veracrypt::{Arg, Flag};

fn main() {
    println!(
        "{:?}",
        veracrypt::Cli::new()
            .add_arg(Arg::Password, "secretpass")
            .add_flag(Flag::Mount)
            .add_arg(Arg::Pim, "500")
            .to_string()
    );
}
