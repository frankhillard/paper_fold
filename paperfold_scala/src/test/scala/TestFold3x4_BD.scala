import org.scalatest.FlatSpec

class TestPaperFold extends FlatSpec {

  "3x4 1D 1B 1D 1B 1D" should "l,k,c,g,h,d,e,f,b,a,j,i" in {
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
    //println("expected", expected)
    assert(foldingPaper.head.head.equals(expected))
  }

}
