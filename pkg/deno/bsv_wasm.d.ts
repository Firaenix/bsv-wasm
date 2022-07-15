/* tslint:disable */
/* eslint-disable */
/**
*/
export function configureStacktrace(): void;
/**
*
* * This entire page is borrowed from rust-sv (https://github.com/brentongunning/rust-sv/blob/master/src/script/op_codes.rs)
* 
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
export enum AESAlgorithms {
  AES128_CBC,
  AES256_CBC,
  AES128_CTR,
  AES256_CTR,
}
/**
*/
export enum SigningHash {
  Sha256,
  Sha256d,
}
/**
*/
export enum DataLengthConstraints {
  Equals,
  GreaterThan,
  LessThan,
  GreaterThanOrEquals,
  LessThanOrEquals,
}
/**
*/
export enum MatchDataTypes {
  Data,
  Signature,
  PublicKey,
  PublicKeyHash,
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
*
* * Bitcoin Signed Message
* 
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
  static isValidMessage(message: Uint8Array, signature: Signature, address: P2PKHAddress): boolean;
/**
* @param {Uint8Array} message
* @param {Signature} signature
* @param {P2PKHAddress} address
* @returns {boolean}
*/
  static verifyMessage(message: Uint8Array, signature: Signature, address: P2PKHAddress): boolean;
/**
* @param {PrivateKey} priv_key
* @param {Uint8Array} message
* @returns {Signature}
*/
  static signMessage(priv_key: PrivateKey, message: Uint8Array): Signature;
/**
* @param {PrivateKey} priv_key
* @param {PrivateKey} ephemeral_key
* @param {Uint8Array} message
* @returns {Signature}
*/
  static signMessageWithK(priv_key: PrivateKey, ephemeral_key: PrivateKey, message: Uint8Array): Signature;
}
/**
*
* * A handy struct to allow calling of various utility methods
* 
*/
export class Bytes {
  free(): void;
/**
* @returns {Uint8Array}
*/
  readReverse(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  read(): Uint8Array;
/**
*/
  reverse(): void;
/**
* @returns {string}
*/
  toHex(): string;
/**
* @param {string} hex_str
* @returns {Bytes}
*/
  static fromHex(hex_str: string): Bytes;
}
/**
*/
export class ChainParams {
  free(): void;
/**
*/
  constructor();
/**
* @param {number} p2pkh
* @param {number} p2sh
* @param {number} privkey
* @param {number} xpub
* @param {number} xpriv
* @param {number} magic
* @returns {ChainParams}
*/
  static new(p2pkh: number, p2sh: number, privkey: number, xpub: number, xpriv: number, magic: number): ChainParams;
/**
* @returns {ChainParams}
*/
  static Mainnet(): ChainParams;
/**
* @returns {ChainParams}
*/
  static Testnet(): ChainParams;
/**
* @returns {ChainParams}
*/
  static Regtest(): ChainParams;
/**
* @returns {ChainParams}
*/
  static STN(): ChainParams;
/**
*/
  magic: number;
/**
*/
  p2pkh: number;
/**
*/
  p2sh: number;
/**
*/
  privkey: number;
/**
*/
  xpriv: number;
/**
*/
  xpub: number;
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
export class ECDH {
  free(): void;
/**
* @param {PrivateKey} priv_key
* @param {PublicKey} pub_key
* @returns {Uint8Array}
*/
  static deriveSharedKey(priv_key: PrivateKey, pub_key: PublicKey): Uint8Array;
}
/**
*
* * Utility struct for low level ECDSA primitives
* 
*/
export class ECDSA {
  free(): void;
/**
* @param {Uint8Array} message
* @param {PublicKey} pub_key
* @param {Signature} signature
* @param {number} hash_algo
* @returns {boolean}
*/
  static verify(message: Uint8Array, pub_key: PublicKey, signature: Signature, hash_algo: number): boolean;
/**
* @param {Signature} signature
* @param {PublicKey} public_key
* @param {PrivateKey} ephemeral_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @returns {PrivateKey}
*/
  static privateKeyFromSignatureK(signature: Signature, public_key: PublicKey, ephemeral_key: PrivateKey, preimage: Uint8Array, hash_algo: number): PrivateKey;
/**
* @param {PrivateKey} private_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @param {boolean} reverse_k
* @returns {Signature}
*/
  static signWithRandomK(private_key: PrivateKey, preimage: Uint8Array, hash_algo: number, reverse_k: boolean): Signature;
/**
* @param {PrivateKey} private_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @param {boolean} reverse_k
* @returns {Signature}
*/
  static sign(private_key: PrivateKey, preimage: Uint8Array, hash_algo: number, reverse_k: boolean): Signature;
/**
* @param {PrivateKey} private_key
* @param {PrivateKey} ephemeral_key
* @param {Uint8Array} preimage
* @param {number} hash_algo
* @returns {Signature}
*/
  static signWithK(private_key: PrivateKey, ephemeral_key: PrivateKey, preimage: Uint8Array, hash_algo: number): Signature;
}
/**
*
* * Electrum compatible ECIES implementation.
* * Comparable to Ecies.electrumEncrypt in BSV.JS
* 
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
  static encryptWithEphemeralKey(message: Uint8Array, recipient_pub_key: PublicKey): ECIESCiphertext;
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
  static deriveCipherKeys(priv_key: PrivateKey, pub_key: PublicKey): CipherKeys;
}
/**
*/
export class ECIESCiphertext {
  free(): void;
/**
* @returns {Uint8Array}
*/
  getCiphertext(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  getHMAC(): Uint8Array;
/**
* @returns {CipherKeys | undefined}
*/
  getCipherKeys(): CipherKeys | undefined;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {PublicKey}
*/
  extractPublicKey(): PublicKey;
/**
* @param {Uint8Array} buffer
* @param {boolean} has_pub_key
* @returns {ECIESCiphertext}
*/
  static fromBytes(buffer: Uint8Array, has_pub_key: boolean): ECIESCiphertext;
}
/**
*/
export class ExtendedPrivateKey {
  free(): void;
/**
* @returns {PrivateKey}
*/
  getPrivateKey(): PrivateKey;
/**
* @returns {PublicKey}
*/
  getPublicKey(): PublicKey;
/**
* @returns {Uint8Array}
*/
  getChainCode(): Uint8Array;
/**
* @returns {number}
*/
  getDepth(): number;
/**
* @returns {Uint8Array}
*/
  getParentFingerprint(): Uint8Array;
/**
* @returns {number}
*/
  getIndex(): number;
/**
* @param {number} index
* @returns {ExtendedPrivateKey}
*/
  deriveChild(index: number): ExtendedPrivateKey;
/**
* @param {string} path
* @returns {ExtendedPrivateKey}
*/
  derive(path: string): ExtendedPrivateKey;
/**
* @param {Uint8Array} seed
* @returns {ExtendedPrivateKey}
*/
  static fromSeed(seed: Uint8Array): ExtendedPrivateKey;
/**
* @returns {ExtendedPrivateKey}
*/
  static fromRandom(): ExtendedPrivateKey;
/**
* @param {string} xprv_string
* @returns {ExtendedPrivateKey}
*/
  static fromString(xprv_string: string): ExtendedPrivateKey;
/**
* @returns {string}
*/
  toString(): string;
/**
* @param {Uint8Array} mnemonic
* @param {Uint8Array | undefined} passphrase
* @returns {ExtendedPrivateKey}
*/
  static fromMnemonic(mnemonic: Uint8Array, passphrase?: Uint8Array): ExtendedPrivateKey;
}
/**
*/
export class ExtendedPublicKey {
  free(): void;
/**
* @returns {PublicKey}
*/
  getPublicKey(): PublicKey;
/**
* @param {ExtendedPrivateKey} xpriv
* @returns {ExtendedPublicKey}
*/
  static fromXPriv(xpriv: ExtendedPrivateKey): ExtendedPublicKey;
/**
* @returns {Uint8Array}
*/
  getChainCode(): Uint8Array;
/**
* @returns {number}
*/
  getDepth(): number;
/**
* @returns {Uint8Array}
*/
  getParentFingerprint(): Uint8Array;
/**
* @returns {number}
*/
  getIndex(): number;
/**
* @param {number} index
* @returns {ExtendedPublicKey}
*/
  deriveChild(index: number): ExtendedPublicKey;
/**
* @param {string} path
* @returns {ExtendedPublicKey}
*/
  derive(path: string): ExtendedPublicKey;
/**
* @param {Uint8Array} seed
* @returns {ExtendedPublicKey}
*/
  static fromSeed(seed: Uint8Array): ExtendedPublicKey;
/**
* @returns {ExtendedPublicKey}
*/
  static fromRandom(): ExtendedPublicKey;
/**
* @param {string} xpub_string
* @returns {ExtendedPublicKey}
*/
  static fromString(xpub_string: string): ExtendedPublicKey;
/**
* @returns {string}
*/
  toString(): string;
}
/**
*/
export class Hash {
  free(): void;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {string}
*/
  toHex(): string;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha256d(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha256(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha1(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static ripemd160(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static hash160(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @returns {Hash}
*/
  static sha512(input: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha512Hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha256Hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha256dHmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static sha1Hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static ripemd160Hmac(input: Uint8Array, key: Uint8Array): Hash;
/**
* @param {Uint8Array} input
* @param {Uint8Array} key
* @returns {Hash}
*/
  static hash160Hmac(input: Uint8Array, key: Uint8Array): Hash;
}
/**
*/
export class KDF {
  free(): void;
/**
* @returns {Hash}
*/
  getHash(): Hash;
/**
* @returns {Uint8Array}
*/
  getSalt(): Uint8Array;
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
export class MatchCriteria {
  free(): void;
/**
*/
  constructor();
/**
* @param {ScriptTemplate} script_template
* @returns {MatchCriteria}
*/
  setScriptTemplate(script_template: ScriptTemplate): MatchCriteria;
/**
* @param {bigint} value
* @returns {MatchCriteria}
*/
  setValue(value: bigint): MatchCriteria;
/**
* @param {bigint} min
* @returns {MatchCriteria}
*/
  setMin(min: bigint): MatchCriteria;
/**
* @param {bigint} max
* @returns {MatchCriteria}
*/
  setMax(max: bigint): MatchCriteria;
}
/**
*/
export class P2PKHAddress {
  free(): void;
/**
* @returns {Uint8Array}
*/
  toPubKeyHashBytes(): Uint8Array;
/**
* @returns {string}
*/
  toPubKeyHashHex(): string;
/**
*
*     * Check if message is signed by this Address.
*     *
*     * Returns a boolean
*     
* @param {Uint8Array} message
* @param {Signature} signature
* @returns {boolean}
*/
  isValidBitcoinMessage(message: Uint8Array, signature: Signature): boolean;
/**
* @param {Uint8Array} hash_bytes
* @returns {P2PKHAddress}
*/
  static fromPubKeyHash(hash_bytes: Uint8Array): P2PKHAddress;
/**
* @param {PublicKey} pub_key
* @returns {P2PKHAddress}
*/
  static fromPubKey(pub_key: PublicKey): P2PKHAddress;
/**
* @param {ChainParams} chain_params
* @returns {P2PKHAddress}
*/
  setChainParams(chain_params: ChainParams): P2PKHAddress;
/**
* @returns {string}
*/
  toString(): string;
/**
* @param {string} address_string
* @returns {P2PKHAddress}
*/
  static fromString(address_string: string): P2PKHAddress;
/**
* @returns {Script}
*/
  toLockingScript(): Script;
/**
* @param {PublicKey} pub_key
* @param {SighashSignature} sig
* @returns {Script}
*/
  toUnlockingScript(pub_key: PublicKey, sig: SighashSignature): Script;
/**
*
*     * Verify if message is signed by this Address.
*     *
*     * Throws an error if invalid.
*     
* @param {Uint8Array} message
* @param {Signature} signature
* @returns {boolean}
*/
  verifyBitcoinMessage(message: Uint8Array, signature: Signature): boolean;
}
/**
*/
export class PrivateKey {
  free(): void;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {string}
*/
  toHex(): string;
/**
* @returns {PrivateKey}
*/
  static fromRandom(): PrivateKey;
/**
*
*     * Finds the Public Key Point.
*     
* @returns {Uint8Array}
*/
  getPoint(): Uint8Array;
/**
* @param {boolean} should_compress
* @returns {PrivateKey}
*/
  compressPublicKey(should_compress: boolean): PrivateKey;
/**
* @param {string} wif_string
* @returns {PrivateKey}
*/
  static fromWIF(wif_string: string): PrivateKey;
/**
* @param {string} hex_str
* @returns {PrivateKey}
*/
  static fromHex(hex_str: string): PrivateKey;
/**
*
*     * Standard ECDSA Message Signing using SHA256 as the digestg
*     
* @param {Uint8Array} msg
* @returns {Signature}
*/
  signMessage(msg: Uint8Array): Signature;
/**
* @returns {string}
*/
  toWIF(): string;
/**
* @param {Uint8Array} bytes
* @returns {PrivateKey}
*/
  static fromBytes(bytes: Uint8Array): PrivateKey;
/**
* @returns {PublicKey}
*/
  toPublicKey(): PublicKey;
/**
*
*     * Encrypt a message to the public key of this private key.
*     
* @param {Uint8Array} message
* @returns {ECIESCiphertext}
*/
  encryptMessage(message: Uint8Array): ECIESCiphertext;
/**
*
*     * Decrypt a message that was sent to the public key corresponding to this private key.
*     
* @param {ECIESCiphertext} ciphertext
* @param {PublicKey} sender_pub_key
* @returns {Uint8Array}
*/
  decryptMessage(ciphertext: ECIESCiphertext, sender_pub_key: PublicKey): Uint8Array;
}
/**
*/
export class PublicKey {
  free(): void;
/**
* @param {Uint8Array} message
* @param {Signature} signature
* @returns {boolean}
*/
  isValidMessage(message: Uint8Array, signature: Signature): boolean;
/**
* @returns {boolean}
*/
  isCompressed(): boolean;
/**
* @param {string} hex_str
* @returns {PublicKey}
*/
  static fromHex(hex_str: string): PublicKey;
/**
* @param {Uint8Array} bytes
* @returns {PublicKey}
*/
  static fromBytes(bytes: Uint8Array): PublicKey;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {string}
*/
  toHex(): string;
/**
* @param {PrivateKey} priv_key
* @returns {PublicKey}
*/
  static fromPrivateKey(priv_key: PrivateKey): PublicKey;
/**
* @param {Uint8Array} message
* @param {Signature} signature
* @returns {boolean}
*/
  verifyMessage(message: Uint8Array, signature: Signature): boolean;
/**
* @returns {P2PKHAddress}
*/
  toAddress(): P2PKHAddress;
/**
* @returns {PublicKey}
*/
  toCompressed(): PublicKey;
/**
* @returns {PublicKey}
*/
  toDecompressed(): PublicKey;
/**
* @param {Uint8Array} message
* @param {PrivateKey} sender_private_key
* @returns {ECIESCiphertext}
*/
  encryptMessage(message: Uint8Array, sender_private_key: PrivateKey): ECIESCiphertext;
}
/**
*/
export class RecoveryInfo {
  free(): void;
/**
* @param {boolean} is_y_odd
* @param {boolean} is_x_reduced
* @param {boolean} is_pubkey_compressed
* @returns {RecoveryInfo}
*/
  static new(is_y_odd: boolean, is_x_reduced: boolean, is_pubkey_compressed: boolean): RecoveryInfo;
/**
* @param {number} recovery_byte
* @param {boolean} is_pubkey_compressed
* @returns {RecoveryInfo}
*/
  static from_byte(recovery_byte: number, is_pubkey_compressed: boolean): RecoveryInfo;
/**
* @returns {RecoveryInfo}
*/
  static default(): RecoveryInfo;
}
/**
*/
export class Script {
  free(): void;
/**
* Matches the Script against the provided ScriptTemplate.
*
* If any data can be gleaned from the Script (ie. OP_DATA, OP_PUBKEY, OP_SIG, etc.), it will return it in a `Vec<Match>`
* @returns {[string, Uint8Array][]}
* @param {ScriptTemplate} script_template
* @returns {any}
*/
  matches(script_template: ScriptTemplate): any;
/**
* Matches the Script against the provided ScriptTemplate.
*
* Returns `true` if the Script matches the ScriptTemplate.
* #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = isMatch))]
* @param {ScriptTemplate} script_template
* @returns {boolean}
*/
  is_match(script_template: ScriptTemplate): boolean;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {number}
*/
  getScriptLength(): number;
/**
* @returns {string}
*/
  toHex(): string;
/**
*/
  removeCodeSeparators(): void;
/**
* @returns {string}
*/
  toASMString(): string;
/**
* @returns {string}
*/
  toExtendedASMString(): string;
/**
* @param {string} hex
* @returns {Script}
*/
  static fromHex(hex: string): Script;
/**
* @param {Uint8Array} bytes
* @returns {Script}
*/
  static fromBytes(bytes: Uint8Array): Script;
/**
* @param {string} asm_string
* @returns {Script}
*/
  static fromASMString(asm_string: string): Script;
/**
* @param {Uint8Array} data_bytes
* @returns {Uint8Array}
*/
  static encodePushData(data_bytes: Uint8Array): Uint8Array;
/**
*
*     * Gets the OP_PUSHDATA prefix varint
*     
* @param {number} length
* @returns {Uint8Array}
*/
  static getPushDataBytes(length: number): Uint8Array;
/**
* @returns {any}
*/
  toScriptBits(): any;
}
/**
*/
export class ScriptTemplate {
  free(): void;
/**
* @param {Script} script
* @returns {ScriptTemplate}
*/
  static from_script(script: Script): ScriptTemplate;
/**
* @param {string} asm
* @returns {ScriptTemplate}
*/
  static from_asm_string(asm: string): ScriptTemplate;
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
  toHex(): string;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @param {Uint8Array} sighash_buffer
* @returns {SighashSignature}
*/
  static fromBytes(bytes: Uint8Array, sighash_buffer: Uint8Array): SighashSignature;
}
/**
*/
export class Signature {
  free(): void;
/**
* DER representation of signature, does not contain any recovery information, so cannot be used for BSM
* @returns {string}
*/
  toHex(): string;
/**
* DER representation of signature, does not contain any recovery information, so cannot be used for BSM
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* NOTE: Provide recovery info if the current signature object doesnt contain it.
* @param {RecoveryInfo | undefined} recovery_info
* @returns {Uint8Array}
*/
  toCompactBytes(recovery_info?: RecoveryInfo): Uint8Array;
/**
* @returns {Uint8Array}
*/
  r(): Uint8Array;
/**
* @returns {string}
*/
  rHex(): string;
/**
* @returns {Uint8Array}
*/
  s(): Uint8Array;
/**
* @returns {string}
*/
  sHex(): string;
/**
* NOTE: Provide recovery info if the current signature object doesnt contain it.
* @param {RecoveryInfo | undefined} recovery_info
* @returns {string}
*/
  toCompactHex(recovery_info?: RecoveryInfo): string;
/**
* @param {Uint8Array} message
* @param {PublicKey} pub_key
* @returns {boolean}
*/
  verifyMessage(message: Uint8Array, pub_key: PublicKey): boolean;
/**
* @param {Uint8Array} bytes
* @returns {Signature}
*/
  static fromDER(bytes: Uint8Array): Signature;
/**
* @param {string} hex
* @returns {Signature}
*/
  static fromHexDER(hex: string): Signature;
/**
* @param {Uint8Array} compact_bytes
* @returns {Signature}
*/
  static fromCompactBytes(compact_bytes: Uint8Array): Signature;
/**
* @param {Uint8Array} message
* @param {number} hash_algo
* @returns {PublicKey}
*/
  recoverPublicKey(message: Uint8Array, hash_algo: number): PublicKey;
}
/**
*/
export class Transaction {
  free(): void;
/**
* @param {PublicKey} pub_key
* @param {SighashSignature} sig
* @returns {boolean}
*/
  verify(pub_key: PublicKey, sig: SighashSignature): boolean;
/**
* @param {PrivateKey} priv_key
* @param {number} sighash
* @param {number} n_tx_in
* @param {Script} unsigned_script
* @param {bigint} value
* @returns {SighashSignature}
*/
  sign(priv_key: PrivateKey, sighash: number, n_tx_in: number, unsigned_script: Script, value: bigint): SighashSignature;
/**
* @param {PrivateKey} priv_key
* @param {PrivateKey} ephemeral_key
* @param {number} sighash
* @param {number} n_tx_in
* @param {Script} unsigned_script
* @param {bigint} value
* @returns {SighashSignature}
*/
  signWithK(priv_key: PrivateKey, ephemeral_key: PrivateKey, sighash: number, n_tx_in: number, unsigned_script: Script, value: bigint): SighashSignature;
/**
* @param {number} sighash
* @param {number} n_tx_in
* @param {Script} unsigned_script
* @param {bigint} value
* @returns {Uint8Array}
*/
  sighashPreimage(sighash: number, n_tx_in: number, unsigned_script: Script, value: bigint): Uint8Array;
/**
* @returns {number}
*/
  getVersion(): number;
/**
* @returns {number}
*/
  getInputsCount(): number;
/**
* @returns {number}
*/
  getOutputsCount(): number;
/**
* @param {number} index
* @returns {TxIn | undefined}
*/
  getInput(index: number): TxIn | undefined;
/**
* @param {number} index
* @returns {TxOut | undefined}
*/
  getOutput(index: number): TxOut | undefined;
/**
* @returns {number}
*/
  getNLocktime(): number;
/**
* @returns {Uint8Array}
*/
  getNLocktimeAsBytes(): Uint8Array;
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
  setVersion(version: number): Transaction;
/**
* @param {number} n_locktime
* @returns {Transaction}
*/
  setNLocktime(n_locktime: number): Transaction;
/**
* @param {TxIn} input
*/
  addInput(input: TxIn): void;
/**
* @param {TxIn} input
*/
  prependInput(input: TxIn): void;
/**
* @param {number} index
* @param {TxIn} input
*/
  insertInput(index: number, input: TxIn): void;
/**
* @param {TxOut} output
*/
  addOutput(output: TxOut): void;
/**
* @param {TxOut} output
*/
  prependOutput(output: TxOut): void;
/**
* @param {number} index
* @param {TxOut} output
*/
  insertOutput(index: number, output: TxOut): void;
/**
* @param {number} index
* @param {TxIn} input
*/
  setInput(index: number, input: TxIn): void;
/**
* @param {number} index
* @param {TxOut} output
*/
  setOutput(index: number, output: TxOut): void;
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
* @returns {bigint | undefined}
*/
  satoshisIn(): bigint | undefined;
/**
*
*     * Returns the combined sum of all output satoshis.
*     
* @returns {bigint}
*/
  satoshisOut(): bigint;
/**
* @param {string} hex_str
* @returns {Transaction}
*/
  static fromHex(hex_str: string): Transaction;
/**
* @param {Uint8Array} tx_bytes
* @returns {Transaction}
*/
  static fromBytes(tx_bytes: Uint8Array): Transaction;
/**
* @returns {string}
*/
  toString(): string;
/**
* @param {string} json_string
* @returns {Transaction}
*/
  static fromJsonString(json_string: string): Transaction;
/**
* @returns {any}
*/
  toJSON(): any;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {string}
*/
  toHex(): string;
/**
*
*     * Get size of current serialised Transaction object
*     
* @returns {number}
*/
  getSize(): number;
/**
*
*     * Adds an array of TxIn's to the transaction
*     * @param {TxIn[]} tx_ins
*     
* @param {any[]} tx_ins
*/
  addInputs(tx_ins: any[]): void;
/**
*
*     * Returns all outpoints from this transaction as a 2D array of 36 byte buffers.
*     *
*     * @returns {Uint8Array[]} outpoint_array
*     
* @returns {any}
*/
  getOutpoints(): any;
/**
*
*     * Adds an array of TxOuts to the transaction
*     * @param {TxOut[]} tx_outs
*     
* @param {any[]} tx_outs
*/
  addOutputs(tx_outs: any[]): void;
/**
*
*     * Gets the ID of the current transaction as a hex string.
*     
* @returns {string}
*/
  getIdHex(): string;
/**
*
*     * Gets the ID of the current transaction as a Uint8Array.
*     
* @returns {Uint8Array}
*/
  getIdBytes(): Uint8Array;
/**
*
*     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
*     
* @returns {Uint8Array}
*/
  toCompactBytes(): Uint8Array;
/**
* @returns {string}
*/
  toCompactHex(): string;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {Uint8Array} compact_buffer
* @returns {Transaction}
*/
  static fromCompactBytes(compact_buffer: Uint8Array): Transaction;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {string} compact_hex
* @returns {Transaction}
*/
  static fromCompactHex(compact_hex: string): Transaction;
/**
* @returns {boolean}
*/
  isCoinbase(): boolean;
/**
*
*     * Returns the first output index that matches the given parameters, returns None or null if not found.
*     
* @param {MatchCriteria} criteria
* @returns {number | undefined}
*/
  matchOutput(criteria: MatchCriteria): number | undefined;
/**
*
*     * Returns a list of outputs indexes that match the given parameters
*     
* @param {MatchCriteria} criteria
* @returns {Uint32Array}
*/
  matchOutputs(criteria: MatchCriteria): Uint32Array;
/**
*
*     * Returns the first input index that matches the given parameters, returns None or null if not found.
*     
* @param {MatchCriteria} criteria
* @returns {number | undefined}
*/
  matchInput(criteria: MatchCriteria): number | undefined;
/**
*
*     * Returns a list of input indexes that match the given parameters
*     
* @param {MatchCriteria} criteria
* @returns {Uint32Array}
*/
  matchInputs(criteria: MatchCriteria): Uint32Array;
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
  getPrevTxId(little_endian?: boolean): Uint8Array;
/**
* @param {boolean | undefined} little_endian
* @returns {string}
*/
  getPrevTxIdHex(little_endian?: boolean): string;
/**
* @returns {number}
*/
  getVOut(): number;
/**
* @returns {bigint}
*/
  getScriptSigSize(): bigint;
/**
* @returns {Script}
*/
  getScriptSig(): Script;
/**
* @returns {string}
*/
  getScriptSigHex(): string;
/**
* @returns {number}
*/
  getSequence(): number;
/**
* @returns {Uint8Array}
*/
  getSequenceAsBytes(): Uint8Array;
/**
* @param {boolean | undefined} little_endian
* @returns {Uint8Array}
*/
  getOutpointBytes(little_endian?: boolean): Uint8Array;
/**
* @param {boolean | undefined} little_endian
* @returns {string}
*/
  getOutpointHex(little_endian?: boolean): string;
/**
* @param {Script} script
*/
  setUnlockingScript(script: Script): void;
/**
* @param {Uint8Array} txid
*/
  setPrevTxId(txid: Uint8Array): void;
/**
* @param {number} vout
*/
  setVOut(vout: number): void;
/**
* @param {number} sequence
*/
  setSequence(sequence: number): void;
/**
* @param {bigint} satoshis
*/
  setSatoshis(satoshis: bigint): void;
/**
* @returns {bigint | undefined}
*/
  getSatoshis(): bigint | undefined;
/**
* @param {Script} locking_script
*/
  setLockingScript(locking_script: Script): void;
/**
* @returns {Script | undefined}
*/
  getUnlockingScript(): Script | undefined;
/**
* @returns {Uint8Array | undefined}
*/
  getUnlockingScriptBytes(): Uint8Array | undefined;
/**
* @param {string} hex_str
* @returns {TxIn}
*/
  static fromHex(hex_str: string): TxIn;
/**
* @returns {any}
*/
  toJSON(): any;
/**
* @returns {string}
*/
  toString(): string;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {string}
*/
  toHex(): string;
/**
* @param {Uint8Array} outpoint
* @returns {TxIn}
*/
  static fromOutpointBytes(outpoint: Uint8Array): TxIn;
/**
*
*     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
*     
* @returns {Uint8Array}
*/
  toCompactBytes(): Uint8Array;
/**
* @returns {string}
*/
  toCompactHex(): string;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {Uint8Array} compact_buffer
* @returns {TxIn}
*/
  static fromCompactBytes(compact_buffer: Uint8Array): TxIn;
/**
*
*     * Deserialises the provided CBOR buffer to the TX+ format
*     
* @param {string} compact_hex
* @returns {TxIn}
*/
  static fromCompactHex(compact_hex: string): TxIn;
/**
* Concatenates ScriptSig and UnlockingScript into a single script.
* @returns {Script}
*/
  getFinalisedScript(): Script;
/**
* @returns {boolean}
*/
  isCoinbase(): boolean;
}
/**
*/
export class TxOut {
  free(): void;
/**
* @param {bigint} value
* @param {Script} script_pub_key
*/
  constructor(value: bigint, script_pub_key: Script);
/**
* @returns {bigint}
*/
  getSatoshis(): bigint;
/**
* @returns {Uint8Array}
*/
  getSatoshisAsBytes(): Uint8Array;
/**
* @returns {number}
*/
  getScriptPubKeySize(): number;
/**
* @returns {Script}
*/
  getScriptPubKey(): Script;
/**
* @returns {string}
*/
  getScriptPubKeyHex(): string;
/**
* @param {string} hex_str
* @returns {TxOut}
*/
  static fromHex(hex_str: string): TxOut;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @returns {string}
*/
  toHex(): string;
/**
* @returns {any}
*/
  toJSON(): any;
/**
* @returns {string}
*/
  toString(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_hash_free: (a: number) => void;
  readonly hash_toBytes: (a: number, b: number) => void;
  readonly hash_toHex: (a: number, b: number) => void;
  readonly hash_sha256d: (a: number, b: number) => number;
  readonly hash_sha256: (a: number, b: number) => number;
  readonly hash_sha1: (a: number, b: number) => number;
  readonly hash_ripemd160: (a: number, b: number) => number;
  readonly hash_hash160: (a: number, b: number) => number;
  readonly hash_sha512: (a: number, b: number) => number;
  readonly hash_sha512Hmac: (a: number, b: number, c: number, d: number) => number;
  readonly hash_sha256Hmac: (a: number, b: number, c: number, d: number) => number;
  readonly hash_sha256dHmac: (a: number, b: number, c: number, d: number) => number;
  readonly hash_sha1Hmac: (a: number, b: number, c: number, d: number) => number;
  readonly hash_ripemd160Hmac: (a: number, b: number, c: number, d: number) => number;
  readonly hash_hash160Hmac: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_kdf_free: (a: number) => void;
  readonly kdf_getHash: (a: number) => number;
  readonly __wbg_cipherkeys_free: (a: number) => void;
  readonly cipherkeys_get_iv: (a: number, b: number) => void;
  readonly cipherkeys_get_ke: (a: number, b: number) => void;
  readonly cipherkeys_get_km: (a: number, b: number) => void;
  readonly ecies_encrypt: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly ecies_encryptWithEphemeralKey: (a: number, b: number, c: number, d: number) => void;
  readonly ecies_decrypt: (a: number, b: number, c: number, d: number) => void;
  readonly ecies_deriveCipherKeys: (a: number, b: number, c: number) => void;
  readonly kdf_getSalt: (a: number, b: number) => void;
  readonly __wbg_ecies_free: (a: number) => void;
  readonly __wbg_publickey_free: (a: number) => void;
  readonly publickey_isValidMessage: (a: number, b: number, c: number, d: number) => number;
  readonly publickey_isCompressed: (a: number) => number;
  readonly publickey_fromHex: (a: number, b: number, c: number) => void;
  readonly publickey_fromBytes: (a: number, b: number, c: number) => void;
  readonly publickey_toBytes: (a: number, b: number) => void;
  readonly publickey_toHex: (a: number, b: number) => void;
  readonly publickey_fromPrivateKey: (a: number) => number;
  readonly publickey_verifyMessage: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly publickey_toAddress: (a: number, b: number) => void;
  readonly publickey_toCompressed: (a: number, b: number) => void;
  readonly publickey_toDecompressed: (a: number, b: number) => void;
  readonly publickey_encryptMessage: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly transaction_verify: (a: number, b: number, c: number) => number;
  readonly transaction_sign: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly transaction_signWithK: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly transaction_sighashPreimage: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly __wbg_sighashsignature_free: (a: number) => void;
  readonly sighashsignature_new: (a: number, b: number, c: number, d: number) => number;
  readonly sighashsignature_toHex: (a: number, b: number) => void;
  readonly sighashsignature_toBytes: (a: number, b: number) => void;
  readonly sighashsignature_fromBytes: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly aes_encrypt: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly aes_decrypt: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly ecdsa_verify: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_eciesciphertext_free: (a: number) => void;
  readonly eciesciphertext_getCiphertext: (a: number, b: number) => void;
  readonly eciesciphertext_getHMAC: (a: number, b: number) => void;
  readonly eciesciphertext_getCipherKeys: (a: number) => number;
  readonly eciesciphertext_toBytes: (a: number, b: number) => void;
  readonly eciesciphertext_extractPublicKey: (a: number, b: number) => void;
  readonly eciesciphertext_fromBytes: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_aes_free: (a: number) => void;
  readonly __wbg_ecdsa_free: (a: number) => void;
  readonly __wbg_extendedprivatekey_free: (a: number) => void;
  readonly extendedprivatekey_getPrivateKey: (a: number) => number;
  readonly extendedprivatekey_getPublicKey: (a: number) => number;
  readonly extendedprivatekey_getChainCode: (a: number, b: number) => void;
  readonly extendedprivatekey_getDepth: (a: number) => number;
  readonly extendedprivatekey_getParentFingerprint: (a: number, b: number) => void;
  readonly extendedprivatekey_getIndex: (a: number) => number;
  readonly extendedprivatekey_deriveChild: (a: number, b: number, c: number) => void;
  readonly extendedprivatekey_derive: (a: number, b: number, c: number, d: number) => void;
  readonly extendedprivatekey_fromSeed: (a: number, b: number, c: number) => void;
  readonly extendedprivatekey_fromRandom: (a: number) => void;
  readonly extendedprivatekey_fromString: (a: number, b: number, c: number) => void;
  readonly extendedprivatekey_toString: (a: number, b: number) => void;
  readonly extendedprivatekey_fromMnemonic: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_extendedpublickey_free: (a: number) => void;
  readonly extendedpublickey_getPublicKey: (a: number) => number;
  readonly extendedpublickey_fromXPriv: (a: number) => number;
  readonly extendedpublickey_getChainCode: (a: number, b: number) => void;
  readonly extendedpublickey_getDepth: (a: number) => number;
  readonly extendedpublickey_getParentFingerprint: (a: number, b: number) => void;
  readonly extendedpublickey_getIndex: (a: number) => number;
  readonly extendedpublickey_deriveChild: (a: number, b: number, c: number) => void;
  readonly extendedpublickey_derive: (a: number, b: number, c: number, d: number) => void;
  readonly extendedpublickey_fromSeed: (a: number, b: number, c: number) => void;
  readonly extendedpublickey_fromRandom: (a: number) => void;
  readonly extendedpublickey_fromString: (a: number, b: number, c: number) => void;
  readonly extendedpublickey_toString: (a: number, b: number) => void;
  readonly __wbg_txin_free: (a: number) => void;
  readonly txin_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly txin_default: () => number;
  readonly txin_getPrevTxId: (a: number, b: number, c: number) => void;
  readonly txin_getPrevTxIdHex: (a: number, b: number, c: number) => void;
  readonly txin_getVOut: (a: number) => number;
  readonly txin_getScriptSigSize: (a: number, b: number) => void;
  readonly txin_getScriptSig: (a: number) => number;
  readonly txin_getScriptSigHex: (a: number, b: number) => void;
  readonly txin_getSequence: (a: number) => number;
  readonly txin_getSequenceAsBytes: (a: number, b: number) => void;
  readonly txin_getOutpointBytes: (a: number, b: number, c: number) => void;
  readonly txin_getOutpointHex: (a: number, b: number, c: number) => void;
  readonly txin_setUnlockingScript: (a: number, b: number) => void;
  readonly txin_setPrevTxId: (a: number, b: number, c: number) => void;
  readonly txin_setVOut: (a: number, b: number) => void;
  readonly txin_setSequence: (a: number, b: number) => void;
  readonly txin_setSatoshis: (a: number, b: number, c: number) => void;
  readonly txin_getSatoshis: (a: number, b: number) => void;
  readonly txin_setLockingScript: (a: number, b: number) => void;
  readonly txin_getUnlockingScript: (a: number) => number;
  readonly txin_getUnlockingScriptBytes: (a: number, b: number) => void;
  readonly txin_fromHex: (a: number, b: number, c: number) => void;
  readonly txin_toJSON: (a: number, b: number) => void;
  readonly txin_toString: (a: number, b: number) => void;
  readonly txin_toBytes: (a: number, b: number) => void;
  readonly txin_toHex: (a: number, b: number) => void;
  readonly txin_fromOutpointBytes: (a: number, b: number, c: number) => void;
  readonly txin_toCompactBytes: (a: number, b: number) => void;
  readonly txin_toCompactHex: (a: number, b: number) => void;
  readonly txin_fromCompactBytes: (a: number, b: number, c: number) => void;
  readonly txin_fromCompactHex: (a: number, b: number, c: number) => void;
  readonly txin_getFinalisedScript: (a: number, b: number) => void;
  readonly txin_isCoinbase: (a: number) => number;
  readonly ecdh_deriveSharedKey: (a: number, b: number, c: number) => void;
  readonly __wbg_ecdh_free: (a: number) => void;
  readonly __wbg_chainparams_free: (a: number) => void;
  readonly __wbg_get_chainparams_p2pkh: (a: number) => number;
  readonly __wbg_set_chainparams_p2pkh: (a: number, b: number) => void;
  readonly __wbg_get_chainparams_p2sh: (a: number) => number;
  readonly __wbg_set_chainparams_p2sh: (a: number, b: number) => void;
  readonly __wbg_get_chainparams_privkey: (a: number) => number;
  readonly __wbg_set_chainparams_privkey: (a: number, b: number) => void;
  readonly __wbg_get_chainparams_xpub: (a: number) => number;
  readonly __wbg_set_chainparams_xpub: (a: number, b: number) => void;
  readonly __wbg_get_chainparams_xpriv: (a: number) => number;
  readonly __wbg_set_chainparams_xpriv: (a: number, b: number) => void;
  readonly __wbg_get_chainparams_magic: (a: number) => number;
  readonly __wbg_set_chainparams_magic: (a: number, b: number) => void;
  readonly chainparams_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly chainparams_Mainnet: () => number;
  readonly chainparams_Testnet: () => number;
  readonly chainparams_Regtest: () => number;
  readonly chainparams_STN: () => number;
  readonly __wbg_recoveryinfo_free: (a: number) => void;
  readonly recoveryinfo_new: (a: number, b: number, c: number) => number;
  readonly recoveryinfo_from_byte: (a: number, b: number) => number;
  readonly recoveryinfo_default: () => number;
  readonly __wbg_signature_free: (a: number) => void;
  readonly signature_toHex: (a: number, b: number) => void;
  readonly signature_toBytes: (a: number, b: number) => void;
  readonly signature_toCompactBytes: (a: number, b: number, c: number) => void;
  readonly signature_r: (a: number, b: number) => void;
  readonly signature_rHex: (a: number, b: number) => void;
  readonly signature_s: (a: number, b: number) => void;
  readonly signature_sHex: (a: number, b: number) => void;
  readonly signature_toCompactHex: (a: number, b: number, c: number) => void;
  readonly signature_verifyMessage: (a: number, b: number, c: number, d: number) => number;
  readonly signature_fromDER: (a: number, b: number, c: number) => void;
  readonly signature_fromHexDER: (a: number, b: number, c: number) => void;
  readonly signature_fromCompactBytes: (a: number, b: number, c: number) => void;
  readonly signature_recoverPublicKey: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_p2pkhaddress_free: (a: number) => void;
  readonly p2pkhaddress_toPubKeyHashBytes: (a: number, b: number) => void;
  readonly p2pkhaddress_toPubKeyHashHex: (a: number, b: number) => void;
  readonly p2pkhaddress_isValidBitcoinMessage: (a: number, b: number, c: number, d: number) => number;
  readonly p2pkhaddress_fromPubKeyHash: (a: number, b: number, c: number) => void;
  readonly p2pkhaddress_fromPubKey: (a: number, b: number) => void;
  readonly p2pkhaddress_setChainParams: (a: number, b: number, c: number) => void;
  readonly p2pkhaddress_toString: (a: number, b: number) => void;
  readonly p2pkhaddress_fromString: (a: number, b: number, c: number) => void;
  readonly p2pkhaddress_toLockingScript: (a: number, b: number) => void;
  readonly p2pkhaddress_toUnlockingScript: (a: number, b: number, c: number, d: number) => void;
  readonly p2pkhaddress_verifyBitcoinMessage: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly chainparams_default: () => number;
  readonly __wbg_scripttemplate_free: (a: number) => void;
  readonly scripttemplate_from_script: (a: number, b: number) => void;
  readonly scripttemplate_from_asm_string: (a: number, b: number, c: number) => void;
  readonly script_matches: (a: number, b: number, c: number) => void;
  readonly script_is_match: (a: number, b: number) => number;
  readonly kdf_pbkdf2: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly bsm_isValidMessage: (a: number, b: number, c: number, d: number) => number;
  readonly bsm_verifyMessage: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly bsm_signMessage: (a: number, b: number, c: number, d: number) => void;
  readonly bsm_signMessageWithK: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_bsm_free: (a: number) => void;
  readonly __wbg_privatekey_free: (a: number) => void;
  readonly privatekey_toBytes: (a: number, b: number) => void;
  readonly privatekey_toHex: (a: number, b: number) => void;
  readonly privatekey_fromRandom: () => number;
  readonly privatekey_getPoint: (a: number, b: number) => void;
  readonly privatekey_compressPublicKey: (a: number, b: number) => number;
  readonly privatekey_fromWIF: (a: number, b: number, c: number) => void;
  readonly privatekey_fromHex: (a: number, b: number, c: number) => void;
  readonly privatekey_signMessage: (a: number, b: number, c: number, d: number) => void;
  readonly privatekey_toWIF: (a: number, b: number) => void;
  readonly privatekey_fromBytes: (a: number, b: number, c: number) => void;
  readonly privatekey_toPublicKey: (a: number, b: number) => void;
  readonly privatekey_encryptMessage: (a: number, b: number, c: number, d: number) => void;
  readonly privatekey_decryptMessage: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_bytes_free: (a: number) => void;
  readonly bytes_readReverse: (a: number, b: number) => void;
  readonly bytes_read: (a: number, b: number) => void;
  readonly bytes_reverse: (a: number) => void;
  readonly bytes_toHex: (a: number, b: number) => void;
  readonly bytes_fromHex: (a: number, b: number, c: number) => void;
  readonly ecdsa_privateKeyFromSignatureK: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly ecdsa_signWithRandomK: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly ecdsa_sign: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly ecdsa_signWithK: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly configureStacktrace: () => void;
  readonly __wbg_transaction_free: (a: number) => void;
  readonly transaction_getVersion: (a: number) => number;
  readonly transaction_getInputsCount: (a: number) => number;
  readonly transaction_getOutputsCount: (a: number) => number;
  readonly transaction_getInput: (a: number, b: number) => number;
  readonly transaction_getOutput: (a: number, b: number) => number;
  readonly transaction_getNLocktime: (a: number) => number;
  readonly transaction_getNLocktimeAsBytes: (a: number, b: number) => void;
  readonly transaction_new: (a: number, b: number) => number;
  readonly transaction_default: () => number;
  readonly transaction_setVersion: (a: number, b: number) => number;
  readonly transaction_setNLocktime: (a: number, b: number) => number;
  readonly transaction_addInput: (a: number, b: number) => void;
  readonly transaction_prependInput: (a: number, b: number) => void;
  readonly transaction_insertInput: (a: number, b: number, c: number) => void;
  readonly transaction_addOutput: (a: number, b: number) => void;
  readonly transaction_prependOutput: (a: number, b: number) => void;
  readonly transaction_insertOutput: (a: number, b: number, c: number) => void;
  readonly transaction_setInput: (a: number, b: number, c: number) => void;
  readonly transaction_setOutput: (a: number, b: number, c: number) => void;
  readonly transaction_satoshisIn: (a: number, b: number) => void;
  readonly transaction_satoshisOut: (a: number, b: number) => void;
  readonly transaction_fromHex: (a: number, b: number, c: number) => void;
  readonly transaction_fromBytes: (a: number, b: number, c: number) => void;
  readonly transaction_toString: (a: number, b: number) => void;
  readonly transaction_fromJsonString: (a: number, b: number, c: number) => void;
  readonly transaction_toJSON: (a: number, b: number) => void;
  readonly transaction_toBytes: (a: number, b: number) => void;
  readonly transaction_toHex: (a: number, b: number) => void;
  readonly transaction_getSize: (a: number, b: number) => void;
  readonly transaction_addInputs: (a: number, b: number, c: number) => void;
  readonly transaction_getOutpoints: (a: number, b: number) => void;
  readonly transaction_addOutputs: (a: number, b: number, c: number) => void;
  readonly transaction_getIdHex: (a: number, b: number) => void;
  readonly transaction_getIdBytes: (a: number, b: number) => void;
  readonly transaction_toCompactBytes: (a: number, b: number) => void;
  readonly transaction_toCompactHex: (a: number, b: number) => void;
  readonly transaction_fromCompactBytes: (a: number, b: number, c: number) => void;
  readonly transaction_fromCompactHex: (a: number, b: number, c: number) => void;
  readonly transaction_isCoinbase: (a: number) => number;
  readonly transaction_is_coinbase_impl: (a: number) => number;
  readonly __wbg_txout_free: (a: number) => void;
  readonly txout_new: (a: number, b: number, c: number) => number;
  readonly txout_getSatoshis: (a: number, b: number) => void;
  readonly txout_getSatoshisAsBytes: (a: number, b: number) => void;
  readonly txout_getScriptPubKeySize: (a: number) => number;
  readonly txout_getScriptPubKey: (a: number) => number;
  readonly txout_getScriptPubKeyHex: (a: number, b: number) => void;
  readonly txout_fromHex: (a: number, b: number, c: number) => void;
  readonly txout_toBytes: (a: number, b: number) => void;
  readonly txout_toHex: (a: number, b: number) => void;
  readonly txout_toJSON: (a: number, b: number) => void;
  readonly txout_toString: (a: number, b: number) => void;
  readonly __wbg_matchcriteria_free: (a: number) => void;
  readonly matchcriteria_new: () => number;
  readonly matchcriteria_setScriptTemplate: (a: number, b: number) => number;
  readonly matchcriteria_setValue: (a: number, b: number, c: number) => number;
  readonly matchcriteria_setMin: (a: number, b: number, c: number) => number;
  readonly matchcriteria_setMax: (a: number, b: number, c: number) => number;
  readonly transaction_matchOutput: (a: number, b: number, c: number) => void;
  readonly transaction_matchOutputs: (a: number, b: number, c: number) => void;
  readonly transaction_matchInput: (a: number, b: number, c: number) => void;
  readonly transaction_matchInputs: (a: number, b: number, c: number) => void;
  readonly __wbg_script_free: (a: number) => void;
  readonly script_toBytes: (a: number, b: number) => void;
  readonly script_getScriptLength: (a: number) => number;
  readonly script_toHex: (a: number, b: number) => void;
  readonly script_removeCodeSeparators: (a: number) => void;
  readonly script_toASMString: (a: number, b: number) => void;
  readonly script_toExtendedASMString: (a: number, b: number) => void;
  readonly script_fromHex: (a: number, b: number, c: number) => void;
  readonly script_fromBytes: (a: number, b: number, c: number) => void;
  readonly script_fromASMString: (a: number, b: number, c: number) => void;
  readonly script_encodePushData: (a: number, b: number, c: number) => void;
  readonly script_getPushDataBytes: (a: number, b: number) => void;
  readonly script_toScriptBits: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
