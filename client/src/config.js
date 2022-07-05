import * as nearAPI from 'near-api-js';

const CONTRACT_NAME_TESTNET = 'game.tamagotchi.testnet';

export function getConfig() {
  return {
    networkId: 'testnet',
    keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    contractName: CONTRACT_NAME_TESTNET,
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org',
  }
}