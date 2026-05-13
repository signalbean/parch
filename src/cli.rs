pub struct Args {
    pub id: Option<u64>,
    pub verbose: bool,
    pub local: bool,
}

pub fn parse() -> Result<Args, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let Some(first) = args.next() else {
        return Err(format!("No arguments.\nTry '{} help'", env!("CARGO_PKG_NAME")).into());
    };

    let mut config = ParseState::default();

    for arg in std::iter::once(first).chain(args) {
        match arg.as_str() {
            "-h" | "help" => exit_with_help(),
            "-v" | "version" => exit_with_version(),

            "local" => config.local = true,
            "-V" | "verbose" => config.verbose = true,
            "id" => config.parse_id(&mut std::env::args())?,
            _ => {
                return Err(
                    format!("Unknown: {}\nTry '{} help'", arg, env!("CARGO_PKG_NAME")).into(),
                );
            }
        }
    }

    config.validate()
}

#[derive(Default)]
struct ParseState {
    id: Option<u64>,
    verbose: bool,
    local: bool,
}

impl ParseState {
    fn parse_id(&mut self, args: &mut std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
        let id_str = args.next().ok_or("id requires a value")?;
        self.id = Some(id_str.parse().map_err(|_| "Invalid ID")?);
        Ok(())
    }

    fn validate(self) -> Result<Args, Box<dyn std::error::Error>> {
        if self.local && self.id.is_some() {
            return Err("Can't use local with id".into());
        }
        if self.id.is_none() && !self.local {
            return Err("Post ID or local is required".into());
        }

        Ok(Args {
            id: self.id,
            verbose: self.verbose,
            local: self.local,
        })
    }
}

fn exit_with_help() -> ! {
    print_help();
    std::process::exit(0)
}

fn exit_with_version() -> ! {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0)
}

fn print_help() {
    println!(
        "{} v{}\nFetch and apply wallpapers from Konachan\n\nUSAGE:\n    {} [OPTIONS]\n\nOPTIONS:\n    \n    local                Use local images\n    \n    id <ID>              By post id\n    \n    -V, verbose          Verbose mode\n    \n    -h, help             Help menu\n    \n    -v, version          Version",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME")
    );
}
