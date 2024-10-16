use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &String) -> io::Result<()> {
    define_ast(
        &output_dir,
        &"Expr".to_string(),
        &vec!["error", "token", "object"],
        &vec![
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Option<Object> value".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
            "Variable : Token name".to_string(),
        ],
    )?;

    define_ast(
        &output_dir,
        &"Stmt".to_string(),
        &vec!["error", "expr", "token"],
        &vec![
            "Expression : Expr expression".to_string(),
            "Print : Expr expression".to_string(),
            "Var : Token name, Option<Expr> initializer".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(
    output_dir: &String,
    base_name: &String,
    imports: &[&str],
    types: &[String],
) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    for i in imports {
        write!(file, "use crate::{}::*;\n", i)?;
    }

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.split(",");
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    write!(file, "\npub enum {base_name} {{\n")?;
    for t in &tree_types {
        write!(file, "    {}({}),\n", t.base_class_name, t.class_name)?;
    }
    write!(file, "}}\n\n")?;

    write!(file, "impl {} {{\n", base_name)?;
    write!(file, "    pub fn accept<T>(&self, {}_visitor: &dyn {base_name}Visitor<T>) -> Result<T, LoxError> {{\n", base_name.to_lowercase())?;
    write!(file, "        match self {{\n")?;

    for t in &tree_types {
        write!(
            file,
            "            {}::{}(v) => v.accept({}_visitor),\n",
            base_name,
            t.base_class_name,
            base_name.to_lowercase()
        )?;
    }
    write!(file, "        }}\n")?;
    write!(file, "    }}\n")?;
    write!(file, "}}\n\n")?;

    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.class_name)?;
        for f in &t.fields {
            write!(file, "    pub {},\n", f)?;
        }
        write!(file, "}}\n\n")?;
    }

    write!(file, "pub trait {}Visitor<T> {{\n", base_name)?;
    for t in &tree_types {
        write!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    write!(file, "}}\n\n")?;

    for t in &tree_types {
        write!(file, "impl {} {{\n", t.class_name)?;
        write!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{\n",
            base_name
        )?;
        write!(
            file,
            "      visitor.visit_{}_{}(self)\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        write!(file, "    }}\n")?;
        write!(file, "}}\n\n")?;
    }

    Ok(())
}
