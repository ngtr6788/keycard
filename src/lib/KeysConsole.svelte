<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { createEventDispatcher } from "svelte";

  export let keysList: string[] = [];
  export let disabled = false;

  let keysScreenElement: HTMLParagraphElement;
  let keysScreenClicked = false;

  // we can only type on the console if console is clicked on
  const clickedOn = (event: MouseEvent) => {
    let targetElement = event.target;
    if (targetElement === keysScreenElement && !disabled) {
      keysScreenClicked = true;
    } else {
      keysScreenClicked = false;
    }
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    // also, we can only type on the console if it's clicked on
    if (keysScreenClicked && !disabled) {
      // we clear console with ESC button, unless it's empty,
      // in which case, we display the word escape
      if (event.key === "Escape" && keysList.length !== 0) {
        keysList = [];
      } else {
        keysList.push(event.key);
        keysList = keysList;
      }
    }
  };

  onMount(() => {
    document.addEventListener("click", clickedOn);
    document.addEventListener("keydown", handleKeyDown);
  });

  onDestroy(() => {
    document.removeEventListener("click", clickedOn);
    document.removeEventListener("keydown", handleKeyDown);
  });

  const dispatch = createEventDispatcher();
  dispatch("input", { keysList });
</script>

<p
  class="w-96 min-h-[3rem] bg-zinc-500 p-2 rounded-lg border-4 hover:cursor-pointer text-center text-white whitespace-pre-wrap"
  class:border-yellow-400={keysScreenClicked}
  class:border-zinc-500={!keysScreenClicked}
  bind:this={keysScreenElement}
>
  {keysList.reduce((keysString, k) => {
    if (k.length > 1) {
      return keysString + ` ${k} `;
    } else {
      return keysString + k;
    }
  }, "")}
</p>
