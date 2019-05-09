import scala.collection.mutable
import scala.collection.mutable.Map

object Solution6{
  def main(args: Array[String]): Unit = {
    println(chunkRow("PAYP",2 ))
    println(chunkRow("PAYPAL",4 ))
    println(createMap( Map[Int, String](), "PAYPAL", 4))
//    println("P" == convert("P", 1))
    println("PPA" == convert("PPA", 1))
    println("PA" == convert("PA", 2))
    println("PAHNAPLSIIGYIR" == convert("PAYPALISHIRING", 3))
    println("PINALSIGYAHRPI" == convert("PAYPALISHIRING", 4))
  }
  def chunkRow(s: String, row: Int): String ={
    val first = row-1
    val last = s.length-row + 1
    if (last == s.length || first == last) s"${s(first)}".replace(" ", "")
    else s"${s(first)}${s(last)}".replace(" ", "")
  }

  // １ループの個数
  def splitZig(numRows: Int): Int = {
    if (numRows == 1) 1
    else numRows * 2 - 2
  }

  def createMap(map: Map[Int, String], s: String, numRows: Int): Map[Int,String] = {
    (1 to numRows).zipWithIndex.foreach{ case (row, i)=>
      if (map.isDefinedAt(i)) map(i) = map(i) ++ chunkRow(s, row)
      else map(i) = chunkRow(s, row)
    }
    map
  }
  def merge(resultArr: Map[Int, String] ,numRows: Int):String = {
    (0 to numRows-1).foldLeft(""){(str, i) =>
      str ++ resultArr(i)
    }
  }

  def convert(s: String, numRows: Int): String = {
    val splitCount = splitZig(numRows)
    val splitIndex = splitCount - 1 // indexだから-1
    var result: Map[Int,String] = Map[Int, String]()

    // numRow:2 => in case 3, 2loop
    // numRow:3 => in case 5, 2loop
    var str = s
    println(s"last: ${(s.length / splitCount) + 1}")
    (1 to (s.length / splitCount + 1)).foreach{ i =>
      val subString = str.take(splitCount)
      str = str.drop(splitCount)
      println(s"sub: $subString")
      if (subString.length < splitCount) createMap(result, subString ++ " " * (splitCount-subString.length) ,numRows)
      else createMap(result, subString, numRows)
    }
    merge(result, numRows)
  }
}