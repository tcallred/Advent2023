let testInput = [
    "1abc2",
    "pqr3stu8vwx",
    "a1b2c3d4e5f",
    "treb7uchet"
]

extension String {

public func firstAndLastDigits() -> Int {
    let digits = self.filter(\.isNumber) 
    return Int(String([digits.first!, digits.last!]))!
}
}


let ans = testInput
    .map(\.firstAndLastDigits())
    .reduce(0){$0 + $1}
print(ans)
