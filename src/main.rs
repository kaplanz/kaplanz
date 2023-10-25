use std::ops::BitOr;

fn main() {
    use Area::*;
    use Languages::*;
    use Skills::*;
    use Tools::*;

    let zakhary = Person {
        about: Contact {
            name: "Zakhary Kaplan",
            email: "me@zakhary.dev",
            country: "🇨🇦",
        },
        edu: vec![Degree {
            school: "University of Toronto",
            discipline: "Computer Engineering",
            level: Level::Bachelors,
            grad: 2023,
        }],
        area: Hardware | Software,
        work: Developer {
            languages: vec![Assembly, C, Cpp, Lua, Python, Rust, Verilog],
            skills: vec![Algorithms, Compilers, Embedded, Emulation, Fpga],
            tools: vec![Docker, GnuMake, Neovim, Tmux, Unix, Zsh],
            os: "*nix",
            editor: "(neo)?vim",
        },
    };

    println!("{:#?};", zakhary);
}

#[derive(Debug)]
enum Area {
    Hardware = 0b01,
    Software = 0b10,
}

impl BitOr for Area {
    type Output = Vec<Self>;

    fn bitor(self, rhs: Self) -> Self::Output {
        vec![self, rhs]
    }
}

#[derive(Debug)]
enum Languages {
    Assembly,
    C,
    Cpp,
    Python,
    Lua,
    Rust,
    Verilog,
}

#[derive(Debug)]
enum Skills {
    Algorithms,
    Compilers,
    Embedded,
    Emulation,
    Fpga,
}

#[derive(Debug)]
enum Tools {
    Docker,
    GnuMake,
    Neovim,
    Tmux,
    Unix,
    Zsh,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    about: Contact,
    edu: Vec<Degree>,
    area: Vec<Area>,
    work: Developer,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Contact {
    name: &'static str,
    email: &'static str,
    country: &'static str,
}

#[derive(Debug)]
enum Level {
    Bachelors,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Degree {
    school: &'static str,
    level: Level,
    discipline: &'static str,
    grad: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Developer {
    languages: Vec<Languages>,
    skills: Vec<Skills>,
    tools: Vec<Tools>,
    os: &'static str,
    editor: &'static str,
}
