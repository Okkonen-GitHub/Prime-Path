package controllers

import javax.inject._
import play.api._
import play.api.mvc._

import services.GameManager



/**
 * This controller creates an `Action` to handle HTTP requests to the
 * application's home page.
 */
@Singleton
class GameController @Inject()(val controllerComponents: ControllerComponents, val gameManager: GameManager) extends BaseController {


  def createGame(name: String, creator: String) = Action { implicit request: Request[AnyContent] =>
    val code = gameManager.createGame(name, creator)
    Ok(s"/game/$code")
  }
  def joinGame(code: String, name: String): Action[AnyContent] = Action { implicit request: Request[AnyContent] =>
    gameManager(code).getOrElse(return Action {NotFound}).addOpponent(name)
    Ok(s"/game/$code")
  }
  def gameView(code: String): Action[AnyContent] = Action { implicit request: Request[AnyContent] =>
    val name = gameManager(code).getOrElse(return Action { NotFound }).name
    Ok(views.html.game(name))
  }
}
