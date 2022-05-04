use std::path::Path;

use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm
        .load_file(Path::new("./src/sample.js"))
        .expect("failed to load test.js");
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        // JscTarget defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let _module = parser
        .parse_module()
        .map_err(|mut e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");

    for body in _module.body {
        // TODO: swc がこの出力をどのようにハンドリングしているのか確認する
        // TODO: unwrap のハンドリングを適切にしないとネストした瞬間エラー
        // TODO: 再帰で取得する
        for decl in body.stmt().unwrap().decl().unwrap().var().unwrap().decls {
            println!("{:?}", decl.name.ident().unwrap().id.sym);
        }
    }
}
