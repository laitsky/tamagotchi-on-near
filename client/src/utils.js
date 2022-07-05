import * as nearAPI from 'near-api-js';
import { getConfig } from './config';

const nearConfig = getConfig();

// Initialize connection to NEAR contract and set global variables
export async function initApp() {
  // Initialize connection to network
  const near = await nearAPI.connect(nearConfig);

  // Connect wallet so users can sign transactions
  window.walletConnection = new nearAPI.WalletConnection(near);

  // Getting accountID
  window.accountID = window.walletConnection.getAccountId();

  // Load contract for later user
  window.contract = await new nearAPI.Contract(
    window.walletConnection.account(),
    nearConfig.contractName,
    {
      viewMethods: [
        'get_state',

      ],
      changeMethods: [
        'feed',
        'play',
        'cure',
        'check_if_sick'
      ]
    }
  );
}

export function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName, "Tamagotchi");
}

export function logout() {
  window.walletConnection.signOut();
  window.location.href = '/';
}
