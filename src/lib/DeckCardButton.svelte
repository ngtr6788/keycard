<script lang="ts">
  import { navigate } from "svelte-routing";
  import type { Deck } from "src/types";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, createEventDispatcher } from "svelte";
  import { link } from "svelte-routing";

  // Temporary deck list
  export let deck: Deck;

  const dispatch = createEventDispatcher();

  let cardBox: HTMLElement;
  let hover = false;

  const hoverOn = () => {
    hover = true;
  };

  const hoverOff = () => {
    hover = false;
  };

  const deleteDeck = async () => {
    await invoke("delete_deck", { deckId: deck.id });
    dispatch("deckdelete");
  };

  onMount(() => {
    cardBox.addEventListener("mouseenter", hoverOn);
    cardBox.addEventListener("mouseleave", hoverOff);
  });
</script>

<div class="relative" bind:this={cardBox}>
  <div
    class="border-2 border-black p-2 rounded-md hover:cursor-pointer"
    on:keyup={() => navigate(`/card-list/${deck.id}`)}
    on:click={() => navigate(`/card-list/${deck.id}`)}
  >
    <h5 class="text-m overflow-x-hidden">{deck.deck_name}</h5>
    <p
      class="text-sm overflow-x-hidden"
      class:text-gray-500={!deck.deck_description}
    >
      {deck.deck_description || "No description"}
    </p>
  </div>
  {#if hover}
    <div class="absolute top-1 right-1">
      <a
        class="bg-green-500 text-white py-1 px-2 rounded-md"
        href={`/edit-deck/${deck.id}`}
        use:link>Edit</a
      >
      <button
        class="bg-red-500 text-white py-1 px-2 rounded-md"
        on:click={deleteDeck}>Delete</button
      >
    </div>
  {/if}
</div>
