# `ec`

A set of utilities for Ethereum consensus.

## Blobs

Facilities for mapping an arbitrary stream of data into blobs and back.

To produce blobs from some data, do something like this:

```bash
$ cat some-data | ec blobs encode > encoded-blobs.json
```

This produces a JSON array of complete blobs on STDOUT.
The blobs are sequenced in the same order to match the incoming data.
If you change the order of the blobs, the recovered data will not be the same.

To bundle the blobs for including into an EIP-4844 transaction (including computing commitments and proofs):

```bash
$ cat encoded-blobs.json | ec blobs bundle > blobs-bundle.json
```

The blobs, commitments, and proofs are required when making a valid 4844 transaction to submit to an execution node.
This utility currently does not support making the 4844 transaction; refer to something like [alloy](https://github.com/alloy-rs/alloy) for this.

To recover some data from a set of blobs (e.g. from the chain), assemble them into a JSON array and provide as input on STDIN:

```bash
$ cat blobs.json | ec blobs decode > some-data
```

Note that the order of the blobs must be maintained as was first produced by `ec blobs encode` if one wishes to recover the same data stream as initially provided.

### Framing

The blob command supports various framing modes to support placing arbitrary data into blobs and being able to recover it.

Supported modes:

* `raw`
* `sized`

The `raw` mode just writes whatever data is provided directly into blobs. Note that given the fixed size of blobs, this could mean padding bytes are added to the end of the stream and there is no way to know from the blob data where the original data ended. There is no (local) limit to the amount of data that can {en,de}coded to/from blobs this way.

The `sized` mode adds a header to the payload data so that this utility can read exactly the originally written number of bytes when decoding.
Refer to the documentation for details of the header and payload encoding.
The `sized` mode gives no other sequencing data so users must take care when ordering blobs if the order is meaningful (e.g. when decoding).
If the target data fits within the maximum number of blobs per block, then a user can simply use this tool (keeping blobs in the same order at each step)
and use the Ethereum protocol nonce and blob index as sequencing data. If the target data exceeds the maximum number of blobs per block, the user will either need to manually place blobs such that
 the blob order respects the (nonce, blob index) order, or devise some other sequencing scheme (e.g. the payload data can include in-band sequencing information).
