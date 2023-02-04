<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import NewDeck from "./lib/NewDeck.svelte";
  import EditDeck from "./lib/EditDeck.svelte";
  import HomePage from "./lib/HomePage.svelte";
  import CardForm from "./lib/CardForm.svelte";
  import StudyCard from "./lib/StudyCard.svelte";
  import CardList from "./lib/CardList.svelte";
</script>

<Router>
  <Route path="/" component={HomePage} />
  <Route path="/new-deck" component={NewDeck} />
  <Route path="/edit-deck/:id" component={EditDeck} />
  <Route path="/new-card/:id" component={CardForm} />
  <Route path="/study-card/:id" component={StudyCard} />
  <Route path="/card-list/:id" component={CardList} />
</Router>

<style>
</style>
