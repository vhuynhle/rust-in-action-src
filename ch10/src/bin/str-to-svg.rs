use svg::{
    node::element::{
        path::{Command, Data, Position},
        Path, Rectangle,
    },
    Document,
};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;
const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;
const STROKE_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}
use crate::Operation::*;

enum Orientation {
    North,
    South,
    East,
    West,
}

/// Maintains the state of the diagram
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

use crate::Orientation::*;

impl Artist {
    fn new() -> Self {
        Artist {
            x: HOME_X,
            y: HOME_Y,
            heading: Orientation::North,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y -= distance,
            South => self.y += distance,
            East => self.x += distance,
            West => self.x -= distance,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            East => South,
            West => North,
        };
    }

    /// If the turtle is out of bound, returns in to the center
    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x >= WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y >= HEIGHT {
            self.y = HOME_Y;
            self.heading = South;
        }
    }
}

fn convert(operations: &[Operation]) -> Vec<Command> {
    let mut turtle = Artist::new();
    let mut path_data = vec![];

    // Set the initial position
    let start_at_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());
    path_data.push(start_at_home);

    for &op in operations {
        match op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => {
                eprintln!("Warning: Illegal byte encountered: {:?}", byte);
            }
        }

        let path_segment = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        path_data.push(path_segment);

        turtle.wrap();
    }

    path_data
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", r#"style="outline: 5px solid #800000;""#)
        .add(background)
        .add(sketch)
        .add(border);

    document
}

/// Generate a list of operation from a byte stream
/// Note: The mapping here is almost random, and doesn't have deep meaning.
/// Other mappings can be equally valid.
fn parse(input: &str) -> Vec<Operation> {
    input
        .as_bytes()
        .iter()
        .map(|&byte| match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (byte - b'0') as isize;
                Forward(distance * (HEIGHT / 10))
            }
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte),
        })
        .collect()
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("Usage: str-to-svg <STRING> [output_file]");
    let default = format!("{}.svg", input);
    let save_to = std::env::args().nth(2).unwrap_or(default);

    let operations = parse(&input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(&save_to, &document).unwrap();
}
