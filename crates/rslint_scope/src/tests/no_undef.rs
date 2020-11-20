//! Tests for detecting undefined variables

rule_test! {
    no_undef,
    rule_conf: |conf| conf.no_undef(true),
    filter: DatalogLint::is_no_undef,
    // Should pass
    { "var a = 1, b = 2; a + b;" },
    { "function f() { b; }", globals: ["b"] },
    { "a; function f() { b; a; }", globals: ["b", "a"] },
    { "function a(){}  a();" },
    { "function f(b) { b; }" },
    { "var a; a = 1; a++;" },
    { "var a; function f() { a = 1; }" },
    { "b++;", globals: ["b"] },
    { "window;", browser: true },
    { "require(\"a\");", node: true },
    { "Object; isNaN();", ecma: true },
    { "toString()" },
    { "hasOwnProperty()" },
    { "function evilEval(stuffToEval) { var ultimateAnswer; ultimateAnswer = 42; eval(stuffToEval); }" },
    { "typeof a" },
    { "typeof (a)" },
    { "var b = typeof a" },
    { "typeof a === 'undefined'" },
    { "if (typeof a === 'undefined') {}" },
    { "typeof ((((((a))))))" },
    { "typeof (1, 2, 3, a)" },
    { "typeof (1, 2, 3, (((1, 2, 3, a))))" },
    { "function foo() { var [a, b=4] = [1, 2]; return {a, b}; }" },
    { "var toString = 1;" },
    // FIXME: Requires JSX
    // { "var React, App, a=1; React.render(<App attr={a} />);" },
    { "function myFunc(...foo) {  return foo; }" },
    { "var console; [1,2,3].forEach(obj => { console.log(obj); });", node: true },
    { "var Foo; class Bar extends Foo { constructor() { super(); }}" },
    { "import Warning from '../lib/warning'; var warn = new Warning('text');", module: true },
    { "import * as Warning from '../lib/warning'; var warn = new Warning('text');", module: true },
    { "var a; [a] = [0];" },
    { "var a; ({a} = {});" },
    // FIXME: Assignment pattern parsing is broken
    // { "var obj; [obj.a, obj.b] = [0, 1];" },
    { "var a; ({b: a} = {});" },
    { "URLSearchParams;", browser: true },
    { "Intl;", browser: true },
    { "IntersectionObserver;", browser: true },
    { "Credential;", browser: true },
    { "requestIdleCallback;", browser: true },
    { "customElements;", browser: true },
    { "PromiseRejectionEvent;", browser: true },
    { "(foo, bar) => { foo ||= WeakRef; bar ??= FinalizationRegistry; }", es2021: true },
    { "function f() { b = 1; }", globals: ["b"] },
    { "function f() { b++; }", globals: ["b"] },
    { "b = 1;", globals: ["b"] },
    { "var b = 1;", globals: ["b"] },
    { "Array = 1;" },
    { "class A { constructor() { new.target; } }" },
    { "var { bacon, ...others } = stuff; foo(others)", globals: ["stuff", "foo"] },
    { "export * as ns from \"source\"", module: true },
    { "import.meta", module: true },
    { "let x; x.y" },
    { "let x = 10; while (true) { x += 1; }" },
    {
        "import {",
        "    UpdateEmpty, LoopContinues, X,",
        "    EnsureCompletion, Evaluate, Q,",
        "    GetValue, ToBoolean, Value,",
        "    Completion, NormalCompletion,",
        "} from '../bullshit.mjs';",
        "function* LabelledEvaluation_IterationStatement_DoWhileStatement({ Statement, Expression }, labelSet) {",
        "    // 1. Let V be undefined.",
        "    let V = Value.undefined;",
        "    // 2. Repeat,",
        "    while (true) {",
        "        // a. Let stmtResult be the result of evaluating Statement.",
        "        const stmtResult = EnsureCompletion(yield* Evaluate(Statement));",
        "        // b. If LoopContinues(stmtResult, labelSet) is false, return Completion(UpdateEmpty(stmtResult, V)).",
        "        if (LoopContinues(stmtResult, labelSet) === Value.false) {",
        "            return Completion(UpdateEmpty(stmtResult, V));",
        "        }",
        "        // c. If stmtResult.[[Value]] is not empty, set V to stmtResult.[[Value]].",
        "        if (stmtResult.Value !== undefined) {",
        "            V = stmtResult.Value;",
        "        }",
        "        // d. Let exprRef be the result of evaluating Expression.",
        "        const exprRef = yield* Evaluate(Expression);",
        "        // e. Let exprValue be ? GetValue(exprRef).",
        "        const exprValue = Q(GetValue(exprRef));",
        "        // f. If ! ToBoolean(exprValue) is false, return NormalCompletion(V).",
        "        if (X(ToBoolean(exprValue)) === Value.false) {",
        "            return NormalCompletion(V);",
        "        }",
        "    }",
        "}",
        module: true,
    },
    {
        "export function test() {",
        "   let thingy = {};",
        "   Object.entries(thingy).forEach(([x, y] => {",
        "      x += 1;",
        "      y += 1;",
        "   });",
        "}",
    },
    {
        "const bareKeyRe = /^[a-zA-Z_][a-zA-Z_0-9]*$/;",
        "const INSPECTORS = {",
        "    Something: (x) => {},",
        "    Object: (obj) => {",
        "        try {",
        "            let cache = [];",
        "            for (const key of obj) {",
        "                if (true) {",
        "                    cache.push(",
        "                        bareKeyRe.test(key.stringValue()) ? key.stringValue() : i(key)",
        "                    );",
        "                }",
        "            }",
        "        } catch { }",
        "    },",
        "};",
    },

    // Should fail
    { "a = 1;", errors: [DatalogLint::no_undef("a", 0..1)] },
    { "var a = b;", errors: [DatalogLint::no_undef("b", 8..9)] },
    { "function f() { b; }", errors: [DatalogLint::no_undef("b", 15..16)] },
    { "window", errors: [DatalogLint::no_undef("window", 0..6)] },
    { "require(\"a\");", errors: [DatalogLint::no_undef("require", 0..7)] },
    // FIXME: Requires JSX
    // { "var React; React.render(<img attr={a} />);", errors: ["a"] },
    // { "var React, App; React.render(<App attr={a} />);", errors: ["a"] },
    { "[a] = [0];", errors: [DatalogLint::no_undef("a", 1..2)] },
    { "({a} = {});", errors: [DatalogLint::no_undef("a", 2..3)] },
    { "({b: a} = {});", errors: [DatalogLint::no_undef("a", 5..6)] },
    // FIXME: Assignment pattern parsing is broken
    // { "[obj.a, obj.b] = [0, 1];", errors: [DatalogLint::no_undef("obj", 1..4), DatalogLint::no_undef("obj", 8..11)] },
    { "const c = 0; const a = {...b, c};", errors: [DatalogLint::no_undef("b", 27..28)] },
    { "function b() { var a; } a;", errors: [DatalogLint::no_undef("a", 24..25)] },
    { "function b(a) {} a;", errors: [DatalogLint::no_undef("a", 17..18)] },
    { "a; function b(a) {}", errors: [DatalogLint::no_undef("a", 0..1)] },
}

rule_test! {
    no_undef_typed_array_constructor,
    "corpus/TypedArrayConstructor.mjs",
    module: true,
    config: |_| Config::default(),
}
