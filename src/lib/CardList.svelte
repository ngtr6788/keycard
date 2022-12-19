<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Deck, Card } from "src/types";
  import { onMount } from "svelte";
  import { link } from "svelte-routing";

  export let id: string;
  let deckName = "";
  let deckInfo: Deck | null;
  $: deckName = deckInfo?.deck_name ?? "";

  let cards: Card[] = [];
  onMount(async () => {
    cards = await invoke("get_cards_from_deck", { deckId: parseInt(id) });
    deckInfo = await invoke("get_deck", { deckId: parseInt(id) });
  });
</script>

<div class="mx-10 my-4">
  <div class="flex flex-row justify-between items-center my-2">
    <h3 class="text-lg">Deck: {deckName}</h3>
    <div>
      <a
        class="bg-amber-400 py-1 px-2 text-black rounded hover:shadow-lg hover:bg-amber-500"
        href={`/new-card/${id}`}
        use:link>New Card</a
      >
      <a
        class="bg-sky-400 py-1 px-2 text-white rounded hover:shadow-lg hover:bg-sky-500"
        href={`/study-card/${id}`}
        use:link>Study Deck</a
      >
      <a
        class="bg-rose-600 py-1 px-2 text-white rounded hover:shadow-lg hover:bg-rose-700"
        href="/"
        use:link>Leave Deck</a
      >
    </div>
  </div>
  {#if cards.length !== 0}
    <div class="grid grid-cols-3">
      {#each cards as { card_question, keys_list }}
        <div
          class="border-2 border-black rounded-md py-1 px-2 hover:shadow-md hover:bg-gray-100 m-1"
        >
          <p>{card_question}</p>
          <p>
            {keys_list.reduce((prevValue, curValue) => {
              return prevValue + curValue + " ";
            }, "")}
          </p>
        </div>
      {/each}
    </div>
  {:else}
    <p class="text-base text-gray-500 my-1">No cards here.</p>
  {/if}
</div>
