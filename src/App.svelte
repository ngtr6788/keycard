<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import NewDeck from "./lib/NewDeck.svelte";
  import HomePage from "./lib/HomePage.svelte";
  import NewCard from "./lib/NewCard.svelte";
  import StudyCard from "./lib/StudyCard.svelte";
  import CardList from "./lib/CardList.svelte";
</script>

<Router>
  <Route path="/" component={HomePage} />
  <Route path="/new-deck" component={NewDeck} />
  <Route path="/new-card/:id" component={NewCard} />
  <Route path="/study-card/:id" component={StudyCard} />
  <Route path="/card-list/:id" component={CardList} />
</Router>

<style>
</style>
