package services

import scala.util.Random


class Game(val code: String, val name: String, val creator: String) {
    var opponent: Option[String] = None

    private var currentTurn = 0
    private var currentNum = Random.between(400, 800)
    private var nums = Vector(currentNum)

    def addOpponent(name: String) = opponent = Some(name)

    def turn = currentTurn
    def advanceTurn() = currentTurn = currentTurn ^ 1

    def storeNumber() = {
        nums = nums :+ currentNum
    }

    def add(x: Int) = {
        currentNum += x
        storeNumber()
        advanceTurn()
    }

    def divide(): Option[Unit] = {
      if (currentNum % 2 == 0) {
        currentNum = currentNum / 2;
        storeNumber()
        Some(())
      }
      else None
    }

}
