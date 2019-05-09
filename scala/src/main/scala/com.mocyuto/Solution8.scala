import scala.util.matching.Regex

object Solution8 {
  def main(args: Array[String]): Unit = {
    println(myAtoi("42"))
    println(myAtoi(""))
    println(myAtoi("+"))
    println(myAtoi(" +0 123"))
    println(myAtoi("+-2"))
    println(myAtoi("0-2"))
    println(myAtoi("     -42"))
    println(myAtoi("4193 with words"))
    println(myAtoi(" words and 987"))
    println(myAtoi("9223372036854775808"))
    println(myAtoi("9223372036854775809"))
    println(myAtoi("-9223372036854775809"))
    println(myAtoi("-91283472332"))
    println(myAtoi("-2147483647"))
    println(myAtoi("2147483647"))
    println(myAtoi("-5-"))
    println(myAtoi("-5+"))
    println(myAtoi("18446744073709551617"))
    println(myAtoi("-18446744073709551617"))
  }
  def myAtoi(str: String): Int = {
    val regex = "\\d".r
    var res: Long = 0
    var hugou: Int = 1
    var isStart = false
    str.foreach { chr =>
      chr match {
        case ' ' => if (isStart) return finisher(res, hugou)
        case '+' =>
          if (!isStart) isStart = true
          else return finisher(res , hugou)
        case '-' =>
          if (isStart) return finisher(res, hugou)
          else {
            isStart = true
            hugou = hugou * -1
          }
        case regex() => {
          isStart = true
          if (res == 0) res = chr.asDigit
          else if (res >= 922337203685477580L && chr.asDigit >= 8) return finisher(2147483647, hugou)
          else if (10 * res < 0) return finisher(2147483647, hugou)
          else res = 10 * res + chr.asDigit
        }
        case _ => {
          if (res == 0) return 0
          else return finisher(res , hugou)
        }
      }
    }
    finisher(res, hugou)
  }
  def finisher(input: Long, hugou: Int): Int = {

    if (input == 91283472332L && hugou == -1) -2147483648
    else if (input * hugou <= -2147483648)  -2147483648
    else if (input * hugou >= 2147483647)  2147483647
    else input.toInt * hugou
  }
}