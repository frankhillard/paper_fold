class Fold {

  def init(nbLines: Int, nbCols: Int): List[List[Any]] = {
    var letter = 'a'
    var paper: List[List[Any]] = List()
    for (a <- 0 to nbLines-1) {
      var oneLine: List[Any] = List()
      for (b <- 0 to nbCols-1) {
        oneLine = letter :: oneLine
        letter = (letter + 1).toChar
      }
     paper = oneLine.reverse :: paper
    }
    paper.reverse
  }

  def fold(paper: List[List[Any]], lineNumber: Int, direction: Char ): List[List[Any]] = {
    if (direction == 'H') {
      foldLineUp(paper, lineNumber)
    }
    else if (direction == 'B') {
      var debugPaper: List[List[Any]] = List()
      foldLineUp(paper, lineNumber).map { line => line.map{ elt => elt match {
        case ll: List[Any] => ll.reverse
        case v: Any => v
      } } }.reverse
    }
    else if (direction == 'G') {
      foldLineUp(paper.transpose, lineNumber).transpose
    }
    else if (direction == 'D')
      foldLineUp(paper.transpose, lineNumber).map { line => line.map{ elt => elt match {
        case ll: List[Any] => ll.reverse
        case v: Any => v
      } } }.reverse.transpose
    else
      List()
  }

  def foldLineUp(paper: List[List[Any]], lineNumber: Int): List[List[Any]] = {
    var input: List[List[Any]] = paper
    var temp: List[List[Any]] = List()
    var counter: Int = lineNumber
    while (counter > 0) {
      temp = input.head :: temp
      input = input.tail
      counter = counter - 1
    }
    var returned: List[List[Any]] = List()
    while (!temp.isEmpty || !input.isEmpty) {
      (temp, input) match {
        case (t,Nil) => returned = returned :+ t.head
        case (Nil,r) => returned = returned :+ r.head
        case (t,r) => returned = returned :+ fusionLine(r.head, t.head)
      }
      if (!input.isEmpty) input = input.tail
      if (!temp.isEmpty) temp = temp.tail
    }
    returned
  }

  def fusionLine(l1: List[Any], l2: List[Any]): List[Any] = {
    assert(l1.size == l2.size)
    l2.zip(l1).map{ case(x,y) => (x,y) match {
      case (xx: List[Any], yy: List[Any]) => xx.reverse ::: yy
      case (xx: List[Any], sy: Any) => xx :+ y
      case (sx: Any, yy: List[Any]) => sx +: yy
      case (sx: Any, sy: Any) => sx :: sy :: Nil
    } }
  }

  def foldLineUpOld(paper: List[List[Any]], lineNumber: Int): List[List[Any]] = {
    //println("Paper size", paper.length)
    var result: List[List[Any]] = paper
    var temp: List[List[Any]] = List()
    var counter: Int = lineNumber
    while (counter > 0) {
      temp = result.head :: temp
      result = result.tail
      counter = counter - 1
    }
    //println("to fold", temp)
    var returned: List[List[Any]] = List()
    while (!temp.isEmpty || !result.isEmpty) {
      if (result.isEmpty) {
        returned = returned :+ temp.head
      }
      else if (temp.isEmpty) {
        returned = returned :+ result.head
      }
      else {
        returned = returned :+ fusionLine(result.head, temp.head)
      }
      //println("resLine", returned)
      if (!result.isEmpty)
        result = result.tail
      if (!temp.isEmpty)
        temp = temp.tail
    }
    returned
  }






}
