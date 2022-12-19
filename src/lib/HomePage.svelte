<script lang="ts">
  import { link, navigate } from "svelte-routing";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Deck } from "src/types";

  // Temporary deck list
  let deckNamesPromise: Promise<Deck[]> = invoke("get_decks");
</script>

<div class="mx-10 my-4">
  <div class="flex flex-row justify-between items-center">
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
      <div class="grid grid-cols-2 my-1">
        {#each deckNames as { id, deck_name, deck_description }}
          <div
            class="border-2 border-black my-2 p-2 rounded-md hover:bg-slate-100 hover:shadow-lg hover:cursor-pointer odd:mr-2 even:ml-2"
            on:keyup={() => navigate(`/card-list/${id}`)}
            on:click={() => navigate(`/card-list/${id}`)}
          >
            <h5 class="text-m">{deck_name}</h5>
            <p
              class={deck_description
                ? "text-black text-sm"
                : "text-gray-500 text-sm"}
            >
              {deck_description || "No description"}
            </p>
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
