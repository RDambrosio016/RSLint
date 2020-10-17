use crate::{parse_module, ParserError};
use expect_test::expect_file;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn parser_smoke_test() {
    let src = r#"
    console.log("hello world");
    "#;

    assert!(parse_module(src, 0).ok().is_ok());
}

fn test_data_dir() -> PathBuf {
    project_dir().join("rslint_parser/test_data")
}

#[test]
fn parser_tests() {
    dir_tests(&test_data_dir(), &["inline/ok"], "rast", |text, path| {
        let parse = parse_module(text, 0);
        let errors = parse.errors();
        assert_errors_are_absent(&errors, path);
        format!("{:#?}", parse.syntax())
    });
    dir_tests(&test_data_dir(), &["inline/err"], "rast", |text, path| {
        let parse = parse_module(text, 0);
        let errors = parse.errors();
        assert_errors_are_present(&errors, path);
        let mut files = SimpleFiles::new();
        files.add(
            path.file_name().unwrap().to_string_lossy().to_string(),
            text,
        );
        let mut ret = format!("{:#?}", parse.syntax());

        for diag in parse.errors() {
            let mut buf = Buffer::no_color();

            emit(&mut buf, &Config::default(), &files, diag).expect("failed to emit diagnostic");
            ret.push_str(&format!(
                "--\n{}",
                std::str::from_utf8(buf.as_slice()).expect("non utf8 in error buffer")
            ));
        }
        ret.push_str(&format!("--\n{}", text));
        ret
    });
}

fn dir_tests<F>(test_data_dir: &Path, paths: &[&str], outfile_extension: &str, f: F)
where
    F: Fn(&str, &Path) -> String,
{
    for (path, input_code) in collect_js_files(test_data_dir, paths) {
        let actual = f(&input_code, &path);
        let path = path.with_extension(outfile_extension);
        expect_file![path].assert_eq(&actual)
    }
}

pub fn project_dir() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).parent().unwrap().to_path_buf()
}

fn collect_js_files(root_dir: &Path, paths: &[&str]) -> Vec<(PathBuf, String)> {
    paths
        .iter()
        .flat_map(|path| {
            let path = root_dir.to_owned().join(path);
            js_files_in_dir(&path).into_iter()
        })
        .map(|path| {
            let text = fs::read_to_string(&path).expect("Could not read js file");
            (path, text)
        })
        .collect()
}

fn js_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut acc = Vec::new();
    println!("{:?}", dir);
    for file in fs::read_dir(&dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().unwrap_or_default() == "js" {
            acc.push(path);
        }
    }
    acc.sort();
    acc
}

fn assert_errors_are_present(errors: &[ParserError], path: &Path) {
    assert!(
        !errors.is_empty(),
        "There should be errors in the file {:?}",
        path.display()
    );
}

fn assert_errors_are_absent(errors: &[ParserError], path: &Path) {
    assert!(
        errors.is_empty(),
        "There should be no errors in the file {:?}",
        path.display(),
    );
}
