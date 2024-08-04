import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58';
import * as prompt from 'prompt-sync';
//Generate a new keypair
let kp = Keypair.generate()

console.log(`You've generated a new Solana wallet:
${kp.publicKey.toBase58()} [${kp.secretKey}]`)
//4dgxP8ajjfjrGFv53rRgnEwL6qcVS4Gqg6b1E4YncqhB