/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_pipelines.json`.
 */
export type LibreplexPipelines = {
  "address": "Pipe6YuqZmoHeKTpwETFaZEiALNREGfZqCjMbk9P4UG",
  "metadata": {
    "name": "libreplexPipelines",
    "version": "0.0.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/LibrePlex/metadata"
  },
  "instructions": [
    {
      "name": "addLiquidity",
      "discriminator": [
        181,
        157,
        89,
        67,
        143,
        182,
        52,
        72
      ],
      "accounts": [
        {
          "name": "pipeline",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true
        },
        {
          "name": "creatorFeeTreasury",
          "writable": true
        },
        {
          "name": "deploymentConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  101,
                  112,
                  108,
                  111,
                  121,
                  109,
                  101,
                  110,
                  116,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "deployment"
              }
            ],
            "program": {
              "kind": "account",
              "path": "libreplexFairLaunchProgram"
            }
          }
        },
        {
          "name": "hashlist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  104,
                  97,
                  115,
                  104,
                  108,
                  105,
                  115,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "deployment"
              }
            ],
            "program": {
              "kind": "account",
              "path": "libreplexFairLaunchProgram"
            }
          }
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
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "auth",
          "writable": true,
          "signer": true
        },
        {
          "name": "pipelineFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "liquidityProviderEscrowTokenAccount",
          "writable": true
        },
        {
          "name": "liquidityProviderEscrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  105,
                  113,
                  95,
                  112,
                  114,
                  111,
                  118,
                  105,
                  100,
                  101,
                  114,
                  95,
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "path": "liquidityProvider"
              }
            ]
          }
        },
        {
          "name": "liquidityProvider"
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "liquidityFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "liquidityNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "libreplexFairLaunchProgram",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "tokenProgram22"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "libreplexLiquidityProgram",
          "address": "LiquGRWGrp8JKspo8zDDu6qpRmX1p6U3PX2USqiE1eg"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "sysvarInstructions"
        }
      ],
      "args": []
    },
    {
      "name": "claimSplAsLiquidityProvider",
      "discriminator": [
        249,
        186,
        61,
        90,
        23,
        4,
        49,
        170
      ],
      "accounts": [
        {
          "name": "pipeline"
        },
        {
          "name": "liquidity"
        },
        {
          "name": "deployment"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "liquidityProviderEscrowTokenAccount",
          "writable": true
        },
        {
          "name": "fungibleMint"
        },
        {
          "name": "liquidityProviderEscrow",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  105,
                  113,
                  95,
                  112,
                  114,
                  111,
                  118,
                  105,
                  100,
                  101,
                  114,
                  95,
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "path": "recipient"
              }
            ]
          }
        },
        {
          "name": "recipientTokenAccount",
          "writable": true
        },
        {
          "name": "recipient"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": []
    },
    {
      "name": "createSwap",
      "discriminator": [
        176,
        207,
        238,
        60,
        195,
        2,
        203,
        91
      ],
      "accounts": [
        {
          "name": "pipeline",
          "writable": true
        },
        {
          "name": "pipelineSwapMarker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  109,
                  97,
                  114,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "path": "nonFungibleMintIncoming"
              }
            ]
          }
        },
        {
          "name": "monoswapSwapMarker",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true
        },
        {
          "name": "nonFungibleMintIncoming",
          "writable": true
        },
        {
          "name": "fungibleMint",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "auth",
          "writable": true,
          "signer": true
        },
        {
          "name": "payerNonfungibleTokenAccount",
          "writable": true
        },
        {
          "name": "payerFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "pipelineFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "monoswapNonfungibleTokenAccount",
          "writable": true
        },
        {
          "name": "escrowHolder",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  119,
                  97,
                  112,
                  95,
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "pipeline"
              },
              {
                "kind": "account",
                "path": "fungibleMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                5,
                84,
                133,
                123,
                203,
                68,
                213,
                25,
                196,
                238,
                61,
                86,
                213,
                94,
                96,
                140,
                221,
                218,
                10,
                6,
                240,
                19,
                72,
                19,
                138,
                85,
                122,
                171,
                125,
                172,
                250,
                110
              ]
            }
          }
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "tokenProgram22"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "sysvarInstructions"
        },
        {
          "name": "libreplexMonoswapProgram"
        },
        {
          "name": "libreplexPipelinesProgram"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "filterInput"
            }
          }
        }
      ]
    },
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
          "name": "pipeline",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  105,
                  112,
                  101,
                  108,
                  105,
                  110,
                  101
                ]
              },
              {
                "kind": "arg",
                "path": "input.ticker"
              }
            ]
          }
        },
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true
        },
        {
          "name": "deploymentConfig",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "auth",
          "writable": true,
          "signer": true
        },
        {
          "name": "libreplexFairLaunchProgram"
        },
        {
          "name": "libreplexLiquidityProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "initialisePipeline"
            }
          }
        }
      ]
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
      "name": "liquidity",
      "discriminator": [
        54,
        252,
        249,
        226,
        137,
        172,
        121,
        58
      ]
    },
    {
      "name": "pipeline",
      "discriminator": [
        30,
        82,
        16,
        218,
        196,
        77,
        115,
        224
      ]
    },
    {
      "name": "pipelineSwapMarker",
      "discriminator": [
        108,
        57,
        214,
        16,
        155,
        147,
        242,
        119
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "badMint",
      "msg": "Metadata has a bad mint"
    },
    {
      "code": 6001,
      "name": "cannotInscribeFungible",
      "msg": "Cannot inscribe a fungible asset"
    },
    {
      "code": 6002,
      "name": "badAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6003,
      "name": "badAuthorityForHolderInscription",
      "msg": "Bad authority for holder inscription"
    },
    {
      "code": 6004,
      "name": "badAuthorityForUpdateAuthInscription",
      "msg": "Bad authority for update auth inscription"
    },
    {
      "code": 6005,
      "name": "multiSigThresholdMustBeOne",
      "msg": "Multi Signature threshold must be one to create / edit inscriptions"
    },
    {
      "code": 6006,
      "name": "notSquadsMember",
      "msg": "Not squads member"
    },
    {
      "code": 6007,
      "name": "inscription2KeyMismatch",
      "msg": "Inscription V2 key mismatch"
    },
    {
      "code": 6008,
      "name": "inscriptionV3KeyMismatch",
      "msg": "Inscription V3 key mismatch"
    },
    {
      "code": 6009,
      "name": "dataHashMismatch",
      "msg": "Metadata data missmatch"
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
      "name": "filter",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "mcc",
            "fields": [
              {
                "name": "collectionId",
                "type": "pubkey"
              }
            ]
          },
          {
            "name": "firstCreatorId",
            "fields": [
              {
                "name": "firstCreatorId",
                "type": "pubkey"
              }
            ]
          },
          {
            "name": "hashlist",
            "fields": [
              {
                "name": "root",
                "type": {
                  "array": [
                    "u8",
                    32
                  ]
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "filterInput",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "hashlist",
            "fields": [
              {
                "name": "proof",
                "type": {
                  "vec": {
                    "array": [
                      "u8",
                      32
                    ]
                  }
                }
              }
            ]
          },
          {
            "name": "other"
          }
        ]
      }
    },
    {
      "name": "initialisePipeline",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "limitPerMint",
            "type": "u64"
          },
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          },
          {
            "name": "decimals",
            "type": "u8"
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
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "creatorFeeTreasury",
            "type": "pubkey"
          },
          {
            "name": "filter",
            "type": {
              "defined": {
                "name": "filter"
              }
            }
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "liquiditySeed",
            "type": "pubkey"
          },
          {
            "name": "liquidityProviderAmountInLamports",
            "type": "u64"
          },
          {
            "name": "liquidityProviderAmountInSpl",
            "type": "u64"
          },
          {
            "name": "hashlistUrl",
            "type": "string"
          },
          {
            "name": "requireCosigner",
            "type": "bool"
          },
          {
            "name": "transferFeeWithdrawAuthority",
            "type": {
              "option": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "liquidity",
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
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "poolBootstrapped",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "deployment",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "totalMints",
            "type": "u64"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "lookupTableAddress",
            "type": "pubkey"
          },
          {
            "name": "cosignerProgramId",
            "type": "pubkey"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requiredDoubleMints",
            "type": {
              "option": "u32"
            }
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
    },
    {
      "name": "pipeline",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "fairLaunchDeployment",
            "type": "pubkey"
          },
          {
            "name": "liquidity",
            "type": "pubkey"
          },
          {
            "name": "auth",
            "type": "pubkey"
          },
          {
            "name": "processedItemCount",
            "type": "u64"
          },
          {
            "name": "creationTime",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "filter",
            "type": {
              "defined": {
                "name": "filter"
              }
            }
          },
          {
            "name": "liquidityProviderAmountInSpl",
            "type": "u64"
          },
          {
            "name": "fungibleChunkCount",
            "type": "u64"
          },
          {
            "name": "fungibleAmountNet",
            "type": "u64"
          },
          {
            "name": "fungibleAmountTotal",
            "type": "u64"
          },
          {
            "name": "createdSwapCount",
            "type": "u64"
          },
          {
            "name": "authProgramId",
            "type": "pubkey"
          },
          {
            "name": "splSwapAmountPrimary",
            "type": "u64"
          },
          {
            "name": "splSwapAmountSecondary",
            "type": "u64"
          },
          {
            "name": "requireCosigner",
            "type": "bool"
          },
          {
            "name": "hashlistUrl",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "pipelineSwapMarker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pipeline",
            "type": "pubkey"
          },
          {
            "name": "incomingMint",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
};
