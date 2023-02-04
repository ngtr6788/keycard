<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import DeckForm from "./DeckForm.svelte";

  const createDeck = async (event: CustomEvent) => {
    await invoke("add_deck", event.detail);
  };
</script>

<DeckForm title="New Deck" on:decksubmit={createDeck} />
