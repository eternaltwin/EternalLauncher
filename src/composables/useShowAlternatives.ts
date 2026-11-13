import { type Ref, ref, watch } from "vue";
import { readShowAlternatives, writeShowAlternatives } from "../launchOptions";

// Partagé entre la fiche jeu et les Réglages.
const show = ref(readShowAlternatives(localStorage));
watch(show, (value) => writeShowAlternatives(localStorage, value));

export function useShowAlternatives(): Ref<boolean> {
  return show;
}
