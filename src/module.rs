// https://webassembly.github.io/spec/core/binary/modules.html

struct Section {
    id: u8,
    contents: Vec<u8>,
}

struct TypeSection {
    types: Vec<FuncType>,
}

struct ImportSection {
    imports: Vec<Import>,
}

struct FunctionSection {
    types: Vec<u32>,
}

struct TableSection {
    tables: Vec<TableType>,
}

struct MemorySection {
    memories: Vec<MemType>,
}

struct GlobalSection {
    globals: Vec<Global>,
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

struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

struct ImportDesc {
    desc: ImportDescType,
}

enum ImportDescType {
    Func(u32),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

struct TableType {
    elem_type: RefType,
    limits: Limits,
}

enum Limits {
    Min(u32),
    MinMax(u32, u32),
}

struct MemType {
    limits: Limits,
}

struct GlobalType {
    val_type: ValueType,
    mutability: Mutability,
}

enum Mutability {
    Const,
    Var,
}

struct Global {
    global_type: GlobalType,
    init: Vec<Instr>,
}

enum Instr {
    Unreachable,
    Nop,
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    IfElse(BlockType, BlockType),
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>, u32),
    Return,
    Call { func: u32 },
    CallIndirect { type_index: u32 },
}

enum BlockType {
    Empty,
    ValType(ValueType),
    TypeIndex(u32),
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
