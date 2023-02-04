<script lang="ts">
  import DeckCardButton from './DeckCardButton.svelte';

  import { link } from "svelte-routing";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Deck } from "src/types";

  // Temporary deck list
  let deckNamesPromise: Promise<Deck[]> = invoke("get_decks");

  const updateDecks = () => {
    deckNamesPromise = invoke("get_decks");
  }
</script>

<div class="mx-10 my-4">
  <div class="flex flex-row justify-between items-center h-8">
    <h1 class="text-2xl">Your Decks</h1>
    <a
      class="bg-purple-700 py-1 px-2 text-white rounded hover:shadow-lg hover:bg-purple-800"
      href="/new-deck"
      use:link>New Deck</a
    >
  </div>
  {#await deckNamesPromise}
    <h4 class="text-base text-gray-500 my-1">Loading decks...</h4>
  {:then deckNames}
    {#if deckNames.length !== 0}
      <div class="grid grid-cols-2 auto-rows-min my-1 overflow-y-scroll h-[18.5rem] scrollbar-hidden">
        {#each deckNames as deck}
          <div class="odd:mr-2 even:ml-2 my-1">
            <DeckCardButton deck={deck} on:deckdelete={updateDecks} />
          </div>
        {/each}
      </div>
    {:else}
      <h4 class="text-base text-gray-500 my-1">
        No decks here. Click "New Deck" to create a new deck.
      </h4>
    {/if}
  {:catch}
    <h4 class="text-base text-gray-500 my-1">
      Cannot load decks right now. Try closing this window and try again.
    </h4>
  {/await}
</div>
