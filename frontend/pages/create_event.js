import React, { useState } from 'react';
import { createEvent } from '../utils/freighter';

export default function CreateEvent() {
  const [eventId, setEventId] = useState('');
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [outcomes, setOutcomes] = useState('');
  const [bettingDeadline, setBettingDeadline] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await createEvent(eventId, name, description, outcomes.split(','), bettingDeadline);
      alert('Event created successfully');
    } catch (error) {
      alert(`Error creating event: ${error.message}`);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <input type="text" value={eventId} onChange={(e) => setEventId(e.target.value)} placeholder="Event ID" required />
      <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Event Name" required />
      <input type="text" value={description} onChange={(e) => setDescription(e.target.value)} placeholder="Description" required />
      <input type="text" value={outcomes} onChange={(e) => setOutcomes(e.target.value)} placeholder="Outcomes (comma separated)" required />
      <input type="number" value={bettingDeadline} onChange={(e) => setBettingDeadline(e.target.value)} placeholder="Betting Deadline" required />
      <button type="submit">Create Event</button>
    </form>
  );
}
