/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: i64, b: i64) -> i64 {
    return a ^ b;
}
