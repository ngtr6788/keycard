<script lang="ts">
  import NewDeck from "./lib/NewDeck.svelte";

  let home = true;
  let newDeckName = "";
  let decks: string[] = [];
</script>

{#if home}
  <button
    on:click={() => {
      home = false;
    }}>New Deck</button
  >
  {#each decks as deck}
    <p>{deck}</p>
  {/each}
  {#if newDeckName !== ""}
    <p>Deck {newDeckName} is created.</p>
  {/if}
{:else}
  <NewDeck
    on:deckcreate={(event) => {
      home = true;
      newDeckName = event.detail.deckName;
      decks.push(newDeckName);
      decks = decks;
      setTimeout(() => {
        newDeckName = "";
      }, 2000);
    }}
    on:deckcancel={() => {
      home = true;
    }}
  />
{/if}

<style>
</style>
