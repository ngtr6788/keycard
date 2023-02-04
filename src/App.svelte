<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import NewDeck from "./lib/NewDeck.svelte";
  import EditDeck from "./lib/EditDeck.svelte";
  import HomePage from "./lib/HomePage.svelte";
  import NewCard from "./lib/NewCard.svelte";
  import EditCard from "./lib/EditCard.svelte";
  import StudyCard from "./lib/StudyCard.svelte";
  import CardList from "./lib/CardList.svelte";
</script>

<Router>
  <Route path="/" component={HomePage} />
  <Route path="/new-deck" component={NewDeck} />
  <Route path="/edit-deck/:id" component={EditDeck} />
  <Route path="/new-card/:deck_id" component={NewCard} />
  <Route path="/edit-card/:deck_id/:card_id" component={EditCard} />
  <Route path="/study-card/:id" component={StudyCard} />
  <Route path="/card-list/:id" component={CardList} />
</Router>

<style>
</style>
