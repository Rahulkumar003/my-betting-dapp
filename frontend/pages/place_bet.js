import React, { useState } from 'react';
import { placeBet } from '../utils/freighter';

export default function PlaceBet() {
  const [eventId, setEventId] = useState('');
  const [outcome, setOutcome] = useState('');
  const [amount, setAmount] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await placeBet(eventId, outcome, amount);
      alert('Bet placed successfully');
    } catch (error) {
      alert(`Error placing bet: ${error.message}`);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <input type="text" value={eventId} onChange={(e) => setEventId(e.target.value)} placeholder="Event ID" required />
      <input type="text" value={outcome} onChange={(e) => setOutcome(e.target.value)} placeholder="Outcome" required />
      <input type="number" value={amount} onChange={(e) => setAmount(e.target.value)} placeholder="Amount" required />
      <button type="submit">Place Bet</button>
    </form>
  );
}
