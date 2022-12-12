<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { link, navigate } from "svelte-routing";
  import { invoke } from "@tauri-apps/api/tauri";

  let deckName: string = "";
  let deckDescription: string = "";
  let error = false;

  const dispatch = createEventDispatcher();

  const createDeck = () => {
    if (deckName === "") {
      error = true;
      return;
    }

    error = false;
    invoke("add_deck", { deckName, deckDescription });
    dispatch("deckcreate", {
      deckName,
      deckDescription,
    });

    deckName = "";
    deckDescription = "";
    navigate("/");
  };

  const cancelDeckCreation = () => {
    dispatch("deckcancel");
    error = false;
    deckName = "";
    deckDescription = "";
  };
</script>

<div class="flex flex-col items-center m-4">
  <h3 class="text-lg">New Deck</h3>
  <input
    class="w-96 my-2 px-2 py-1 border-2 rounded border-black"
    class:border-red-600={error}
    on:input={() => {
      error = false;
    }}
    type="text"
    placeholder="Deck Name"
    bind:value={deckName}
  />
  <textarea
    class="w-96 my-2 px-2 py-1 border-2 rounded border-black"
    placeholder="Description"
    bind:value={deckDescription}
  />
  <div class="flex flex-row">
    <button
      class="bg-sky-600 m-5 px-3 py-2 text-white rounded-md hover:bg-sky-700 hover:shadow-lg"
      on:click={createDeck}>Create</button
    >
    <a
      class="bg-red-600 m-5 px-3 py-2 text-white rounded-md hover:bg-red-700 hover:shadow-lg"
      href="/"
      on:click={cancelDeckCreation}
      use:link>Cancel</a
    >
  </div>
  {#if error}
    <p class=" text-red-600">Deck name is required</p>
  {/if}
</div>
