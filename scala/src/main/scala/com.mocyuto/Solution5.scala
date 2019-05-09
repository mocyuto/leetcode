object Solution5 {
  def main(args: Array[String]): Unit = {
    println(longestPalindrome("babad"))
    println(longestPalindrome("cbbd"))
    println(longestPalindrome("a"))
    println(checkPalindrome("bb"))
    println(checkPalindrome("bab"))
    println(checkPalindrome("baba"))
    // 最後から見てく回文
  }
  def checkPalindrome(s:String): Boolean = {
    if (s.length == 1) return true
    if (s.length%2==0) {
      for (i <- 0 to s.length/2) {
        if (s(i) != s(s.length - i - 1)) return false
      }
    } else {
      for (i <- 0 to s.length/2 - 1) {
        if (s(i) != s(s.length - i - 1)) return false
      }
    }
    true
  }
  def longestPalindrome(s: String): String = {
    var max = 0
    var res = ""
    for (i <- 0 to s.length) {
      for (j <- i+1 to s.length) {
        val sub = s.slice(i,j)
        if (checkPalindrome(sub) && (j-i) > max) {
          max = j - i
          res = sub
        }
      }
    }
    res.toInt
    return res
  }
}