export type Deck = {
  id: number;
  deck_name: string;
  deck_description: string;
};

export type Card = {
  deck_id: number;
  id: number;
  card_question: string;
  keys_list: string[];
};
