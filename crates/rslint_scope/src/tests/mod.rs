#![cfg(test)]

macro_rules! rule_test {
    (
        $rule_name:ident,
        rule_conf: $rule_conf:expr,
        $(filter: $filter:expr,)?
        $({
            $($code:literal),+
            $(, globals: [$($global:literal),* $(,)?])?
            $(, browser: $browser:literal)?
            $(, node: $node:literal)?
            $(, ecma: $ecma:literal)?
            $(, module: $module:literal)?
            $(, es2021: $es2021:literal)?
            $(, errors: [$($error:expr),* $(,)?])?
            $(, config: $config:expr)?
            $(, trans: $trans:expr)?
            $(,)?
        }),* $(,)?
    ) => {
        #[test]
        #[allow(unused_imports, clippy::needless_update, clippy::redundant_closure_call)]
        fn $rule_name() {
            use crate::{
                tests::DatalogTestHarness,
                datalog::DatalogLint::{self, *},
                NoUnusedVarsConfig, RegexSet,
            };
            use ast::Span;
            use config::{Config, NoShadowHoisting::{self, *}};
            use std::borrow::Cow;
            use rayon::iter::{ParallelIterator, IntoParallelIterator};
            use tracing_subscriber::{layer::SubscriberExt, Layer, EnvFilter, Registry};

            // let filter_layer = EnvFilter::try_from_env("RSLINT_LOG")
            //     .or_else(|_| EnvFilter::try_new("trace"))
            //     .unwrap();
            // let fmt_layer = tracing_subscriber::fmt::layer().pretty();
            // let subscriber = Registry::default().with(filter_layer).with(fmt_layer);
            //
            // let _ = tracing::subscriber::set_global_default(subscriber);

            let config = ($rule_conf as fn(Config) -> Config)(Config::empty());
            let analyzer = DatalogTestHarness::new()
                $(.with_filter($filter as fn(&DatalogLint) -> bool))?;

            vec![$(
                analyzer
                    .test(vec![$($code,)+].join("\n"), stringify!($rule_name))
                    $(.with_globals(vec![$(Cow::Borrowed($global)),*]))?
                    $(.with_browser($browser))?
                    $(.with_node($node))?
                    $(.with_ecma($ecma))?
                    $(.is_module($module))?
                    $(.with_es2021($es2021))?
                    $(.with_errors(vec![$($error),*]))?
                    $(.with_transaction($trans))?
                    .with_config($(($config as fn(Config) -> Config))?(config.clone())),
            )?]
            .into_par_iter()
            .for_each(|test| test.run());

            analyzer.report_outcome();
        }
    };

    (
        $rule_name:ident,
        $code:literal
        $(, filter: $filter:expr)?
        $(, globals: [$($global:literal),* $(,)?])?
        $(, browser: $browser:literal)?
        $(, node: $node:literal)?
        $(, ecma: $ecma:literal)?
        $(, module: $module:literal)?
        $(, es2021: $es2021:literal)?
        $(, errors: [$($error:expr),* $(,)?])?
        $(, config: $config:expr)?
        $(,)?
    ) => {
        #[test]
        #[allow(unused_imports, clippy::needless_update, clippy::redundant_closure_call)]
        fn $rule_name() {
            use crate::{
                tests::DatalogTestHarness,
                datalog::DatalogLint::{self, *},
            };
            use ast::Span;
            use config::{Config, NoShadowHoisting::{self, *}};
            use std::borrow::Cow;
            use rayon::iter::{ParallelIterator, IntoParallelIterator};

            let analyzer = DatalogTestHarness::new()
                $(.with_filter($filter as fn(&DatalogLint) -> bool))?;

            analyzer
                .test(include_str!($code), stringify!($rule_name))
                $(.with_globals(vec![$(Cow::Borrowed($global)),*]))?
                $(.with_browser($browser))?
                $(.with_node($node))?
                $(.with_ecma($ecma))?
                $(.is_module($module))?
                $(.with_es2021($es2021))?
                $(.with_errors(vec![$($error),*]))?
                .with_config($(($config as fn(Config) -> Config))?(Config::empty()))
                .run();

            analyzer.report_outcome();
        }
    };
}

mod no_shadow;
mod no_undef;
mod no_unused_labels;
mod no_unused_vars;
mod typeof_undef;
mod use_before_def;

use crate::{
    datalog::DatalogLint,
    globals::{JsGlobal, BROWSER, BUILTIN, ES2021, NODE},
    DatalogResult, ScopeAnalyzer,
};
use ast::FileId;
use config::Config;
use rslint_parser::{parse_module, parse_text};
use std::{
    borrow::Cow,
    fs::{self, OpenOptions},
    io::Write as _,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

struct DatalogTestHarness {
    datalog: ScopeAnalyzer,
    passing: AtomicUsize,
    failing: AtomicUsize,
    counter: AtomicUsize,
    filter: Option<fn(&DatalogLint) -> bool>,
}

impl DatalogTestHarness {
    pub fn new() -> Self {
        Self {
            datalog: ScopeAnalyzer::new().expect("failed to create ddlog instance"),
            passing: AtomicUsize::new(0),
            failing: AtomicUsize::new(0),
            counter: AtomicUsize::new(0),
            filter: None,
        }
    }

    pub fn with_filter(mut self, filter: fn(&DatalogLint) -> bool) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn test<C, R>(&self, code: C, rule_name: R) -> TestCase<'_>
    where
        C: Into<Cow<'static, str>>,
        R: Into<Cow<'static, str>>,
    {
        TestCase::new(
            self,
            code.into(),
            rule_name.into(),
            self.counter.fetch_add(1, Ordering::SeqCst),
        )
    }

    pub fn report_outcome(self) {
        if self.failing.load(Ordering::SeqCst) != 0 {
            panic!(
                "Test failed with {} passing, {} failing, logs stored in `{}/output.log/`",
                self.passing.load(Ordering::SeqCst),
                self.failing.load(Ordering::SeqCst),
                env!("CARGO_MANIFEST_DIR"),
            );
        }
    }
}

struct TestCase<'a> {
    rule_name: Cow<'static, str>,
    code: Cow<'static, str>,
    globals: Vec<Cow<'static, str>>,
    browser: bool,
    node: bool,
    ecma: bool,
    is_module: bool,
    es2021: bool,
    errors: Vec<DatalogLint>,
    harness: &'a DatalogTestHarness,
    id: usize,
    config: Config,
    transaction: Option<fn(&ScopeAnalyzer, FileId) -> DatalogResult<()>>,
}

