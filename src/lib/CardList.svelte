<script lang="ts">
  import CardInfoButton from './CardInfoButton.svelte';

  import { invoke } from "@tauri-apps/api/tauri";
  import type { Deck, Card } from "src/types";
  import { onMount } from "svelte";
  import { link } from "svelte-routing";

  const MAX_DECK_NAME_LENGTH = 50;

  export let id: string;
  let deckInfo: Deck;
  $: deckName = deckInfo?.deck_name ?? "";
  $: displayDeckName = deckName.length >= MAX_DECK_NAME_LENGTH ? `${deckName.substring(0, MAX_DECK_NAME_LENGTH)}...` : deckName;

  let cards: Card[] = [];
  onMount(async () => {
    [cards, deckInfo] = await Promise.all<[Promise<Card[]>, Promise<Deck>]>([
      invoke("get_cards_from_deck", { deckId: parseInt(id) }),
      invoke("get_deck", { deckId: parseInt(id) }),
    ]);
  });
</script>

<div class="mx-10 my-4">
  <div class="sticky top-4 h-[4.5rem]">
    <h3 class="text-lg text-center">Deck: {displayDeckName}</h3>
    <div class="flex flex-row justify-center my-2">
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
  </div>
  {#if cards.length !== 0}
    <div class="overflow-y-scroll h-64 scrollbar-hidden">
      {#each cards as card}
        <CardInfoButton card={card} />
      {/each}
    </div>
  {:else}
    <p class="text-base text-gray-500 my-1">No cards here.</p>
  {/if}
</div>
