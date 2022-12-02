<script lang="ts">
  import { createEventDispatcher } from "svelte";

  let deckName: string = "";
  let deckDescription: string = "";

  const dispatch = createEventDispatcher();

  document.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      createDeck();
    }
  });

  const createDeck = () => {
    dispatch("deckcreate", {
      deckName,
      deckDescription,
    });

    deckName = "";
    deckDescription = "";
  };

  const cancelDeckCreation = () => {
    dispatch("deckcancel");
    deckName = "";
    deckDescription = "";
  };
</script>

<div class="flex flex-col">
  <h3 class="mx-auto text-lg">New Deck</h3>
  <input
    class="mx-auto w-60 my-2 px-2 py-1 border-2 rounded border-black"
    type="text"
    bind:value={deckName}
  />
  <textarea
    class="mx-auto w-60 my-2 px-2 py-1 border-2 rounded border-black"
    bind:value={deckDescription}
  />
  <div class="flex flex-row mx-auto">
    <button
      class="bg-sky-600 m-5 px-3 py-2 text-white rounded-md"
      on:click={createDeck}>Create [press enter]</button
    >
    <button
      class="bg-red-600 m-5 px-3 py-2 text-white rounded-md"
      on:click={cancelDeckCreation}>Cancel</button
    >
  </div>
</div>
