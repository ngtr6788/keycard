<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Card } from "src/types";
  import { onMount } from "svelte";
  import CardForm from "./CardForm.svelte";

  export let deck_id: string;
  export let card_id: string;
  let card: Card | null;

  const editCard = async (event: CustomEvent) => {
    await invoke("edit_card", {
      cardId: parseInt(card_id),
      ...event.detail,
    });
  };

  onMount(async () => {
    card = await invoke("get_card", { cardId: parseInt(card_id) });
  });
</script>

<CardForm
  title="Edit Card"
  deckId={deck_id}
  cardQuestion={card?.card_question || ""}
  on:cardsubmit={editCard}
/>
