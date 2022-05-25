use crate::api::*;
use lexopt::prelude::*;

pub type Args = Vec<Id>;

#[derive(Debug, Clone)]
pub struct Id {
    pub ty: ContentType,
    pub id: usize,
    pub body: String,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.ty == ContentType::Img {
            write!(
                f,
                r#"{{"tag":"img","attrs":{{"src":"{img}"}}}}"#,
                img = self.body.as_str()
            )
        } else if self.ty == ContentType::Text {
            write!(
                f,
                r#"{{"tag":"p","children":["{text}"]}}"#,
                text = self.body.as_str()
            )
        } else {
            Err(std::fmt::Error)
        }
    }
}

pub fn parse_args(args: &mut Args) -> Result<(), lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();
    let mut id = 0;

    while let Some(arg) = parser.next()? {
        match arg {
            Short('i') => {
                for f in parser.values()? {
                    args.push(Id {
                        ty: ContentType::Img,
                        id,
                        body: f.into_string().expect("").clone(),
                    });
                    id += 1;
                }
            }
            Short('t') => {
                for f in parser.values()? {
                    args.push(Id {
                        ty: ContentType::Text,
                        id,
                        body: f.into_string().expect("").clone(),
                    });
                    id += 1;
                }
            }

            Short('h') | Long("help") => {
                println!(r#"telegraph -t "123abc" -i test.jpg test.jpg "#);
                std::process::exit(127);
            }
            Value(_v) => {}

            _ => return Err(arg.unexpected()),
        }
    }

    Ok(())
}
