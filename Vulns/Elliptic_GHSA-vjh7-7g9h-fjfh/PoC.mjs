// k reuse example
import elliptic from 'elliptic'

const { ec: EC } = elliptic
import crypto from 'crypto'
import Buffer from 'Buffer'

const privateKey = crypto.getRandomValues(new Uint8Array(32))
const curve = 'ed25519' // or any other curve, e.g. secp256k1
const ec = new EC(curve)
const prettyprint = ({ r, s }) => `r: ${r}, s: ${s}`
const sig0 = prettyprint(ec.sign(Buffer.alloc(32, 1), privateKey)) // array of ones
const sig1 = prettyprint(ec.sign('01'.repeat(32), privateKey)) // same message in hex form
const sig2 = prettyprint(ec.sign('-' + '01'.repeat(32), privateKey)) // same `r`, different `s`
console.log({ sig0, sig1, sig2 })