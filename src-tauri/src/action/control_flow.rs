use crate::sequence::Sequence;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ControlFlow {
    Loop {
        body: Sequence,
    },
    While {
        condition: Sequence,
        body: Sequence,
    },
    For {
        init: Sequence,
        condition: Sequence,
        update: Sequence,
        body: Sequence,
    },
    If {
        condition: Sequence,
        body: Sequence,
        else_body: Option<Sequence>,
    },
    IfElseIf {
        condition: Sequence,
        body: Sequence,
        else_if: Box<ControlFlow>,
    },
}
