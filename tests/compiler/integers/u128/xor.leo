/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: u128, b: u128) -> u128 {
    return a ^ b;
}