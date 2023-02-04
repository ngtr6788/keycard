<script lang="ts">
  import KeysConsole from "./KeysConsole.svelte";
  import { link } from "svelte-routing";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import type { Deck } from "src/types";

  export let id: string;
  let deckInfo: Deck;
  $: deckName = deckInfo?.deck_name ?? "";

  let commandFunc = "";
  let keyboardShortcut: string[] = [];
  let error = false;

  const handleAdd = () => {
    if (commandFunc === "" || keyboardShortcut.length === 0) {
      error = true;
    } else {
      invoke("add_card", {
        deckId: parseInt(id),
        cardQuestion: commandFunc,
        keysList: keyboardShortcut,
      });
      commandFunc = "";
      keyboardShortcut = [];
    }
  };

  onMount(async () => {
    deckInfo = await invoke("get_deck", { deckId: parseInt(id) });
  });
</script>

<div class="flex flex-col items-center m-4">
  <h3 class="text-lg">Deck: {deckName}</h3>
  <h3 class="text-lg">New Card</h3>
  <input
    class="w-96 my-2 px-2 py-1 border-2 rounded border-black"
    class:border-red-600={error}
    on:input={() => {
      error = false;
    }}
    type="text"
    placeholder="Enter question"
    bind:value={commandFunc}
  />
  <KeysConsole bind:keysList={keyboardShortcut} />
  <div class="flex flex-row">
    <button
      class="bg-emerald-300 m-5 px-3 py-2 text-black rounded-md hover:bg-emerald-400 hover:shadow-lg"
      on:click={handleAdd}>Add</button
    >
    <a
      class="bg-yellow-300 m-5 px-3 py-2 text-black rounded-md hover:bg-yellow-400 hover:shadow-lg"
      href={`/card-list/${id}`}
      use:link>Done</a
    >
  </div>
  {#if error}
    <p class="text-red-600">Card question and answer is required</p>
  {/if}
</div>
