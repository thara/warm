// https://webassembly.github.io/spec/core/binary/modules.html

struct Section {
    id: u8,
    contents: Vec<u8>,
}

struct TypeSection {
    types: Vec<FuncType>,
}

struct FuncType {
    param: ResultType,
    result: ResultType,
}

struct ResultType {
    types: Vec<ValueType>,
}

enum ValueType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

enum NumType {
    I32,
    I64,
    F32,
    F64,
}

struct VecType {
    v128: u128,
}

enum RefType {
    FuncRef,
    ExternRef,
}

enum SectionID {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
}
