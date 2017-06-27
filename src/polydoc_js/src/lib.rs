extern crate esprit;
extern crate easter;
extern crate joker;
extern crate polydoc_core;


use polydoc_core::{DeclItem, DeclType};


pub fn extract_declarations(source: &str) -> Vec<DeclItem>
{
    use esprit;
    use easter::stmt::StmtListItem;
    use easter::decl::Decl;
    use joker::word::Name;

    let mut items = Vec::new();
    let script = esprit::script(source).expect("Failed to parse javascript.");

    for item in &script.body
    {
        match item
        {
            &StmtListItem::Decl(Decl::Fun(ref f)) =>
            {
                let line = f.location.expect("Missing location").start.line as u64;
                let ref name = f.id.as_ref().expect("Expected function name.").name;
                if let &Name::String(ref s) = name
                {
                    let doc = DeclItem
                    {
                        line,
                        name: s.clone(),
                        data: DeclType::Function
                    };
                    items.push(doc);
                }
            },
            _ => unimplemented!()
        }
    }

    items
}


#[cfg(test)]
mod tests
{
    use super::*;

}