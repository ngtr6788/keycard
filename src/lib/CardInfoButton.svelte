<script lang="ts">
  import type { Card } from "src/types";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, createEventDispatcher } from "svelte";
  import { link } from "svelte-routing";

  export let card: Card;
  export let deckId: string;

  const dispatch = createEventDispatcher();

  let cardBox: HTMLElement;
  let hover = false;

  const hoverOn = () => {
    hover = true;
  };

  const hoverOff = () => {
    hover = false;
  };

  const deleteCard = async () => {
    await invoke("delete_card", { cardId: card.id });
    dispatch("carddelete");
  };

  onMount(() => {
    cardBox.addEventListener("mouseenter", hoverOn);
    cardBox.addEventListener("mouseleave", hoverOff);
  });
</script>

<div
  class="border-2 border-black rounded-md py-1 px-2 hover:shadow-md hover:bg-gray-100 my-1 relative"
  bind:this={cardBox}
>
  <p>{card.card_question}</p>
  <p>
    {card.keys_list.reduce((prevValue, curValue) => {
      return prevValue + curValue + " ";
    }, "")}
  </p>
  {#if hover}
    <div class="absolute top-1 right-1">
      <a
        class="bg-green-500 text-white py-1 px-2 rounded-md"
        href={`/edit-card/${deckId}/${card.id}`}
        use:link>Edit</a
      >
      <button
        class="bg-red-500 text-white py-1 px-2 rounded-md"
        on:click={deleteCard}>Delete</button
      >
    </div>
  {/if}
</div>
