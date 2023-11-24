#[derive(Clone, Copy, Debug)]
pub enum PathType {
    Initial,
    Path {
        infinite: bool,
        truncated: bool,
        finite: bool,
        complete: bool,
    },
}

impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathType::Initial => write!(f, "initial"),
            PathType::Path {
                infinite,
                truncated,
                finite,
                complete,
            } => {
                let mut res = "".to_string();
                let mut first = true;
                if *infinite {
                    res += "infinite";
                    first = false;
                }
                if *truncated {
                    if !first {
                        res += " + ";
                    }
                    res += "truncated";
                    first = false;
                }
                if *finite {
                    if !first {
                        res += " + ";
                    }
                    res += "finite";
                    first = false;
                }
                if *complete {
                    if !first {
                        res += " + ";
                    }
                    res += "complete";
                }
                write!(f, "{}", res)
            }
        }
    }
}
