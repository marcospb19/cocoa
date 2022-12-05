use glob::glob;

#[test]
fn test_cacau_passes() {
    for path in glob("tests/pass/*").unwrap() {
        let path = path.unwrap();
        let input = test_utils::read_file(&path);

        let ast = cocoa::parse_input(&input).unwrap_or_else(|err| {
            panic!(
                "expected file at '{path}' to parse succcessfully\n\
                 parsing failed: {error:#?}",
                path = path.display(),
                error = Err::<(), _>(err)
            )
        });

        cocoa::interpret_ast(&ast).unwrap_or_else(|err| {
            panic!(
                "expected file at '{path}' to be interpreted successfully\n\
                 it failed: {error:#?}",
                path = path.display(),
                error = Err::<(), _>(err)
            )
        });
    }
}

#[test]
fn test_cacau_fail_to_interpret() {
    for path in glob("tests/fail-interpret/*").unwrap() {
        let path = path.unwrap();
        let input = test_utils::read_file(&path);

        let ast = cocoa::parse_input(&input).unwrap_or_else(|err| {
            panic!(
                "expected file at '{path}' to parse succcessfully\n\
                 parsing failed: {error:#?}",
                path = path.display(),
                error = Err::<(), _>(err)
            )
        });

        if cocoa::interpret_ast(&ast).is_ok() {
            panic!(
                "expected file at '{path}' to fail to be interpreted\n\
                 it succeeded: {ast:#?}",
                path = path.display(),
                ast = Ok::<_, ()>(ast)
            );
        }
    }
}

mod test_utils {
    use std::path::Path;

    use fs_err as fs;

    pub fn read_file(path: impl AsRef<Path>) -> String {
        fs::read_to_string(path.as_ref()).unwrap_or_else(|err| {
            panic!(
                "Failed to read file '{}': {err}.",
                path.as_ref().to_str().unwrap()
            );
        })
    }
}
