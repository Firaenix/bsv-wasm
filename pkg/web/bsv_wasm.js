
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4);
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayU32FromWasm0(ptr, len) {
    return getUint32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*
* * This entire page is borrowed from rust-sv (https://github.com/brentongunning/rust-sv/blob/master/src/script/op_codes.rs)
*
*/
export const OpCodes = Object.freeze({
/**
* Pushes 0 onto the stack
*/
OP_0:0,"0":"OP_0",
/**
* Pushes 0 onto the stack
* The next byte sets the number of bytes to push onto the stack
*/
OP_PUSHDATA1:76,"76":"OP_PUSHDATA1",
/**
* The next two bytes sets the number of bytes to push onto the stack
*/
OP_PUSHDATA2:77,"77":"OP_PUSHDATA2",
/**
* The next four bytes sets the number of bytes to push onto the stack
*/
OP_PUSHDATA4:78,"78":"OP_PUSHDATA4",
/**
* Pushes -1 onto the stack
*/
OP_1NEGATE:79,"79":"OP_1NEGATE",
/**
* Pushes 1 onto the stack
*/
OP_1:81,"81":"OP_1",
/**
* Pushes 1 onto the stack
* Pushes 2 onto the stack
*/
OP_2:82,"82":"OP_2",
/**
* Pushes 3 onto the stack
*/
OP_3:83,"83":"OP_3",
/**
* Pushes 4 onto the stack
*/
OP_4:84,"84":"OP_4",
/**
* Pushes 5 onto the stack
*/
OP_5:85,"85":"OP_5",
/**
* Pushes 6 onto the stack
*/
OP_6:86,"86":"OP_6",
/**
* Pushes 7 onto the stack
*/
OP_7:87,"87":"OP_7",
/**
* Pushes 8 onto the stack
*/
OP_8:88,"88":"OP_8",
/**
* Pushes 9 onto the stack
*/
OP_9:89,"89":"OP_9",
/**
* Pushes 10 onto the stack
*/
OP_10:90,"90":"OP_10",
/**
* Pushes 11 onto the stack
*/
OP_11:91,"91":"OP_11",
/**
* Pushes 12 onto the stack
*/
OP_12:92,"92":"OP_12",
/**
* Pushes 13 onto the stack
*/
OP_13:93,"93":"OP_13",
/**
* Pushes 14 onto the stack
*/
OP_14:94,"94":"OP_14",
/**
* Pushes 15 onto the stack
*/
OP_15:95,"95":"OP_15",
/**
* Pushes 16 onto the stack
*/
OP_16:96,"96":"OP_16",
/**
* Does nothing
*/
OP_NOP:97,"97":"OP_NOP",
/**
* If the top stack is true, statements are executed. Top stack value is removed.
*/
OP_IF:99,"99":"OP_IF",
/**
* If the top stack is false, statements are executed. Top stack value is removed.
*/
OP_NOTIF:100,"100":"OP_NOTIF",
/**
* If the preceding OP_IF or OP_NOTIF statemetns were not executed, then statements are executed.
*/
OP_ELSE:103,"103":"OP_ELSE",
/**
* Ends an if-else block
*/
OP_ENDIF:104,"104":"OP_ENDIF",
/**
* Marks a statement as invalid if the top stack value is false. Top stack value is removed.
*/
OP_VERIFY:105,"105":"OP_VERIFY",
/**
* Marks a statements as invalid
*/
OP_RETURN:106,"106":"OP_RETURN",
/**
* Moves the top item on the main stack to the alt stack
*/
OP_TOALTSTACK:107,"107":"OP_TOALTSTACK",
/**
* Moves the top item on the alt stack to the main stack
*/
OP_FROMALTSTACK:108,"108":"OP_FROMALTSTACK",
/**
* Duplicates the top stack value if it is not zero
*/
OP_IFDUP:115,"115":"OP_IFDUP",
/**
* Puts the number of stack items onto the stack
*/
OP_DEPTH:116,"116":"OP_DEPTH",
/**
* Drops the top stack value
*/
OP_DROP:117,"117":"OP_DROP",
/**
* Duplicates the top stack item
*/
OP_DUP:118,"118":"OP_DUP",
/**
* Removes the second-to-top stack item
*/
OP_NIP:119,"119":"OP_NIP",
/**
* Copies the second-to-top stack item to the top
*/
OP_OVER:120,"120":"OP_OVER",
/**
* The item n back in the stack is copied to the top
*/
OP_PICK:121,"121":"OP_PICK",
/**
* The item n back in the stack is moved to the top
*/
OP_ROLL:122,"122":"OP_ROLL",
/**
* The top three items on the stack are rotated to the left
*/
OP_ROT:123,"123":"OP_ROT",
/**
* The top two items on the stack are swapped
*/
OP_SWAP:124,"124":"OP_SWAP",
/**
* The item at the top of the stack is copied and inserted before the second-to-top item
*/
OP_TUCK:125,"125":"OP_TUCK",
/**
* Removes the top two items from the stack
*/
OP_2DROP:109,"109":"OP_2DROP",
/**
* Duplicates the top two stack items
*/
OP_2DUP:110,"110":"OP_2DUP",
/**
* Duplicates the top three stack items
*/
OP_3DUP:111,"111":"OP_3DUP",
/**
* Copies the pair of items two spaces back to the front
*/
OP_2OVER:112,"112":"OP_2OVER",
/**
* The fifth and sixth items back are moved to the top of the stack
*/
OP_2ROT:113,"113":"OP_2ROT",
/**
* Swaps the top two pairs of items
*/
OP_2SWAP:114,"114":"OP_2SWAP",
/**
* Concatenates two byte sequences
*/
OP_CAT:126,"126":"OP_CAT",
/**
* Splits the byte sequence at position n
*/
OP_SPLIT:127,"127":"OP_SPLIT",
/**
* Pushes the byte sequence length of the top stack item without popping it
*/
OP_SIZE:130,"130":"OP_SIZE",
/**
* Flips all of the bits in the input
*/
OP_INVERT:131,"131":"OP_INVERT",
/**
* Boolean and between each bit in the inputs
*/
OP_AND:132,"132":"OP_AND",
/**
* Boolean or between each bit in the inputs
*/
OP_OR:133,"133":"OP_OR",
/**
* Boolean exclusive or between each bit in the inputs
*/
OP_XOR:134,"134":"OP_XOR",
/**
* Returns 1 if the inputs are exactly equal, 0 otherwise
*/
OP_EQUAL:135,"135":"OP_EQUAL",
/**
* Same as OP_EQUAL, but runs OP_VERIFY afterward
*/
OP_EQUALVERIFY:136,"136":"OP_EQUALVERIFY",
/**
* Adds 1 to the input
*/
OP_1ADD:139,"139":"OP_1ADD",
/**
* Subtracts 1 from the input
*/
OP_1SUB:140,"140":"OP_1SUB",
/**
* The sign of the input is flipped
*/
OP_NEGATE:143,"143":"OP_NEGATE",
/**
* The input is made positive
*/
OP_ABS:144,"144":"OP_ABS",
/**
* If the input is 0 or 1, it is flipped. Otherwise, the output will be 0.
*/
OP_NOT:145,"145":"OP_NOT",
/**
* Returns 0 if the input is 0. 1 otherwise.
*/
OP_0NOTEQUAL:146,"146":"OP_0NOTEQUAL",
/**
* Adds a to b
*/
OP_ADD:147,"147":"OP_ADD",
/**
* Subtracts b from a
*/
OP_SUB:148,"148":"OP_SUB",
/**
* Multiplies a by b
*/
OP_MUL:149,"149":"OP_MUL",
/**
* Divides a by b
*/
OP_DIV:150,"150":"OP_DIV",
/**
* Returns the remainder after dividing a by b
*/
OP_MOD:151,"151":"OP_MOD",
/**
* Shifts a left b bits, preserving sign
*/
OP_LSHIFT:152,"152":"OP_LSHIFT",
/**
* Shifts a right b bits, preserving sign
*/
OP_RSHIFT:153,"153":"OP_RSHIFT",
/**
* If both a and b are not empty, the output is 1. Otherwise, 0.
*/
OP_BOOLAND:154,"154":"OP_BOOLAND",
/**
* If a or b is not empty, the output is 1. Otherwise, 0.
*/
OP_BOOLOR:155,"155":"OP_BOOLOR",
/**
* Returns 1 if the numbers are equal. Otherwise, 0.
*/
OP_NUMEQUAL:156,"156":"OP_NUMEQUAL",
/**
* Same as OP_NUMEQUAL, but runs OP_VERIFY afterward
*/
OP_NUMEQUALVERIFY:157,"157":"OP_NUMEQUALVERIFY",
/**
* Returns 1 if the numbers are not equal. Otherwise, 0.
*/
OP_NUMNOTEQUAL:158,"158":"OP_NUMNOTEQUAL",
/**
* Returns 1 if a is less than b. Otherwise, 0.
*/
OP_LESSTHAN:159,"159":"OP_LESSTHAN",
/**
* Returns 1 if a is greater than b. Otherwise, 0.
*/
OP_GREATERTHAN:160,"160":"OP_GREATERTHAN",
/**
* Returns 1 if a is less than or equal to b. Otherwise, 0.
*/
OP_LESSTHANOREQUAL:161,"161":"OP_LESSTHANOREQUAL",
/**
* Returns 1 if a is greater than or equal to b. Otherwise, 0.
*/
OP_GREATERTHANOREQUAL:162,"162":"OP_GREATERTHANOREQUAL",
/**
* Returns the smaller of a and b
*/
OP_MIN:163,"163":"OP_MIN",
/**
* Returns the larger of a and b
*/
OP_MAX:164,"164":"OP_MAX",
/**
* Returns 1 if x is within the specified range, left inclusive. Otherwise, 0.
*/
OP_WITHIN:165,"165":"OP_WITHIN",
/**
* Converts numeric value a into a byte sequence of length b
*/
OP_NUM2BIN:128,"128":"OP_NUM2BIN",
/**
* Converts byte sequence x into a numeric value
*/
OP_BIN2NUM:129,"129":"OP_BIN2NUM",
/**
* The input is hashed using RIPEMD-160
*/
OP_RIPEMD160:166,"166":"OP_RIPEMD160",
/**
* The input is hashed using SHA-1
*/
OP_SHA1:167,"167":"OP_SHA1",
/**
* The input is hashed using SHA-256
*/
OP_SHA256:168,"168":"OP_SHA256",
/**
* The input is hashed twice: first with SHA-256 and then with RIPEMD-160
*/
OP_HASH160:169,"169":"OP_HASH160",
/**
* The input is hashed two times with SHA-256
*/
OP_HASH256:170,"170":"OP_HASH256",
/**
* Marks the part of the script after which the signature will begin matching
*/
OP_CODESEPARATOR:171,"171":"OP_CODESEPARATOR",
/**
* Puts 1 on the stack if the signature authorizes the public key and transaction hash. Otherwise 0.
*/
OP_CHECKSIG:172,"172":"OP_CHECKSIG",
/**
* Same as OP_CHECKSIG, but OP_VERIFY is executed afterward
*/
OP_CHECKSIGVERIFY:173,"173":"OP_CHECKSIGVERIFY",
/**
* Puts 1 on the stack if m of n signatures authorize the public key and transaction hash. Otherwise 0.
*/
OP_CHECKMULTISIG:174,"174":"OP_CHECKMULTISIG",
/**
* Same as OP_CHECKMULTISIG, but OP_VERIFY is executed afterward
*/
OP_CHECKMULTISIGVERIFY:175,"175":"OP_CHECKMULTISIGVERIFY",
/**
* Marks transaction as invalid if the top stack item is greater than the transaction's lock_time
*/
OP_CHECKLOCKTIMEVERIFY:177,"177":"OP_CHECKLOCKTIMEVERIFY",
/**
* Marks transaction as invalid if the top stack item is less than the transaction's sequence used for relative lock time
*/
OP_CHECKSEQUENCEVERIFY:178,"178":"OP_CHECKSEQUENCEVERIFY",
/**
* OP_DATA followed by a varint represents arbitrary data on chain. Used for matching Script Templates.
*/
OP_DATA:251,"251":"OP_DATA",
/**
* Represents a secp256k1 signature
*/
OP_SIG:252,"252":"OP_SIG",
/**
* Represents a public key hashed with OP_HASH160
*/
OP_PUBKEYHASH:253,"253":"OP_PUBKEYHASH",
/**
* Represents a public key compatible with OP_CHECKSIG
*/
OP_PUBKEY:254,"254":"OP_PUBKEY",
/**
* Matches any opcode that is not yet assigned
*/
OP_INVALIDOPCODE:255,"255":"OP_INVALIDOPCODE",
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
OP_RESERVED:80,"80":"OP_RESERVED",
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
OP_VER:98,"98":"OP_VER",
/**
* Transaction is invalid even when occuring in an unexecuted OP_IF branch
*/
OP_VERIF:101,"101":"OP_VERIF",
/**
* Transaction is invalid even when occuring in an unexecuted OP_IF branch
*/
OP_VERNOTIF:102,"102":"OP_VERNOTIF",
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
OP_RESERVED1:137,"137":"OP_RESERVED1",
/**
* Transaction is invalid unless occuring in an unexecuted OP_IF branch
*/
OP_RESERVED2:138,"138":"OP_RESERVED2",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP1:176,"176":"OP_NOP1",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP4:179,"179":"OP_NOP4",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP5:180,"180":"OP_NOP5",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP6:181,"181":"OP_NOP6",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP7:182,"182":"OP_NOP7",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP8:183,"183":"OP_NOP8",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP9:184,"184":"OP_NOP9",
/**
* The word is ignored. Does not mark transaction as invalid.
*/
OP_NOP10:185,"185":"OP_NOP10",
/**
* Words at or above this number are invalid
*/
OP_INVALID_ABOVE:186,"186":"OP_INVALID_ABOVE",
/**
* The input is multiplied by 2
*/
OP_2MUL:141,"141":"OP_2MUL",
/**
* The input is divided by 2
*/
OP_2DIV:142,"142":"OP_2DIV", });
/**
*/
export const SigHash = Object.freeze({ FORKID:64,"64":"FORKID",ALL:1,"1":"ALL",NONE:2,"2":"NONE",SINGLE:3,"3":"SINGLE",ANYONECANPAY:128,"128":"ANYONECANPAY",
/**
*
*     * ALL | FORKID
*
*/
InputsOutputs:65,"65":"InputsOutputs",
/**
*
*     * NONE | FORKID
*
*/
Inputs:66,"66":"Inputs",
/**
*
*     * SINGLE | FORKID
*
*/
InputsOutput:67,"67":"InputsOutput",
/**
*
*     * ALL | ANYONECANPAY | FORKID
*
*/
InputOutputs:193,"193":"InputOutputs",
/**
*
*     * NONE | ANYONECANPAY | FORKID
*
*/
Input:194,"194":"Input",
/**
*
*     * SINGLE | ANYONECANPAY | FORKID
*
*/
InputOutput:195,"195":"InputOutput",
/**
*
*     * ALL | ANYONECANPAY
*
*/
Legacy_InputOutputs:129,"129":"Legacy_InputOutputs",
/**
*
*     * NONE | ANYONECANPAY
*
*/
Legacy_Input:130,"130":"Legacy_Input",
/**
*
*     * SINGLE | ANYONECANPAY
*
*/
Legacy_InputOutput:131,"131":"Legacy_InputOutput", });
/**
*/
export const PBKDF2Hashes = Object.freeze({ SHA1:0,"0":"SHA1",SHA256:1,"1":"SHA256",SHA512:2,"2":"SHA512", });
/**
*/
export const DataLengthConstraints = Object.freeze({ Equals:0,"0":"Equals",GreaterThan:1,"1":"GreaterThan",LessThan:2,"2":"LessThan",GreaterThanOrEquals:3,"3":"GreaterThanOrEquals",LessThanOrEquals:4,"4":"LessThanOrEquals", });
/**
*/
export const MatchDataTypes = Object.freeze({ Data:0,"0":"Data",Signature:1,"1":"Signature",PublicKey:2,"2":"PublicKey",PublicKeyHash:3,"3":"PublicKeyHash", });
/**
*/
export const AESAlgorithms = Object.freeze({ AES128_CBC:0,"0":"AES128_CBC",AES256_CBC:1,"1":"AES256_CBC",AES128_CTR:2,"2":"AES128_CTR",AES256_CTR:3,"3":"AES256_CTR", });
/**
*/
export const SigningHash = Object.freeze({ Sha256:0,"0":"Sha256",Sha256d:1,"1":"Sha256d", });
/**
*/
export class AES {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_aes_free(ptr);
    }
    /**
    * @param {Uint8Array} key
    * @param {Uint8Array} iv
    * @param {Uint8Array} message
    * @param {number} algo
    * @returns {Uint8Array}
    */
    static encrypt(key, iv, message, algo) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passArray8ToWasm0(iv, wasm.__wbindgen_malloc);
            var len1 = WASM_VECTOR_LEN;
            var ptr2 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            var len2 = WASM_VECTOR_LEN;
            wasm.aes_encrypt(retptr, ptr0, len0, ptr1, len1, ptr2, len2, algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v3 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v3;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} key
    * @param {Uint8Array} iv
    * @param {Uint8Array} message
    * @param {number} algo
    * @returns {Uint8Array}
    */
    static decrypt(key, iv, message, algo) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passArray8ToWasm0(iv, wasm.__wbindgen_malloc);
            var len1 = WASM_VECTOR_LEN;
            var ptr2 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            var len2 = WASM_VECTOR_LEN;
            wasm.aes_decrypt(retptr, ptr0, len0, ptr1, len1, ptr2, len2, algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v3 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v3;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*
* * Bitcoin Signed Message
*
*/
export class BSM {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bsm_free(ptr);
    }
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
    static isValidMessage(message, signature, address) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        _assertClass(address, P2PKHAddress);
        var ret = wasm.bsm_isValidMessage(ptr0, len0, signature.ptr, address.ptr);
        return ret !== 0;
    }
    /**
    * @param {Uint8Array} message
    * @param {Signature} signature
    * @param {P2PKHAddress} address
    * @returns {boolean}
    */
    static verifyMessage(message, signature, address) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        _assertClass(address, P2PKHAddress);
        var ret = wasm.bsm_verifyMessage(ptr0, len0, signature.ptr, address.ptr);
        return ret !== 0;
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {Uint8Array} message
    * @returns {Signature}
    */
    static signMessage(priv_key, message) {
        _assertClass(priv_key, PrivateKey);
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.bsm_signMessage(priv_key.ptr, ptr0, len0);
        return Signature.__wrap(ret);
    }
}
/**
*
* * A handy struct to allow calling of various utility methods
*
*/
export class Bytes {

    static __wrap(ptr) {
        const obj = Object.create(Bytes.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bytes_free(ptr);
    }
    /**
    * @returns {Uint8Array}
    */
    readReverse() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bytes_readReverse(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    read() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bytes_read(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    */
    reverse() {
        wasm.bytes_reverse(this.ptr);
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bytes_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} hex_str
    * @returns {Bytes}
    */
    static fromHex(hex_str) {
        var ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.bytes_fromHex(ptr0, len0);
        return Bytes.__wrap(ret);
    }
}
/**
*/
export class CipherKeys {

    static __wrap(ptr) {
        const obj = Object.create(CipherKeys.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cipherkeys_free(ptr);
    }
    /**
    * @returns {Uint8Array}
    */
    get_iv() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cipherkeys_get_iv(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    get_ke() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cipherkeys_get_ke(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    get_km() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cipherkeys_get_km(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class ECDH {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ecdh_free(ptr);
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {PublicKey} pub_key
    * @returns {Uint8Array}
    */
    static deriveSharedKey(priv_key, pub_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(priv_key, PrivateKey);
            _assertClass(pub_key, PublicKey);
            wasm.ecdh_deriveSharedKey(retptr, priv_key.ptr, pub_key.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*
* * Utility struct for low level ECDSA primitives
*
*/
export class ECDSA {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ecdsa_free(ptr);
    }
    /**
    * @param {Uint8Array} message
    * @param {PublicKey} pub_key
    * @param {Signature} signature
    * @param {number} hash_algo
    * @returns {boolean}
    */
    static verify(message, pub_key, signature, hash_algo) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(pub_key, PublicKey);
        _assertClass(signature, Signature);
        var ret = wasm.ecdsa_verify(ptr0, len0, pub_key.ptr, signature.ptr, hash_algo);
        return ret !== 0;
    }
    /**
    * @param {PrivateKey} private_key
    * @param {Uint8Array} preimage
    * @param {number} hash_algo
    * @param {boolean} reverse_k
    * @returns {Signature}
    */
    static signWithRandomK(private_key, preimage, hash_algo, reverse_k) {
        _assertClass(private_key, PrivateKey);
        var ptr0 = passArray8ToWasm0(preimage, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.ecdsa_signWithRandomK(private_key.ptr, ptr0, len0, hash_algo, reverse_k);
        return Signature.__wrap(ret);
    }
    /**
    * @param {PrivateKey} private_key
    * @param {Uint8Array} preimage
    * @param {number} hash_algo
    * @param {boolean} reverse_k
    * @returns {Signature}
    */
    static sign(private_key, preimage, hash_algo, reverse_k) {
        _assertClass(private_key, PrivateKey);
        var ptr0 = passArray8ToWasm0(preimage, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.ecdsa_sign(private_key.ptr, ptr0, len0, hash_algo, reverse_k);
        return Signature.__wrap(ret);
    }
}
/**
*
* * Electrum compatible ECIES implementation.
* * Comparable to Ecies.electrumEncrypt in BSV.JS
*
*/
export class ECIES {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ecies_free(ptr);
    }
    /**
    * @param {Uint8Array} message
    * @param {PrivateKey} sender_priv_key
    * @param {PublicKey} recipient_pub_key
    * @param {boolean} exclude_pub_key
    * @returns {ECIESCiphertext}
    */
    static encrypt(message, sender_priv_key, recipient_pub_key, exclude_pub_key) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(sender_priv_key, PrivateKey);
        _assertClass(recipient_pub_key, PublicKey);
        var ret = wasm.ecies_encrypt(ptr0, len0, sender_priv_key.ptr, recipient_pub_key.ptr, exclude_pub_key);
        return ECIESCiphertext.__wrap(ret);
    }
    /**
    *
    *     * Encrypt with a randomly generate private key.
    *     * This is intended to be used if you want to anonymously send a party an encrypted message.
    *
    * @param {Uint8Array} message
    * @param {PublicKey} recipient_pub_key
    * @returns {ECIESCiphertext}
    */
    static encryptWithEphemeralKey(message, recipient_pub_key) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(recipient_pub_key, PublicKey);
        var ret = wasm.ecies_encryptWithEphemeralKey(ptr0, len0, recipient_pub_key.ptr);
        return ECIESCiphertext.__wrap(ret);
    }
    /**
    * @param {ECIESCiphertext} ciphertext
    * @param {PrivateKey} recipient_priv_key
    * @param {PublicKey} sender_pub_key
    * @returns {Uint8Array}
    */
    static decrypt(ciphertext, recipient_priv_key, sender_pub_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(ciphertext, ECIESCiphertext);
            _assertClass(recipient_priv_key, PrivateKey);
            _assertClass(sender_pub_key, PublicKey);
            wasm.ecies_decrypt(retptr, ciphertext.ptr, recipient_priv_key.ptr, sender_pub_key.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {PublicKey} pub_key
    * @returns {CipherKeys}
    */
    static deriveCipherKeys(priv_key, pub_key) {
        _assertClass(priv_key, PrivateKey);
        _assertClass(pub_key, PublicKey);
        var ret = wasm.ecies_deriveCipherKeys(priv_key.ptr, pub_key.ptr);
        return CipherKeys.__wrap(ret);
    }
}
/**
*/
export class ECIESCiphertext {

    static __wrap(ptr) {
        const obj = Object.create(ECIESCiphertext.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_eciesciphertext_free(ptr);
    }
    /**
    * @returns {Uint8Array}
    */
    getCiphertext() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cipherkeys_get_ke(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    getHMAC() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cipherkeys_get_km(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {CipherKeys | undefined}
    */
    getCipherKeys() {
        var ret = wasm.eciesciphertext_getCipherKeys(this.ptr);
        return ret === 0 ? undefined : CipherKeys.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.eciesciphertext_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {PublicKey}
    */
    extractPublicKey() {
        var ret = wasm.eciesciphertext_extractPublicKey(this.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @param {Uint8Array} buffer
    * @param {boolean} has_pub_key
    * @returns {ECIESCiphertext}
    */
    static fromBytes(buffer, has_pub_key) {
        var ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.eciesciphertext_fromBytes(ptr0, len0, has_pub_key);
        return ECIESCiphertext.__wrap(ret);
    }
}
/**
*/
export class ExtendedPrivateKey {

    static __wrap(ptr) {
        const obj = Object.create(ExtendedPrivateKey.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_extendedprivatekey_free(ptr);
    }
    /**
    * @returns {PrivateKey}
    */
    getPrivateKey() {
        var ret = wasm.extendedprivatekey_getPrivateKey(this.ptr);
        return PrivateKey.__wrap(ret);
    }
    /**
    * @returns {PublicKey}
    */
    getPublicKey() {
        var ret = wasm.extendedprivatekey_getPublicKey(this.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    getChainCode() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedprivatekey_getChainCode(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getDepth() {
        var ret = wasm.extendedprivatekey_getDepth(this.ptr);
        return ret;
    }
    /**
    * @returns {Uint8Array}
    */
    getParentFingerprint() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedprivatekey_getParentFingerprint(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getIndex() {
        var ret = wasm.extendedprivatekey_getIndex(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} index
    * @returns {ExtendedPrivateKey}
    */
    deriveChild(index) {
        var ret = wasm.extendedprivatekey_deriveChild(this.ptr, index);
        return ExtendedPrivateKey.__wrap(ret);
    }
    /**
    * @param {string} path
    * @returns {ExtendedPrivateKey}
    */
    derive(path) {
        var ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.extendedprivatekey_derive(this.ptr, ptr0, len0);
        return ExtendedPrivateKey.__wrap(ret);
    }
    /**
    * @param {Uint8Array} seed
    * @returns {ExtendedPrivateKey}
    */
    static fromSeed(seed) {
        var ptr0 = passArray8ToWasm0(seed, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.extendedprivatekey_fromSeed(ptr0, len0);
        return ExtendedPrivateKey.__wrap(ret);
    }
    /**
    * @returns {ExtendedPrivateKey}
    */
    static fromRandom() {
        var ret = wasm.extendedprivatekey_fromRandom();
        return ExtendedPrivateKey.__wrap(ret);
    }
    /**
    * @param {string} xprv_string
    * @returns {ExtendedPrivateKey}
    */
    static fromString(xprv_string) {
        var ptr0 = passStringToWasm0(xprv_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.extendedprivatekey_fromString(ptr0, len0);
        return ExtendedPrivateKey.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedprivatekey_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {Uint8Array} mnemonic
    * @param {Uint8Array | undefined} passphrase
    * @returns {ExtendedPrivateKey}
    */
    static fromMnemonic(mnemonic, passphrase) {
        var ptr0 = passArray8ToWasm0(mnemonic, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(passphrase) ? 0 : passArray8ToWasm0(passphrase, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.extendedprivatekey_fromMnemonic(ptr0, len0, ptr1, len1);
        return ExtendedPrivateKey.__wrap(ret);
    }
}
/**
*/
export class ExtendedPublicKey {

    static __wrap(ptr) {
        const obj = Object.create(ExtendedPublicKey.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_extendedpublickey_free(ptr);
    }
    /**
    * @returns {PublicKey}
    */
    getPublicKey() {
        var ret = wasm.extendedpublickey_getPublicKey(this.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @param {ExtendedPrivateKey} xpriv
    * @returns {ExtendedPublicKey}
    */
    static fromXPriv(xpriv) {
        _assertClass(xpriv, ExtendedPrivateKey);
        var ret = wasm.extendedpublickey_fromXPriv(xpriv.ptr);
        return ExtendedPublicKey.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    getChainCode() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedpublickey_getChainCode(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getDepth() {
        var ret = wasm.extendedpublickey_getDepth(this.ptr);
        return ret;
    }
    /**
    * @returns {Uint8Array}
    */
    getParentFingerprint() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedpublickey_getParentFingerprint(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getIndex() {
        var ret = wasm.extendedpublickey_getIndex(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} index
    * @returns {ExtendedPublicKey}
    */
    deriveChild(index) {
        var ret = wasm.extendedpublickey_deriveChild(this.ptr, index);
        return ExtendedPublicKey.__wrap(ret);
    }
    /**
    * @param {string} path
    * @returns {ExtendedPublicKey}
    */
    derive(path) {
        var ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.extendedpublickey_derive(this.ptr, ptr0, len0);
        return ExtendedPublicKey.__wrap(ret);
    }
    /**
    * @param {Uint8Array} seed
    * @returns {ExtendedPublicKey}
    */
    static fromSeed(seed) {
        var ptr0 = passArray8ToWasm0(seed, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.extendedpublickey_fromSeed(ptr0, len0);
        return ExtendedPublicKey.__wrap(ret);
    }
    /**
    * @returns {ExtendedPublicKey}
    */
    static fromRandom() {
        var ret = wasm.extendedpublickey_fromRandom();
        return ExtendedPublicKey.__wrap(ret);
    }
    /**
    * @param {string} xpub_string
    * @returns {ExtendedPublicKey}
    */
    static fromString(xpub_string) {
        var ptr0 = passStringToWasm0(xpub_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.extendedpublickey_fromString(ptr0, len0);
        return ExtendedPublicKey.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedpublickey_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
*/
export class Hash {

    static __wrap(ptr) {
        const obj = Object.create(Hash.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hash_free(ptr);
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.hash_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.hash_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static sha256d(input) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha256d(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static sha256(input) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha256(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static sha1(input) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha1(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static ripemd160(input) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.hash_ripemd160(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static hash160(input) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.hash_hash160(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static sha512(input) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha512(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha512Hmac(input, key) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha512Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha256Hmac(input, key) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha256Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha256dHmac(input, key) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha256dHmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha1Hmac(input, key) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.hash_sha1Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static ripemd160Hmac(input, key) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.hash_ripemd160Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static hash160Hmac(input, key) {
        var ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.hash_hash160Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
}
/**
*/
export class KDF {

    static __wrap(ptr) {
        const obj = Object.create(KDF.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_kdf_free(ptr);
    }
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
    static pbkdf2(password, salt, hash_algo, rounds, output_length) {
        var ptr0 = passArray8ToWasm0(password, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(salt) ? 0 : passArray8ToWasm0(salt, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.kdf_pbkdf2(ptr0, len0, ptr1, len1, hash_algo, rounds, output_length);
        return KDF.__wrap(ret);
    }
    /**
    * @returns {Hash}
    */
    getHash() {
        var ret = wasm.kdf_getHash(this.ptr);
        return Hash.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    getSalt() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.kdf_getSalt(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class MatchCriteria {

    static __wrap(ptr) {
        const obj = Object.create(MatchCriteria.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_matchcriteria_free(ptr);
    }
    /**
    */
    constructor() {
        var ret = wasm.matchcriteria_new();
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {ScriptTemplate} script_template
    * @returns {MatchCriteria}
    */
    setScriptTemplate(script_template) {
        _assertClass(script_template, ScriptTemplate);
        var ret = wasm.matchcriteria_setScriptTemplate(this.ptr, script_template.ptr);
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {BigInt} value
    * @returns {MatchCriteria}
    */
    setValue(value) {
        uint64CvtShim[0] = value;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.matchcriteria_setValue(this.ptr, low0, high0);
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {BigInt} min
    * @returns {MatchCriteria}
    */
    setMin(min) {
        uint64CvtShim[0] = min;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.matchcriteria_setMin(this.ptr, low0, high0);
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {BigInt} max
    * @returns {MatchCriteria}
    */
    setMax(max) {
        uint64CvtShim[0] = max;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.matchcriteria_setMax(this.ptr, low0, high0);
        return MatchCriteria.__wrap(ret);
    }
}
/**
*/
export class P2PKHAddress {

    static __wrap(ptr) {
        const obj = Object.create(P2PKHAddress.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_p2pkhaddress_free(ptr);
    }
    /**
    * @returns {Uint8Array}
    */
    toPubKeyHashBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.p2pkhaddress_toPubKeyHashBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    toPubKeyHashHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.p2pkhaddress_toPubKeyHashHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
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
    isValidBitcoinMessage(message, signature) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        var ret = wasm.p2pkhaddress_isValidBitcoinMessage(this.ptr, ptr0, len0, signature.ptr);
        return ret !== 0;
    }
    /**
    * @param {Uint8Array} hash_bytes
    * @returns {P2PKHAddress}
    */
    static fromPubKeyHash(hash_bytes) {
        var ptr0 = passArray8ToWasm0(hash_bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.p2pkhaddress_fromPubKeyHash(ptr0, len0);
        return P2PKHAddress.__wrap(ret);
    }
    /**
    * @param {PublicKey} pub_key
    * @returns {P2PKHAddress}
    */
    static fromPubKey(pub_key) {
        _assertClass(pub_key, PublicKey);
        var ret = wasm.p2pkhaddress_fromPubKey(pub_key.ptr);
        return P2PKHAddress.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.p2pkhaddress_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} address_string
    * @returns {P2PKHAddress}
    */
    static fromString(address_string) {
        var ptr0 = passStringToWasm0(address_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.p2pkhaddress_fromString(ptr0, len0);
        return P2PKHAddress.__wrap(ret);
    }
    /**
    * @returns {Script}
    */
    toLockingScript() {
        var ret = wasm.p2pkhaddress_toLockingScript(this.ptr);
        return Script.__wrap(ret);
    }
    /**
    * @param {PublicKey} pub_key
    * @param {SighashSignature} sig
    * @returns {Script}
    */
    toUnlockingScript(pub_key, sig) {
        _assertClass(pub_key, PublicKey);
        _assertClass(sig, SighashSignature);
        var ret = wasm.p2pkhaddress_toUnlockingScript(this.ptr, pub_key.ptr, sig.ptr);
        return Script.__wrap(ret);
    }
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
    verifyBitcoinMessage(message, signature) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        var ret = wasm.p2pkhaddress_verifyBitcoinMessage(this.ptr, ptr0, len0, signature.ptr);
        return ret !== 0;
    }
}
/**
*/
export class PrivateKey {

    static __wrap(ptr) {
        const obj = Object.create(PrivateKey.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_privatekey_free(ptr);
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.privatekey_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.privatekey_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {PrivateKey}
    */
    static fromRandom() {
        var ret = wasm.privatekey_fromRandom();
        return PrivateKey.__wrap(ret);
    }
    /**
    *
    *     * Finds the Public Key Point.
    *     * Always returns the compressed point.
    *     * To get the decompressed point: PublicKey::from_bytes(point).to_decompressed()
    *
    * @returns {Uint8Array}
    */
    getPoint() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.privatekey_getPoint(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {boolean} should_compress
    * @returns {PrivateKey}
    */
    compressPublicKey(should_compress) {
        var ret = wasm.privatekey_compressPublicKey(this.ptr, should_compress);
        return PrivateKey.__wrap(ret);
    }
    /**
    * @param {string} wif_string
    * @returns {PrivateKey}
    */
    static fromWIF(wif_string) {
        var ptr0 = passStringToWasm0(wif_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.privatekey_fromWIF(ptr0, len0);
        return PrivateKey.__wrap(ret);
    }
    /**
    * @param {string} hex_str
    * @returns {PrivateKey}
    */
    static fromHex(hex_str) {
        var ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.privatekey_fromHex(ptr0, len0);
        return PrivateKey.__wrap(ret);
    }
    /**
    *
    *     * Standard ECDSA Message Signing using SHA256 as the digestg
    *
    * @param {Uint8Array} msg
    * @returns {Signature}
    */
    signMessage(msg) {
        var ptr0 = passArray8ToWasm0(msg, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.privatekey_signMessage(this.ptr, ptr0, len0);
        return Signature.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    toWIF() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.privatekey_toWIF(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {Uint8Array} bytes
    * @returns {PrivateKey}
    */
    static fromBytes(bytes) {
        var ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.privatekey_fromBytes(ptr0, len0);
        return PrivateKey.__wrap(ret);
    }
    /**
    * @returns {PublicKey}
    */
    toPublicKey() {
        var ret = wasm.privatekey_toPublicKey(this.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    *
    *     * Encrypt a message to the public key of this private key.
    *
    * @param {Uint8Array} message
    * @returns {ECIESCiphertext}
    */
    encryptMessage(message) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.privatekey_encryptMessage(this.ptr, ptr0, len0);
        return ECIESCiphertext.__wrap(ret);
    }
    /**
    *
    *     * Decrypt a message that was sent to the public key corresponding to this private key.
    *
    * @param {ECIESCiphertext} ciphertext
    * @param {PublicKey} sender_pub_key
    * @returns {Uint8Array}
    */
    decryptMessage(ciphertext, sender_pub_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(ciphertext, ECIESCiphertext);
            _assertClass(sender_pub_key, PublicKey);
            wasm.privatekey_decryptMessage(retptr, this.ptr, ciphertext.ptr, sender_pub_key.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class PublicKey {

    static __wrap(ptr) {
        const obj = Object.create(PublicKey.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_publickey_free(ptr);
    }
    /**
    * @param {Uint8Array} message
    * @param {Signature} signature
    * @returns {boolean}
    */
    isValidMessage(message, signature) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        var ret = wasm.publickey_isValidMessage(this.ptr, ptr0, len0, signature.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    isCompressed() {
        var ret = wasm.publickey_isCompressed(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {string} hex_str
    * @returns {PublicKey}
    */
    static fromHex(hex_str) {
        var ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.publickey_fromHex(ptr0, len0);
        return PublicKey.__wrap(ret);
    }
    /**
    * @param {Uint8Array} bytes
    * @returns {PublicKey}
    */
    static fromBytes(bytes) {
        var ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.publickey_fromBytes(ptr0, len0);
        return PublicKey.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.publickey_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.publickey_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {PrivateKey} priv_key
    * @returns {PublicKey}
    */
    static fromPrivateKey(priv_key) {
        _assertClass(priv_key, PrivateKey);
        var ret = wasm.publickey_fromPrivateKey(priv_key.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @param {Uint8Array} message
    * @param {Signature} signature
    * @returns {boolean}
    */
    verifyMessage(message, signature) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        var ret = wasm.publickey_verifyMessage(this.ptr, ptr0, len0, signature.ptr);
        return ret !== 0;
    }
    /**
    * @returns {P2PKHAddress}
    */
    toAddress() {
        var ret = wasm.publickey_toAddress(this.ptr);
        return P2PKHAddress.__wrap(ret);
    }
    /**
    * @returns {PublicKey}
    */
    toCompressed() {
        var ret = wasm.publickey_toCompressed(this.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @returns {PublicKey}
    */
    toDecompressed() {
        var ret = wasm.publickey_toDecompressed(this.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @param {Uint8Array} message
    * @param {PrivateKey} sender_private_key
    * @returns {ECIESCiphertext}
    */
    encryptMessage(message, sender_private_key) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(sender_private_key, PrivateKey);
        var ret = wasm.publickey_encryptMessage(this.ptr, ptr0, len0, sender_private_key.ptr);
        return ECIESCiphertext.__wrap(ret);
    }
}
/**
*/
export class Script {

    static __wrap(ptr) {
        const obj = Object.create(Script.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_script_free(ptr);
    }
    /**
    * Matches the Script against the provided ScriptTemplate.
    *
    * If any data can be gleaned from the Script (ie. OP_DATA, OP_PUBKEY, OP_SIG, etc.), it will return it in a `Vec<Match>`
    * @returns {[string, Uint8Array][]}
    * @param {ScriptTemplate} script_template
    * @returns {any}
    */
    matches(script_template) {
        _assertClass(script_template, ScriptTemplate);
        var ret = wasm.script_matches(this.ptr, script_template.ptr);
        return takeObject(ret);
    }
    /**
    * Matches the Script against the provided ScriptTemplate.
    *
    * Returns `true` if the Script matches the ScriptTemplate.
    * #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = isMatch))]
    * @param {ScriptTemplate} script_template
    * @returns {boolean}
    */
    is_match(script_template) {
        _assertClass(script_template, ScriptTemplate);
        var ret = wasm.script_is_match(this.ptr, script_template.ptr);
        return ret !== 0;
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.script_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getScriptLength() {
        var ret = wasm.script_getScriptLength(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.script_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    */
    remove_codeseparators() {
        wasm.script_remove_codeseparators(this.ptr);
    }
    /**
    * @returns {string}
    */
    toASMString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.script_toASMString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    toExtendedASMString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.script_toExtendedASMString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} hex
    * @returns {Script}
    */
    static fromHex(hex) {
        var ptr0 = passStringToWasm0(hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.script_fromHex(ptr0, len0);
        return Script.__wrap(ret);
    }
    /**
    * @param {Uint8Array} bytes
    * @returns {Script}
    */
    static fromBytes(bytes) {
        var ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.script_fromBytes(ptr0, len0);
        return Script.__wrap(ret);
    }
    /**
    * @param {string} asm_string
    * @returns {Script}
    */
    static fromASMString(asm_string) {
        var ptr0 = passStringToWasm0(asm_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.script_fromASMString(ptr0, len0);
        return Script.__wrap(ret);
    }
    /**
    * @param {Uint8Array} data_bytes
    * @returns {Uint8Array}
    */
    static encodePushData(data_bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = passArray8ToWasm0(data_bytes, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.script_encodePushData(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Gets the OP_PUSHDATA prefix varint
    *
    * @param {number} length
    * @returns {Uint8Array}
    */
    static getPushDataBytes(length) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.script_getPushDataBytes(retptr, length);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class ScriptTemplate {

    static __wrap(ptr) {
        const obj = Object.create(ScriptTemplate.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_scripttemplate_free(ptr);
    }
    /**
    * @param {Script} script
    * @returns {ScriptTemplate}
    */
    static from_script(script) {
        _assertClass(script, Script);
        var ret = wasm.scripttemplate_from_script(script.ptr);
        return ScriptTemplate.__wrap(ret);
    }
    /**
    * @param {string} asm
    * @returns {ScriptTemplate}
    */
    static from_asm_string(asm) {
        var ptr0 = passStringToWasm0(asm, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.scripttemplate_from_asm_string(ptr0, len0);
        return ScriptTemplate.__wrap(ret);
    }
}
/**
*/
export class SighashSignature {

    static __wrap(ptr) {
        const obj = Object.create(SighashSignature.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sighashsignature_free(ptr);
    }
    /**
    * @param {Signature} signature
    * @param {number} sighash_type
    * @param {Uint8Array} sighash_buffer
    */
    constructor(signature, sighash_type, sighash_buffer) {
        _assertClass(signature, Signature);
        var ptr0 = passArray8ToWasm0(sighash_buffer, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.sighashsignature_new(signature.ptr, sighash_type, ptr0, len0);
        return SighashSignature.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sighashsignature_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sighashsignature_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class Signature {

    static __wrap(ptr) {
        const obj = Object.create(Signature.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_signature_free(ptr);
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    toDER() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_toDER(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    toCompactBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_toCompactBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} message
    * @param {PublicKey} pub_key
    * @returns {boolean}
    */
    verifyMessage(message, pub_key) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(pub_key, PublicKey);
        var ret = wasm.signature_verifyMessage(this.ptr, ptr0, len0, pub_key.ptr);
        return ret !== 0;
    }
    /**
    * @param {Uint8Array} bytes
    * @param {boolean} is_recoverable
    * @returns {Signature}
    */
    static fromDER(bytes, is_recoverable) {
        var ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.signature_fromDER(ptr0, len0, is_recoverable);
        return Signature.__wrap(ret);
    }
    /**
    * @param {string} hex
    * @param {boolean} is_recoverable
    * @returns {Signature}
    */
    static fromHexDER(hex, is_recoverable) {
        var ptr0 = passStringToWasm0(hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.signature_fromHexDER(ptr0, len0, is_recoverable);
        return Signature.__wrap(ret);
    }
    /**
    * @param {Uint8Array} compact_bytes
    * @returns {Signature}
    */
    static fromCompactBytes(compact_bytes) {
        var ptr0 = passArray8ToWasm0(compact_bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.signature_fromCompactBytes(ptr0, len0);
        return Signature.__wrap(ret);
    }
    /**
    * @param {Uint8Array} message
    * @param {number} hash_algo
    * @returns {PublicKey}
    */
    recoverPublicKey(message, hash_algo) {
        var ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.signature_recoverPublicKey(this.ptr, ptr0, len0, hash_algo);
        return PublicKey.__wrap(ret);
    }
}
/**
*/
export class Transaction {

    static __wrap(ptr) {
        const obj = Object.create(Transaction.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_transaction_free(ptr);
    }
    /**
    * @param {PublicKey} pub_key
    * @param {SighashSignature} sig
    * @returns {boolean}
    */
    verify(pub_key, sig) {
        _assertClass(pub_key, PublicKey);
        _assertClass(sig, SighashSignature);
        var ret = wasm.transaction_verify(this.ptr, pub_key.ptr, sig.ptr);
        return ret !== 0;
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {number} sighash
    * @param {number} n_tx_in
    * @param {Script} unsigned_script
    * @param {BigInt} value
    * @returns {SighashSignature}
    */
    sign(priv_key, sighash, n_tx_in, unsigned_script, value) {
        _assertClass(priv_key, PrivateKey);
        _assertClass(unsigned_script, Script);
        uint64CvtShim[0] = value;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.transaction_sign(this.ptr, priv_key.ptr, sighash, n_tx_in, unsigned_script.ptr, low0, high0);
        return SighashSignature.__wrap(ret);
    }
    /**
    * @param {number} sighash
    * @param {number} n_tx_in
    * @param {Script} unsigned_script
    * @param {BigInt} value
    * @returns {Uint8Array}
    */
    sighashPreimage(sighash, n_tx_in, unsigned_script, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(unsigned_script, Script);
            uint64CvtShim[0] = value;
            const low0 = u32CvtShim[0];
            const high0 = u32CvtShim[1];
            wasm.transaction_sighashPreimage(retptr, this.ptr, sighash, n_tx_in, unsigned_script.ptr, low0, high0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getVersion() {
        var ret = wasm.transaction_getVersion(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    getInputsCount() {
        var ret = wasm.transaction_getInputsCount(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    getOutputsCount() {
        var ret = wasm.transaction_getOutputsCount(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} index
    * @returns {TxIn | undefined}
    */
    getInput(index) {
        var ret = wasm.transaction_getInput(this.ptr, index);
        return ret === 0 ? undefined : TxIn.__wrap(ret);
    }
    /**
    * @param {number} index
    * @returns {TxOut | undefined}
    */
    getOutput(index) {
        var ret = wasm.transaction_getOutput(this.ptr, index);
        return ret === 0 ? undefined : TxOut.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    getNLocktime() {
        var ret = wasm.transaction_getNLocktime(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Uint8Array}
    */
    getNLocktimeAsBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_getNLocktimeAsBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Creates a new empty transaction where you need to add inputs and outputs
    *     * Transaction.add_input(TxIn) and Transaction.add_output(TxOut)
    *
    * @param {number} version
    * @param {number} n_locktime
    */
    constructor(version, n_locktime) {
        var ret = wasm.transaction_new(version, n_locktime);
        return Transaction.__wrap(ret);
    }
    /**
    * @returns {Transaction}
    */
    static default() {
        var ret = wasm.transaction_default();
        return Transaction.__wrap(ret);
    }
    /**
    * @param {number} version
    * @returns {Transaction}
    */
    setVersion(version) {
        var ret = wasm.transaction_setVersion(this.ptr, version);
        return Transaction.__wrap(ret);
    }
    /**
    * @param {number} n_locktime
    * @returns {Transaction}
    */
    setNLocktime(n_locktime) {
        var ret = wasm.transaction_setNLocktime(this.ptr, n_locktime);
        return Transaction.__wrap(ret);
    }
    /**
    * @param {TxIn} input
    */
    addInput(input) {
        _assertClass(input, TxIn);
        wasm.transaction_addInput(this.ptr, input.ptr);
    }
    /**
    * @param {TxIn} input
    */
    prependInput(input) {
        _assertClass(input, TxIn);
        wasm.transaction_prependInput(this.ptr, input.ptr);
    }
    /**
    * @param {number} index
    * @param {TxIn} input
    */
    insertInput(index, input) {
        _assertClass(input, TxIn);
        wasm.transaction_insertInput(this.ptr, index, input.ptr);
    }
    /**
    * @param {TxOut} output
    */
    addOutput(output) {
        _assertClass(output, TxOut);
        wasm.transaction_addOutput(this.ptr, output.ptr);
    }
    /**
    * @param {TxOut} output
    */
    prependOutput(output) {
        _assertClass(output, TxOut);
        wasm.transaction_prependOutput(this.ptr, output.ptr);
    }
    /**
    * @param {number} index
    * @param {TxOut} output
    */
    insertOutput(index, output) {
        _assertClass(output, TxOut);
        wasm.transaction_insertOutput(this.ptr, index, output.ptr);
    }
    /**
    * @param {number} index
    * @param {TxIn} input
    */
    setInput(index, input) {
        _assertClass(input, TxIn);
        wasm.transaction_setInput(this.ptr, index, input.ptr);
    }
    /**
    * @param {number} index
    * @param {TxOut} output
    */
    setOutput(index, output) {
        _assertClass(output, TxOut);
        wasm.transaction_setOutput(this.ptr, index, output.ptr);
    }
    /**
    *
    *     * XT Method:
    *     * Returns the combined sum of all input satoshis.
    *     * If any of the inputs dont have satoshis defined, this returns None or null
    *
    * @returns {BigInt | undefined}
    */
    satoshisIn() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_satoshisIn(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            u32CvtShim[0] = r1;
            u32CvtShim[1] = r2;
            const n0 = r0 === 0 ? undefined : uint64CvtShim[0];
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Returns the combined sum of all output satoshis.
    *
    * @returns {BigInt}
    */
    satoshisOut() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_satoshisOut(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = uint64CvtShim[0];
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} hex_str
    * @returns {Transaction}
    */
    static fromHex(hex_str) {
        var ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.transaction_fromHex(ptr0, len0);
        return Transaction.__wrap(ret);
    }
    /**
    * @param {Uint8Array} tx_bytes
    * @returns {Transaction}
    */
    static fromBytes(tx_bytes) {
        var ptr0 = passArray8ToWasm0(tx_bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.transaction_fromBytes(ptr0, len0);
        return Transaction.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} json_string
    * @returns {Transaction}
    */
    static fromJsonString(json_string) {
        var ptr0 = passStringToWasm0(json_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.transaction_fromJsonString(ptr0, len0);
        return Transaction.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    toJSON() {
        var ret = wasm.transaction_toJSON(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    toCompactHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toCompactHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    *
    *     * Get size of current serialised Transaction object
    *
    * @returns {number}
    */
    getSize() {
        var ret = wasm.transaction_getSize(this.ptr);
        return ret >>> 0;
    }
    /**
    *
    *     * Adds an array of TxIn's to the transaction
    *     * @param {TxIn[]} tx_ins
    *
    * @param {any[]} tx_ins
    */
    addInputs(tx_ins) {
        var ptr0 = passArrayJsValueToWasm0(tx_ins, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.transaction_addInputs(this.ptr, ptr0, len0);
    }
    /**
    *
    *     * Returns all outpoints from this transaction as a 2D array of 36 byte buffers.
    *     *
    *     * @returns {Uint8Array[]} outpoint_array
    *
    * @returns {any}
    */
    getOutpoints() {
        var ret = wasm.transaction_getOutpoints(this.ptr);
        return takeObject(ret);
    }
    /**
    *
    *     * Adds an array of TxOuts to the transaction
    *     * @param {TxOut[]} tx_outs
    *
    * @param {any[]} tx_outs
    */
    addOutputs(tx_outs) {
        var ptr0 = passArrayJsValueToWasm0(tx_outs, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.transaction_addOutputs(this.ptr, ptr0, len0);
    }
    /**
    *
    *     * Gets the ID of the current transaction as a hex string.
    *
    * @returns {string}
    */
    getIdHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_getIdHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    *
    *     * Gets the ID of the current transaction as a Uint8Array.
    *
    * @returns {Uint8Array}
    */
    getIdBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_getIdBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
    *
    * @returns {Uint8Array}
    */
    toCompactBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toCompactBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
    *
    * @returns {Uint8Array}
    */
    toCompactBytesHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toCompactBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Deserialises the provided CBOR buffer to the TX+ format
    *
    * @param {Uint8Array} compact_buffer
    * @returns {Transaction}
    */
    static fromCompactBytes(compact_buffer) {
        var ptr0 = passArray8ToWasm0(compact_buffer, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.transaction_fromCompactBytes(ptr0, len0);
        return Transaction.__wrap(ret);
    }
    /**
    *
    *     * Returns the first output index that matches the given parameters, returns None or null if not found.
    *
    * @param {MatchCriteria} criteria
    * @returns {number | undefined}
    */
    matchOutput(criteria) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(criteria, MatchCriteria);
            wasm.transaction_matchOutput(retptr, this.ptr, criteria.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Returns a list of outputs indexes that match the given parameters
    *
    * @param {MatchCriteria} criteria
    * @returns {Uint32Array}
    */
    matchOutputs(criteria) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(criteria, MatchCriteria);
            wasm.transaction_matchOutputs(retptr, this.ptr, criteria.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Returns the first input index that matches the given parameters, returns None or null if not found.
    *
    * @param {MatchCriteria} criteria
    * @returns {number | undefined}
    */
    matchInput(criteria) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(criteria, MatchCriteria);
            wasm.transaction_matchInput(retptr, this.ptr, criteria.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Returns a list of input indexes that match the given parameters
    *
    * @param {MatchCriteria} criteria
    * @returns {Uint32Array}
    */
    matchInputs(criteria) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(criteria, MatchCriteria);
            wasm.transaction_matchInputs(retptr, this.ptr, criteria.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class TxIn {

    static __wrap(ptr) {
        const obj = Object.create(TxIn.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_txin_free(ptr);
    }
    /**
    * @param {Uint8Array} prev_tx_id
    * @param {number} vout
    * @param {Script} script_sig
    * @param {number | undefined} sequence
    */
    constructor(prev_tx_id, vout, script_sig, sequence) {
        var ptr0 = passArray8ToWasm0(prev_tx_id, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _assertClass(script_sig, Script);
        var ret = wasm.txin_new(ptr0, len0, vout, script_sig.ptr, !isLikeNone(sequence), isLikeNone(sequence) ? 0 : sequence);
        return TxIn.__wrap(ret);
    }
    /**
    * @returns {TxIn}
    */
    static default() {
        var ret = wasm.txin_default();
        return TxIn.__wrap(ret);
    }
    /**
    * @param {boolean | undefined} little_endian
    * @returns {Uint8Array}
    */
    getPrevTxId(little_endian) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getPrevTxId(retptr, this.ptr, isLikeNone(little_endian) ? 0xFFFFFF : little_endian ? 1 : 0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {boolean | undefined} little_endian
    * @returns {string}
    */
    getPrevTxIdHex(little_endian) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getPrevTxIdHex(retptr, this.ptr, isLikeNone(little_endian) ? 0xFFFFFF : little_endian ? 1 : 0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {number}
    */
    getVOut() {
        var ret = wasm.txin_getVOut(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {BigInt}
    */
    getScriptSigSize() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getScriptSigSize(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = uint64CvtShim[0];
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Script}
    */
    getScriptSig() {
        var ret = wasm.txin_getScriptSig(this.ptr);
        return Script.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    getScriptSigHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getScriptSigHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {number}
    */
    getSequence() {
        var ret = wasm.txin_getSequence(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Uint8Array}
    */
    getSequenceAsBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getSequenceAsBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {boolean | undefined} little_endian
    * @returns {Uint8Array}
    */
    getOutpointBytes(little_endian) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getOutpointBytes(retptr, this.ptr, isLikeNone(little_endian) ? 0xFFFFFF : little_endian ? 1 : 0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {boolean | undefined} little_endian
    * @returns {string}
    */
    getOutpointHex(little_endian) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getOutpointHex(retptr, this.ptr, isLikeNone(little_endian) ? 0xFFFFFF : little_endian ? 1 : 0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {Script} script
    */
    setScript(script) {
        _assertClass(script, Script);
        wasm.txin_setScript(this.ptr, script.ptr);
    }
    /**
    * @param {Uint8Array} txid
    */
    setPrevTxId(txid) {
        var ptr0 = passArray8ToWasm0(txid, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.txin_setPrevTxId(this.ptr, ptr0, len0);
    }
    /**
    * @param {number} vout
    */
    setVOut(vout) {
        wasm.txin_setVOut(this.ptr, vout);
    }
    /**
    * @param {number} sequence
    */
    setSequence(sequence) {
        wasm.txin_setSequence(this.ptr, sequence);
    }
    /**
    * @param {BigInt} satoshis
    */
    setSatoshis(satoshis) {
        uint64CvtShim[0] = satoshis;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        wasm.txin_setSatoshis(this.ptr, low0, high0);
    }
    /**
    * @returns {BigInt | undefined}
    */
    getSatoshis() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getSatoshis(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            u32CvtShim[0] = r1;
            u32CvtShim[1] = r2;
            const n0 = r0 === 0 ? undefined : uint64CvtShim[0];
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Script} unlocking_script
    */
    setUnlockingScript(unlocking_script) {
        _assertClass(unlocking_script, Script);
        wasm.txin_setUnlockingScript(this.ptr, unlocking_script.ptr);
    }
    /**
    * @returns {Uint8Array | undefined}
    */
    getUnlockingScriptBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getUnlockingScriptBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v0;
            if (r0 !== 0) {
                v0 = getArrayU8FromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} hex_str
    * @returns {TxIn}
    */
    static fromHex(hex_str) {
        var ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.txin_fromHex(ptr0, len0);
        return TxIn.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    toJSON() {
        var ret = wasm.txin_toJSON(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {Uint8Array} outpoint
    * @returns {TxIn}
    */
    static fromOutpointBytes(outpoint) {
        var ptr0 = passArray8ToWasm0(outpoint, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.txin_fromOutpointBytes(ptr0, len0);
        return TxIn.__wrap(ret);
    }
}
/**
*/
export class TxOut {

    static __wrap(ptr) {
        const obj = Object.create(TxOut.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_txout_free(ptr);
    }
    /**
    * @param {BigInt} value
    * @param {Script} script_pub_key
    */
    constructor(value, script_pub_key) {
        uint64CvtShim[0] = value;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        _assertClass(script_pub_key, Script);
        var ret = wasm.txout_new(low0, high0, script_pub_key.ptr);
        return TxOut.__wrap(ret);
    }
    /**
    * @returns {BigInt}
    */
    getSatoshis() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_getSatoshis(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = uint64CvtShim[0];
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint8Array}
    */
    getSatoshisAsBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_getSatoshisAsBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getScriptPubKeySize() {
        var ret = wasm.txout_getScriptPubKeySize(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Script}
    */
    getScriptPubKey() {
        var ret = wasm.txout_getScriptPubKey(this.ptr);
        return Script.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    getScriptPubKeyHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_getScriptPubKeyHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} hex_str
    * @returns {TxOut}
    */
    static fromHex(hex_str) {
        var ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.txout_fromHex(ptr0, len0);
        return TxOut.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    toHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_toHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {any}
    */
    toJSON() {
        var ret = wasm.txout_toJSON(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('bsv_wasm_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_json_serialize = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = JSON.stringify(obj === undefined ? null : obj);
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_randomFillSync_64cc7d048f228ca8 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_98117e9a7e993920 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_process_2f24d6544ea7b200 = function(arg0) {
        var ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        var ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_versions_6164651e75405d4a = function(arg0) {
        var ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_4b517d861cbcb3bc = function(arg0) {
        var ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        var ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_modulerequire_3440a4bcf44437db = function() { return handleError(function (arg0, arg1) {
        var ret = module.require(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_crypto_98fc271021c7d2ad = function(arg0) {
        var ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_msCrypto_a2cdb043d2bfe57f = function(arg0) {
        var ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_be86524d73f67598 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_888d259a5fefc347 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_c6fbdfc2918d5e58 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_baec038b5ab35c54 = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_3f735a5746d41fbd = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_1bc0b39582740e95 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_buffer_397eaa4d72ee94dd = function(arg0) {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_a7ce447f15ff496f = function(arg0) {
        var ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_969ad0a60e51d320 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_1eb8fc608a0d4cdb = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_929232475839a482 = function(arg0) {
        var ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_8b658422a224f479 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

