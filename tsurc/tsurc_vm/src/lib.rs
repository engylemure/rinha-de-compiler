use tsurc_parser::ast::File;

pub fn execute(file: File) {
    use tsurc_parser::ast::Term::*;
    match file.expression.as_ref().as_ref() {
        Parameter(_) => todo!(),
        Var(_) => todo!(),
        Fn(_) => todo!(),
        Let(_) => todo!(),
        Call(_) => todo!(),
        Str(_) => todo!(),
        Binary(_) => todo!(),
        If(_) => todo!(),
        Bool(_) => todo!(),
        First(_) => todo!(),
        Second(_) => todo!(),
        Print(_) => todo!(),
        Int(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
