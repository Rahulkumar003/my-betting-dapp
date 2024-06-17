import { Keypair, Server, Networks, TransactionBuilder, BASE_FEE, Operation } from 'stellar-sdk';

const server = new Server('https://horizon-testnet.stellar.org');

export async function connectFreighter() {
  if (window.freighterApi) {
    const publicKey = await window.freighterApi.getPublicKey();
    return publicKey;
  }
  throw new Error('Freighter wallet not detected');
}

export async function createEvent(eventId, name, description, outcomes, bettingDeadline) {
  const sourcePublicKey = await connectFreighter();
  const account = await server.loadAccount(sourcePublicKey);
  const transaction = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(Operation.manageData({
      name: eventId,
      value: JSON.stringify({ name, description, outcomes, bettingDeadline }),
    }))
    .setTimeout(30)
    .build();
  transaction.sign(Keypair.fromSecret(sourcePublicKey));
  return server.submitTransaction(transaction);
}

export async function placeBet(eventId, outcome, amount) {
  const sourcePublicKey = await connectFreighter();
  const account = await server.loadAccount(sourcePublicKey);
  const transaction = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(Operation.manageData({
      name: `${eventId}:${outcome}`,
      value: amount.toString(),
    }))
    .setTimeout(30)
    .build();
  transaction.sign(Keypair.fromSecret(sourcePublicKey));
  return server.submitTransaction(transaction);
}

export async function updateOutcome(eventId, outcome) {
  const sourcePublicKey = await connectFreighter();
  const account = await server.loadAccount(sourcePublicKey);
  const transaction = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(Operation.manageData({
      name: `${eventId}:outcome`,
      value: outcome,
    }))
    .setTimeout(30)
    .build();
  transaction.sign(Keypair.fromSecret(sourcePublicKey));
  return server.submitTransaction(transaction);
}
