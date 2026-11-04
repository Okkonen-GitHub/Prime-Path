package services

import javax.inject.Inject

import org.apache.pekko.actor.ActorSystem
import org.apache.pekko.stream.Materializer
import org.apache.pekko.actor._
import play.api.libs.streams.ActorFlow
import play.api.mvc._
import play.api.libs.json._


object WebSocketActor {
  def props(out: ActorRef) = Props(new WebSocketActor(out))
}

class WebSocketActor(out: ActorRef) extends Actor {
  def receive = {
    case msg: String =>
      out ! ("I received your message: " + msg)
  }
}


class WSHandler @Inject() (cc: ControllerComponents)(implicit system: ActorSystem, mat: Materializer)
    extends AbstractController(cc) {

  implicit val inEventFormat: Format[InEvent]   = Json.format[InEvent]
  implicit val outEventFormat: Format[OutEvent] = Json.format[OutEvent]
  def socket = WebSocket.accept[InEvent, OutEvent] { request =>
    ActorFlow.actorRef { out => WebSocketActor.props(out) }
  }
}