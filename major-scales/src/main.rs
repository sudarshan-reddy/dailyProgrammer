use std::error::Error;
struct Scales {
    scales: [&'static str; 12],
}

impl Scales {
    fn new() -> Self {
        Scales {
            scales: [
                "C",
                "C#",
                "D",
                "D#",
                "E",
                "F",
                "F#",
                "G",
                "G#",
                "A",
                "A#",
                "B",
            ],
        }
    }

    fn solfege_to_scale(&self, sc: &str, s: &str) -> Result<&str, Box<Error>> {
        let offset = match s {
            "Do" => Ok(0),
            "Re" => Ok(2),
            "Mi" => Ok(4),
            "Fa" => Ok(5),
            "So" => Ok(7),
            "La" => Ok(9),
            "Ti" => Ok(11),
            _ => Err("error processing solfege"),
        }?;

        let index = self.scales
            .iter()
            .position(|&s| s == sc)
            .ok_or("invalid scale")?;

        let mut op = offset + index;
        if op > 11 {
            op -= 12;
        }

        println!("{}", self.scales[op]);
        Ok("test")
    }
}

fn main() {
    let sc = Scales::new();
    let r = sc.solfege_to_scale("C", "Do");
    println!("{:?}", r);
    let r = sc.solfege_to_scale("C", "Re");
    println!("{:?}", r);
    let r = sc.solfege_to_scale("C", "Mi");
    println!("{:?}", r);
    let r = sc.solfege_to_scale("A#", "Fa");
    println!("{:?}", r);
}
