use wasm_bindgen::prelude::*;

/**
 * This entire page is borrowed from rust-sv (https://github.com/brentongunning/rust-sv/blob/master/src/script/op_codes.rs)
 */

// --------------------------------------------------------------------------------------------
// Constants
// --------------------------------------------------------------------------------------------

pub struct OpCodes {
}

// Cant expose consts with wasm-bindgen atm. need to find a work around.
impl OpCodes {

/// Pushes 0 onto the stack
pub const OP_0: u8 = 0;
/// Pushes 0 onto the stack
pub const OP_FALSE: u8 = 0;

/// Offset by n to push n bytes onto the stack, where n: [1-75]
pub const OP_PUSH: u8 = 0;

/// The next byte sets the number of bytes to push onto the stack
pub const OP_PUSHDATA1: u8 = 76;
/// The next two bytes sets the number of bytes to push onto the stack
pub const OP_PUSHDATA2: u8 = 77;
/// The next four bytes sets the number of bytes to push onto the stack
pub const OP_PUSHDATA4: u8 = 78;

/// Pushes -1 onto the stack
pub const OP_1NEGATE: u8 = 79;
/// Pushes 1 onto the stack
pub const OP_1: u8 = 81;
/// Pushes 1 onto the stack
pub const OP_TRUE: u8 = 81;

/// Pushes 2 onto the stack
pub const OP_2: u8 = 82;
/// Pushes 3 onto the stack
pub const OP_3: u8 = 83;
/// Pushes 4 onto the stack
pub const OP_4: u8 = 84;
/// Pushes 5 onto the stack
pub const OP_5: u8 = 85;
/// Pushes 6 onto the stack
pub const OP_6: u8 = 86;
/// Pushes 7 onto the stack
pub const OP_7: u8 = 87;
/// Pushes 8 onto the stack
pub const OP_8: u8 = 88;
/// Pushes 9 onto the stack
pub const OP_9: u8 = 89;
/// Pushes 10 onto the stack
pub const OP_10: u8 = 90;
/// Pushes 11 onto the stack
pub const OP_11: u8 = 91;
/// Pushes 12 onto the stack
pub const OP_12: u8 = 92;
/// Pushes 13 onto the stack
pub const OP_13: u8 = 93;
/// Pushes 14 onto the stack
pub const OP_14: u8 = 94;
/// Pushes 15 onto the stack
pub const OP_15: u8 = 95;
/// Pushes 16 onto the stack
pub const OP_16: u8 = 96;

// --------------------------------------------------------------------------------------------
// Flow Control
// --------------------------------------------------------------------------------------------

/// Does nothing
pub const OP_NOP: u8 = 97;
/// If the top stack is true, statements are executed. Top stack value is removed.
pub const OP_IF: u8 = 99;
/// If the top stack is false, statements are executed. Top stack value is removed.
pub const OP_NOTIF: u8 = 100;
/// If the preceding OP_IF or OP_NOTIF statemetns were not executed, then statements are executed.
pub const OP_ELSE: u8 = 103;
/// Ends an if-else block
pub const OP_ENDIF: u8 = 104;
/// Marks a statement as invalid if the top stack value is false. Top stack value is removed.
pub const OP_VERIFY: u8 = 105;
/// Marks a statements as invalid
pub const OP_RETURN: u8 = 106;

// --------------------------------------------------------------------------------------------
// Stack
// --------------------------------------------------------------------------------------------

/// Moves the top item on the main stack to the alt stack
pub const OP_TOALTSTACK: u8 = 107;
/// Moves the top item on the alt stack to the main stack
pub const OP_FROMALTSTACK: u8 = 108;
/// Duplicates the top stack value if it is not zero
pub const OP_IFDUP: u8 = 115;
/// Puts the number of stack items onto the stack
pub const OP_DEPTH: u8 = 116;
/// Drops the top stack value
pub const OP_DROP: u8 = 117;
/// Duplicates the top stack item
pub const OP_DUP: u8 = 118;
/// Removes the second-to-top stack item
pub const OP_NIP: u8 = 119;
/// Copies the second-to-top stack item to the top
pub const OP_OVER: u8 = 120;
/// The item n back in the stack is copied to the top
pub const OP_PICK: u8 = 121;
/// The item n back in the stack is moved to the top
pub const OP_ROLL: u8 = 122;
/// The top three items on the stack are rotated to the left
pub const OP_ROT: u8 = 123;
/// The top two items on the stack are swapped
pub const OP_SWAP: u8 = 124;
/// The item at the top of the stack is copied and inserted before the second-to-top item
pub const OP_TUCK: u8 = 125;
/// Removes the top two items from the stack
pub const OP_2DROP: u8 = 109;
/// Duplicates the top two stack items
pub const OP_2DUP: u8 = 110;
/// Duplicates the top three stack items
pub const OP_3DUP: u8 = 111;
/// Copies the pair of items two spaces back to the front
pub const OP_2OVER: u8 = 112;
/// The fifth and sixth items back are moved to the top of the stack
pub const OP_2ROT: u8 = 113;
/// Swaps the top two pairs of items
pub const OP_2SWAP: u8 = 114;

// --------------------------------------------------------------------------------------------
// Splice
// --------------------------------------------------------------------------------------------

/// Concatenates two byte sequences
pub const OP_CAT: u8 = 126;
/// Splits the byte sequence at position n
pub const OP_SPLIT: u8 = 127;
/// Pushes the byte sequence length of the top stack item without popping it
pub const OP_SIZE: u8 = 130;

// --------------------------------------------------------------------------------------------
// Bitwise Logic
// --------------------------------------------------------------------------------------------

/// Flips all of the bits in the input
pub const OP_INVERT: u8 = 131;
/// Boolean and between each bit in the inputs
pub const OP_AND: u8 = 132;
/// Boolean or between each bit in the inputs
pub const OP_OR: u8 = 133;
/// Boolean exclusive or between each bit in the inputs
pub const OP_XOR: u8 = 134;
/// Returns 1 if the inputs are exactly equal, 0 otherwise
pub const OP_EQUAL: u8 = 135;
/// Same as OP_EQUAL, but runs OP_VERIFY afterward
pub const OP_EQUALVERIFY: u8 = 136;

// --------------------------------------------------------------------------------------------
// Arithmetic
// --------------------------------------------------------------------------------------------

/// Adds 1 to the input
pub const OP_1ADD: u8 = 139;
/// Subtracts 1 from the input
pub const OP_1SUB: u8 = 140;
/// The sign of the input is flipped
pub const OP_NEGATE: u8 = 143;
/// The input is made positive
pub const OP_ABS: u8 = 144;
/// If the input is 0 or 1, it is flipped. Otherwise, the output will be 0.
pub const OP_NOT: u8 = 145;
/// Returns 0 if the input is 0. 1 otherwise.
pub const OP_0NOTEQUAL: u8 = 146;
/// Adds a to b
pub const OP_ADD: u8 = 147;
/// Subtracts b from a
pub const OP_SUB: u8 = 148;
/// Multiplies a by b
pub const OP_MUL: u8 = 149;
/// Divides a by b
pub const OP_DIV: u8 = 150;
/// Returns the remainder after dividing a by b
pub const OP_MOD: u8 = 151;
/// Shifts a left b bits, preserving sign
pub const OP_LSHIFT: u8 = 152;
/// Shifts a right b bits, preserving sign
pub const OP_RSHIFT: u8 = 153;
/// If both a and b are not empty, the output is 1. Otherwise, 0.
pub const OP_BOOLAND: u8 = 154;
/// If a or b is not empty, the output is 1. Otherwise, 0.
pub const OP_BOOLOR: u8 = 155;
/// Returns 1 if the numbers are equal. Otherwise, 0.
pub const OP_NUMEQUAL: u8 = 156;
/// Same as OP_NUMEQUAL, but runs OP_VERIFY afterward
pub const OP_NUMEQUALVERIFY: u8 = 157;
/// Returns 1 if the numbers are not equal. Otherwise, 0.
pub const OP_NUMNOTEQUAL: u8 = 158;
/// Returns 1 if a is less than b. Otherwise, 0.
pub const OP_LESSTHAN: u8 = 159;
/// Returns 1 if a is greater than b. Otherwise, 0.
pub const OP_GREATERTHAN: u8 = 160;
/// Returns 1 if a is less than or equal to b. Otherwise, 0.
pub const OP_LESSTHANOREQUAL: u8 = 161;
/// Returns 1 if a is greater than or equal to b. Otherwise, 0.
pub const OP_GREATERTHANOREQUAL: u8 = 162;
/// Returns the smaller of a and b
pub const OP_MIN: u8 = 163;
/// Returns the larger of a and b
pub const OP_MAX: u8 = 164;
/// Returns 1 if x is within the specified range, left inclusive. Otherwise, 0.
pub const OP_WITHIN: u8 = 165;
/// Converts numeric value a into a byte sequence of length b
pub const OP_NUM2BIN: u8 = 128;
/// Converts byte sequence x into a numeric value
pub const OP_BIN2NUM: u8 = 129;

// --------------------------------------------------------------------------------------------
// Cryptography
// --------------------------------------------------------------------------------------------

/// The input is hashed using RIPEMD-160
pub const OP_RIPEMD160: u8 = 166;
/// The input is hashed using SHA-1
pub const OP_SHA1: u8 = 167;
/// The input is hashed using SHA-256
pub const OP_SHA256: u8 = 168;
/// The input is hashed twice: first with SHA-256 and then with RIPEMD-160
pub const OP_HASH160: u8 = 169;
/// The input is hashed two times with SHA-256
pub const OP_HASH256: u8 = 170;
/// Marks the part of the script after which the signature will begin matching
pub const OP_CODESEPARATOR: u8 = 171;
/// Puts 1 on the stack if the signature authorizes the public key and transaction hash. Otherwise 0.
pub const OP_CHECKSIG: u8 = 172;
/// Same as OP_CHECKSIG, but OP_VERIFY is executed afterward
pub const OP_CHECKSIGVERIFY: u8 = 173;
/// Puts 1 on the stack if m of n signatures authorize the public key and transaction hash. Otherwise 0.
pub const OP_CHECKMULTISIG: u8 = 174;
/// Same as OP_CHECKMULTISIG, but OP_VERIFY is executed afterward
pub const OP_CHECKMULTISIGVERIFY: u8 = 175;

// --------------------------------------------------------------------------------------------
// Locktime
// --------------------------------------------------------------------------------------------

/// Marks transaction as invalid if the top stack item is greater than the transaction's lock_time
pub const OP_CHECKLOCKTIMEVERIFY: u8 = 177;
/// Marks transaction as invalid if the top stack item is less than the transaction's sequence used for relative lock time
pub const OP_CHECKSEQUENCEVERIFY: u8 = 178;

// --------------------------------------------------------------------------------------------
// Pseudo-words
// --------------------------------------------------------------------------------------------

/// Represents a public key hashed with OP_HASH160
pub const OP_PUBKEYHASH: u8 = 253;
/// Represents a public key compatible with OP_CHECKSIG
pub const OP_PUBKEY: u8 = 254;
/// Matches any opcode that is not yet assigned
pub const OP_INVALIDOPCODE: u8 = 255;

// --------------------------------------------------------------------------------------------
// Reserved words
// --------------------------------------------------------------------------------------------

/// Transaction is invalid unless occuring in an unexecuted OP_IF branch
pub const OP_RESERVED: u8 = 80;
/// Transaction is invalid unless occuring in an unexecuted OP_IF branch
pub const OP_VER: u8 = 98;
/// Transaction is invalid even when occuring in an unexecuted OP_IF branch
pub const OP_VERIF: u8 = 101;
/// Transaction is invalid even when occuring in an unexecuted OP_IF branch
pub const OP_VERNOTIF: u8 = 102;
/// Transaction is invalid unless occuring in an unexecuted OP_IF branch
pub const OP_RESERVED1: u8 = 137;
/// Transaction is invalid unless occuring in an unexecuted OP_IF branch
pub const OP_RESERVED2: u8 = 138;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP1: u8 = 176;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP4: u8 = 179;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP5: u8 = 180;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP6: u8 = 181;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP7: u8 = 182;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP8: u8 = 183;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP9: u8 = 184;
/// The word is ignored. Does not mark transaction as invalid.
pub const OP_NOP10: u8 = 185;

/// Words at or above this number are invalid
pub const OP_INVALID_ABOVE: u8 = 186;

// --------------------------------------------------------------------------------------------
// Disabled words
// --------------------------------------------------------------------------------------------

/// The input is multiplied by 2
pub const OP_2MUL: u8 = 141;
/// The input is divided by 2
pub const OP_2DIV: u8 = 142;
}