/*
defang ip address - https://leetcode.com/problems/defanging-an-ip-address/
*/

pub fn solve() -> bool {
    let input = "1.1.1.1".to_owned();
    let solution = defang_i_paddr(input);
    solution == "1[.]1[.]1[.]1".to_owned()
}

pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}