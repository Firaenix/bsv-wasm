/* tslint:disable */
/* eslint-disable */
/**
*/
export enum SigningType {
  Raw,
  Message,
  SigHash,
  SigHashR,
}
/**
*/
export enum TwetchPayActionType {
  Twetch,
  Sigil,
}
/**
*/
export enum Networks {
  BSV,
  TBSV,
}
/**
*/
export enum OpCodes {
/**
* Pushes 0 onto the stack
*/
  OP_0,
/**
* Pushes 0 onto the stack
* The next byte sets the number of bytes to push onto the stack
*/
  OP_PUSHDATA1,
/**
* The next two bytes sets the number of bytes to push onto the stack
*/
  OP_PUSHDATA2,
/**
* The next four bytes sets the number of bytes to push onto the stack
*/
  OP_PUSHDATA4,
/**
* Pushes -1 onto the stack
*/
  OP_1NEGATE,
/**
* Pushes 1 onto the stack
*/
  OP_1,
/**
* Pushes 1 onto the stack
* Pushes 2 onto the stack
*/
  OP_2,
/**
* Pushes 3 onto the stack
*/
  OP_3,
/**
* Pushes 4 onto the stack
*/
  OP_4,
/**
* Pushes 5 onto the stack
*/
  OP_5,
/**
* Pushes 6 onto the stack
*/
  OP_6,
/**
* Pushes 7 onto the stack
*/
  OP_7,
/**
* Pushes 8 onto the stack
*/
  OP_8,
/**
* Pushes 9 onto the stack
*/
  OP_9,
/**
* Pushes 10 onto the stack
*/
  OP_10,
/**
* Pushes 11 onto the stack
*/
  OP_11,
/**
* Pushes 12 onto the stack
*/
  OP_12,
/**
* Pushes 13 onto the stack
*/
  OP_13,
/**
* Pushes 14 onto the stack
*/
  OP_14,
/**
* Pushes 15 onto the stack
*/
  OP_15,
/**
* Pushes 16 onto the stack
*/
  OP_16,
/**
* Does nothing
*/
  OP_NOP,
/**
* If the top stack is true, statements are executed. Top stack value is removed.
*/
  OP_IF,
/**
* If the top stack is false, statements are executed. Top stack value is removed.
*/
  OP_NOTIF,
/**
* If the preceding OP_IF or OP_NOTIF statemetns were not executed, then statements are executed.
*/
  OP_ELSE,
/**
* Ends an if-else block
*/
  OP_ENDIF,
/**
* Marks a statement as invalid if the top stack value is false. Top stack value is removed.
*/
  OP_VERIFY,
/**
* Marks a statements as invalid
*/
  OP_RETURN,
/**
* Moves the top item on the main stack to the alt stack
*/
  OP_TOALTSTACK,
/**
* Moves the top item on the alt stack to the main stack
*/
  OP_FROMALTSTACK,
/**
* Duplicates the top stack value if it is not zero
*/
  OP_IFDUP,
/**
* Puts the number of stack items onto the stack
*/
  OP_DEPTH,
/**
* Drops the top stack value
*/
  OP_DROP,
/**
* Duplicates the top stack item
*/
  OP_DUP,
/**
* Removes the second-to-top stack item
*/
  OP_NIP,
/**
* Copies the second-to-top stack item to the top
*/
  OP_OVER,
/**
* The item n back in the stack is copied to the top
*/
  OP_PICK,
/**
* The item n back in the stack is moved to the top
*/
  OP_ROLL,
/**
* The top three items on the stack are rotated to the left
*/
  OP_ROT,
/**
* The top two items on the stack are swapped
*/
  OP_SWAP,
/**
* The item at the top of the stack is copied and inserted before the second-to-top item
*/
  OP_TUCK,
/**
* Removes the top two items from the stack
*/
  OP_2DROP,
/**
* Duplicates the top two stack items
*/
  OP_2DUP,
/**
* Duplicates the top three stack items
*/
  OP_3DUP,
/**
* Copies the pair of items two spaces back to the front
*/
  OP_2OVER,
/**
* The fifth and sixth items back are moved to the top of the stack
*/
  OP_2ROT,
/**
* Swaps the top two pairs of items
*/
  OP_2SWAP,
/**
* Concatenates two byte sequences
*/
  OP_CAT,
/**
* Splits the byte sequence at position n
*/
  OP_SPLIT,
/**
* Pushes the byte sequence length of the top stack item without popping it
*/
  OP_SIZE,
/**
* Flips all of the bits in the input
*/
  OP_INVERT,
/**
* Boolean and between each bit in the inputs
*/
  OP_AND,
/**
* Boolean or between each bit in the inputs
*/
  OP_OR,
/**
* Boolean exclusive or between each bit in the inputs
*/
  OP_XOR,
/**
* Returns 1 if the inputs are exactly equal, 0 otherwise
*/
  OP_EQUAL,
/**
* Same as OP_EQUAL, but runs OP_VERIFY afterward
*/
  OP_EQUALVERIFY,
/**
* Adds 1 to the input
*/
  OP_1ADD,
/**
* Subtracts 1 from the input
*/
  OP_1SUB,
/**
* The sign of the input is flipped
*/
  OP_NEGATE,
/**
* The input is made positive
*/
  OP_ABS,
/**
* If the input is 0 or 1, it is flipped. Otherwise, the output will be 0.
*/
  OP_NOT,
/**
* Returns 0 if the input is 0. 1 otherwise.
*/
  OP_0NOTEQUAL,
/**
* Adds a to b
*/
  OP_ADD,
/**
* Subtracts b from a
*/
  OP_SUB,
/**
* Multiplies a by b
*/
  OP_MUL,
/**
* Divides a by b
*/
  OP_DIV,
/**
* Returns the remainder after dividing a by b
*/
  OP_MOD,
/**
* Shifts a left b bits, preserving sign
*/
  OP_LSHIFT,
/**
* Shifts a right b bits, preserving sign
*/
  OP_RSHIFT,
/**
* If both a and b are not empty, the output is 1. Otherwise, 0.
*/
  OP_BOOLAND,
/**
* If a or b is not empty, the output is 1. Otherwise, 0.
*/
  OP_BOOLOR,
/**
* Returns 1 if the numbers are equal. Otherwise, 0.
*/
  OP_NUMEQUAL,
/**
* Same as OP_NUMEQUAL, but runs OP_VERIFY afterward
*/
  OP_NUMEQUALVERIFY,
/**
* Returns 1 if the numbers are not equal. Otherwise, 0.
*/
  OP_NUMNOTEQUAL,
/**
* Returns 1 if a is less than b. Otherwise, 0.
*/
  OP_LESSTHAN,
/**
* Returns 1 if a is greater than b. Otherwise, 0.
*/
  OP_GREATERTHAN,
/**
* Returns 1 if a is less than or equal to b. Otherwise, 0.
*/
  OP_LESSTHANOREQUAL,
/**
* Returns 1 if a is greater than or equal to b. Otherwise, 0.
*/
  OP_GREATERTHANOREQUAL,
/**
* Returns the smaller of a and b
*/
  OP_MIN,
/**
* Returns the larger of a and b
*/
  OP_MAX,
/**
* Returns 1 if x is within the specified range, left inclusive. Otherwise, 0.
*/
  OP_WITHIN,
/**
* Converts numeric value a into a byte sequence of length b
*/
  OP_NUM2BIN,
/**
* Converts byte sequence x into a numeric value
*/
  OP_BIN2NUM,
/**
* The input is hashed using RIPEMD-160
*/
  OP_RIPEMD160,
/**
* The input is hashed using SHA-1
*/
  OP_SHA1,
/**
* The input is hashed using SHA-256
*/
  OP_SHA256,
/**
* The input is hashed twice: first with SHA-256 and then with RIPEMD-160
*/
  OP_HASH160,
/**
* The input is hashed two times with SHA-256
*/
  OP_HASH256,
/**
* Marks the part of the script after which the signature will begin matching
*/
  OP_CODESEPARATOR,
/**
* Puts 1 on the stack if the signature authorizes the public key and transaction hash. Otherwise 0.
*/
  OP_CHECKSIG,
/**
* Same as OP_CHECKSIG, but OP_VERIFY is executed afterward
*/
  OP_CHECKSIGVERIFY,
/**
* Puts 1 on the stack if m of n signatures authorize the public key and transaction hash. Otherwise 0.
*/
  OP_CHECKMULTISIG,
/**
* Same as OP_CHECKMULTISIG, but OP_VERIFY is executed afterward
*/
  OP_CHECKMULTISIGVERIFY,
/**
* Marks transaction as invalid if the top stack item is greater than the transaction's lock_time
*/
  OP_CHECKLOCKTIMEVERIFY,
/**
* Marks transaction as invalid if the top stack item is less than the transaction's sequence used for relative lock time
*/
  OP_CHECKSEQUENCEVERIFY,
/**
* OP_DATA followed by a varint represents arbitrary data on chain. Used for matching Script Templates.
*/
  OP_DATA,
/**
* Represents a secp256k1 signature
*/
  OP_SIG,
/**
* Represents a public key hashed with OP_HASH160
*/
  OP_PUBKEYHASH,
/**
* Represents a public key compatible with OP_CHECKSIG
*/
  OP_PUBKEY,
/**
* Matches any opcode that is not yet assigned
*/
  OP_INVALIDOPCODE,
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
  OP_RESERVED,
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
  OP_VER,
/**
* Transaction is invalid even when occuring in an unexecuted OP_IF branch
*/
  OP_VERIF,
/**
* Transaction is invalid even when occuring in an unexecuted OP_IF branch
*/
  OP_VERNOTIF,
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
  OP_RESERVED1,
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
  OP_RESERVED2,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP1,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP4,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP5,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP6,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP7,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP8,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP9,
/**
* The word is ignored. Does not mark transaction as invalid.
*/
  OP_NOP10,
/**
* Words at or above this number are invalid
*/
  OP_INVALID_ABOVE,
/**
* The input is multiplied by 2
*/
  OP_2MUL,
/**
* The input is divided by 2
*/
  OP_2DIV,
}
/**
*/
export enum SigHash {
  FORKID,
  ALL,
  NONE,
  SINGLE,
  ANYONECANPAY,
/**
*
*     * ALL | FORKID
*     
*/
  InputsOutputs,
/**
*
*     * NONE | FORKID
*     
*/
  Inputs,
/**
*
*     * SINGLE | FORKID
*     
*/
  InputsOutput,
/**
*
*     * ALL | ANYONECANPAY | FORKID
*     
*/
  InputOutputs,
/**
*
*     * NONE | ANYONECANPAY | FORKID
*     
*/
  Input,
/**
*
*     * SINGLE | ANYONECANPAY | FORKID
*     
*/
  InputOutput,
/**
*
*     * ALL | ANYONECANPAY
*     
*/
  Legacy_InputOutputs,
/**
*
*     * NONE | ANYONECANPAY
*     
*/
  Legacy_Input,
/**
*
*     * SINGLE | ANYONECANPAY
*     
*/
  Legacy_InputOutput,
}
/**
*/
export enum Status {
  Running,
  Finished,
}
/**
*/
export enum PBKDF2Hashes {
  SHA1,
  SHA256,
  SHA512,
}
/**
*/
export enum SigningHash {
  Sha256,
  Sha256d,
}
/**
*/
export enum AESAlgorithms {
  AES128_CBC,
  AES256_CBC,
  AES128_CTR,
  AES256_CTR,
}
/**
*/
export class AES {
  free(): void;
/**
* @param {Uint8Array} key
* @param {Uint8Array} iv
* @param {Uint8Array} message
* @param {number} algo
* @returns {Uint8Array}
*/
  static encrypt(key: Uint8Array, iv: Uint8Array, message: Uint8Array, algo: number): Uint8Array;
/**
* @param {Uint8Array} key
* @param {Uint8Array} iv
* @param {Uint8Array} message
* @param {number} algo
* @returns {Uint8Array}
*/
  static decrypt(key: Uint8Array, iv: Uint8Array, message: Uint8Array, algo: number): Uint8Array;
}
/**
*/
export class AuthToken {
  free(): void;
}
/**
*/
export class Authentication {
  free(): void;
/**
* @param {string} email
* @param {string} password
* @returns {AuthenticationCipher}
*/
  static getCipher(email: string, password: string): AuthenticationCipher;
}
/**
*/
export class AuthenticationCipher {
  free(): void;
/**
* @returns {string}
*/
  getEmailHash(): string;
/**
* @returns {string}
*/
  getPasswordHash(): string;
/**
* @returns {string}
*/
  getCipher(): string;
/**
* @param {string} encrypted_mnemonic
* @returns {string | undefined}
*/
  decryptMnemonic(encrypted_mnemonic: string): string | undefined;
}
/**
*/
export class BSM {
  free(): void;
/**
*
*     * Sign a message with the intention of verifying with this same Address.
*     * Used when using Bitcoin Signed Messages
*     *
*     * Returns boolean
*     
* @param {Uint8Array} message
* @param {Signature} signature
* @param {P2PKHAddress} address
* @returns {boolean}
*/
  static is_valid_message(message: Uint8Array, signature: Signature, address: P2PKHAddress): boolean;
/**
* @param {Uint8Array} message
* @param {Signature} signature
* @param {P2PKHAddress} address
* @returns {boolean}
*/
  static verify_message(message: Uint8Array, signature: Signature, address: P2PKHAddress): boolean;
/**
* @param {PrivateKey} priv_key
* @param {Uint8Array} message
* @returns {Signature}
*/
  static sign_message(priv_key: PrivateKey, message: Uint8Array): Signature;
/**
* @param {PrivateKey} priv_key
* @param {PrivateKey} ephemeral_key
* @param {Uint8Array} message
* @returns {Signature}
*/
  static sign_message_with_k(priv_key: PrivateKey, ephemeral_key: PrivateKey, message: Uint8Array): Signature;
}
/**
*/
export class BuiltTx {
  free(): void;
/**
* @param {number} index
* @returns {PaymentDestination}
*/
  get_payment_destination(index: number): PaymentDestination;
/**
*/
  readonly encrypted_hash: string | undefined;
/**
*/
  readonly extended_tx: string | undefined;
/**
*/
  readonly fee_sats: BigInt;
/**
*/
  readonly nfts: any;
/**
*/
  readonly num_payment_destinations: number;
/**
*/
  readonly rawtx: string | undefined;
/**
*/
  readonly total_cost_sats: BigInt;
/**
*/
  readonly txid: string;
}
/**
*/
export class ChainParams {
  free(): void;
/**
*/
  constructor();
/**
* @returns {ChainParams}
*/
  static mainnet(): ChainParams;
/**
* @returns {ChainParams}
*/
  static testnet(): ChainParams;
/**
* @returns {ChainParams}
*/
  static regtest(): ChainParams;
/**
* @returns {ChainParams}
*/
  static stn(): ChainParams;
}
/**
*/
export class ChatMessage {
  free(): void;
/**
* @param {Uint8Array} key
* @param {string} plaintext
* @returns {string | undefined}
*/
  static encrypt(key: Uint8Array, plaintext: string): string | undefined;
/**
* @param {Uint8Array} key
* @param {Uint8Array} description
* @returns {ChatMessage | undefined}
*/
  static decrypt(key: Uint8Array, description: Uint8Array): ChatMessage | undefined;
/**
* @returns {string}
*/
  plaintext(): string;
}
/**
*/
export class CipherKeys {
  free(): void;
/**
* @returns {Uint8Array}
*/
  get_iv(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  get_ke(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  get_km(): Uint8Array;
}
/**
*/
export class Conversation {
  free(): void;
/**
* @returns {string}
*/
  static generateKey(): string;
/**
* @param {string} key
* @param {string} pubkey
* @returns {string | undefined}
*/
  static encrypt(key: string, pubkey: string): string | undefined;
/**
* @param {string} encrypted_key
* @param {string} seed
* @returns {Uint8Array | undefined}
*/
  static decrypt(encrypted_key: string, seed: string): Uint8Array | undefined;
}
/**
*/
export class ECDH {
  free(): void;
/**
* @param {PrivateKey} priv_key
* @param {PublicKey} pub_key
* @returns {Uint8Array}
*/
  static derive_shared_key(priv_key: PrivateKey, pub_key: PublicKey): Uint8Array;
}
/**
*/
export class ECDSA {
  free(): void;
/**
* @param {Signature} signature
* @param {PublicKey} public_key
* @param {PrivateKey} ephemeral_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @returns {PrivateKey}
*/
  static private_key_from_signature_k(signature: Signature, public_key: PublicKey, ephemeral_key: PrivateKey, preimage: Uint8Array, hash_algo: number): PrivateKey;
/**
* @param {PrivateKey} private_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @param {boolean} reverse_k
* @returns {Signature}
*/
  static sign_with_random_k(private_key: PrivateKey, preimage: Uint8Array, hash_algo: number, reverse_k: boolean): Signature;
/**
* @param {PrivateKey} private_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @param {boolean} reverse_k
* @returns {Signature}
*/
  static sign_with_deterministic_k(private_key: PrivateKey, preimage: Uint8Array, hash_algo: number, reverse_k: boolean): Signature;
/**
* @param {PrivateKey} private_key
* @param {PrivateKey} ephemeral_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @returns {Signature}
*/
  static sign_with_k(private_key: PrivateKey, ephemeral_key: PrivateKey, preimage: Uint8Array, hash_algo: number): Signature;
/**
* @param {Uint8Array} message
* @param {PublicKey} pub_key
* @param {Signature} signature
* @param {number} hash_algo
* @returns {boolean}
*/
  static verify_digest(message: Uint8Array, pub_key: PublicKey, signature: Signature, hash_algo: number): boolean;
}
/**
*/
export class ECIES {
  free(): void;
/**
* @param {Uint8Array} message
* @param {PrivateKey} sender_priv_key
* @param {PublicKey} recipient_pub_key
* @param {boolean} exclude_pub_key
* @returns {ECIESCiphertext}
*/
  static encrypt(message: Uint8Array, sender_priv_key: PrivateKey, recipient_pub_key: PublicKey, exclude_pub_key: boolean): ECIESCiphertext;
/**
*
*     * Encrypt with a randomly generate private key.
*     * This is intended to be used if you want to anonymously send a party an encrypted message.
*     
* @param {Uint8Array} message
* @param {PublicKey} recipient_pub_key
* @returns {ECIESCiphertext}
*/
  static encrypt_with_ephemeral_private_key(message: Uint8Array, recipient_pub_key: PublicKey): ECIESCiphertext;
/**
* @param {ECIESCiphertext} ciphertext
* @param {PrivateKey} recipient_priv_key
* @param {PublicKey} sender_pub_key
* @returns {Uint8Array}
*/
  static decrypt(ciphertext: ECIESCiphertext, recipient_priv_key: PrivateKey, sender_pub_key: PublicKey): Uint8Array;
/**
* @param {PrivateKey} priv_key
* @param {PublicKey} pub_key
* @returns {CipherKeys}
*/
  static derive_cipher_keys(priv_key: PrivateKey, pub_key: PublicKey): CipherKeys;
}
/**
*/
export class ECIESCiphertext {
  free(): void;
/**
* @returns {Uint8Array}
*/
  get_ciphertext(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  get_hmac(): Uint8Array;
/**
* @returns {CipherKeys | undefined}
*/
  get_cipher_keys(): CipherKeys | undefined;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {PublicKey}
*/
  extract_public_key(): PublicKey;
/**
* @param {Uint8Array} buffer
* @param {boolean} has_pub_key
* @returns {ECIESCiphertext}
*/
  static from_bytes(buffer: Uint8Array, has_pub_key: boolean): ECIESCiphertext;
}
/**
*/
export class EphemeralCipher {
  free(): void;
/**
*/
  readonly cipher_text: Uint8Array;
/**
*/
  readonly hash: Uint8Array;
}
/**
*/
export class ExtendedPrivateKey {
  free(): void;
/**
* @returns {PrivateKey}
*/
  get_private_key(): PrivateKey;
/**
* @returns {PublicKey}
*/
  get_public_key(): PublicKey;
/**
* @returns {Uint8Array}
*/
  get_chain_code(): Uint8Array;
/**
* @returns {number}
*/
  get_depth(): number;
/**
* @returns {Uint8Array}
*/
  get_parent_fingerprint(): Uint8Array;
/**
* @returns {number}
*/
  get_index(): number;
/**
* @param {number} index
* @returns {ExtendedPrivateKey}
*/
  derive(index: number): ExtendedPrivateKey;
/**
* @param {string} path
* @returns {ExtendedPrivateKey}
*/
  derive_from_path(path: string): ExtendedPrivateKey;
/**
* @param {Uint8Array} seed
* @returns {ExtendedPrivateKey}
*/
  static from_seed(seed: Uint8Array): ExtendedPrivateKey;
/**
* @returns {ExtendedPrivateKey}
*/
  static from_random(): ExtendedPrivateKey;
/**
* @param {string} xprv_string
* @returns {ExtendedPrivateKey}
*/
  static from_string(xprv_string: string): ExtendedPrivateKey;
/**
* @returns {string}
*/
  to_string(): string;
/**
* @param {Uint8Array} mnemonic
* @param {Uint8Array | undefined} passphrase
* @returns {ExtendedPrivateKey}
*/
  static from_mnemonic(mnemonic: Uint8Array, passphrase?: Uint8Array): ExtendedPrivateKey;
}
/**
*/
export class ExtendedPublicKey {
  free(): void;
/**
* @returns {PublicKey}
*/
  get_public_key(): PublicKey;
/**
* @param {ExtendedPrivateKey} xpriv
* @returns {ExtendedPublicKey}
*/
  static from_xpriv(xpriv: ExtendedPrivateKey): ExtendedPublicKey;
/**
* @returns {Uint8Array}
*/
  get_chain_code(): Uint8Array;
/**
* @returns {number}
*/
  get_depth(): number;
/**
* @returns {Uint8Array}
*/
  get_parent_fingerprint(): Uint8Array;
/**
* @returns {number}
*/
  get_index(): number;
/**
* @param {number} index
* @returns {ExtendedPublicKey}
*/
  derive(index: number): ExtendedPublicKey;
/**
* @param {string} path
* @returns {ExtendedPublicKey}
*/
  derive_from_path(path: string): ExtendedPublicKey;
/**
* @param {Uint8Array} seed
* @returns {ExtendedPublicKey}
*/
  static from_seed(seed: Uint8Array): ExtendedPublicKey;
/**
* @returns {ExtendedPublicKey}
*/
  static from_random(): ExtendedPublicKey;
/**
* @param {string} xpub_string
* @returns {ExtendedPublicKey}
*/
  static from_string(xpub_string: string): ExtendedPublicKey;
/**
* @returns {string}
*/
  to_string(): string;
}
/**
*/
export class Hash {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha_256d(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha_256(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha_1(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static ripemd_160(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static hash_160(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha_512(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha_512_hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha_256_hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha_256d_hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha_1_hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static ripemd_160_hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static hash_160_hmac(input: Uint8Array, key: Uint8Array): Hash;
}
/**
*/
export class Interpreter {
  free(): void;
/**
* @param {Transaction} tx
* @param {number} txin_idx
* @returns {Interpreter}
*/
  static from_transaction(tx: Transaction, txin_idx: number): Interpreter;
/**
* @param {Script} script
* @returns {Interpreter}
*/
  static from_script(script: Script): Interpreter;
/**
*/
  run(): void;
/**
* @returns {State | undefined}
*/
  next(): State | undefined;
/**
* @returns {State}
*/
  get_state(): State;
}
/**
*/
export class KDF {
  free(): void;
/**
* @returns {Hash}
*/
  get_hash(): Hash;
/**
* @returns {Uint8Array}
*/
  get_salt(): Uint8Array;
/**
*
*     * Implementation of PBKDF2 - when None is specified for salt, a random salt will be generated
*     
* @param {Uint8Array} password
* @param {Uint8Array | undefined} salt
* @param {number} hash_algo
* @param {number} rounds
* @param {number} output_length
* @returns {KDF}
*/
  static pbkdf2(password: Uint8Array, salt: Uint8Array | undefined, hash_algo: number, rounds: number, output_length: number): KDF;
}
/**
*/
export class P2PKHAddress {
  free(): void;
/**
* @param {Uint8Array} hash_bytes
* @returns {P2PKHAddress}
*/
  static from_pubkey_hash(hash_bytes: Uint8Array): P2PKHAddress;
/**
* @param {PublicKey} pub_key
* @returns {P2PKHAddress}
*/
  static from_pubkey(pub_key: PublicKey): P2PKHAddress;
/**
* @param {ChainParams} chain_params
* @returns {P2PKHAddress}
*/
  set_chain_params(chain_params: ChainParams): P2PKHAddress;
/**
* @returns {string}
*/
  to_string(): string;
/**
* @param {string} address_string
* @returns {P2PKHAddress}
*/
  static from_string(address_string: string): P2PKHAddress;
/**
* @returns {Script}
*/
  get_locking_script(): Script;
/**
* @param {PublicKey} pub_key
* @param {SighashSignature} sig
* @returns {Script}
*/
  get_unlocking_script(pub_key: PublicKey, sig: SighashSignature): Script;
/**
* @param {Uint8Array} message
* @param {Signature} signature
* @returns {boolean}
*/
  verify_bitcoin_message(message: Uint8Array, signature: Signature): boolean;
}
/**
*/
export class PayCommand {
  free(): void;
/**
* @param {string} description
* @returns {PayCommand | undefined}
*/
  static from_string(description: string): PayCommand | undefined;
/**
* @param {number} exchange_rate
* @returns {number}
*/
  get_amount_usd(exchange_rate: number): number;
/**
* @param {number} exchange_rate
* @returns {number}
*/
  get_amount_bsv(exchange_rate: number): number;
}
/**
*/
export class PaymentDestination {
  free(): void;
/**
*/
  readonly paymail: string;
/**
*/
  readonly reference: string;
}
/**
*/
export class Post {
  free(): void;
/**
* @param {string} description
* @returns {Post}
*/
  static fromDescription(description: string): Post;
/**
* @param {number} exchange_rate
* @returns {number}
*/
  estimateUsd(exchange_rate: number): number;
/**
* @param {number} exchange_rate
* @returns {string | undefined}
*/
  getPayCommand(exchange_rate: number): string | undefined;
}
/**
*/
export class PrivateKey {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @returns {PrivateKey}
*/
  static from_random(): PrivateKey;
/**
* @returns {Uint8Array}
*/
  get_point(): Uint8Array;
/**
* @param {boolean} should_compress
* @returns {PrivateKey}
*/
  compress_public_key(should_compress: boolean): PrivateKey;
/**
* @param {string} wif_string
* @returns {PrivateKey}
*/
  static from_wif(wif_string: string): PrivateKey;
/**
* @param {string} hex_str
* @returns {PrivateKey}
*/
  static from_hex(hex_str: string): PrivateKey;
/**
*
*     * Standard ECDSA Message Signing using SHA256 as the digestg
*     
* @param {Uint8Array} msg
* @returns {Signature}
*/
  sign_message(msg: Uint8Array): Signature;
/**
* @returns {string}
*/
  to_wif(): string;
/**
* @param {Uint8Array} bytes
* @returns {PrivateKey}
*/
  static from_bytes(bytes: Uint8Array): PrivateKey;
/**
* @returns {PublicKey}
*/
  to_public_key(): PublicKey;
/**
*
*     * Encrypt a message to the public key of this private key.
*     
* @param {Uint8Array} message
* @returns {ECIESCiphertext}
*/
  encrypt_message(message: Uint8Array): ECIESCiphertext;
/**
*
*     * Decrypt a message that was sent to the public key corresponding to this private key.
*     
* @param {ECIESCiphertext} ciphertext
* @param {PublicKey} sender_pub_key
* @returns {Uint8Array}
*/
  decrypt_message(ciphertext: ECIESCiphertext, sender_pub_key: PublicKey): Uint8Array;
}
/**
*/
export class PublicKey {
  free(): void;
/**
* @returns {P2PKHAddress}
*/
  to_address(): P2PKHAddress;
/**
* @param {string} hex_str
* @returns {PublicKey}
*/
  static from_hex(hex_str: string): PublicKey;
/**
* @param {Uint8Array} bytes
* @returns {PublicKey}
*/
  static from_bytes(bytes: Uint8Array): PublicKey;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {PrivateKey} priv_key
* @returns {PublicKey}
*/
  static from_private_key(priv_key: PrivateKey): PublicKey;
/**
* @param {Uint8Array} message
* @param {Signature} signature
* @returns {boolean}
*/
  verify_message(message: Uint8Array, signature: Signature): boolean;
/**
* @returns {P2PKHAddress}
*/
  to_p2pkh_address(): P2PKHAddress;
/**
* @returns {PublicKey}
*/
  to_compressed(): PublicKey;
/**
* @returns {PublicKey}
*/
  to_decompressed(): PublicKey;
/**
* @param {Uint8Array} message
* @param {PrivateKey} sender_private_key
* @returns {ECIESCiphertext}
*/
  encrypt_message(message: Uint8Array, sender_private_key: PrivateKey): ECIESCiphertext;
/**
* @param {Uint8Array} message
* @param {Signature} signature
* @returns {boolean}
*/
  is_valid_message(message: Uint8Array, signature: Signature): boolean;
/**
* @returns {boolean}
*/
  is_compressed(): boolean;
}
/**
*/
export class PublishParams {
  free(): void;
/**
*/
  readonly token: string | undefined;
}
/**
*/
export class RecoveryInfo {
  free(): void;
/**
* @param {boolean} is_y_odd
* @param {boolean} is_x_reduced
* @param {boolean} is_pubkey_compressed
*/
  constructor(is_y_odd: boolean, is_x_reduced: boolean, is_pubkey_compressed: boolean);
/**
* @param {number} recovery_byte
* @param {boolean} is_pubkey_compressed
* @returns {RecoveryInfo}
*/
  static from_byte(recovery_byte: number, is_pubkey_compressed: boolean): RecoveryInfo;
}
/**
*/
export class Script {
  free(): void;
/**
* @returns {string}
*/
  to_asm_string(): string;
/**
* @returns {string}
*/
  to_extended_asm_string(): string;
/**
* @param {string} hex
* @returns {Script}
*/
  static from_hex(hex: string): Script;
/**
* @param {Uint8Array} bytes
* @returns {Script}
*/
  static from_bytes(bytes: Uint8Array): Script;
/**
* @param {string} asm_string
* @returns {Script}
*/
  static from_asm_string(asm_string: string): Script;
/**
* @param {Uint8Array} data_bytes
* @returns {Uint8Array}
*/
  static encode_pushdata(data_bytes: Uint8Array): Uint8Array;
/**
*
*     * Gets the OP_PUSHDATA prefix varint
*     
* @param {number} length
* @returns {Uint8Array}
*/
  static get_pushdata_bytes(length: number): Uint8Array;
/**
* @returns {any}
*/
  to_script_bits(): any;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {number}
*/
  get_script_length(): number;
/**
* @returns {string}
*/
  to_hex(): string;
/**
*/
  remove_codeseparators(): void;
}
/**
*/
export class SighashSignature {
  free(): void;
/**
* @param {Signature} signature
* @param {number} sighash_type
* @param {Uint8Array} sighash_buffer
*/
  constructor(signature: Signature, sighash_type: number, sighash_buffer: Uint8Array);
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @param {Uint8Array} sighash_buffer
* @returns {SighashSignature}
*/
  static from_bytes(bytes: Uint8Array, sighash_buffer: Uint8Array): SighashSignature;
}
/**
*/
export class Signature {
  free(): void;
/**
* You should use to_der_hex() now
* @returns {string}
*/
  to_hex(): string;
/**
* You should use to_der_bytes() now
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Signature}
*/
  static from_der(bytes: Uint8Array): Signature;
/**
* @param {string} hex
* @returns {Signature}
*/
  static from_hex_der(hex: string): Signature;
/**
* @param {Uint8Array} compact_bytes
* @returns {Signature}
*/
  static from_compact_bytes(compact_bytes: Uint8Array): Signature;
/**
* @param {Uint8Array} message
* @param {number} hash_algo
* @returns {PublicKey}
*/
  recover_public_key(message: Uint8Array, hash_algo: number): PublicKey;
/**
* @returns {string}
*/
  to_der_hex(): string;
/**
* @returns {Uint8Array}
*/
  to_der_bytes(): Uint8Array;
/**
* @param {RecoveryInfo | undefined} recovery_info
* @returns {Uint8Array}
*/
  to_compact_bytes(recovery_info?: RecoveryInfo): Uint8Array;
/**
* @returns {Uint8Array}
*/
  r(): Uint8Array;
/**
* @returns {string}
*/
  r_hex(): string;
/**
* @returns {Uint8Array}
*/
  s(): Uint8Array;
/**
* @returns {string}
*/
  s_hex(): string;
/**
* @param {RecoveryInfo | undefined} recovery_info
* @returns {string}
*/
  to_compact_hex(recovery_info?: RecoveryInfo): string;
/**
* @param {Uint8Array} message
* @param {PublicKey} pub_key
* @returns {boolean}
*/
  verify_message(message: Uint8Array, pub_key: PublicKey): boolean;
}
/**
*/
export class State {
  free(): void;
/**
* @returns {Script}
*/
  get_executed_script(): Script;
/**
* @returns {any}
*/
  get_stack(): any;
/**
* @returns {any}
*/
  get_alt_stack(): any;
/**
* @returns {number}
*/
  get_status(): number;
}
/**
*/
export class Transaction {
  free(): void;
/**
* @returns {number}
*/
  get_version(): number;
/**
* @returns {number}
*/
  get_ninputs(): number;
/**
* @returns {number}
*/
  get_noutputs(): number;
/**
* @param {number} index
* @returns {TxIn | undefined}
*/
  get_input(index: number): TxIn | undefined;
/**
* @param {number} index
* @returns {TxOut | undefined}
*/
  get_output(index: number): TxOut | undefined;
/**
* @returns {number}
*/
  get_n_locktime(): number;
/**
* @returns {Uint8Array}
*/
  get_n_locktime_as_bytes(): Uint8Array;
/**
*
*     * Creates a new empty transaction where you need to add inputs and outputs
*     * Transaction.add_input(TxIn) and Transaction.add_output(TxOut)
*     
* @param {number} version
* @param {number} n_locktime
*/
  constructor(version: number, n_locktime: number);
/**
* @returns {Transaction}
*/
  static default(): Transaction;
/**
* @param {number} version
* @returns {Transaction}
*/
  set_version(version: number): Transaction;
/**
* @param {number} n_locktime
* @returns {Transaction}
*/
  set_nlocktime(n_locktime: number): Transaction;
/**
* @param {TxIn} input
*/
  add_input(input: TxIn): void;
/**
* @param {TxIn} input
*/
  prepend_input(input: TxIn): void;
/**
* @param {number} index
* @param {TxIn} input
*/
  insert_input(index: number, input: TxIn): void;
/**
* @param {TxOut} output
*/
  add_output(output: TxOut): void;
/**
* @param {TxOut} output
*/
  prepend_output(output: TxOut): void;
/**
* @param {number} index
* @param {TxOut} output
*/
  insert_output(index: number, output: TxOut): void;
/**
* @param {number} index
* @param {TxIn} input
*/
  set_input(index: number, input: TxIn): void;
/**
* @param {number} index
* @param {TxOut} output
*/
  set_output(index: number, output: TxOut): void;
/**
* @returns {boolean}
*/
  is_coinbase_impl(): boolean;
/**
*
*     * XT Method:
*     * Returns the combined sum of all input satoshis.
*     * If any of the inputs dont have satoshis defined, this returns None or null
*     
* @returns {BigInt | undefined}
*/
  satoshis_in(): BigInt | undefined;
/**
*
*     * Returns the combined sum of all output satoshis.
*     
* @returns {BigInt}
*/
  satoshis_out(): BigInt;
/**
* @param {string} hex_str
* @returns {Transaction}
*/
  static from_hex(hex_str: string): Transaction;
/**
* @param {Uint8Array} tx_bytes
* @returns {Transaction}
*/
  static from_bytes(tx_bytes: Uint8Array): Transaction;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @param {string} json_string
* @returns {Transaction}
*/
  static from_json_string(json_string: string): Transaction;
/**
* @returns {any}
*/
  to_json(): any;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_hex(): string;
/**
*
*     * Get size of current serialised Transaction object
*     
* @returns {number}
*/
  get_size(): number;
/**
*
*     * Adds an array of TxIn's to the transaction
*     * @param {TxIn[]} tx_ins
*     
* @param {any[]} tx_ins
*/
  add_inputs(tx_ins: any[]): void;
/**
*
*     * Returns all outpoints from this transaction as a 2D array of 36 byte buffers.
*     *
*     * @returns {Uint8Array[]} outpoint_array
*     
* @returns {any}
*/
  get_outpoints(): any;
/**
*
*     * Adds an array of TxOuts to the transaction
*     * @param {TxOut[]} tx_outs
*     
* @param {any[]} tx_outs
*/
  add_outputs(tx_outs: any[]): void;
/**
*
*     * Gets the ID of the current transaction as a hex string.
*     
* @returns {string}
*/
  get_id_hex(): string;
/**
*
*     * Gets the ID of the current transaction as a Uint8Array.
*     
* @returns {Uint8Array}
*/
  get_id_bytes(): Uint8Array;
/**
*
*     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
*     
* @returns {Uint8Array}
*/
  to_compact_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_compact_hex(): string;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {Uint8Array} compact_buffer
* @returns {Transaction}
*/
  static from_compact_bytes(compact_buffer: Uint8Array): Transaction;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {string} compact_hex
* @returns {Transaction}
*/
  static from_compact_hex(compact_hex: string): Transaction;
/**
* @returns {boolean}
*/
  is_coinbase(): boolean;
/**
* @param {PrivateKey} priv_key
* @param {number} sighash
* @param {number} n_tx_in
* @param {Script} unsigned_script
* @param {BigInt} value
* @returns {SighashSignature}
*/
  sign(priv_key: PrivateKey, sighash: number, n_tx_in: number, unsigned_script: Script, value: BigInt): SighashSignature;
/**
* @param {PrivateKey} priv_key
* @param {PrivateKey} ephemeral_key
* @param {number} sighash
* @param {number} n_tx_in
* @param {Script} unsigned_script
* @param {BigInt} value
* @returns {SighashSignature}
*/
  sign_with_k(priv_key: PrivateKey, ephemeral_key: PrivateKey, sighash: number, n_tx_in: number, unsigned_script: Script, value: BigInt): SighashSignature;
/**
* @param {number} sighash
* @param {number} n_tx_in
* @param {Script} unsigned_script
* @param {BigInt} value
* @returns {Uint8Array}
*/
  sighash_preimage(sighash: number, n_tx_in: number, unsigned_script: Script, value: BigInt): Uint8Array;
}
/**
*/
export class TwetchPay {
  free(): void;
/**
* @param {any} value
* @param {Wallet} wallet
* @returns {Promise<TwetchPayAction>}
*/
  static run(value: any, wallet: Wallet): Promise<TwetchPayAction>;
/**
* @param {TwetchPayAction} action
* @param {Wallet} wallet
* @returns {Promise<PublishParams>}
*/
  static submit(action: TwetchPayAction, wallet: Wallet): Promise<PublishParams>;
}
/**
*/
export class TwetchPayAction {
  free(): void;
/**
*/
  readonly built_tx: BuiltTx;
/**
*/
  readonly call: TwetchPayCall;
/**
*/
  readonly is_troll_toll: boolean | undefined;
}
/**
*/
export class TwetchPayCall {
  free(): void;
}
/**
*/
export class TxBuilder {
  free(): void;
/**
*/
  constructor();
/**
* @param {any} value
* @returns {TxBuilder}
*/
  static from_json(value: any): TxBuilder;
/**
* @param {any} value
* @param {Wallet} wallet
* @returns {Promise<BuiltTx>}
*/
  static build(value: any, wallet: Wallet): Promise<BuiltTx>;
}
/**
*/
export class TxIn {
  free(): void;
/**
* @param {Uint8Array} prev_tx_id
* @param {number} vout
* @param {Script} unlocking_script
* @param {number | undefined} sequence
*/
  constructor(prev_tx_id: Uint8Array, vout: number, unlocking_script: Script, sequence?: number);
/**
* @returns {TxIn}
*/
  static default(): TxIn;
/**
* @param {boolean | undefined} little_endian
* @returns {Uint8Array}
*/
  get_prev_tx_id(little_endian?: boolean): Uint8Array;
/**
* @param {boolean | undefined} little_endian
* @returns {string}
*/
  get_prev_tx_id_hex(little_endian?: boolean): string;
/**
* @returns {number}
*/
  get_vout(): number;
/**
* @returns {BigInt}
*/
  get_unlocking_script_size(): BigInt;
/**
* @returns {Script}
*/
  get_unlocking_script(): Script;
/**
* @returns {string}
*/
  get_unlocking_script_hex(): string;
/**
* @returns {number}
*/
  get_sequence(): number;
/**
* @returns {Uint8Array}
*/
  get_sequence_as_bytes(): Uint8Array;
/**
* @param {boolean | undefined} little_endian
* @returns {Uint8Array}
*/
  get_outpoint_bytes(little_endian?: boolean): Uint8Array;
/**
* @param {boolean | undefined} little_endian
* @returns {string}
*/
  get_outpoint_hex(little_endian?: boolean): string;
/**
* @param {Script} script
*/
  set_unlocking_script(script: Script): void;
/**
* @param {Uint8Array} txid
*/
  set_prev_tx_id(txid: Uint8Array): void;
/**
* @param {number} vout
*/
  set_vout(vout: number): void;
/**
* @param {number} sequence
*/
  set_sequence(sequence: number): void;
/**
* @param {BigInt} satoshis
*/
  set_satoshis(satoshis: BigInt): void;
/**
* @returns {BigInt | undefined}
*/
  get_satoshis(): BigInt | undefined;
/**
* @param {Script} locking_script
*/
  set_locking_script(locking_script: Script): void;
/**
* @returns {Script | undefined}
*/
  get_locking_script(): Script | undefined;
/**
* @returns {Uint8Array | undefined}
*/
  get_locking_script_bytes(): Uint8Array | undefined;
/**
* @param {string} hex_str
* @returns {TxIn}
*/
  static from_hex(hex_str: string): TxIn;
/**
* @returns {any}
*/
  to_json(): any;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @param {Uint8Array} outpoint
* @returns {TxIn}
*/
  static from_outpoint_bytes(outpoint: Uint8Array): TxIn;
/**
*
*     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
*     
* @returns {Uint8Array}
*/
  to_compact_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_compact_hex(): string;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {Uint8Array} compact_buffer
* @returns {TxIn}
*/
  static from_compact_bytes(compact_buffer: Uint8Array): TxIn;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {string} compact_hex
* @returns {TxIn}
*/
  static from_compact_hex(compact_hex: string): TxIn;
/**
* Concatenates ScriptSig and UnlockingScript into a single script.
* @returns {Script}
*/
  get_finalised_script(): Script;
/**
* @returns {boolean}
*/
  is_coinbase(): boolean;
}
/**
*/
export class TxOut {
  free(): void;
/**
* @param {BigInt} value
* @param {Script} script_pub_key
*/
  constructor(value: BigInt, script_pub_key: Script);
/**
* @returns {BigInt}
*/
  get_satoshis(): BigInt;
/**
* @returns {Uint8Array}
*/
  get_satoshis_as_bytes(): Uint8Array;
/**
* @returns {number}
*/
  get_script_pub_key_size(): number;
/**
* @returns {Script}
*/
  get_script_pub_key(): Script;
/**
* @returns {string}
*/
  get_script_pub_key_hex(): string;
/**
* @param {string} hex_str
* @returns {TxOut}
*/
  static from_hex(hex_str: string): TxOut;
/**
* @returns {Uint8Array}
*/
  to_bytes(): Uint8Array;
/**
* @returns {string}
*/
  to_hex(): string;
/**
* @returns {any}
*/
  to_json(): any;
/**
* @returns {string}
*/
  to_json_string(): string;
}
/**
*/
export class TypedSignature {
  free(): void;
/**
*/
  readonly sighash: number | undefined;
/**
*/
  readonly signature: Uint8Array | undefined;
/**
*/
  readonly signing_type: number;
}
/**
*/
export class TypedSigning {
  free(): void;
/**
* @param {number} index
* @returns {TypedSignature}
*/
  get_signature(index: number): TypedSignature;
/**
*/
  readonly data: Uint8Array;
/**
*/
  readonly num_signatures: number;
}
/**
*/
export class Wallet {
  free(): void;
/**
* @param {string} seed
*/
  constructor(seed: string);
/**
* @param {string} seed
* @param {string} token
* @returns {Wallet | undefined}
*/
  static from_seed_and_token(seed: string, token: string): Wallet | undefined;
/**
* @returns {ExtendedPublicKey | undefined}
*/
  xpub(): ExtendedPublicKey | undefined;
/**
* @returns {P2PKHAddress | undefined}
*/
  account_address(): P2PKHAddress | undefined;
/**
* @returns {PublicKey | undefined}
*/
  account_public_key(): PublicKey | undefined;
/**
* @returns {ExtendedPublicKey | undefined}
*/
  wallet_xpub(): ExtendedPublicKey | undefined;
/**
* @param {number} network
* @returns {string | undefined}
*/
  display_address(network: number): string | undefined;
/**
* @param {Uint8Array} plain_text
* @returns {EphemeralCipher | undefined}
*/
  ephemeral_encrypt(plain_text: Uint8Array): EphemeralCipher | undefined;
/**
* @param {TypedSigning} typed_signing
* @returns {TypedSigning | undefined}
*/
  sign_typed(typed_signing: TypedSigning): TypedSigning | undefined;
/**
* @param {string} message
* @returns {string | undefined}
*/
  sign_message(message: string): string | undefined;
/**
* @param {Transaction} transaction
* @returns {Transaction | undefined}
*/
  sign_transaction(transaction: Transaction): Transaction | undefined;
/**
* @param {PublicKey} account_public_key
* @param {number} network
* @returns {Promise<any>}
*/
  static utxos(account_public_key: PublicKey, network: number): Promise<any>;
/**
* @param {P2PKHAddress} account_address
* @param {number} network
* @returns {Promise<any>}
*/
  static account_utxos(account_address: P2PKHAddress, network: number): Promise<any>;
/**
* @param {PublicKey} account_public_key
* @param {number} network
* @returns {Promise<any>}
*/
  static wallet_utxos(account_public_key: PublicKey, network: number): Promise<any>;
/**
* @param {P2PKHAddress} account_address
* @param {number} network
* @returns {Promise<any>}
*/
  static account_balance(account_address: P2PKHAddress, network: number): Promise<any>;
}
