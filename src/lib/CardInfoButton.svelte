<script lang="ts">
  import type { Card } from "src/types";

  export let card: Card;
</script>

<div
  class="border-2 border-black rounded-md py-1 px-2 hover:shadow-md hover:bg-gray-100 my-1"
>
  <p>{card.card_question}</p>
  <p>
    {card.keys_list.reduce((prevValue, curValue) => {
      return prevValue + curValue + " ";
    }, "")}
  </p>
</div>
