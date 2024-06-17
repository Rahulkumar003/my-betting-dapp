import Link from 'next/link';

export default function Home() {
  return (
    <div>
      <h1>Betting DApp</h1>
      <nav>
        <ul>
          <li><Link href="/create_event">Create Event</Link></li>
          <li><Link href="/place_bet">Place Bet</Link></li>
          <li><Link href="/update_outcome">Update Outcome</Link></li>
        </ul>
      </nav>
    </div>
  );
}
