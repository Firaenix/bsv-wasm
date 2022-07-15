
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

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

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

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

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let cachedFloat64Memory0 = new Float64Array();

function getFloat64Memory0() {
    if (cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

const u32CvtShim = new Uint32Array(2);
/**
*/
export function configureStacktrace() {
    wasm.configureStacktrace();
}

let cachedUint32Memory0 = new Uint32Array();

function getUint32Memory0() {
    if (cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
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
export const AESAlgorithms = Object.freeze({ AES128_CBC:0,"0":"AES128_CBC",AES256_CBC:1,"1":"AES256_CBC",AES128_CTR:2,"2":"AES128_CTR",AES256_CTR:3,"3":"AES256_CTR", });
/**
*/
export const SigningHash = Object.freeze({ Sha256:0,"0":"Sha256",Sha256d:1,"1":"Sha256d", });
/**
*/
export const DataLengthConstraints = Object.freeze({ Equals:0,"0":"Equals",GreaterThan:1,"1":"GreaterThan",LessThan:2,"2":"LessThan",GreaterThanOrEquals:3,"3":"GreaterThanOrEquals",LessThanOrEquals:4,"4":"LessThanOrEquals", });
/**
*/
export const MatchDataTypes = Object.freeze({ Data:0,"0":"Data",Signature:1,"1":"Signature",PublicKey:2,"2":"PublicKey",PublicKeyHash:3,"3":"PublicKeyHash", });
/**
*/
export const PBKDF2Hashes = Object.freeze({ SHA1:0,"0":"SHA1",SHA256:1,"1":"SHA256",SHA512:2,"2":"SHA512", });

const AESFinalization = new FinalizationRegistry(ptr => wasm.__wbg_aes_free(ptr));
/**
*/
export class AES {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        AESFinalization.unregister(this);
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
            const ptr0 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(iv, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len2 = WASM_VECTOR_LEN;
            wasm.aes_encrypt(retptr, ptr0, len0, ptr1, len1, ptr2, len2, algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
            const ptr0 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(iv, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len2 = WASM_VECTOR_LEN;
            wasm.aes_decrypt(retptr, ptr0, len0, ptr1, len1, ptr2, len2, algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v3 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v3;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const BSMFinalization = new FinalizationRegistry(ptr => wasm.__wbg_bsm_free(ptr));
/**
*
* * Bitcoin Signed Message
*
*/
export class BSM {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        BSMFinalization.unregister(this);
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
        const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        _assertClass(address, P2PKHAddress);
        const ret = wasm.bsm_isValidMessage(ptr0, len0, signature.ptr, address.ptr);
        return ret !== 0;
    }
    /**
    * @param {Uint8Array} message
    * @param {Signature} signature
    * @param {P2PKHAddress} address
    * @returns {boolean}
    */
    static verifyMessage(message, signature, address) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(signature, Signature);
            _assertClass(address, P2PKHAddress);
            wasm.bsm_verifyMessage(retptr, ptr0, len0, signature.ptr, address.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {Uint8Array} message
    * @returns {Signature}
    */
    static signMessage(priv_key, message) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(priv_key, PrivateKey);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.bsm_signMessage(retptr, priv_key.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {PrivateKey} ephemeral_key
    * @param {Uint8Array} message
    * @returns {Signature}
    */
    static signMessageWithK(priv_key, ephemeral_key, message) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(priv_key, PrivateKey);
            _assertClass(ephemeral_key, PrivateKey);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.bsm_signMessageWithK(retptr, priv_key.ptr, ephemeral_key.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const BytesFinalization = new FinalizationRegistry(ptr => wasm.__wbg_bytes_free(ptr));
/**
*
* * A handy struct to allow calling of various utility methods
*
*/
export class Bytes {

    static __wrap(ptr) {
        const obj = Object.create(Bytes.prototype);
        obj.ptr = ptr;
        BytesFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        BytesFinalization.unregister(this);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.bytes_fromHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Bytes.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ChainParamsFinalization = new FinalizationRegistry(ptr => wasm.__wbg_chainparams_free(ptr));
/**
*/
export class ChainParams {

    static __wrap(ptr) {
        const obj = Object.create(ChainParams.prototype);
        obj.ptr = ptr;
        ChainParamsFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ChainParamsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chainparams_free(ptr);
    }
    /**
    * @returns {number}
    */
    get p2pkh() {
        const ret = wasm.__wbg_get_chainparams_p2pkh(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set p2pkh(arg0) {
        wasm.__wbg_set_chainparams_p2pkh(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get p2sh() {
        const ret = wasm.__wbg_get_chainparams_p2sh(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set p2sh(arg0) {
        wasm.__wbg_set_chainparams_p2sh(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get privkey() {
        const ret = wasm.__wbg_get_chainparams_privkey(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set privkey(arg0) {
        wasm.__wbg_set_chainparams_privkey(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get xpub() {
        const ret = wasm.__wbg_get_chainparams_xpub(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set xpub(arg0) {
        wasm.__wbg_set_chainparams_xpub(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get xpriv() {
        const ret = wasm.__wbg_get_chainparams_xpriv(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set xpriv(arg0) {
        wasm.__wbg_set_chainparams_xpriv(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get magic() {
        const ret = wasm.__wbg_get_chainparams_magic(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set magic(arg0) {
        wasm.__wbg_set_chainparams_magic(this.ptr, arg0);
    }
    /**
    */
    constructor() {
        const ret = wasm.chainparams_Mainnet();
        return ChainParams.__wrap(ret);
    }
    /**
    * @param {number} p2pkh
    * @param {number} p2sh
    * @param {number} privkey
    * @param {number} xpub
    * @param {number} xpriv
    * @param {number} magic
    * @returns {ChainParams}
    */
    static new(p2pkh, p2sh, privkey, xpub, xpriv, magic) {
        const ret = wasm.chainparams_new(p2pkh, p2sh, privkey, xpub, xpriv, magic);
        return ChainParams.__wrap(ret);
    }
    /**
    * @returns {ChainParams}
    */
    static Mainnet() {
        const ret = wasm.chainparams_Mainnet();
        return ChainParams.__wrap(ret);
    }
    /**
    * @returns {ChainParams}
    */
    static Testnet() {
        const ret = wasm.chainparams_Testnet();
        return ChainParams.__wrap(ret);
    }
    /**
    * @returns {ChainParams}
    */
    static Regtest() {
        const ret = wasm.chainparams_Regtest();
        return ChainParams.__wrap(ret);
    }
    /**
    * @returns {ChainParams}
    */
    static STN() {
        const ret = wasm.chainparams_STN();
        return ChainParams.__wrap(ret);
    }
}

const CipherKeysFinalization = new FinalizationRegistry(ptr => wasm.__wbg_cipherkeys_free(ptr));
/**
*/
export class CipherKeys {

    static __wrap(ptr) {
        const obj = Object.create(CipherKeys.prototype);
        obj.ptr = ptr;
        CipherKeysFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        CipherKeysFinalization.unregister(this);
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

const ECDHFinalization = new FinalizationRegistry(ptr => wasm.__wbg_ecdh_free(ptr));
/**
*/
export class ECDH {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ECDHFinalization.unregister(this);
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ECDSAFinalization = new FinalizationRegistry(ptr => wasm.__wbg_ecdsa_free(ptr));
/**
*
* * Utility struct for low level ECDSA primitives
*
*/
export class ECDSA {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ECDSAFinalization.unregister(this);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(pub_key, PublicKey);
            _assertClass(signature, Signature);
            wasm.ecdsa_verify(retptr, ptr0, len0, pub_key.ptr, signature.ptr, hash_algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Signature} signature
    * @param {PublicKey} public_key
    * @param {PrivateKey} ephemeral_key
    * @param {Uint8Array} preimage
    * @param {number} hash_algo
    * @returns {PrivateKey}
    */
    static privateKeyFromSignatureK(signature, public_key, ephemeral_key, preimage, hash_algo) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(signature, Signature);
            _assertClass(public_key, PublicKey);
            _assertClass(ephemeral_key, PrivateKey);
            const ptr0 = passArray8ToWasm0(preimage, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.ecdsa_privateKeyFromSignatureK(retptr, signature.ptr, public_key.ptr, ephemeral_key.ptr, ptr0, len0, hash_algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PrivateKey} private_key
    * @param {Uint8Array} preimage
    * @param {number} hash_algo
    * @param {boolean} reverse_k
    * @returns {Signature}
    */
    static signWithRandomK(private_key, preimage, hash_algo, reverse_k) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(private_key, PrivateKey);
            const ptr0 = passArray8ToWasm0(preimage, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.ecdsa_signWithRandomK(retptr, private_key.ptr, ptr0, len0, hash_algo, reverse_k);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PrivateKey} private_key
    * @param {Uint8Array} preimage
    * @param {number} hash_algo
    * @param {boolean} reverse_k
    * @returns {Signature}
    */
    static sign(private_key, preimage, hash_algo, reverse_k) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(private_key, PrivateKey);
            const ptr0 = passArray8ToWasm0(preimage, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.ecdsa_sign(retptr, private_key.ptr, ptr0, len0, hash_algo, reverse_k);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PrivateKey} private_key
    * @param {PrivateKey} ephemeral_key
    * @param {Uint8Array} preimage
    * @param {number} hash_algo
    * @returns {Signature}
    */
    static signWithK(private_key, ephemeral_key, preimage, hash_algo) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(private_key, PrivateKey);
            _assertClass(ephemeral_key, PrivateKey);
            const ptr0 = passArray8ToWasm0(preimage, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.ecdsa_signWithK(retptr, private_key.ptr, ephemeral_key.ptr, ptr0, len0, hash_algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ECIESFinalization = new FinalizationRegistry(ptr => wasm.__wbg_ecies_free(ptr));
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
        ECIESFinalization.unregister(this);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(sender_priv_key, PrivateKey);
            _assertClass(recipient_pub_key, PublicKey);
            wasm.ecies_encrypt(retptr, ptr0, len0, sender_priv_key.ptr, recipient_pub_key.ptr, exclude_pub_key);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ECIESCiphertext.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(recipient_pub_key, PublicKey);
            wasm.ecies_encryptWithEphemeralKey(retptr, ptr0, len0, recipient_pub_key.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ECIESCiphertext.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(priv_key, PrivateKey);
            _assertClass(pub_key, PublicKey);
            wasm.ecies_deriveCipherKeys(retptr, priv_key.ptr, pub_key.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return CipherKeys.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ECIESCiphertextFinalization = new FinalizationRegistry(ptr => wasm.__wbg_eciesciphertext_free(ptr));
/**
*/
export class ECIESCiphertext {

    static __wrap(ptr) {
        const obj = Object.create(ECIESCiphertext.prototype);
        obj.ptr = ptr;
        ECIESCiphertextFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ECIESCiphertextFinalization.unregister(this);
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
            wasm.eciesciphertext_getCiphertext(retptr, this.ptr);
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
            wasm.eciesciphertext_getHMAC(retptr, this.ptr);
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
        const ret = wasm.eciesciphertext_getCipherKeys(this.ptr);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.eciesciphertext_extractPublicKey(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} buffer
    * @param {boolean} has_pub_key
    * @returns {ECIESCiphertext}
    */
    static fromBytes(buffer, has_pub_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.eciesciphertext_fromBytes(retptr, ptr0, len0, has_pub_key);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ECIESCiphertext.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ExtendedPrivateKeyFinalization = new FinalizationRegistry(ptr => wasm.__wbg_extendedprivatekey_free(ptr));
/**
*/
export class ExtendedPrivateKey {

    static __wrap(ptr) {
        const obj = Object.create(ExtendedPrivateKey.prototype);
        obj.ptr = ptr;
        ExtendedPrivateKeyFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ExtendedPrivateKeyFinalization.unregister(this);
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
        const ret = wasm.extendedprivatekey_getPrivateKey(this.ptr);
        return PrivateKey.__wrap(ret);
    }
    /**
    * @returns {PublicKey}
    */
    getPublicKey() {
        const ret = wasm.extendedprivatekey_getPublicKey(this.ptr);
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
        const ret = wasm.extendedprivatekey_getDepth(this.ptr);
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
        const ret = wasm.extendedprivatekey_getIndex(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} index
    * @returns {ExtendedPrivateKey}
    */
    deriveChild(index) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedprivatekey_deriveChild(retptr, this.ptr, index);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} path
    * @returns {ExtendedPrivateKey}
    */
    derive(path) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.extendedprivatekey_derive(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} seed
    * @returns {ExtendedPrivateKey}
    */
    static fromSeed(seed) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(seed, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.extendedprivatekey_fromSeed(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {ExtendedPrivateKey}
    */
    static fromRandom() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedprivatekey_fromRandom(retptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} xprv_string
    * @returns {ExtendedPrivateKey}
    */
    static fromString(xprv_string) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(xprv_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.extendedprivatekey_fromString(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * @param {Uint8Array} mnemonic
    * @param {Uint8Array | undefined} passphrase
    * @returns {ExtendedPrivateKey}
    */
    static fromMnemonic(mnemonic, passphrase) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(mnemonic, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(passphrase) ? 0 : passArray8ToWasm0(passphrase, wasm.__wbindgen_malloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.extendedprivatekey_fromMnemonic(retptr, ptr0, len0, ptr1, len1);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ExtendedPublicKeyFinalization = new FinalizationRegistry(ptr => wasm.__wbg_extendedpublickey_free(ptr));
/**
*/
export class ExtendedPublicKey {

    static __wrap(ptr) {
        const obj = Object.create(ExtendedPublicKey.prototype);
        obj.ptr = ptr;
        ExtendedPublicKeyFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ExtendedPublicKeyFinalization.unregister(this);
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
        const ret = wasm.extendedpublickey_getPublicKey(this.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @param {ExtendedPrivateKey} xpriv
    * @returns {ExtendedPublicKey}
    */
    static fromXPriv(xpriv) {
        _assertClass(xpriv, ExtendedPrivateKey);
        const ret = wasm.extendedpublickey_fromXPriv(xpriv.ptr);
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
        const ret = wasm.extendedpublickey_getDepth(this.ptr);
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
        const ret = wasm.extendedpublickey_getIndex(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} index
    * @returns {ExtendedPublicKey}
    */
    deriveChild(index) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedpublickey_deriveChild(retptr, this.ptr, index);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} path
    * @returns {ExtendedPublicKey}
    */
    derive(path) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.extendedpublickey_derive(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} seed
    * @returns {ExtendedPublicKey}
    */
    static fromSeed(seed) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(seed, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.extendedpublickey_fromSeed(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {ExtendedPublicKey}
    */
    static fromRandom() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.extendedpublickey_fromRandom(retptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} xpub_string
    * @returns {ExtendedPublicKey}
    */
    static fromString(xpub_string) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(xpub_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.extendedpublickey_fromString(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ExtendedPublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
}

const HashFinalization = new FinalizationRegistry(ptr => wasm.__wbg_hash_free(ptr));
/**
*/
export class Hash {

    static __wrap(ptr) {
        const obj = Object.create(Hash.prototype);
        obj.ptr = ptr;
        HashFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        HashFinalization.unregister(this);
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
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha256d(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static sha256(input) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha256(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static sha1(input) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha1(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static ripemd160(input) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hash_ripemd160(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static hash160(input) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hash_hash160(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @returns {Hash}
    */
    static sha512(input) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha512(ptr0, len0);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha512Hmac(input, key) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha512Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha256Hmac(input, key) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha256Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha256dHmac(input, key) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha256dHmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static sha1Hmac(input, key) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.hash_sha1Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static ripemd160Hmac(input, key) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.hash_ripemd160Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
    /**
    * @param {Uint8Array} input
    * @param {Uint8Array} key
    * @returns {Hash}
    */
    static hash160Hmac(input, key) {
        const ptr0 = passArray8ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.hash_hash160Hmac(ptr0, len0, ptr1, len1);
        return Hash.__wrap(ret);
    }
}

const KDFFinalization = new FinalizationRegistry(ptr => wasm.__wbg_kdf_free(ptr));
/**
*/
export class KDF {

    static __wrap(ptr) {
        const obj = Object.create(KDF.prototype);
        obj.ptr = ptr;
        KDFFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        KDFFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_kdf_free(ptr);
    }
    /**
    * @returns {Hash}
    */
    getHash() {
        const ret = wasm.kdf_getHash(this.ptr);
        return Hash.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    getSalt() {
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
        const ptr0 = passArray8ToWasm0(password, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(salt) ? 0 : passArray8ToWasm0(salt, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.kdf_pbkdf2(ptr0, len0, ptr1, len1, hash_algo, rounds, output_length);
        return KDF.__wrap(ret);
    }
}

const MatchCriteriaFinalization = new FinalizationRegistry(ptr => wasm.__wbg_matchcriteria_free(ptr));
/**
*/
export class MatchCriteria {

    static __wrap(ptr) {
        const obj = Object.create(MatchCriteria.prototype);
        obj.ptr = ptr;
        MatchCriteriaFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        MatchCriteriaFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_matchcriteria_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.matchcriteria_new();
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {ScriptTemplate} script_template
    * @returns {MatchCriteria}
    */
    setScriptTemplate(script_template) {
        _assertClass(script_template, ScriptTemplate);
        const ret = wasm.matchcriteria_setScriptTemplate(this.ptr, script_template.ptr);
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {bigint} value
    * @returns {MatchCriteria}
    */
    setValue(value) {
        // Instruction::I32Split64
        // Mask the low 32 bytes
        u32CvtShim[0] = Number(value & 0xffffffffn);
        // Offset the high 32 bytes
        u32CvtShim[1] = Number(value >> 32n);
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.matchcriteria_setValue(this.ptr, low0, high0);
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {bigint} min
    * @returns {MatchCriteria}
    */
    setMin(min) {
        // Instruction::I32Split64
        // Mask the low 32 bytes
        u32CvtShim[0] = Number(min & 0xffffffffn);
        // Offset the high 32 bytes
        u32CvtShim[1] = Number(min >> 32n);
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.matchcriteria_setMin(this.ptr, low0, high0);
        return MatchCriteria.__wrap(ret);
    }
    /**
    * @param {bigint} max
    * @returns {MatchCriteria}
    */
    setMax(max) {
        // Instruction::I32Split64
        // Mask the low 32 bytes
        u32CvtShim[0] = Number(max & 0xffffffffn);
        // Offset the high 32 bytes
        u32CvtShim[1] = Number(max >> 32n);
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.matchcriteria_setMax(this.ptr, low0, high0);
        return MatchCriteria.__wrap(ret);
    }
}

const P2PKHAddressFinalization = new FinalizationRegistry(ptr => wasm.__wbg_p2pkhaddress_free(ptr));
/**
*/
export class P2PKHAddress {

    static __wrap(ptr) {
        const obj = Object.create(P2PKHAddress.prototype);
        obj.ptr = ptr;
        P2PKHAddressFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        P2PKHAddressFinalization.unregister(this);
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
        const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        const ret = wasm.p2pkhaddress_isValidBitcoinMessage(this.ptr, ptr0, len0, signature.ptr);
        return ret !== 0;
    }
    /**
    * @param {Uint8Array} hash_bytes
    * @returns {P2PKHAddress}
    */
    static fromPubKeyHash(hash_bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(hash_bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.p2pkhaddress_fromPubKeyHash(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return P2PKHAddress.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PublicKey} pub_key
    * @returns {P2PKHAddress}
    */
    static fromPubKey(pub_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(pub_key, PublicKey);
            wasm.p2pkhaddress_fromPubKey(retptr, pub_key.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return P2PKHAddress.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {ChainParams} chain_params
    * @returns {P2PKHAddress}
    */
    setChainParams(chain_params) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(chain_params, ChainParams);
            wasm.p2pkhaddress_setChainParams(retptr, this.ptr, chain_params.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return P2PKHAddress.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * @param {string} address_string
    * @returns {P2PKHAddress}
    */
    static fromString(address_string) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(address_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.p2pkhaddress_fromString(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return P2PKHAddress.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Script}
    */
    toLockingScript() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.p2pkhaddress_toLockingScript(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Script.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PublicKey} pub_key
    * @param {SighashSignature} sig
    * @returns {Script}
    */
    toUnlockingScript(pub_key, sig) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(pub_key, PublicKey);
            _assertClass(sig, SighashSignature);
            wasm.p2pkhaddress_toUnlockingScript(retptr, this.ptr, pub_key.ptr, sig.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Script.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(signature, Signature);
            wasm.p2pkhaddress_verifyBitcoinMessage(retptr, this.ptr, ptr0, len0, signature.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const PrivateKeyFinalization = new FinalizationRegistry(ptr => wasm.__wbg_privatekey_free(ptr));
/**
*/
export class PrivateKey {

    static __wrap(ptr) {
        const obj = Object.create(PrivateKey.prototype);
        obj.ptr = ptr;
        PrivateKeyFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        PrivateKeyFinalization.unregister(this);
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
        const ret = wasm.privatekey_fromRandom();
        return PrivateKey.__wrap(ret);
    }
    /**
    *
    *     * Finds the Public Key Point.
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
        const ret = wasm.privatekey_compressPublicKey(this.ptr, should_compress);
        return PrivateKey.__wrap(ret);
    }
    /**
    * @param {string} wif_string
    * @returns {PrivateKey}
    */
    static fromWIF(wif_string) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(wif_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.privatekey_fromWIF(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} hex_str
    * @returns {PrivateKey}
    */
    static fromHex(hex_str) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.privatekey_fromHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Standard ECDSA Message Signing using SHA256 as the digestg
    *
    * @param {Uint8Array} msg
    * @returns {Signature}
    */
    signMessage(msg) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(msg, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.privatekey_signMessage(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * @param {Uint8Array} bytes
    * @returns {PrivateKey}
    */
    static fromBytes(bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.privatekey_fromBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PrivateKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {PublicKey}
    */
    toPublicKey() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.privatekey_toPublicKey(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Encrypt a message to the public key of this private key.
    *
    * @param {Uint8Array} message
    * @returns {ECIESCiphertext}
    */
    encryptMessage(message) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.privatekey_encryptMessage(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ECIESCiphertext.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const PublicKeyFinalization = new FinalizationRegistry(ptr => wasm.__wbg_publickey_free(ptr));
/**
*/
export class PublicKey {

    static __wrap(ptr) {
        const obj = Object.create(PublicKey.prototype);
        obj.ptr = ptr;
        PublicKeyFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        PublicKeyFinalization.unregister(this);
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
        const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(signature, Signature);
        const ret = wasm.publickey_isValidMessage(this.ptr, ptr0, len0, signature.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    isCompressed() {
        const ret = wasm.publickey_isCompressed(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {string} hex_str
    * @returns {PublicKey}
    */
    static fromHex(hex_str) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.publickey_fromHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} bytes
    * @returns {PublicKey}
    */
    static fromBytes(bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.publickey_fromBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * @param {PrivateKey} priv_key
    * @returns {PublicKey}
    */
    static fromPrivateKey(priv_key) {
        _assertClass(priv_key, PrivateKey);
        const ret = wasm.publickey_fromPrivateKey(priv_key.ptr);
        return PublicKey.__wrap(ret);
    }
    /**
    * @param {Uint8Array} message
    * @param {Signature} signature
    * @returns {boolean}
    */
    verifyMessage(message, signature) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(signature, Signature);
            wasm.publickey_verifyMessage(retptr, this.ptr, ptr0, len0, signature.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {P2PKHAddress}
    */
    toAddress() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.publickey_toAddress(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return P2PKHAddress.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {PublicKey}
    */
    toCompressed() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.publickey_toCompressed(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {PublicKey}
    */
    toDecompressed() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.publickey_toDecompressed(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} message
    * @param {PrivateKey} sender_private_key
    * @returns {ECIESCiphertext}
    */
    encryptMessage(message, sender_private_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(sender_private_key, PrivateKey);
            wasm.publickey_encryptMessage(retptr, this.ptr, ptr0, len0, sender_private_key.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ECIESCiphertext.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const RecoveryInfoFinalization = new FinalizationRegistry(ptr => wasm.__wbg_recoveryinfo_free(ptr));
/**
*/
export class RecoveryInfo {

    static __wrap(ptr) {
        const obj = Object.create(RecoveryInfo.prototype);
        obj.ptr = ptr;
        RecoveryInfoFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        RecoveryInfoFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_recoveryinfo_free(ptr);
    }
    /**
    * @param {boolean} is_y_odd
    * @param {boolean} is_x_reduced
    * @param {boolean} is_pubkey_compressed
    * @returns {RecoveryInfo}
    */
    static new(is_y_odd, is_x_reduced, is_pubkey_compressed) {
        const ret = wasm.recoveryinfo_new(is_y_odd, is_x_reduced, is_pubkey_compressed);
        return RecoveryInfo.__wrap(ret);
    }
    /**
    * @param {number} recovery_byte
    * @param {boolean} is_pubkey_compressed
    * @returns {RecoveryInfo}
    */
    static from_byte(recovery_byte, is_pubkey_compressed) {
        const ret = wasm.recoveryinfo_from_byte(recovery_byte, is_pubkey_compressed);
        return RecoveryInfo.__wrap(ret);
    }
    /**
    * @returns {RecoveryInfo}
    */
    static default() {
        const ret = wasm.recoveryinfo_default();
        return RecoveryInfo.__wrap(ret);
    }
}

const ScriptFinalization = new FinalizationRegistry(ptr => wasm.__wbg_script_free(ptr));
/**
*/
export class Script {

    static __wrap(ptr) {
        const obj = Object.create(Script.prototype);
        obj.ptr = ptr;
        ScriptFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ScriptFinalization.unregister(this);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(script_template, ScriptTemplate);
            wasm.script_matches(retptr, this.ptr, script_template.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
        const ret = wasm.script_is_match(this.ptr, script_template.ptr);
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
        const ret = wasm.script_getScriptLength(this.ptr);
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
    removeCodeSeparators() {
        wasm.script_removeCodeSeparators(this.ptr);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.script_fromHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Script.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} bytes
    * @returns {Script}
    */
    static fromBytes(bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.script_fromBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Script.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} asm_string
    * @returns {Script}
    */
    static fromASMString(asm_string) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(asm_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.script_fromASMString(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Script.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} data_bytes
    * @returns {Uint8Array}
    */
    static encodePushData(data_bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data_bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.script_encodePushData(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {any}
    */
    toScriptBits() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.script_toScriptBits(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ScriptTemplateFinalization = new FinalizationRegistry(ptr => wasm.__wbg_scripttemplate_free(ptr));
/**
*/
export class ScriptTemplate {

    static __wrap(ptr) {
        const obj = Object.create(ScriptTemplate.prototype);
        obj.ptr = ptr;
        ScriptTemplateFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        ScriptTemplateFinalization.unregister(this);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(script, Script);
            wasm.scripttemplate_from_script(retptr, script.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ScriptTemplate.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} asm
    * @returns {ScriptTemplate}
    */
    static from_asm_string(asm) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(asm, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.scripttemplate_from_asm_string(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return ScriptTemplate.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const SighashSignatureFinalization = new FinalizationRegistry(ptr => wasm.__wbg_sighashsignature_free(ptr));
/**
*/
export class SighashSignature {

    static __wrap(ptr) {
        const obj = Object.create(SighashSignature.prototype);
        obj.ptr = ptr;
        SighashSignatureFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        SighashSignatureFinalization.unregister(this);
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
        const ptr0 = passArray8ToWasm0(sighash_buffer, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sighashsignature_new(signature.ptr, sighash_type, ptr0, len0);
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} bytes
    * @param {Uint8Array} sighash_buffer
    * @returns {SighashSignature}
    */
    static fromBytes(bytes, sighash_buffer) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(sighash_buffer, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            wasm.sighashsignature_fromBytes(retptr, ptr0, len0, ptr1, len1);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return SighashSignature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const SignatureFinalization = new FinalizationRegistry(ptr => wasm.__wbg_signature_free(ptr));
/**
*/
export class Signature {

    static __wrap(ptr) {
        const obj = Object.create(Signature.prototype);
        obj.ptr = ptr;
        SignatureFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        SignatureFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_signature_free(ptr);
    }
    /**
    * DER representation of signature, does not contain any recovery information, so cannot be used for BSM
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
    * DER representation of signature, does not contain any recovery information, so cannot be used for BSM
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_toBytes(retptr, this.ptr);
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
    * NOTE: Provide recovery info if the current signature object doesnt contain it.
    * @param {RecoveryInfo | undefined} recovery_info
    * @returns {Uint8Array}
    */
    toCompactBytes(recovery_info) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            let ptr0 = 0;
            if (!isLikeNone(recovery_info)) {
                _assertClass(recovery_info, RecoveryInfo);
                ptr0 = recovery_info.ptr;
                recovery_info.ptr = 0;
            }
            wasm.signature_toCompactBytes(retptr, this.ptr, ptr0);
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
    * @returns {Uint8Array}
    */
    r() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_r(retptr, this.ptr);
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
    rHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_rHex(retptr, this.ptr);
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
    s() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_s(retptr, this.ptr);
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
    sHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.signature_sHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * NOTE: Provide recovery info if the current signature object doesnt contain it.
    * @param {RecoveryInfo | undefined} recovery_info
    * @returns {string}
    */
    toCompactHex(recovery_info) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            let ptr0 = 0;
            if (!isLikeNone(recovery_info)) {
                _assertClass(recovery_info, RecoveryInfo);
                ptr0 = recovery_info.ptr;
                recovery_info.ptr = 0;
            }
            wasm.signature_toCompactHex(retptr, this.ptr, ptr0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {Uint8Array} message
    * @param {PublicKey} pub_key
    * @returns {boolean}
    */
    verifyMessage(message, pub_key) {
        const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(pub_key, PublicKey);
        const ret = wasm.signature_verifyMessage(this.ptr, ptr0, len0, pub_key.ptr);
        return ret !== 0;
    }
    /**
    * @param {Uint8Array} bytes
    * @returns {Signature}
    */
    static fromDER(bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.signature_fromDER(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} hex
    * @returns {Signature}
    */
    static fromHexDER(hex) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.signature_fromHexDER(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} compact_bytes
    * @returns {Signature}
    */
    static fromCompactBytes(compact_bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(compact_bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.signature_fromCompactBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Signature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} message
    * @param {number} hash_algo
    * @returns {PublicKey}
    */
    recoverPublicKey(message, hash_algo) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(message, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.signature_recoverPublicKey(retptr, this.ptr, ptr0, len0, hash_algo);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return PublicKey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const TransactionFinalization = new FinalizationRegistry(ptr => wasm.__wbg_transaction_free(ptr));
/**
*/
export class Transaction {

    static __wrap(ptr) {
        const obj = Object.create(Transaction.prototype);
        obj.ptr = ptr;
        TransactionFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        TransactionFinalization.unregister(this);
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
        const ret = wasm.transaction_verify(this.ptr, pub_key.ptr, sig.ptr);
        return ret !== 0;
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {number} sighash
    * @param {number} n_tx_in
    * @param {Script} unsigned_script
    * @param {bigint} value
    * @returns {SighashSignature}
    */
    sign(priv_key, sighash, n_tx_in, unsigned_script, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(priv_key, PrivateKey);
            _assertClass(unsigned_script, Script);
            // Instruction::I32Split64
            // Mask the low 32 bytes
            u32CvtShim[0] = Number(value & 0xffffffffn);
            // Offset the high 32 bytes
            u32CvtShim[1] = Number(value >> 32n);
            const low0 = u32CvtShim[0];
            const high0 = u32CvtShim[1];
            wasm.transaction_sign(retptr, this.ptr, priv_key.ptr, sighash, n_tx_in, unsigned_script.ptr, low0, high0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return SighashSignature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {PrivateKey} priv_key
    * @param {PrivateKey} ephemeral_key
    * @param {number} sighash
    * @param {number} n_tx_in
    * @param {Script} unsigned_script
    * @param {bigint} value
    * @returns {SighashSignature}
    */
    signWithK(priv_key, ephemeral_key, sighash, n_tx_in, unsigned_script, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(priv_key, PrivateKey);
            _assertClass(ephemeral_key, PrivateKey);
            _assertClass(unsigned_script, Script);
            // Instruction::I32Split64
            // Mask the low 32 bytes
            u32CvtShim[0] = Number(value & 0xffffffffn);
            // Offset the high 32 bytes
            u32CvtShim[1] = Number(value >> 32n);
            const low0 = u32CvtShim[0];
            const high0 = u32CvtShim[1];
            wasm.transaction_signWithK(retptr, this.ptr, priv_key.ptr, ephemeral_key.ptr, sighash, n_tx_in, unsigned_script.ptr, low0, high0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return SighashSignature.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number} sighash
    * @param {number} n_tx_in
    * @param {Script} unsigned_script
    * @param {bigint} value
    * @returns {Uint8Array}
    */
    sighashPreimage(sighash, n_tx_in, unsigned_script, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(unsigned_script, Script);
            // Instruction::I32Split64
            // Mask the low 32 bytes
            u32CvtShim[0] = Number(value & 0xffffffffn);
            // Offset the high 32 bytes
            u32CvtShim[1] = Number(value >> 32n);
            const low0 = u32CvtShim[0];
            const high0 = u32CvtShim[1];
            wasm.transaction_sighashPreimage(retptr, this.ptr, sighash, n_tx_in, unsigned_script.ptr, low0, high0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
        const ret = wasm.transaction_getVersion(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    getInputsCount() {
        const ret = wasm.transaction_getInputsCount(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    getOutputsCount() {
        const ret = wasm.transaction_getOutputsCount(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} index
    * @returns {TxIn | undefined}
    */
    getInput(index) {
        const ret = wasm.transaction_getInput(this.ptr, index);
        return ret === 0 ? undefined : TxIn.__wrap(ret);
    }
    /**
    * @param {number} index
    * @returns {TxOut | undefined}
    */
    getOutput(index) {
        const ret = wasm.transaction_getOutput(this.ptr, index);
        return ret === 0 ? undefined : TxOut.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    getNLocktime() {
        const ret = wasm.transaction_getNLocktime(this.ptr);
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
        const ret = wasm.transaction_new(version, n_locktime);
        return Transaction.__wrap(ret);
    }
    /**
    * @returns {Transaction}
    */
    static default() {
        const ret = wasm.transaction_default();
        return Transaction.__wrap(ret);
    }
    /**
    * @param {number} version
    * @returns {Transaction}
    */
    setVersion(version) {
        const ret = wasm.transaction_setVersion(this.ptr, version);
        return Transaction.__wrap(ret);
    }
    /**
    * @param {number} n_locktime
    * @returns {Transaction}
    */
    setNLocktime(n_locktime) {
        const ret = wasm.transaction_setNLocktime(this.ptr, n_locktime);
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
    * @returns {boolean}
    */
    is_coinbase_impl() {
        const ret = wasm.transaction_isCoinbase(this.ptr);
        return ret !== 0;
    }
    /**
    *
    *     * XT Method:
    *     * Returns the combined sum of all input satoshis.
    *     * If any of the inputs dont have satoshis defined, this returns None or null
    *
    * @returns {bigint | undefined}
    */
    satoshisIn() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_satoshisIn(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            // Instruction::Option64FromI32
            u32CvtShim[0] = r1;
            u32CvtShim[1] = r2;
            const n0 = r0 === 0 ? undefined : (BigInt(u32CvtShim[1]) << 32n) | BigInt(u32CvtShim[0]);
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Returns the combined sum of all output satoshis.
    *
    * @returns {bigint}
    */
    satoshisOut() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_satoshisOut(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            // Instruction::I64FromLoHi
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = (BigInt(u32CvtShim[1]) << 32n) | BigInt(u32CvtShim[0]);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.transaction_fromHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Transaction.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} tx_bytes
    * @returns {Transaction}
    */
    static fromBytes(tx_bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(tx_bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.transaction_fromBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Transaction.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * @param {string} json_string
    * @returns {Transaction}
    */
    static fromJsonString(json_string) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(json_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.transaction_fromJsonString(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Transaction.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toJSON(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    *
    *     * Get size of current serialised Transaction object
    *
    * @returns {number}
    */
    getSize() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_getSize(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Adds an array of TxIn's to the transaction
    *     * @param {TxIn[]} tx_ins
    *
    * @param {any[]} tx_ins
    */
    addInputs(tx_ins) {
        const ptr0 = passArrayJsValueToWasm0(tx_ins, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_getOutpoints(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Adds an array of TxOuts to the transaction
    *     * @param {TxOut[]} tx_outs
    *
    * @param {any[]} tx_outs
    */
    addOutputs(tx_outs) {
        const ptr0 = passArrayJsValueToWasm0(tx_outs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
    toCompactHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toCompactHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(compact_buffer, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.transaction_fromCompactBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Transaction.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Deserialises the provided CBOR buffer to the TX+ format
    *
    * @param {string} compact_hex
    * @returns {Transaction}
    */
    static fromCompactHex(compact_hex) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(compact_hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.transaction_fromCompactHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Transaction.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {boolean}
    */
    isCoinbase() {
        const ret = wasm.transaction_isCoinbase(this.ptr);
        return ret !== 0;
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

const TxInFinalization = new FinalizationRegistry(ptr => wasm.__wbg_txin_free(ptr));
/**
*/
export class TxIn {

    static __wrap(ptr) {
        const obj = Object.create(TxIn.prototype);
        obj.ptr = ptr;
        TxInFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        TxInFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_txin_free(ptr);
    }
    /**
    * @param {Uint8Array} prev_tx_id
    * @param {number} vout
    * @param {Script} unlocking_script
    * @param {number | undefined} sequence
    */
    constructor(prev_tx_id, vout, unlocking_script, sequence) {
        const ptr0 = passArray8ToWasm0(prev_tx_id, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(unlocking_script, Script);
        const ret = wasm.txin_new(ptr0, len0, vout, unlocking_script.ptr, !isLikeNone(sequence), isLikeNone(sequence) ? 0 : sequence);
        return TxIn.__wrap(ret);
    }
    /**
    * @returns {TxIn}
    */
    static default() {
        const ret = wasm.txin_default();
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
        const ret = wasm.txin_getVOut(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {bigint}
    */
    getScriptSigSize() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getScriptSigSize(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            // Instruction::I64FromLoHi
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = (BigInt(u32CvtShim[1]) << 32n) | BigInt(u32CvtShim[0]);
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Script}
    */
    getScriptSig() {
        const ret = wasm.txin_getScriptSig(this.ptr);
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
        const ret = wasm.txin_getSequence(this.ptr);
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
    setUnlockingScript(script) {
        _assertClass(script, Script);
        wasm.txin_setUnlockingScript(this.ptr, script.ptr);
    }
    /**
    * @param {Uint8Array} txid
    */
    setPrevTxId(txid) {
        const ptr0 = passArray8ToWasm0(txid, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
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
    * @param {bigint} satoshis
    */
    setSatoshis(satoshis) {
        // Instruction::I32Split64
        // Mask the low 32 bytes
        u32CvtShim[0] = Number(satoshis & 0xffffffffn);
        // Offset the high 32 bytes
        u32CvtShim[1] = Number(satoshis >> 32n);
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        wasm.txin_setSatoshis(this.ptr, low0, high0);
    }
    /**
    * @returns {bigint | undefined}
    */
    getSatoshis() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getSatoshis(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            // Instruction::Option64FromI32
            u32CvtShim[0] = r1;
            u32CvtShim[1] = r2;
            const n0 = r0 === 0 ? undefined : (BigInt(u32CvtShim[1]) << 32n) | BigInt(u32CvtShim[0]);
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Script} locking_script
    */
    setLockingScript(locking_script) {
        _assertClass(locking_script, Script);
        wasm.txin_setLockingScript(this.ptr, locking_script.ptr);
    }
    /**
    * @returns {Script | undefined}
    */
    getUnlockingScript() {
        const ret = wasm.txin_getUnlockingScript(this.ptr);
        return ret === 0 ? undefined : Script.__wrap(ret);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.txin_fromHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return TxIn.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_toJSON(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * @param {Uint8Array} outpoint
    * @returns {TxIn}
    */
    static fromOutpointBytes(outpoint) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(outpoint, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.txin_fromOutpointBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return TxIn.__wrap(r0);
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
            wasm.txin_toCompactBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
    toCompactHex() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_toCompactHex(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    *
    *     * Deserialises the provided CBOR buffer to the TX+ format
    *
    * @param {Uint8Array} compact_buffer
    * @returns {TxIn}
    */
    static fromCompactBytes(compact_buffer) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(compact_buffer, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.txin_fromCompactBytes(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return TxIn.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    *
    *     * Deserialises the provided CBOR buffer to the TX+ format
    *
    * @param {string} compact_hex
    * @returns {TxIn}
    */
    static fromCompactHex(compact_hex) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(compact_hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.txin_fromCompactHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return TxIn.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Concatenates ScriptSig and UnlockingScript into a single script.
    * @returns {Script}
    */
    getFinalisedScript() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txin_getFinalisedScript(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Script.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {boolean}
    */
    isCoinbase() {
        const ret = wasm.txin_isCoinbase(this.ptr);
        return ret !== 0;
    }
}

const TxOutFinalization = new FinalizationRegistry(ptr => wasm.__wbg_txout_free(ptr));
/**
*/
export class TxOut {

    static __wrap(ptr) {
        const obj = Object.create(TxOut.prototype);
        obj.ptr = ptr;
        TxOutFinalization.register(obj, obj.ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;
        TxOutFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_txout_free(ptr);
    }
    /**
    * @param {bigint} value
    * @param {Script} script_pub_key
    */
    constructor(value, script_pub_key) {
        // Instruction::I32Split64
        // Mask the low 32 bytes
        u32CvtShim[0] = Number(value & 0xffffffffn);
        // Offset the high 32 bytes
        u32CvtShim[1] = Number(value >> 32n);
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        _assertClass(script_pub_key, Script);
        const ret = wasm.txout_new(low0, high0, script_pub_key.ptr);
        return TxOut.__wrap(ret);
    }
    /**
    * @returns {bigint}
    */
    getSatoshis() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_getSatoshis(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            // Instruction::I64FromLoHi
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = (BigInt(u32CvtShim[1]) << 32n) | BigInt(u32CvtShim[0]);
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
        const ret = wasm.txout_getScriptPubKeySize(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Script}
    */
    getScriptPubKey() {
        const ret = wasm.txout_getScriptPubKey(this.ptr);
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
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.txout_fromHex(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return TxOut.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.txout_toJSON(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
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
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
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

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'bigint';
        return ret;
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_BigInt_73b2c10d8e6eb5a5 = function(arg0, arg1) {
        // Instruction::I64FromLoHi
        u32CvtShim[0] = arg0;
        u32CvtShim[1] = arg1;
        const n0 = BigInt.asIntN(64, (BigInt(u32CvtShim[1]) << 32n) | BigInt(u32CvtShim[0]));
        const ret = BigInt(n0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_BigInt_1a499fbb5f402f4c = function(arg0, arg1) {
        // Instruction::I64FromLoHi
        u32CvtShim[0] = arg0;
        u32CvtShim[1] = arg1;
        const n0 = (BigInt(u32CvtShim[1]) << 32n) | BigInt(u32CvtShim[0]);
        const ret = BigInt(n0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        const ret = getObject(arg0) === null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_BigInt_4365947136b5327c = function(arg0, arg1) {
        const ret = BigInt(getObject(arg1));
        // Instruction::I32Split64
        // Mask the low 32 bytes
        u32CvtShim[0] = Number(ret & 0xffffffffn);
        // Offset the high 32 bytes
        u32CvtShim[1] = Number(ret >> 32n);
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        getInt32Memory0()[arg0 / 4 + 1] = high0;
        getInt32Memory0()[arg0 / 4 + 0] = low0;
    };
    imports.wbg.__wbg_BigInt_6b6f34a01a71ad51 = function(arg0, arg1) {
        const ret = BigInt(getObject(arg1));
        // Instruction::I32Split64
        // Mask the low 32 bytes
        u32CvtShim[0] = Number(ret & 0xffffffffn);
        // Offset the high 32 bytes
        u32CvtShim[1] = Number(ret >> 32n);
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        getInt32Memory0()[arg0 / 4 + 1] = high0;
        getInt32Memory0()[arg0 / 4 + 0] = low0;
    };
    imports.wbg.__wbg_String_7462bcc0fcdbaf7d = function(arg0, arg1) {
        const ret = String(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_get_093fe3cdafaf8976 = function(arg0, arg1) {
        const ret = getObject(arg0)[takeObject(arg1)];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_e93b31d47b90bff6 = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbg_new_693216e109162396 = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_randomFillSync_91e2b39becca6147 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_b14734aa289bc356 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_process_e56fd54cf6319b6c = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_77e21455908dad33 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_0dd25d832e4785d5 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_static_accessor_NODE_MODULE_26b231378c1be7dd = function() {
        const ret = module;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_0db1598d9ccecb30 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_crypto_b95d7173266618a9 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_msCrypto_5a86d77a66230f81 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_f0f4f1608ebf633e = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_93debb0e2e184ab6 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_2ab697f1555e0dbc = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_newnoargs_fc5356289219b93b = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_0e1ee6203bc0f8ed = function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_9ef803116340cdc1 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_2a1e30464aae6a4d = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_a495c29471c31da6 = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_6ac6eb1e020f18e3 = function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_89247d3aeaa38cc5 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_4573f605ca4b5f10 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_306ce8d57919e6ae = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_ba1ddafe9ea7a3a2 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_be3cc430364fd32c = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_56d9c9f814daeeee = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_8c35aeee4ac77f2b = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_c1d04f8b45a036e7 = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbg_isArray_628aca8c24017cde = function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_a91000e6b0653ed1 = function(arg0) {
        const ret = getObject(arg0) instanceof ArrayBuffer;
        return ret;
    };
    imports.wbg.__wbg_new_651776e932b7e9c7 = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_isSafeInteger_a8e223ff3885fa1d = function(arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_entries_b24687f151d83be3 = function(arg0) {
        const ret = Object.entries(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_is_aafa609b540ad47f = function(arg0, arg1) {
        const ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_buffer_de1150f91b23aa89 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_97cf52648830a70d = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_a0172b213e2469e9 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_e09c0b925ab8de5d = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Uint8Array_fd17ec67c77de602 = function(arg0) {
        const ret = getObject(arg0) instanceof Uint8Array;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_e833b89f9db02732 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_9482ae5cd5cd99d3 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_has_ded5f0e31f1ff6ad = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.has(getObject(arg0), getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = new Float64Array();
    cachedInt32Memory0 = new Int32Array();
    cachedUint32Memory0 = new Uint32Array();
    cachedUint8Memory0 = new Uint8Array();


    return wasm;
}

function initSync(bytes) {
    const imports = getImports();

    initMemory(imports);

    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('bsv_wasm_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
