mod constants;
mod parser;
mod types;
mod utils;

pub use parser::parse_bsp;
pub use types::Bsp;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let file = include_bytes!("tests/bsp_compile.bsp");

        parse_bsp(file).unwrap();
    }

    #[test]
    fn parse2() {
        let file = include_bytes!("tests/normal.bsp");

        parse_bsp(file).unwrap();
    }

    #[test]
    fn parse_write() {
        let file = include_bytes!("tests/bsp_compile.bsp");

        let bsp = Bsp::from_bytes(file).unwrap();

        let res = bsp.write_to_bytes();

        println!("file {} res {}", file.len(), res.len());

        // assert_eq!(file, res.as_slice());
        bsp.write_to_file("/home/khang/gchimp/bsp/src/tests/bsp_out.bsp")
            .unwrap()
    }

    #[test]
    fn parse_write2() {
        let file = include_bytes!("tests/normal.bsp");
        let bsp = Bsp::from_bytes(file).unwrap();
        bsp.write_to_file("/home/khang/gchimp/bsp/src/tests/normal_out.bsp")
            .unwrap();

        // assert_eq!(file, res.as_slice());
    }

    #[test]
    fn parse_write_parse() {
        let file = include_bytes!("tests/normal.bsp");
        let bsp = Bsp::from_bytes(file).unwrap();
        let file_again = bsp.write_to_bytes();
        Bsp::from_bytes(&file_again).unwrap();
    }

    #[test]
    fn parse_write_parse2() {
        let file = include_bytes!("tests/bsp_compile.bsp");
        let bsp = Bsp::from_bytes(file).unwrap();
        let file_again = bsp.write_to_bytes();
        Bsp::from_bytes(&file_again).unwrap();
    }

    #[test]
    fn parse_write_parse3() {
        let file = include_bytes!("tests/surf_ben10.bsp");
        let bsp = Bsp::from_bytes(file).unwrap();

        println!("nummodel {}", bsp.models.len());

        bsp.models.iter().for_each(|model| {
            println!("model sufrace count is {}", model.face_count);
        });
    }
}
