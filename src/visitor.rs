use syntax::{ visit };
use syntax::ast::{ FnDecl, Block, NodeId, Expr, Pat_, Expr_, Lit_, StrStyle, Stmt, Stmt_ };
use syntax::visit::{ FnKind };
use syntax::codemap::{ Span };

macro_rules! unsupported {
    ($node:expr) => (panic!("{:?} is not yet supported", $node))
}

pub struct RjscVisitor {
    indent: u32,
    at_start: bool,
    pub seen_main: bool,
}

impl RjscVisitor {
    pub fn new() -> RjscVisitor {
        RjscVisitor {
            indent: 0,
            at_start: true,
            seen_main: false,
        }
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn outdent(&mut self) {
        self.indent -= 1;
    }

    fn do_indent(&self) {
        if self.at_start {
            for _ in 0..self.indent {
                print!("    ");
            }
        }
    }

    fn println(&mut self, s: String) {
        self.do_indent();
        println!("{}", s);
        self.at_start = true;
    }

    fn print(&mut self, s: String) {
        self.do_indent();
        print!("{}", s);
        self.at_start = false;
    }
}

impl<'v> visit::Visitor<'v> for RjscVisitor {
    fn visit_fn(&mut self, fk: FnKind<'v>, fd: &'v FnDecl, b: &'v Block, s: Span, _: NodeId) {
        match fk {
            FnKind::FkItemFn(ident, _, _, _) => {
                if ident.as_str() == "main" {
                    self.seen_main = true;
                }
                self.print(format!("function {}(", ident));
            },
            _ => unimplemented!(),
        };
        for arg in &(*fd).inputs {
            match arg.pat.node {
                Pat_::PatIdent(_, ref node, _) => self.print(format!("{}", node.node.name)),
                _ => unimplemented!(),
            }
        }
        self.print(format!(") "));
        visit::walk_fn(self, fk, fd, b, s);
    }

    fn visit_block(&mut self, b: &'v Block) {
        self.println(format!("{{"));
        self.indent();
        visit::walk_block(self, b);
        self.outdent();
        self.println(format!("}}"));
    }

    fn visit_stmt(&mut self, s: &'v Stmt) {
        match s.node {
            Stmt_::StmtSemi(..) => {
                visit::walk_stmt(self, s);
                self.println(format!(";"));
            },
            _ => unsupported!(s.node),
        }
    }

    fn visit_expr(&mut self, ex: &'v Expr) {
        match ex.node {
            Expr_::ExprCall(ref fun, ref args) => {
                self.visit_expr(fun);
                self.print(format!("("));
                for arg in args {
                    self.visit_expr(arg);
                }
                self.print(format!(")"));
            },
            Expr_::ExprLit(ref lit) => {
                match (*lit).node {
                    Lit_::LitStr(ref st, StrStyle::CookedStr) => {
                        self.print(format!("\"{}\"", st));
                    },
                    _ => unsupported!((*lit).node),
                }
            },
            Expr_::ExprPath(_, ref path) => {
                let mut first = true;
                for segment in &path.segments {
                    if !first { self.print(format!(".")) }
                    self.print(format!("{}", segment.identifier));
                    first = false;
                }
            },
            Expr_::ExprBlock(..) => {
                visit::walk_expr(self, ex);
            },
            _ => unsupported!(ex.node),
        }
    }
}

