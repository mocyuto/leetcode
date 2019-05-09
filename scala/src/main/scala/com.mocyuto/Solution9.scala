import scala.collection.mutable.ArrayBuffer

object Solution9 {
  def main(args: Array[String]): Unit = {
//    println(isPalindrome(12121))
//    println(isPalindrome(2121))
//    println(isPalindrome(212))
    println(isPalindrome(2100000012))
    println(isPalindrome2(2000000002))
  }
  def isPalindrome(x: Int): Boolean = {
    if (x < 0) return false
    var y = x
    var digit: Int = 1
    var res: Int = 0
    do {
      val d = y % (digit*10) / digit
      res = res * 10 + d
      y -= d * digit
      if (digit >= 1000000000 && y >= digit) {
        res += y/digit
        digit = y + 1
      }
      else digit *= 10
    } while(y >= digit)
    println(s"res: $res, digit: $digit, y: $y")
    res == x
  }
  def isPalindrome2(x: Int): Boolean = {
    if (x < 0) return false
    var y = x
    var digit = 1
    var arr: ArrayBuffer[Int] = new ArrayBuffer()
    do {
      val d = y % (digit*10) / digit
      arr += d
      y -= d * digit
      digit *= 10
    } while(y >= digit)
    println(s"arr: $arr, digit: $digit, y: $y")
    true
  }

}
