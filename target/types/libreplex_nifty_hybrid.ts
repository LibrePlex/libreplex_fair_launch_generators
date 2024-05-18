/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_nifty_hybrid.json`.
 */
export type LibreplexNiftyHybrid = {
  "address": "N1FTSwV4sLvYzPK1wxkeTV88ycD9m6vUoSVV34wkf7c",
  "metadata": {
    "name": "libreplexNiftyHybrid",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/Libreplex/libreplex-program-library"
  },
  "instructions": [
    {
      "name": "initialise",
      "discriminator": [
        162,
        198,
        118,
        235,
        215,
        247,
        25,
        118
      ],
      "accounts": [
        {
          "name": "deployment",
          "writable": true
        },
        {
          "name": "groupMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "niftyHybrid",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  110,
                  105,
                  102,
                  116,
                  121,
                  95,
                  104,
                  121,
                  98,
                  114,
                  105,
                  100
                ]
              },
              {
                "kind": "arg",
                "path": "input.seed"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "niftyProgram"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "initialiseInput"
            }
          }
        }
      ]
    },
    {
      "name": "mint",
      "discriminator": [
        51,
        57,
        225,
        47,
        182,
        146,
        137,
        166
      ],
      "accounts": [
        {
          "name": "receiver",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "niftyHybrid",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "deployment",
          "writable": true,
          "relations": [
            "niftyHybrid"
          ]
        },
        {
          "name": "hashlist",
          "writable": true
        },
        {
          "name": "swapMarker",
          "writable": true
        },
        {
          "name": "fungibleMintTargetAta",
          "writable": true
        },
        {
          "name": "fungibleMintMinterAta",
          "writable": true
        },
        {
          "name": "fungibleMintSourceAta",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "groupMint",
          "writable": true
        },
        {
          "name": "incomingAssetProgram",
          "writable": true
        },
        {
          "name": "hashlistMarker",
          "writable": true
        },
        {
          "name": "fungibleMint",
          "writable": true
        },
        {
          "name": "fairLaunch",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "niftyProgram"
        },
        {
          "name": "monoswapProgram"
        }
      ],
      "args": []
    },
    {
      "name": "updateMetadata",
      "discriminator": [
        170,
        182,
        43,
        239,
        97,
        78,
        225,
        186
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "niftyHybrid",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true
        },
        {
          "name": "niftyProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "deploymentV2",
      "discriminator": [
        48,
        38,
        175,
        59,
        5,
        208,
        85,
        104
      ]
    },
    {
      "name": "niftyHybrid",
      "discriminator": [
        135,
        82,
        25,
        72,
        188,
        150,
        255,
        28
      ]
    }
  ],
  "events": [
    {
      "name": "mint",
      "discriminator": [
        63,
        11,
        213,
        134,
        148,
        194,
        24,
        203
      ]
    },
    {
      "name": "niftyHybridCreate",
      "discriminator": [
        24,
        46,
        121,
        44,
        111,
        34,
        244,
        9
      ]
    }
  ],
  "types": [
    {
      "name": "deploymentV2",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "limitPerMint",
            "type": "u64"
          },
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          },
          {
            "name": "numberOfTokensIssued",
            "type": "u64"
          },
          {
            "name": "fungibleDecimals",
            "type": "u8"
          },
          {
            "name": "escrowNonFungibleCount",
            "type": "u64"
          },
          {
            "name": "ticker",
            "type": "string"
          },
          {
            "name": "fungibleMint",
            "type": "pubkey"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "proxyProgramId",
            "type": "pubkey"
          },
          {
            "name": "cosignerMint",
            "type": "pubkey"
          },
          {
            "name": "cosignerSwapToNft",
            "type": "pubkey"
          },
          {
            "name": "cosignerSwapToSpl",
            "type": "pubkey"
          },
          {
            "name": "fungibleType",
            "type": {
              "defined": {
                "name": "fungibleType"
              }
            }
          },
          {
            "name": "nonFungibleType",
            "type": {
              "defined": {
                "name": "nonFungibleType"
              }
            }
          },
          {
            "name": "deployed",
            "type": "bool"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                200
              ]
            }
          }
        ]
      }
    },
    {
      "name": "fungibleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "tokenKeg"
          },
          {
            "name": "token2022"
          }
        ]
      }
    },
    {
      "name": "initialiseInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "pubkey"
          },
          {
            "name": "cosigner",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "mint",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "niftyHybrid",
            "type": "pubkey"
          },
          {
            "name": "totalMints",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "niftyHybrid",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "deployment",
            "type": "pubkey"
          },
          {
            "name": "groupMint",
            "type": "pubkey"
          },
          {
            "name": "cosigner",
            "type": "pubkey"
          },
          {
            "name": "cosignerProgramId",
            "type": "pubkey"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                62
              ]
            }
          }
        ]
      }
    },
    {
      "name": "niftyHybridCreate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "niftyHybrid",
            "type": {
              "defined": {
                "name": "niftyHybrid"
              }
            }
          }
        ]
      }
    },
    {
      "name": "nonFungibleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "tokenKeg"
          },
          {
            "name": "token2022"
          },
          {
            "name": "nifty"
          }
        ]
      }
    }
  ]
};
