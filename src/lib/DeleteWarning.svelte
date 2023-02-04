<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let display = false;

  const dispatch = createEventDispatcher();

  const handleDelete = () => {
    dispatch("delete");
    dispatch("exit");
  };

  const handleExit = () => {
    dispatch("exit");
  };
</script>

<div
  class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-[1000000]"
  class:hidden={!display}
>
  <div
    class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white"
  >
    <div class="flex flex-col items-center">
      <p class="mb-3">Are you sure you want to delete this?</p>
      <div class="flex flex-row space-x-10">
        <button
          class="bg-red-700 text-white py-1 px-2 rounded-md"
          on:click={handleDelete}>Yes</button
        >
        <button
          class="bg-white text-black py-1 px-2 rounded-md border-2 border-black"
          on:click={handleExit}>No</button
        >
      </div>
    </div>
  </div>
</div>
