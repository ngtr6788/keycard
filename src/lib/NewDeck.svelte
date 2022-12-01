<script lang="ts">
  let deckName: string = "";
  let deckDescription: string = "";

  const createDeck = () => {
    deckName = "";
    deckDescription = "";
  };

  const cancelDeckCreation = () => {};
</script>

<h3>New Deck</h3>
<input type="text" bind:value={deckName} />
<textarea bind:value={deckDescription} />
<button on:click={createDeck}>Create</button>
<button on:click={cancelDeckCreation}>Cancel</button>
