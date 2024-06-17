import React, { useState } from 'react';
import { updateOutcome } from '../utils/freighter';

export default function UpdateOutcome() {
  const [eventId, setEventId] = useState('');
  const [outcome, setOutcome] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await updateOutcome(eventId, outcome);
      alert('Outcome updated and winnings distributed successfully');
    } catch (error) {
      alert(`Error updating outcome: ${error.message}`);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <input type="text" value={eventId} onChange={(e) => setEventId(e.target.value)} placeholder="Event ID" required />
      <input type="text" value={outcome} onChange={(e) => setOutcome(e.target.value)} placeholder="Outcome" required />
      <button type="submit">Update Outcome</button>
    </form>
  );
}
