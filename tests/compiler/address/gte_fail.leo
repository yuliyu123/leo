/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/address1.in
*/

function main(x: address) -> bool {
    const sender: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;

    return x >= sender;
}
