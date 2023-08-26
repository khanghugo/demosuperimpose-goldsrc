#[macro_export]
macro_rules! write_demo {
    ($demo_name:literal, $demo:ident) => {{
        let mut out = writer::DemoWriter::new(String::from($demo_name));
        out.write_file($demo);
    }};
}
