<script lang="ts">
  import NewDeck from "./lib/NewDeck.svelte";

  let home = true;
</script>

{#if home}
  <button
    on:click={() => {
      home = false;
    }}>New Deck</button
  >
{:else}
  <NewDeck />
{/if}

<style>
</style>
