**Project Overview**

This project is a decentralized betting application (DApp) built on the Stellar blockchain. It allows users to create events, place bets on the outcomes, and receive payouts based on the final outcome. The project consists of two main components: the smart contract (backend) and the frontend DApp.

**Backend: Smart Contract (Rust)**

The backend of the project is a smart contract written in Rust, which is deployed on the Stellar blockchain. Here's how you can deploy and interact with the smart contract:

1. **Compile the Smart Contract**
   - Open a terminal and navigate to the `contracts` directory.
   - Run the following command to compile the smart contract to a WASM file:

     ```
     cargo build --release --target wasm32-unknown-unknown
     ```

   - The compiled contract should be located in `target/wasm32-unknown-unknown/release/YOUR_CONTRACT_NAME.wasm`.

2. **Deploy the Smart Contract**
   - Use the Soroban CLI tool to deploy the contract on the Stellar testnet:

     ```
     soroban contract deploy --wasm target/wasm32-unknown-unknown/release/YOUR_CONTRACT_NAME.wasm --network testnet
     ```

   - This command will return the contract ID, which you'll need for subsequent steps.


3. **Interact with the Smart Contract**
   - You can interact with the smart contract by invoking its functions using the Soroban CLI tool. For example, to create an event:

     ```
     soroban contract invoke --id <contract_id> --fn create_event --arg '{"event_id":"event1","name":"Match 1","description":"A great match","outcomes":["Team A","Team B"],"betting_deadline":1622520000}' --network testnet --from <creator_address>
     ```

   - Replace `<contract_id>` with the contract ID obtained in step 2 and `<creator_address>` with the address creating the event.

**Frontend: Next.js DApp**

The frontend of the project is a Next.js DApp that interacts with the smart contract on the Stellar blockchain. Here's how you can set up and run the frontend:

1. **Install Dependencies**
   - Open a terminal and navigate to the `frontend` directory.
   - Run the following command to install the required dependencies:

     ```
     npm install
     ```

2. **Configure Freighter Wallet**
   - In the `utils/freighter.js` file, you'll need to configure the Freighter wallet by setting the appropriate options and event listeners.

3. **Integrate Smart Contract Interactions**
   - In the `pages` directory, you'll find JavaScript files for different functionalities (e.g., `create-event.js`, `place-bet.js`, `update-outcome.js`).
   - In each file, you'll need to integrate the corresponding smart contract function calls using the Stellar SDK and the Freighter wallet.

4. **Run the Development Server**
   - Start the development server by running the following command:

     ```
     npm run dev
     ```

   - The DApp should now be accessible at `http://localhost:3000`.

5. **Build and Deploy for Production**
   - When you're ready to deploy the DApp to production, run the following command to create an optimized build:

     ```
     npm run build
     ```

   - Then, you can deploy the contents of the `out` directory to your preferred hosting platform.

**Integration and Usage**

Once both the backend (smart contract) and frontend (DApp) are set up and running, users can interact with the DApp through their Freighter wallet. Here's a typical flow:

1. **Connect to Freighter Wallet**
   - When the user visits the DApp, they will be prompted to connect their Freighter wallet.

2. **Create an Event**
   - The user can navigate to the "Create Event" page and fill in the event details (name, description, outcomes, betting deadline).
   - Upon submission, the frontend will create a transaction using the Stellar SDK, sign it with the user's Freighter wallet, and submit it to the blockchain.
   - The smart contract's `create_event` function will be invoked, storing the event details on the blockchain.

3. **Place a Bet**
   - Users can navigate to the "Place Bet" page, select an event, choose an outcome, and enter the bet amount.
   - Upon submission, a similar process will occur: creating a transaction, signing with the user's wallet, and submitting it to the blockchain.
   - The smart contract's `place_bet` function will be invoked, recording the bet on the blockchain.

4. **Update Outcome and Distribute Winnings**
   - After the event is over, the event creator can navigate to the "Update Outcome" page, select the event, and specify the winning outcome.
   - Upon submission, the smart contract's `update_outcome_and_distribute` function will be invoked, updating the outcome and distributing the winnings to the users who placed bets on the correct outcome.

5. **Check Balances**
   - Users can check their balances on the DApp, which will invoke the smart contract's `get_balance` function and display the user's updated balance after the winnings distribution.

By following these steps, you can deploy both the backend (smart contract) and frontend (DApp) components of the betting application and allow users to interact with the DApp through their Freighter wallets.
