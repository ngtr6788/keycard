<script lang="ts">
  import KeysConsole from "./KeysConsole.svelte";
  import { link } from "svelte-routing";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import type { Card } from "../types";

  export let id: string;

  enum StudyStatus {
    Correct = "correct",
    Incorrect = "incorrect",
    NotSubmitted = "not submitted",
  }

  let cardId: number;
  let cardQuestion: string; // This can change on the card
  let cardAnswer: string[]; // This can change on the card
  let currentKeysList: string[] = [];

  let studyStatus = StudyStatus.NotSubmitted;

  const compareArray = (arr1: Array<string>, arr2: Array<string>): boolean => {
    if (arr1.length !== arr2.length) {
      return false;
    }

    for (let i = 0; i < arr1.length; i += 1) {
      if (arr1[i] !== arr2[i]) {
        return false;
      }
    }

    return true;
  };

  const submitAndCheck = async () => {
    if (compareArray(currentKeysList, cardAnswer)) {
      studyStatus = StudyStatus.Correct;
    } else {
      studyStatus = StudyStatus.Incorrect;
    }

    await invoke("evaluate_and_update_card", { cardId, answerKeysList: currentKeysList });
  };

  const getCard = async () => {
    studyStatus = StudyStatus.NotSubmitted;
    currentKeysList = [];
    
    const card: Card | null = await invoke("get_first_card_by_date", { deckId: parseInt(id) });
    if (card) {
      cardId = card.id;
      cardQuestion = card.card_question;
      cardAnswer = card.keys_list;
    } else {
      cardId = -1;
      cardQuestion = "There are no cards here. Please create a new card";
      cardAnswer = [];
    }
  };

  const handleExit = () => {
    studyStatus = StudyStatus.NotSubmitted;
  };

  onMount(() => {
    getCard();
  })
</script>

<div class="flex flex-col items-center m-4">
  <h2 class="text-2xl">Study your deck</h2>
  <h3
    class="text-md text-center w-96 my-2 px-2 py-1 border-2 rounded border-black"
  >
    {cardQuestion}
  </h3>
  <KeysConsole
    bind:keysList={currentKeysList}
    disabled={!(studyStatus === StudyStatus.NotSubmitted)}
  />
  {#if studyStatus === StudyStatus.Correct}
    <p
      class="text-center border-2 border-green-800 bg-green-200 text-green-800 w-96 my-2 py-1 px-4 rounded-md"
    >
      You are correct
    </p>
  {:else if studyStatus === StudyStatus.Incorrect}
    <p
      class="text-center border-2 border-red-800 bg-red-200 text-red-800 w-96 my-2 py-1 px-4 rounded-md"
    >
      You are incorrect
    </p>
  {:else if studyStatus === StudyStatus.NotSubmitted}
    <p class="text-center border-2 border-black w-96 my-2 py-1 px-4 rounded-md">
      Enter your answer now
    </p>
  {/if}
  <div class="flex flex-row justify-between">
    {#if studyStatus === StudyStatus.NotSubmitted}
      <button
        class="bg-orange-400 my-2 mx-1 px-3 py-2 text-black rounded-md hover:bg-orange-500 hover:text-white hover:shadow-lg"
        on:click={submitAndCheck}>Submit</button
      >
    {:else}
      <button
        class="bg-sky-600 my-2 mx-1 px-3 py-2 text-white rounded-md hover:bg-sky-700 hover:shadow-lg"
        on:click={getCard}>Next</button
      >
    {/if}
    <a
      class="bg-yellow-300 my-2 mx-1 px-3 py-2 text-black rounded-md hover:bg-yello-400 hover:shadow-lg"
      href={`/card-list/${id}`}
      use:link
      on:click={handleExit}>Exit</a
    >
  </div>
</div>
