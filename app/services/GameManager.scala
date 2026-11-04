package services

import scala.collection.mutable.HashMap
import scala.util.Random
import javax.inject._

import services.Game

/**
 * This trait demonstrates how to create a component that is injected
 * into a controller. The trait represents a counter that returns a
 * incremented number each time it is called.
 */


trait Manager {
    def createGame(name:String, creator: String): String
    def apply(code: String): Option[Game]
}

/**
 *
 * This class has a `Singleton` annotation because we need to make
 * sure we only use one counter per application. Without this
 * annotation we would get a new instance every time a [[Manager]] is
 * injected.
 */
@Singleton
class GameManager extends Manager {
    private val games = HashMap[String, Game]()

    def apply(code: String) = games.get(code)

    def createGame(name: String, creator: String): String = {
        val code = Random.alphanumeric.take(6).mkString
        games.addOne((code, new Game(code, name, creator)))
        code
    }
}
