<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Deck } from "src/types";
  import { onMount } from "svelte";
  import DeckForm from "./DeckForm.svelte";

  export let id: string;

  let deck: Deck;
  $: deckName = deck?.deck_name ?? "";
  $: deckDescription = deck?.deck_description ?? "";

  const editDeck = async (event: CustomEvent) => {
    const payload = { ...event.detail, deckId: parseInt(id) };
    await invoke("edit_deck", payload);
  };

  onMount(async () => {
    deck = await invoke("get_deck", { deckId: parseInt(id) });
  });
</script>

<DeckForm
  title="Edit Deck"
  {deckName}
  {deckDescription}
  on:decksubmit={editDeck}
/>
