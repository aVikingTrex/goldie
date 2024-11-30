use super::*;

use serde::Serialize;

#[test]
fn goldie_golden_file() {
    let tests = [
        (
            ("/repo/src/lib.rs", "crate::tests::func"),
            "/repo/src/testdata/func.golden",
        ),
        (
            ("/repo/src/foo.rs", "crate::foo::tests::func"),
            "/repo/src/testdata/func.golden",
        ),
        (
            ("/repo/src/foo/mod.rs", "crate::foo::tests::func"),
            "/repo/src/foo/testdata/func.golden",
        ),
        (
            ("/repo/src/foo/bar.rs", "crate::foo::bar::tests::func"),
            "/repo/src/foo/testdata/func.golden",
        ),
        (
            ("/repo/src/bin/foo.rs", "crate::tests::func"),
            "/repo/src/bin/testdata/func.golden",
        ),
    ];

    for ((file, path), exp) in tests {
        let g = Goldie::new(file, path);
        assert_eq!(g.golden_file, Path::new(exp))
    }
}

#[test]
fn goldie_assert() {
    crate::assert!("testing...\n");
}

#[test]
fn goldie_assert_custom_path() {
    crate::assert!("goldie_assert-custom", "custom testing...\n");
}

#[test]
fn goldie_assert_debug() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct User {
        name: &'static str,
        surname: &'static str,
    }

    let u = User {
        name: "Steve",
        surname: "Harrington",
    };

    crate::assert_debug!(&u);
}
#[test]
fn goldie_assert_debug_custom_path() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct User {
        name: &'static str,
        surname: &'static str,
    }

    let u = User {
        name: "custom Steve",
        surname: "custom Harrington",
    };

    crate::assert_debug!("goldie_assert_debug-custom", &u);
}

#[test]
fn goldie_assert_template() {
    #[derive(Serialize)]
    struct Context {
        test: &'static str,
    }
    let ctx = Context { test: "testing..." };
    crate::assert_template!(&ctx, "Such testing...\n");
}

#[test]
fn goldie_assert_template_custom_path() {
    #[derive(Serialize)]
    struct Context {
        test: &'static str,
    }
    let ctx = Context { test: "custom testing..." };
    crate::assert_template!("goldie_assert_template-custom", &ctx, "Such custom testing...\n");
}

#[test]
fn goldie_assert_json() {
    #[derive(Serialize)]
    struct User {
        name: &'static str,
        surname: &'static str,
    }

    let u = User {
        name: "Steve",
        surname: "Harrington",
    };

    crate::assert_json!(&u);
}

#[test]
fn goldie_assert_json_custom_path() {
    #[derive(Serialize)]
    struct User {
        name: &'static str,
        surname: &'static str,
    }

    let u = User {
        name: "custom Steve",
        surname: "custom Harrington",
    };

    crate::assert_json!("goldie_assert_json-custom", &u);
}
