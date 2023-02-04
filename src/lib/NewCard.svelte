<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import CardForm from "./CardForm.svelte";

  export let deck_id: string;

  const createCard = async (event: CustomEvent) => {
    await invoke("add_card", {
      deckId: parseInt(deck_id),
      ...event.detail,
    });
  };
</script>

<CardForm title="New Card" deckId={deck_id} on:cardsubmit={createCard} />
