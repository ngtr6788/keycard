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

<h3>New Deck</h3>
<input type="text" bind:value={deckName} />
<textarea bind:value={deckDescription} />
<button on:click={createDeck}>Create</button>
<button on:click={cancelDeckCreation}>Cancel</button>
