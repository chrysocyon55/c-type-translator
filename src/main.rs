#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct TypeQualifiers {
    is_const: bool,
    is_volatile: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct PointerQualifiers {
    is_const: bool,
    is_volatile: bool,
    is_restrict: bool,
}

/// Represents a single C type.
enum CType {
    Ident {
        name: String,
        qualifiers: TypeQualifiers,
    },
    Pointer {
        pointee: Box<CType>,
        qualifiers: PointerQualifiers,
    },
    Array {
        element: Box<CType>,
        qualifiers: TypeQualifiers,
    },
}

impl ToString for CType {
    fn to_string(&self) -> String {
        todo!()
    }
}

fn main() {
    todo!()
}
