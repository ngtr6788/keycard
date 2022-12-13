<script lang="ts">
  import { link, navigate } from "svelte-routing";

  // Temporary deck list
  let deckNames = [
    {
      name: "VSCode Keyboard Shortcuts",
      description: "Shortcuts I used in VSCode",
      numberOfCards: 20,
    },
    {
      name: "Vim Commands",
      description: "Vim commands are not that easy to learn",
      numberOfCards: 25,
    },
    {
      name: "Emacs Commands made by (who?)",
      description: "",
      numberOfCards: 0,
    },
    {
      name: "One Card only",
      description: "Read the title",
      numberOfCards: 1,
    },
  ];
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
  {#if deckNames.length !== 0}
    <div class="grid grid-cols-2 my-1">
      {#each deckNames as { name, description, numberOfCards }}
        <div
          class="border-2 border-black my-2 p-2 rounded-md hover:bg-slate-100 hover:shadow-lg hover:cursor-pointer odd:mr-2 even:ml-2"
          on:keyup={() => navigate("/card-list")}
          on:click={() => navigate("/card-list")}
        >
          <h5 class="text-m">{name}</h5>
          <p
            class={description ? "text-black text-sm" : "text-gray-500 text-sm"}
          >
            {description || "No description"}
          </p>
          <p class="text-sm">{numberOfCards} cards</p>
        </div>
      {/each}
    </div>
  {:else}
    <h4 class="text-base text-gray-500 my-1">
      No decks here. Click "New Deck" to create a new deck.
    </h4>
  {/if}
</div>
