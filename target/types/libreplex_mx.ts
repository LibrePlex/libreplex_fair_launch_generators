/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_mx.json`.
 */
export type LibreplexMx = {
  "address": "",
  "metadata": {
    "name": "libreplexMx",
    "version": "0.0.0",
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
          "name": "metaplexJoiner",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  112,
                  108,
                  101,
                  120,
                  95,
                  106,
                  111,
                  105,
                  110,
                  101,
                  114
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
          "name": "fairLaunch",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
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
      "name": "join",
      "discriminator": [
        206,
        55,
        2,
        106,
        113,
        220,
        17,
        163
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "metaplexJoiner",
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
            "metaplexJoiner"
          ]
        },
        {
          "name": "deploymentConfig",
          "writable": true
        },
        {
          "name": "creatorFeeTreasury",
          "writable": true
        },
        {
          "name": "hashlist",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true
        },
        {
          "name": "nonFungibleMetadata",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "const",
                "value": [
                  11,
                  112,
                  101,
                  177,
                  227,
                  209,
                  124,
                  69,
                  56,
                  157,
                  82,
                  127,
                  107,
                  4,
                  195,
                  205,
                  88,
                  184,
                  108,
                  115,
                  26,
                  160,
                  253,
                  181,
                  73,
                  182,
                  209,
                  188,
                  3,
                  248,
                  41,
                  70
                ]
              },
              {
                "kind": "account",
                "path": "nonFungibleMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                11,
                112,
                101,
                177,
                227,
                209,
                124,
                69,
                56,
                157,
                82,
                127,
                107,
                4,
                195,
                205,
                88,
                184,
                108,
                115,
                26,
                160,
                253,
                181,
                73,
                182,
                209,
                188,
                3,
                248,
                41,
                70
              ]
            }
          }
        },
        {
          "name": "hashlistMarker",
          "writable": true
        },
        {
          "name": "fairLaunch",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "deployment",
      "discriminator": [
        66,
        90,
        104,
        89,
        183,
        130,
        64,
        178
      ]
    },
    {
      "name": "deploymentConfig",
      "discriminator": [
        13,
        112,
        57,
        81,
        43,
        26,
        156,
        18
      ]
    },
    {
      "name": "metaplexJoiner",
      "discriminator": [
        33,
        213,
        22,
        119,
        204,
        103,
        121,
        50
      ]
    }
  ],
  "types": [
    {
      "name": "deployment",
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
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "useInscriptions",
            "type": "bool"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requireCreatorCosign",
            "type": "bool"
          },
          {
            "name": "migratedFromLegacy",
            "type": "bool"
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
            "name": "deploymentTemplate",
            "type": "string"
          },
          {
            "name": "mintTemplate",
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
            "name": "disableSwapCosigner",
            "docs": [
              "when cosigner is active this can be toggled to disable swap cosigner",
              "while still requiring cosigner for other actions such as join"
            ],
            "type": "bool"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                199
              ]
            }
          }
        ]
      }
    },
    {
      "name": "deploymentConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "deployment",
            "type": "pubkey"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "pubkey"
          },
          {
            "name": "creatorFeePerMintLamports",
            "type": "u64"
          },
          {
            "name": "transferFeeInBasisPoints",
            "type": "u16"
          },
          {
            "name": "cosignerProgramId",
            "type": "pubkey"
          },
          {
            "name": "multiplierLimits",
            "type": {
              "option": {
                "defined": {
                  "name": "multiplierLimits"
                }
              }
            }
          },
          {
            "name": "transferFeeWithdrawAuthority",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "transferFeeTargetWallet",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "totalSplEquivalentMinted",
            "type": "u64"
          },
          {
            "name": "splExcessInEscrow",
            "type": "u64"
          },
          {
            "name": "allowBurn",
            "docs": [
              "used for variable-rate swaps"
            ],
            "type": "bool"
          },
          {
            "name": "allowClaimTransferFeeAuthAsCreator",
            "type": "bool"
          },
          {
            "name": "uncheckedFungible",
            "type": "bool"
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
          },
          {
            "name": "filterValue",
            "type": "pubkey"
          },
          {
            "name": "filterType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "metaplexJoiner",
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
            "name": "filterValue",
            "type": "pubkey"
          },
          {
            "name": "filterType",
            "docs": [
              "0 - collection",
              "1 - creator"
            ],
            "type": "u8"
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
                100
              ]
            }
          }
        ]
      }
    },
    {
      "name": "metaplexJoinerCreate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "metaplexJoiner",
            "type": {
              "defined": {
                "name": "metaplexJoiner"
              }
            }
          }
        ]
      }
    },
    {
      "name": "multiplierLimits",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxNumerator",
            "type": "u16"
          },
          {
            "name": "minDenominator",
            "type": "u16"
          }
        ]
      }
    }
  ]
};
