<script lang="ts">
  import KeysConsole from "./KeysConsole.svelte";
  import { link } from "svelte-routing";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, createEventDispatcher } from "svelte";
  import type { Deck } from "src/types";

  export let title: string;
  export let deckId: string;
  export let cardQuestion = "";
  export let keysList: string[] = [];

  let deckInfo: Deck;
  $: deckName = deckInfo?.deck_name ?? "";
  let error = false;

  const dispatch = createEventDispatcher();

  const handleSubmit = () => {
    if (cardQuestion === "" || keysList.length === 0) {
      error = true;
    } else {
      dispatch("cardsubmit", {
        cardQuestion,
        keysList,
      });

      cardQuestion = "";
      keysList = [];
    }
  };

  onMount(async () => {
    deckInfo = await invoke("get_deck", { deckId: parseInt(deckId) });
  });
</script>

<div class="flex flex-col items-center m-4">
  <h3 class="text-lg">Deck: {deckName}</h3>
  <h3 class="text-lg">{title}</h3>
  <input
    class="w-96 my-2 px-2 py-1 border-2 rounded border-black"
    class:border-red-600={error}
    on:input={() => {
      error = false;
    }}
    type="text"
    placeholder="Enter question"
    bind:value={cardQuestion}
  />
  <KeysConsole bind:keysList />
  <div class="flex flex-row">
    <button
      class="bg-emerald-300 m-5 px-3 py-2 text-black rounded-md hover:bg-emerald-400 hover:shadow-lg"
      on:click={handleSubmit}>Submit</button
    >
    <a
      class="bg-yellow-300 m-5 px-3 py-2 text-black rounded-md hover:bg-yellow-400 hover:shadow-lg"
      href={`/card-list/${deckId}`}
      use:link>Done</a
    >
  </div>
  {#if error}
    <p class="text-red-600">Card question and answer is required</p>
  {/if}
</div>
