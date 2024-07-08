import Game from "./routes/Game.svelte";
import Home from "./routes/Home.svelte";

export default {
  "/": Home,
  "/:gameid": Game,
};
