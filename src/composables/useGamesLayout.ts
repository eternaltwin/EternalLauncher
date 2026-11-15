import { type Ref, ref, watch } from "vue";
import { type GamesLayout, readLayout, writeLayout } from "../gamesLayout";

// Partagé entre la page Jeux et les Réglages : un changement se voit tout de suite.
const layout = ref<GamesLayout>(readLayout(localStorage));
watch(layout, (value) => writeLayout(localStorage, value));

export function useGamesLayout(): Ref<GamesLayout> {
  return layout;
}
