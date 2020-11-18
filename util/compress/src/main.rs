#![feature(box_patterns, core_intrinsics, const_type_name, negative_impls)]
use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug)]
enum Node<T> {
    Leaf(T),
    Branch(Box<Node<T>>, Box<Node<T>>),
}

type Code = u32;
const CODE_TY: &'static str = std::intrinsics::type_name::<Code>();
// TODO: try out 16-bit symbols
type Symbol = u8;
const SYMBOL_TY: &'static str = std::intrinsics::type_name::<Symbol>();
// Prefix-free codes can't be compared for equality
struct PrefixFreeCode(Code);
impl !Eq for PrefixFreeCode {}
impl !PartialEq for PrefixFreeCode {}
type SymbolStream<'a, 'b> = &'a mut dyn Iterator<Item = &'b Symbol>;

#[derive(Debug)]
struct SymbolCount {
    node: Node<Symbol>,
    count: usize,
}

#[derive(Clone)]
struct Entry {
    symbol: Symbol,
    code: Code,
}

fn count_symbols(input: SymbolStream) -> Vec<SymbolCount> {
    let mut pairs = Vec::new();
    let input = input.cloned().collect::<Vec<Symbol>>();
    for symbol in 0..=(1 << (size_of::<Symbol>() * 8)) {
        let count = input.iter().filter(|&&s| usize::from(s) == symbol).count();
        if count > 0 {
            pairs.push(SymbolCount {
                node: Node::Leaf(symbol.try_into().unwrap()),
                count,
            });
        }
    }
    pairs
}

fn build_tree(leaves: &mut Vec<SymbolCount>) {
    while leaves.len() != 1 {
        leaves.sort_by_key(|leaf| -(leaf.count as isize));
        let min = leaves.pop().unwrap();
        let second_min = leaves.pop().unwrap();
        let count = min.count + second_min.count;
        let node = Node::Branch(Box::new(min.node), Box::new(second_min.node));
        leaves.push(SymbolCount { node, count });
    }
}

fn assign_codes(root: Node<Symbol>, suffix: Option<Code>) -> Vec<Entry> {
    // All codes are prefixed with a 1 to disambiguate leading zeros
    // For example 0b0 and 0b00 are different codes
    // Adding a leading 1 helps disambiguate them since 0b10 != 0b100
    let suffix = suffix.unwrap_or(1);
    match root {
        Node::Leaf(t) => vec![Entry {
            symbol: t,
            code: suffix,
        }],
        Node::Branch(box left, box right) => {
            let left_entries = assign_codes(left, Some((suffix << 1) | 1));
            let right_entries = assign_codes(right, Some((suffix << 1) | 0));
            left_entries
                .iter()
                .chain(right_entries.iter())
                .cloned()
                .collect()
        },
    }
}

fn len(c: Code) -> usize {
    let leading_zeros = c.leading_zeros() as usize;
    (size_of::<Code>() * 8) - leading_zeros - 1
}

fn remove_prefix(c: Code) -> PrefixFreeCode {
    PrefixFreeCode(c & !(1 << len(c)))
}

fn output_code(file_name: String, entries: &mut Vec<Entry>) {
    let mut s = String::from("// This file was automatically generated by `compress`\n");
    // TODO: come up with some hasher for this
    //let symbol_universe_size = 1 << (size_of::<Symbol>() * 8);
    //s.push_str(&format!("pub const ENTRIES: [{}; {}] = {{\n", SYMBOL_TY,
    // symbol_universe_size)); s.push_str(&format!("    let mut init = [0;
    // {}];\n", symbol_universe_size)); for entry in entries {
    //    s.push_str(&format!("    init[{}] = {};\n", entry.code, entry.symbol));
    //}
    //s.push_str("    init\n");
    //s.push_str("}};");
    s.push_str(&format!(
        "pub const CODES: [{}; {}] = [\n",
        CODE_TY,
        entries.len()
    ));
    entries.sort_by_key(|e| e.code);
    for entry in entries.iter() {
        // TODO: is this right? prefix-free codes can't be compared without knowing the
        // length but it seems the decompressor is comparing them without any
        // issues
        s.push_str(&format!("    {:#x},\n", remove_prefix(entry.code).0));
    }
    s.push_str("];\n");
    s.push_str(&format!(
        "pub const SYMBOLS: [{}; {}] = [\n",
        SYMBOL_TY,
        entries.len()
    ));
    for entry in entries.iter() {
        s.push_str(&format!("    {:#x},\n", entry.symbol));
    }
    s.push_str("];");
    std::fs::write(file_name, s).expect("Couldn't write codes to file");
}

fn compress(entries: &Vec<Entry>, input: SymbolStream) -> Vec<u8> {
    let mut output_stream = Vec::new();
    let num_bits = size_of::<Code>() * 8;
    let mut remaining_bits = num_bits;
    let mut current_word = 0;
    for symbol in input {
        let encoded = entries
            .iter()
            .find(|e| e.symbol == *symbol)
            .expect("Couldn't find entry for symbol")
            .code;
        let len = len(encoded);
        let prefix_free = remove_prefix(encoded);
        if remaining_bits < len {
            let second_size = len - remaining_bits;
            let first_part = prefix_free.0 >> second_size;
            current_word |= first_part;
            output_stream.push(current_word);
            current_word = 0;
            let second_part = prefix_free.0 << (num_bits - second_size);
            current_word |= second_part;
            remaining_bits = num_bits - second_size;
        } else if remaining_bits == len {
            current_word |= prefix_free.0;
            output_stream.push(current_word);
            current_word = 0;
            remaining_bits = num_bits;
        } else {
            remaining_bits -= len;
            current_word |= prefix_free.0 << remaining_bits;
        }
    }
    if remaining_bits != num_bits {
        output_stream.push(current_word);
    }
    output_stream
        .iter()
        .map(|&x| {
            x.to_le_bytes()
                .iter()
                .cloned()
                .collect::<Vec<u8>>()
                .into_iter()
        })
        .flatten()
        .collect()
}

fn main() {
    let exe = include_bytes!("../ferris.tim").iter().collect::<Vec<_>>();
    let mut symbol_counts = count_symbols(&mut exe.iter().cloned());
    build_tree(&mut symbol_counts);
    let mut entries = assign_codes(symbol_counts.pop().expect("No nodes in tree").node, None);
    assert!(
        symbol_counts.pop().is_none(),
        "Tree contained more than one root"
    );
    output_code("codes.rs".to_string(), &mut entries);
    let output_stream = compress(&entries, &mut exe.iter().cloned());
    fn log2(x: usize) -> usize {
        (size_of::<usize>() * 8) - x.leading_zeros() as usize - 1
    }
    let num_symbols = exe.len() >> log2(size_of::<Symbol>());
    let header = (num_symbols as u32).to_le_bytes();
    let zipped_file = header
        .iter()
        .chain(output_stream.iter())
        .cloned()
        .collect::<Vec<_>>();
    std::fs::write("ferris.tim.zip", zipped_file)
        .expect("Couldn't write compressed stream to file");
}
