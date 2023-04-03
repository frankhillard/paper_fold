
object TestFold {

  def main(args: Array[String]): Unit = {
    val engine = new Fold()
    val paper = engine.init(3,4)
    var foldingPaper: List[List[Any]] = List()
    foldingPaper = engine.fold(paper, 1, 'D')

    foldingPaper = engine.fold(foldingPaper, 1, 'B')

    foldingPaper = engine.fold(foldingPaper, 1, 'D')

    foldingPaper = engine.fold(foldingPaper, 1, 'B')

    foldingPaper = engine.fold(foldingPaper, 1, 'D')
    println("res", foldingPaper)
    var expected: List[Any] = List('l','k','c','g','h','d','e','f','b','a','j','i')
    println("expected", expected)
    assert(foldingPaper.head.head.equals(expected))
  }
}
