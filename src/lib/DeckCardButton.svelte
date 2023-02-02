<script lang="ts">
  import { navigate } from "svelte-routing";
  import type { Deck } from "src/types";

  // Temporary deck list
  export let deck: Deck;
</script>

<div
  class="border-2 border-black my-2 p-2 rounded-md hover:bg-slate-100 hover:shadow-lg hover:cursor-pointer odd:mr-2 even:ml-2"
  on:keyup={() => navigate(`/card-list/${deck.id}`)}
  on:click={() => navigate(`/card-list/${deck.id}`)}
>
  <h5 class="text-m overflow-x-hidden">{deck.deck_name}</h5>
  <p
    class="text-sm overflow-x-hidden"
    class:text-gray-500={!deck.deck_description}
  >
    {deck.deck_description || "No description"}
  </p>
</div>
