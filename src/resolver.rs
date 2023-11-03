use crate::statement::Statement;

struct Resolver {
    pub line: usize,
    pub position: usize,
}

pub fn resolve(statements: &mut Vec<Statement>) {
    let resolver = Resolver {
        line: 0,
        position: 0,
    };
    todo!();
}
