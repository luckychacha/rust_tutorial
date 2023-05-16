fn main() {
    let mut data = 10;
    // 第一次可变借用 data
    let ref1 = &mut data;
    // 对 ref1 解引用后，第二次可变借用 data
    let ref2 = &mut *ref1;

    // ref1 和 ref2 的作用域重叠，我理解这里应该违反了借用规则，但实际可以通过编译
    *ref2 += 2;
    *ref1 += 1;

    println!("{}", data);
}