impl<'a> TestCase<'a> {
    pub fn new(
        harness: &'a DatalogTestHarness,
        code: Cow<'static, str>,
        rule_name: Cow<'static, str>,
        id: usize,
    ) -> Self {
        fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/output.log")).unwrap();

        harness.datalog.outputs().with_output_file(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(
                    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/output.log")).join(&*rule_name),
                )
                .unwrap(),
        );

        Self {
            rule_name,
            code,
            globals: Vec::new(),
            browser: false,
            node: false,
            ecma: false,
            is_module: false,
            es2021: false,
            errors: Vec::new(),
            harness,
            id,
            config: Config::default(),
            transaction: None,
        }
    }

    pub fn with_globals(mut self, globals: Vec<Cow<'static, str>>) -> Self {
        self.globals.extend(globals);
        self
    }

    pub fn with_browser(mut self, browser: bool) -> Self {
        self.browser = browser;
        self
    }

    pub fn with_node(mut self, node: bool) -> Self {
        self.node = node;
        self
    }

    pub fn with_ecma(mut self, ecma: bool) -> Self {
        self.ecma = ecma;
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_module(mut self, is_module: bool) -> Self {
        self.is_module = is_module;
        self
    }

    pub fn with_es2021(mut self, es2021: bool) -> Self {
        self.es2021 = es2021;
        self
    }

    pub fn with_errors(mut self, errors: Vec<DatalogLint>) -> Self {
        self.errors.extend(errors);
        self
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn with_transaction(
        mut self,
        transaction: fn(&ScopeAnalyzer, FileId) -> DatalogResult<()>,
    ) -> Self {
        self.transaction = Some(transaction);
        self
    }

    // TODO: This is so ugly
    pub fn run(mut self) {
        let file_id = FileId::new(self.id as u32);

        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/output.log"))
            .join(format!("{}-{}", self.rule_name, self.id));
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }

        let mut failed = false;
        let ast = if self.is_module {
            parse_module(&*self.code, 0).syntax()
        } else {
            parse_text(&*self.code, 0).syntax()
        };

        if self.config.no_unused_vars {
            self.harness
                .datalog
                .no_unused_vars(file_id, Some(Default::default()))
                .unwrap();
        }
        if let Some(transaction) = self.transaction {
            transaction(&self.harness.datalog, file_id).unwrap();
        }

        self.harness
            .datalog
            .datalog
            .inject_user_globals(
                file_id,
                &self
                    .globals
                    .iter()
                    .map(|g| JsGlobal::new(g.to_string(), false))
                    .collect::<Vec<_>>(),
            )
            .unwrap();

        self.harness
            .datalog
            .datalog
            .inject_user_globals(file_id, BUILTIN)
            .unwrap();

        if self.browser {
            self.harness
                .datalog
                .datalog
                .inject_user_globals(file_id, BROWSER)
                .unwrap();
        }

        if self.node {
            self.harness
                .datalog
                .datalog
                .inject_user_globals(file_id, NODE)
                .unwrap();
        }

        if self.ecma || self.es2021 {
            self.harness
                .datalog
                .datalog
                .inject_user_globals(file_id, ES2021)
                .unwrap();
        }

        self.harness
            .datalog
            .analyze(file_id, &ast, self.config)
            .expect("failed datalog transaction");

        for err in self.errors.iter_mut() {
            *err.file_id_mut() = file_id;
        }

        let mut errors = self.harness.datalog.get_lints(file_id).unwrap();
        if let Some(filter) = self.harness.filter {
            errors = errors.into_iter().filter(filter).collect();
        }

        for error in self.errors.iter() {
            if let Some(idx) = errors.iter().position(|err| err == error) {
                errors.remove(idx);
            } else {
                failed = true;
            }
        }

        if failed || !errors.is_empty() {
            self.harness.failing.fetch_add(1, Ordering::SeqCst);

            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(path.join("failure"))
                .unwrap();

            write!(
                &mut file,
                "============ FAILURE ============\n\n\
                => Source:\n{}\n\n\
                => AST:\n{:#?}\n\n\
                => Expected:\n{:#?}\n\n\
                => Got:\n{:#?}\n\n\
                => Outputs:\n{:#?}\n\n\
                ============ END FAILURE ============\n\n",
                ast.text(),
                &ast,
                self.errors,
                self.harness.datalog.get_lints(file_id).unwrap(),
                // self.harness
                //     .datalog
                //     .dump_inputs()
                //     .unwrap()
                //     .lines()
                //     .filter(|line| line.contains(&format!("ast::FileId{{.id = {}}}", file_id.id)))
                //     .collect::<Vec<_>>()
                //     .join("\n"),
                self.harness.datalog.outputs(),
            )
            .unwrap();
        } else {
            fs::remove_dir_all(&path).unwrap();
            self.harness.passing.fetch_add(1, Ordering::SeqCst);
        }
    }
}
