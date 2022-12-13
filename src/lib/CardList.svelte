<script lang="ts">
  import { link } from "svelte-routing";

  const deckName = "Vim commands";

  const cards = [
    {
      cardDescription: "Copy 5 lines",
      keysList: ["5", "y"],
    },
    {
      cardDescription: "Save and exit the file",
      keysList: [":", "w", "q"],
    },
    {
      cardDescription: "Go up one line",
      keysList: ["k"],
    },
    {
      cardDescription: "Delete 10 lines",
      keysList: ["1", "0", "d"],
    },
  ];
</script>

<div class="mx-10 my-4">
  <div class="flex flex-row justify-between items-center my-2">
    <h3 class="text-lg">Deck: {deckName}</h3>
    <div>
      <a
        class="bg-amber-400 py-1 px-2 text-black rounded hover:shadow-lg hover:bg-amber-500"
        href="/new-card"
        use:link>New Card</a
      >
      <a
        class="bg-sky-400 py-1 px-2 text-white rounded hover:shadow-lg hover:bg-sky-500"
        href="/study-card"
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
      {#each cards as { cardDescription, keysList }}
        <div
          class="border-2 border-black rounded-md py-1 px-2 hover:shadow-md hover:bg-gray-100 m-1"
        >
          <p>{cardDescription}</p>
          <p>
            {keysList.reduce((prevValue, curValue) => {
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
